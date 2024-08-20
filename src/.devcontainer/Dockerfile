FROM mcr.microsoft.com/vscode/devcontainers/base:noble
ENV DEBIAN_FRONTEND=noninteractive
#        clang-format-14 \

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
        && \
    rm -rf /var/lib/apt/lists/*  \
    && apt-get clean

user 1000

RUN rustup default stable && \
    cargo install cargo-generate && \
    cargo install cargo-pio && \
    cargo install espup
RUN export PATH=$PATH:/home/ubuntu/.cargo/bin && \
    espup install

RUN echo "export PATH=$PATH:/home/ubuntu/.cargo/bin" >>/home/ubuntu/.profile && \
    echo ". $HOME/export-esp.sh" >>/home/ubuntu/.profile

user root

RUN echo "ubuntu ALL=(root) NOPASSWD:ALL" >/etc/sudoers.d/ubuntu