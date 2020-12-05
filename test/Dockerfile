FROM number9/roche-baseimage as builder

COPY functions.rs /app-build/src/app/

 RUN cargo build --release

 FROM alpine

 RUN addgroup -S rocheuser && adduser -S rocheuser -G rocheuser

 WORKDIR "/app"

 COPY --from=builder --chown=rocheuser /app-build/run.sh /app-build/Cargo.toml /app-build/target/release/roche-service ./

 USER rocheuser

 ENV PORT 8080

 EXPOSE 8080

 CMD ["./run.sh"]


