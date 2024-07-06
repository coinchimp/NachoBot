# Use an official Rust image from the Docker Hub
FROM rust:1.74 as builder

# Create a new empty shell project
RUN USER=root cargo new --bin nacho_bot
WORKDIR /nacho_bot

# Copy over your manifests
COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml

# This trick will cache your dependencies
RUN cargo build --release
RUN rm src/*.rs

# Now that the dependencies are built, copy your source tree
COPY ./src ./src

# Build for release
RUN rm ./target/release/deps/nacho_bot*
RUN cargo build --release

# Final base image
FROM debian:bullseye-slim
WORKDIR /root/

# Copy the build artifact from the build stage and remove extra files
COPY --from=builder /nacho_bot/target/release/nacho_bot .

# Install needed packages
RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*

# Ensure the binary is executable
RUN chmod +x ./nacho_bot

# Expose the port the server is listening on
ENV PORT "8080"
ENV RUST_LOG info

# Use environment variables to pass into the application
ENV DISCORD_TOKEN ""

# Command to run the executable
CMD ["./nacho_bot"]

# Expose the port the app runs on
EXPOSE 8080