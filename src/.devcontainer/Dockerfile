FROM mcr.microsoft.com/vscode/devcontainers/base:noble
ENV DEBIAN_FRONTEND=noninteractive

RUN apt-get update && apt-get install -y --no-install-recommends\
        mc \
        vim \
        gdb \
        build-essential \
        git \
        zip \
        wget \
        cmake \
        python3-venv \
        rustup \
        pkg-config \
        libudev-dev \
        && \
    rm -rf /var/lib/apt/lists/*  \
    && apt-get clean

user 1000

RUN rustup default stable
RUN cargo install cargo-generate
RUN cargo install ldproxy
RUN cargo install cargo-pio
RUN cargo install espup
RUN cargo install espflash
#RUN cargo install cargo-espflash

ENV PATH=$PATH:/home/ubuntu/.cargo/bin
RUN espup install

RUN echo "export PATH=$PATH:/home/ubuntu/.cargo/bin" >>/home/ubuntu/.profile && \
    echo ". $HOME/export-esp.sh" >>/home/ubuntu/.profile

ENV RUSTUP_TOOLCHAIN=esp
RUN rustup override set esp

user root

RUN echo "ubuntu ALL=(root) NOPASSWD:ALL" >/etc/sudoers.d/ubuntu