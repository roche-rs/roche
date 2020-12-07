# JSON Example

```
$ roche build

$ docker run -p 8080:8080 YOUR_USER/dev-json
tide::log Logger started
    level Info
      Running server on: http://localhost:8080/
tide::server Server listening on http://0.0.0.0:8080

$ curl -s http://localhost:8080/animals
{"animals":[{"name":"chashu","type":"cat"},{"name":"nori","type":"cat"}],"meta":{"count":2}}
```