
# Use the official Rust image as the base image
FROM rust:latest

# Set the working directory to /app
WORKDIR /app

# Copy the current directory contents into the container at /app
COPY . /app

# Build the Rust application
RUN cargo build --release

# Set the startup command to run the built executable
ENTRYPOINT ["./target/release/webhook-notify"]
