FROM alpine:3.19

# Install ca-certificates and wget for healthcheck
RUN apk add --no-cache ca-certificates wget

# Copy PocketBase binary
COPY pocketbase /usr/local/bin/pocketbase
RUN chmod +x /usr/local/bin/pocketbase

# Create pb directories
RUN mkdir -p /pb/pb_data /pb/pb_hooks /pb/pb_migrations

WORKDIR /pb

EXPOSE 8090

CMD ["/usr/local/bin/pocketbase", "serve", "--http=0.0.0.0:8090"]
