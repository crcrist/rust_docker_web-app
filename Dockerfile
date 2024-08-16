# Use the latest stable Rust image as a parent image
FROM rust:latest AS builder

# Set the working directory in the container
WORKDIR /usr/src/app

# Copy the current directory contents into the container at /usr/src/app
COPY . .

# Build the application
RUN cargo build --release

# Use a very recent base image for the final image
FROM debian:bookworm-slim

# Install necessary dependencies
RUN apt-get update && apt-get install -y libssl3 ca-certificates && rm -rf /var/lib/apt/lists/*

# Copy the binary from builder to this new image
# Note: hello-docker should match the name in your Cargo.toml
COPY --from=builder /usr/src/app/target/release/hello-docker /usr/local/bin/hello-docker

# Expose port 5000
EXPOSE 5000

# Run the binary
CMD ["hello-docker"]