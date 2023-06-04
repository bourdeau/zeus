FROM rust:1.69.0-buster

ARG USERNAME=app
ARG USER_UID=1000
ARG USER_GID=$USER_UID

RUN set -eux; \
    groupadd --gid $USER_GID $USERNAME; \
    useradd --uid $USER_UID --gid $USER_GID -m $USERNAME --home-dir /app; \
    apt-get update; \
    apt-get install -y --no-install-recommends \
    build-essential \
    protobuf-compiler \ 
    libprotobuf-dev \
    ; \
    rm -rf /var/lib/apt/lists/*

USER app
WORKDIR /app
COPY --chown=app:app . /app

RUN cargo build
RUN cargo build --release --bin athena-cli

CMD ["cargo", "run", "--release", "--bin", "athena"]