FROM rust
WORKDIR /app
COPY Cargo.* ./
RUN mkdir src
RUN touch src/lib.rs
RUN cargo build
RUN rm -rf src
COPY src/ src/
RUN cargo build
CMD ["target/debug/esb"]