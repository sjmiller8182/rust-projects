FROM rust:1.47

WORKDIR /usr/src/process_monitor
COPY . .

RUN cargo install --path .

CMD ["process_monitor"]