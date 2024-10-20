use core::num::NonZero;
use esp_idf_hal::gpio::{InterruptType, PinDriver, Pull};
use esp_idf_hal::task::notification::Notification;
use esp_idf_hal::timer::TimerDriver;
use esp_idf_sys::EspError;
use std::sync::atomic::AtomicBool;
use std::sync::Arc;

pub struct Anemometer<'a, T>
where
    T: esp_idf_hal::gpio::InputPin + esp_idf_hal::gpio::OutputPin,
{
    pin: PinDriver<'a, T, esp_idf_hal::gpio::Input>,
    notifier: Notification,
    timer: TimerDriver<'a>,
    frequency: f32,
    timer_overflow: Arc<AtomicBool>,
}

#[allow(dead_code)]
impl<'a, T> Anemometer<'a, T>
where
    T: esp_idf_hal::gpio::InputPin + esp_idf_hal::gpio::OutputPin,
{
    pub fn new(
        mut pin: PinDriver<'a, T, esp_idf_hal::gpio::Input>,
        mut timer: TimerDriver<'a>,
    ) -> Result<Self, EspError> {
        let timer_overflow = Arc::new(AtomicBool::new(false));
        timer.enable(false)?;
        timer.set_alarm(timer.tick_hz())?;
        timer.set_counter(0)?;

        // Saftey: make sure the `Notification` object is not dropped while the subscription is active
        unsafe {
            let timer_overflow = timer_overflow.clone();
            timer.subscribe(move || {
                timer_overflow.store(true, std::sync::atomic::Ordering::SeqCst);
            })?;
        }

        pin.set_pull(Pull::Down)?;
        pin.set_interrupt_type(InterruptType::AnyEdge)?;

        let notifier = Notification::new();
        unsafe {
            let waker = notifier.notifier();
            pin.subscribe(move || {
                waker.notify(NonZero::new(1).unwrap());
            })?;
        }

        timer.enable_interrupt()?;
        timer.enable_alarm(true)?;
        timer.enable(true)?;
        pin.enable_interrupt()?;

        Ok(Anemometer {
            pin,
            notifier,
            timer,
            frequency: 0.0,
            timer_overflow,
        })
    }

    pub fn wait_for_event(&mut self) -> Result<(), EspError> {
        self.notifier.wait_any();
        self.timer.enable(false)?;

        if !self
            .timer_overflow
            .load(std::sync::atomic::Ordering::SeqCst)
        {
            let counter = self.timer.counter()?;
            self.frequency = self.timer.tick_hz() as f32 / counter as f32;
        } else {
            self.frequency = -0.0;
        }

        self.timer_overflow
            .store(false, std::sync::atomic::Ordering::SeqCst);
        self.timer.enable_alarm(true)?;
        self.timer.set_counter(0)?;
        self.timer.enable_interrupt()?;
        self.timer.enable(true)?;
        self.pin.enable_interrupt()?;
        Ok(())
    }

    pub fn get_frequency(&self) -> f32 {
        self.frequency
    }
}
