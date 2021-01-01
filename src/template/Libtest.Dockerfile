FROM TEST_BASE_IMAGE
COPY . /app-build/src/
RUN cargo test --lib 

