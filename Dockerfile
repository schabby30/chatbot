# Use the official Rust image as a base image
FROM rust:latest

# Set the working directory inside the container
WORKDIR /usr/src/app

# Copy the project directory contents into the container
COPY . .

# Build the Rust application for musl target
RUN cargo build --release 

COPY ./arlista.csv ./target/release/

# Expose port 5000 to the outside world
EXPOSE 5000

# Run the Rust application
CMD ["./target/release/chatbot"]
