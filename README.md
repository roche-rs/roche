# roche
A cli for building serverless tide containers.

## introduction
Services built with Rust have some fantastic runtime qualities for serverless applications:

* low resource footprint 

* quick startup time 

* zero garbage collection

However these things come with a trade off as build times are not ideal for rapid application development.

roche addresses this short coming by providing a function as a service pattern for [tide](https://github.com/http-rs/tide) that reduces build times to seconds and enables developers to focus on business logic allowing for the speedy delivery of blazing fast and energy efficient software.
It leverages the [nesting feature of tide](https://github.com/http-rs/tide/blob/main/examples/nested.rs) so all that is required to be developed is a handler while the application infrastructure is provided by prebuilt docker containers.

Once the base image is downloaded build times are around 5s for debug and 30s for Release.

## pre-reqs

1. A bash terminal 

2. A docker environment on your local machine.

## install

roche is currently a bash script while we work out some of the ergonomics.
To install just copy the roche file to a location in your $PATH.

## usage

1. In an empty folder generate a function template
```
$ roche init
```
This creates a single function file that you can add functionality into.
```rust
pub fn handler() -> tide::Server<()> {    
    let mut api = tide::new();
    api.at("/").get(|_| async { Ok("Hello, World!") });
    api
}
```
2. Build the function image and give it a docker formated id.
```
$ roche build registry/namespace/imagename:version
```

3. If you would like to run the image use the standard docker commands
```
docker run -p 8080:8080 registry/namespace/imagename:version
```

4. Deploy to your favourite container based FaaS platform.
```
# knative
$ kn service create roche-function --image registry/namespace/imagename:version
# ibmcloud
$ ibmcloud ce app 
```