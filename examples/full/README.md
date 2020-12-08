# Service with Test Example

In this directory run the following command:

```
$ roche build-test

No image tag provided.
Going to use: docker.io/number9/dev-full:latest
Sending build context to Docker daemon  3.631kB
Step 1/12 : FROM number9/roche-baseimage-debug as builder
 ---> 603e43542443
Step 2/12 : COPY functions.rs /app-build/src/app/
 ---> Using cache
 ---> 00d2e882c933
Step 3/12 : RUN cargo build
 ---> Using cache
 ---> 7770ec40b6f9
Step 4/12 : FROM alpine
 ---> d6e46aa2470d
Step 5/12 : RUN apk add --no-cache libgcc
 ---> Using cache
 ---> 13dd42fef2e5
Step 6/12 : RUN addgroup -S rocheuser && adduser -S rocheuser -G rocheuser
 ---> Using cache
 ---> 8e52e3654f2d
Step 7/12 : WORKDIR "/app"
 ---> Using cache
 ---> c5290577d0f0
Step 8/12 : COPY --from=builder --chown=rocheuser /app-build/run.sh /app-build/Cargo.toml /app-build/target/debug/roche-service ./
 ---> Using cache
 ---> a5f9490070c4
Step 9/12 : USER rocheuser
 ---> Using cache
 ---> f8065a53411e
Step 10/12 : ENV PORT 8080
 ---> Using cache
 ---> b208b17127a6
Step 11/12 : EXPOSE 8080
 ---> Using cache
 ---> ea77183716e1
Step 12/12 : CMD ["./run.sh"]
 ---> Using cache
 ---> 3c16f946f1e4
Successfully built 3c16f946f1e4
Successfully tagged number9/dev-full:latest
    Finished test [unoptimized + debuginfo] target(s) in 0.23s
     Running target/debug/deps/full-6db207771b810bbf

running 1 test
tide::log Logger started
    level Info
      Running server on: http://localhost:8080/
tide::server Server listening on http://0.0.0.0:8080
tide::log::middleware <-- Request received
    method GET
    path /
surf::middleware::logger::native sending request
    req.id 0
    req.method GET
    req.uri https://httpbin.org/get
surf::middleware::logger::native request completed
    req.id 0
    req.status 200
    elapsed 410.920775ms
tide::log::middleware --> Response sent
    method GET
    path /
    status 200 - OK
    duration 412.615899ms
test tests::it_works ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out

   Doc-tests full

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out

a717dfb97c16
```