FROM rust:1.67

WORKDIR /backend

COPY . .

RUN cargo install --path .

# Database
RUN cargo install diesel_cli

RUN cargo build

RUN cargo build --release

# fossa analyze
RUN curl -H 'Cache-Control: no-cache' https://raw.githubusercontent.com/fossas/fossa-cli/master/install-latest.sh | bash

CMD ["cargo", "run", "--bin", "main", "--release"]