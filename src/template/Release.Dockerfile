FROM BASE_IMAGE as builder
COPY functions.rs /app-build/src
#LIB_RS
#ENV
RUN cargo build --release 
#TEST  
FROM RUNTIME_IMAGE
RUN addgroup -S rocheuser && adduser -S rocheuser -G rocheuser
WORKDIR "/app"
COPY --from=builder --chown=rocheuser /app-build/run.sh /app-build/Cargo.toml /app-build/target/release/roche-service ./
USER rocheuser
ENV PORT 8080
EXPOSE 8080
CMD ["./run.sh"]
