# env Example

This demonstrates using an .env file for local dev builds.
N.B. .env files are NOT supported in the release build as enviroment variables should be configured as part of your deployment.

```
$ roche build

$ docker run -p 8080:8080 YOUR_USER/dev-json
tide::log Logger started
    level Info
      Running server on: http://localhost:8080/
tide::server Server listening on http://0.0.0.0:8080

$ curl -s http://localhost:8080/
hello
```