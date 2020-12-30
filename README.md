# roche
A cli for rapidly developing tide containers.
Roche is intended to target [knative environments](https://knative.dev/docs/knative-offerings/). But it can also be used to build standard docker containers.

[![Build Status](https://travis-ci.com/roche-rs/roche.svg?branch=main)](https://travis-ci.com/roche-rs/roche)

## introduction
Services built with Rust have some fantastic runtime qualities for serverless applications:

* low resource footprint 

* quick startup time 

* zero garbage collection

However these things come with a trade off as build times are not ideal for rapid application development.

roche addresses this short coming by providing a function as a service pattern for [tide](https://github.com/http-rs/tide) that reduces build times to seconds and enables developers to focus on business logic allowing for the speedy delivery of blazing fast and energy efficient software.

It leverages the [nesting feature of tide](https://github.com/http-rs/tide/blob/main/examples/nested.rs) so all that is required to be developed is a handler while the application infrastructure is provided by prebuilt docker containers.

Once the base images are downloaded build times are around 5s for debug and 30s for Release.

## pre-reqs

1. A docker environment on your local machine.

## install

```
$ cargo install roche
```

## usage

1. make an empty folder and generate a function template
```
$ mkdir tide-faas
$ cd tide-faas
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
That's all you need!
Support for external libs will be added in the future probably through custome base images.


2. Build the function image.
```
$ docker login 
$ roche build
# optionally you can provide an image name
$ roche build -t registry/namespace/devimagename:version
```

3. If you would like to run the image use the standard docker commands
```
docker run -p 8080:8080 registry/namespace/devimagename:version
```

4. For a release build run the following - These take slightly longer as they are compiled with the --release flag
```
$ roche release registry/namespace/imagename:version
```

5. Deploy to your favourite container based FaaS platform.
```
$ docker push registry/namespace/imagename:version
# knative
$ kn service create roche-function --image registry/namespace/imagename:version
# ibmcloud
$ ibmcloud ce app create -n roche-function --image registry/namespace/imagename:version
```

## notes

If you would like to run the build process as part of a CI/CD chain then the following command will generate a `Dockerfile` to ship in the same folder as function.rs.
```
$ roche gen
```

<div>Icons made by <a href="https://www.flaticon.com/authors/freepik" title="Freepik">Freepik</a> from <a href="https://www.flaticon.com/" title="Flaticon">www.flaticon.com</a></div>