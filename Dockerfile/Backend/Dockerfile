FROM rust:1.67

WORKDIR /backend

COPY . .

RUN cargo install --path .

# Database
RUN cargo install diesel_cli

RUN cargo build

RUN cargo build --release

CMD ["cargo", "run", "--release"]