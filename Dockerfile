# Use the official Rust image as the base
FROM rust:slim-bookworm AS builder

# Create a new directory for the application
WORKDIR /app

# Install Diesel CLI
RUN apt-get update && apt-get install -y libpq-dev
RUN cargo install diesel_cli --no-default-features --features postgres

# Copy the Cargo.toml and Cargo.lock files
COPY Cargo.toml Cargo.lock ./

# Create a dummy main.rs to build dependencies
RUN mkdir src
RUN echo "fn main() { println!(\"if you see this, the build failed\") }" > src/main.rs

# Build the dependencies
RUN cargo build --release
RUN rm -f target/release/deps/car-sharing*

# Copy the source code
COPY . .

# Build the actual application
RUN cargo build --release

# Use a newer base image for the final stage
FROM ubuntu:latest

# Install required libraries
RUN apt-get update && apt-get install -y \
    libpq5 libpq-dev curl gnupg

# Install Hurl for API testing
RUN curl -LO https://github.com/Orange-OpenSource/hurl/releases/download/4.3.0/hurl_4.3.0_amd64.deb && \
    apt-get update && apt-get install -y ./hurl_4.3.0_amd64.deb && \
    rm hurl_4.3.0_amd64.deb

# Create a new user for security reasons
RUN useradd -ms /bin/bash appuser

# Create app directory and set permissions
WORKDIR /app
RUN chown -R appuser:appuser /app

# Copy the built binary from the builder stage
COPY --from=builder /app/target/release/car-sharing /usr/local/bin/car-sharing

# Install Cargo in the final stage
RUN apt-get update && apt-get install -y cargo

# Copy Diesel CLI from the builder stage (optional)
# You can comment out this line if you don't need Diesel CLI in the final container
COPY --from=builder /usr/local/cargo/bin/diesel /usr/local/bin/diesel

# Copy source code and set permissions
COPY . .
RUN chown -R appuser:appuser /app

# Copy the scripts and make them executable
COPY scripts /usr/local/bin/scripts
RUN chmod +x /usr/local/bin/scripts/*.sh

# Set environment variables
ENV DATABASE_URL=${DATABASE_URL}
ENV PORT=${PORT}
ENV HOST=${HOST}
ENV RUST_LOG=${RUST_LOG}

# Expose the port the application runs on
EXPOSE ${PORT}

# Switch to the new user
USER appuser

# Run the application
CMD ["car-sharing"]
