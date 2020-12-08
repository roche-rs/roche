# Surf Example

```
$ roche build

$ docker run -p 8080:8080 YOUR_USER/surf-json
tide::log Logger started
    level Info
      Running server on: http://localhost:8080/
tide::server Server listening on http://0.0.0.0:8080

$ curl -s http://localhost:8080/
{
  "args": {}, 
  "headers": {
    "Accept": "*/*", 
    "Accept-Encoding": "deflate, gzip", 
    "Content-Length": "0", 
    "Host": "httpbin.org", 
    "User-Agent": "curl/7.73.0-DEV isahc/0.9.13", 
    "X-Amzn-Trace-Id": "Root=1-5fcebd0b-6f864ba16f1d102e5798dcc9"
  }, 
  "origin": "78.16.190.221", 
  "url": "https://httpbin.org/get"
}
```