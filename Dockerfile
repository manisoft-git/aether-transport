# Multi-stage evaluation container for Aether-Transport
FROM ubuntu:24.04 AS builder

ENV DEBIAN_FRONTEND=noninteractive
ENV RUSTUP_HOME=/usr/local/rustup
ENV CARGO_HOME=/usr/local/cargo
ENV PATH="/usr/local/cargo/bin:${PATH}"

RUN apt-get update && apt-get install -y --no-install-recommends \
    build-essential \
    curl \
    git \
    clang \
    llvm \
    pkg-config \
    libssl-dev \
    iproute2 \
    iptables \
    python3 \
    python3-pip \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y --default-toolchain stable \
    && rustup component add clippy miri rust-src

WORKDIR /workspace
COPY . .

RUN cargo build --workspace --release

# Runtime evaluation stage
FROM ubuntu:24.04 AS runner

ENV DEBIAN_FRONTEND=noninteractive
RUN apt-get update && apt-get install -y --no-install-recommends \
    iproute2 \
    iptables \
    net-tools \
    iperf3 \
    python3 \
    python3-matplotlib \
    python3-numpy \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app
COPY --from=builder /workspace/target/release/aether-core /usr/local/bin/
COPY --from=builder /workspace/target/release/aether-tui /usr/local/bin/
COPY --from=builder /workspace/scripts /app/scripts
COPY --from=builder /workspace/paper /app/paper

CMD ["/bin/bash"]
