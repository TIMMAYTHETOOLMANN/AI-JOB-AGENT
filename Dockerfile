FROM rust:1.73 AS builder
WORKDIR /app
COPY . .
RUN cargo build --release

# Create a Docker-specific config that uses local paths
RUN sed -i 's|C:/OneDrive/Documents/Desktop/FIN RES.pdf|/app/resume.pdf|g' AgentConfig.toml && \
    sed -i 's|C:/OneDrive/Documents/Desktop/FIN CV.pdf|/app/cover_letter.pdf|g' AgentConfig.toml

FROM debian:buster-slim
WORKDIR /app
COPY --from=builder /app/target/release/autonomous_job_agent /usr/local/bin/autonomous_job_agent
COPY --from=builder /app/AgentConfig.toml ./
# You'll need to provide these files during the Docker build or mount them at runtime
ENTRYPOINT ["autonomous_job_agent"]
