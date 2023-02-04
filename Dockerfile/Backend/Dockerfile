FROM rust:1.67

WORKDIR /backend

COPY . .

RUN cargo install --path .

RUN cargo install cargo-license

RUN cargo build

RUN cargo build --release

CMD ["cargo", "run", "--release"]