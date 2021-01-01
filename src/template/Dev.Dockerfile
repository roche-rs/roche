FROM DEV_BASE_IMAGE as builder
COPY . /app-build/src/
RUN cargo build
FROM RUNTIME_IMAGE
RUN addgroup -S rocheuser && adduser -S rocheuser -G rocheuser
WORKDIR "/app"
COPY --from=builder --chown=rocheuser /app-build/run.sh /app-build/Cargo.toml /app-build/target/debug/roche-service INCLUDE_ENV ./
USER rocheuser
ENV PORT 8080
EXPOSE 8080
CMD ["./run.sh"]
