# Stage 1: Download PocketBase binary from GitHub releases
FROM alpine:3.19@sha256:6baf43584bcb78f2e5847d1de515f23499913ac9f12bdf834811a3145eb11ca1 AS downloader

ARG POCKETBASE_VERSION=v0.24.3
ARG TARGETOS=linux
ARG TARGETARCH=amd64

RUN apk add --no-cache wget ca-certificates unzip

WORKDIR /tmp

# Download PocketBase binary from official GitHub releases
# Version format: v0.24.3 becomes 0.24.3 in filename
RUN VERSION=${POCKETBASE_VERSION#v} && \
    wget -q "https://github.com/pocketbase/pocketbase/releases/download/${POCKETBASE_VERSION}/pocketbase_${VERSION}_${TARGETOS}_${TARGETARCH}.zip" \
    && unzip -q "pocketbase_${VERSION}_${TARGETOS}_${TARGETARCH}.zip" \
    && chmod +x pocketbase \
    && rm "pocketbase_${VERSION}_${TARGETOS}_${TARGETARCH}.zip"

# Stage 2: Runtime image with PocketBase
FROM alpine:3.19@sha256:6baf43584bcb78f2e5847d1de515f23499913ac9f12bdf834811a3145eb11ca1

# Install ca-certificates and wget for healthcheck
RUN apk add --no-cache ca-certificates wget

# Copy PocketBase binary from downloader stage
COPY --from=downloader /tmp/pocketbase /usr/local/bin/pocketbase
RUN chmod +x /usr/local/bin/pocketbase

# Create pb directories
RUN mkdir -p /pb/pb_data /pb/pb_hooks /pb/pb_migrations

WORKDIR /pb

EXPOSE 8090

CMD ["/usr/local/bin/pocketbase", "serve", "--http=0.0.0.0:8090"]
