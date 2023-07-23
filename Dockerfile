# Use Alpine Linux as base image
FROM alpine:latest

# Copy binary into the image
COPY target/release/telegram-llm-assistant /usr/local/bin/

# Set execute permissions for the binary
RUN chmod +x /usr/local/bin/telegram-llm-assistant

# Command to run the binary when the container starts
CMD ["telegram-llm-assistant"]
