FROM docker.io/debian:bookworm-slim

ENV USER="1000"
ENV DATA_FOLDER="/data"
ENV RUST_LOG="info"

# For performance reasons write data to docker volume instead of containers writeable fs layer
VOLUME $DATA_FOLDER

# Create data and temp directories
RUN mkdir -p /data /tmp && touch /data/.keep /tmp/.keep && chown -R $USER:$USER /data /tmp

# Copy the pre-built binary
COPY --chown=$USER:$USER target/debug/schweinehund /schweinehund

EXPOSE 9666
USER $USER

ENTRYPOINT ["/schweinehund"]
