FROM rust:1.77 AS builder

WORKDIR /app

COPY . .

RUN cargo build --release

FROM rust:1.77-slim

WORKDIR /app

COPY --from=builder /app/target/release/simple_table_and_query /usr/local/bin/simple_table_and_query

CMD ["simple_table_and_query"]
