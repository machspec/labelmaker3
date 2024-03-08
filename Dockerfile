# --- Build Rust Backend ---
FROM rust:latest as build-backend

# Install system dependencies
RUN apt update && \
    apt install -y build-essential python3 python3-pip && \
    rm -rf /var/lib/apt/lists/*

# Install Python dependencies
RUN pip3 install reportlab pylabels --break-system-packages

# Set the working directory
WORKDIR /app/backend
COPY backend/ ./

# Set environment variables
ENV ROCKET_ADDRESS=0.0.0.0

# Build the backend
RUN cargo build --release

# --- Build Svelte Frontend ---
FROM node:latest as build-frontend

WORKDIR /app/frontend
COPY frontend/ ./
RUN npm install

# Build the frontend
RUN npm run build

# --- Final Stage ---
FROM debian:bookworm-slim

# Copy the backend and frontend to the final stage
WORKDIR /app
COPY --from=build-backend /app/backend/target/release /app/backend
COPY ./backend/config.ini /app
COPY --from=build-frontend /app/frontend/dist /app/frontend/dist

# Install library for libssl.so.3, libpython3
RUN apt-get update && \
    apt install -y libpython3-dev openssl curl && \
    rm -rf /var/lib/apt/lists/*

# Expose the port your Rocket application listens on (default is 8000)
EXPOSE 8000

HEALTHCHECK --interval=60s --retries=5 CMD curl --fail http://localhost:8000 || exit 1

# Run the application
CMD ["./backend/backend"]