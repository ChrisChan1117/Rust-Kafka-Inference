FROM rust:latest AS build  

# View app name in Cargo.toml  
ARG APP_NAME=data_service  

WORKDIR /build  

COPY Cargo.lock Cargo.toml ./  
RUN mkdir src \
    && echo "// dummy file" > src/lib.rs \
    && cargo build --release  

COPY src src  
COPY .env ./
RUN cargo build --locked --release  
RUN cp ./target/release/$APP_NAME /bin/server  

FROM debian:bookworm-slim AS final  

RUN adduser \
    --disabled-password \
    --gecos "" \
    --home "/nonexistent" \
    --shell "/sbin/nologin" \
    --no-create-home \
    --uid "10001" \
    appuser  
USER appuser  

COPY --from=build /bin/server /bin/  

CMD ["/bin/server"]  