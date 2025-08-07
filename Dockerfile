FROM rust:1.76

WORKDIR /usr/src/app
COPY . .

RUN apt-get update && apt-get install -y chromium-driver
RUN cargo build --release

CMD ["./target/release/autonomous_job_agent"]
