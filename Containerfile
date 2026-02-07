# # # # # # # # # # # # # # # # # # # #
# Builder
# # # # # # # # # # # # # # # # # # # # # # # # # # # # # # # # # # # # # # # # # # # # # # # # # # # # # # # # # # # #
FROM docker.io/alpine AS builder

# Create an empty directory that will be used in the final image
RUN mkdir "/empty_dir"

# Install required packages for the staging script
RUN apk update && apk add --no-cache bash file

# Copy all archs into this container
RUN mkdir /work
WORKDIR /work
COPY target .
COPY .container/stage-arch-bin.sh /work

# This will copy the cpu arch corresponding binary to /target/schweinehund
RUN bash stage-arch-bin.sh schweinehund

# # # # # # # # # # # # # # # # # # # #
# Run image
# # # # # # # # # # # # # # # # # # # # # # # # # # # # # # # # # # # # # # # # # # # # # # # # # # # # # # # # # # # #
FROM scratch

ENV USER="1337"
ENV DATA_FOLDER="/data"
ENV RUST_LOG="info"

# For performance reasons write data to docker volume instead of containers writeable fs layer
VOLUME $DATA_FOLDER

# Copy the empty directory as data and temp folder
COPY --chown=$USER:$USER --from=builder /empty_dir $DATA_FOLDER
COPY --chown=$USER:$USER --from=builder /empty_dir /tmp

# Copy the built application from the build image to the run-image
COPY --chown=$USER:$USER --from=builder /work/schweinehund /schweinehund

EXPOSE 9666
USER $USER

ENTRYPOINT ["/schweinehund"]
