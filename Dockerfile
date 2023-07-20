# Use Alpine Linux as base image
FROM alpine:latest

# Copy binary into the image
COPY target/release/helpful-assistant /usr/local/bin/

# Set execute permissions for the binary
RUN chmod +x /usr/local/bin/helpful-assistant

# Command to run the binary when the container starts
CMD ["helpful-assistant"]
