# roche
A cli for rapidly developing [tide](https://github.com/http-rs/tide) in containers.

<div align="center">
  <h4>
    <a href="https://roche-rs.org/cli/index.html#command-line-tool">
      Install
    </a>
    <span> | </span>
    <a href="https://roche-rs.org/cli/index.html#command-line-tool">
      Get Started
    </a>
    <span> | </span>
    <a href="https://roche-rs.org/">
      CLI Docs
    </a>
    <span> | </span>
    <a href="https://github.com/roche-rs/roche/blob/main/CONTRIBUTING.md">
      Contributing
    </a>
    <span> | </span>
    <a href="https://discord.gg/x2gKzst">
      Chat
    </a>
  </h4>
</div>

<div align="center">
  <!-- Crates version -->
  <a href="https://crates.io/crates/roche">
    <img src="https://img.shields.io/crates/v/roche.svg?style=flat-square"
    alt="Crates.io version" />
  </a>
  <!-- Downloads -->
  <a href="https://crates.io/crates/roche">
    <img src="https://img.shields.io/crates/d/roche.svg?style=flat-square"
      alt="Download" />
  </a>
  <!-- docs.rs docs -->
  <a href="https://roche-rs.org/">
    <img src="https://img.shields.io/badge/docs-latest-blue.svg?style=flat-square"
      alt="docs.rs docs" />
  </a>
  <!-- Build status-->
  <a href="https://travis-ci.com/roche-rs/roche/">
    <img src="https://travis-ci.com/roche-rs/roche.svg?branch=main"
      alt="Build Status" />
  </a>
</div>

## introduction

Services built with Rust have some fantastic runtime qualities for serverless applications:

* low resource footprint 

* quick startup time 

* zero garbage collection

However these things come with a trade off as build times are not ideal for rapid application development.

roche addresses this short coming by providing a function as a service pattern for [tide](https://github.com/http-rs/tide) that reduces build times to seconds and enables developers to focus on business logic allowing for the speedy delivery of blazing fast and energy efficient software.

It leverages the [nesting feature of tide](https://github.com/http-rs/tide/blob/main/examples/nested.rs) so all that is required to be developed is a handler while the application infrastructure is provided by prebuilt docker containers.

Once the base images are downloaded build times are around 5s for debug and 30s for Release.

Roche is intended to target [knative environments](https://knative.dev/docs/knative-offerings/) but it can also be used to build standard docker containers. 

See the [Architecture](https://github.com/roche-rs/roche/wiki/Architecture) page for details. 



## pre-reqs

1. A [docker](https://docs.docker.com/get-docker/) or [podman](https://podman.io/getting-started/installation) environment on your local machine.

1. A local [rust](https://rustup.rs/) installation.

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

## contribution

roche is an **OPEN Open Source Project**. This means that:

> Individuals making significant and valuable contributions are given commit-access to the project to contribute as they see fit. This project is more like an open wiki than a standard guarded open source project.

See the [Contribution Guide](CONTRIBUTING.md) for more details.

## attributions

<div>Icons made by <a href="https://www.flaticon.com/authors/freepik" title="Freepik">Freepik</a> from <a href="https://www.flaticon.com/" title="Flaticon">www.flaticon.com</a></div>

Project generation features rely on the excellent [cargo generator project](https://crates.io/crates/cargo-generate)

Insperation from [Appsody](https://github.com/appsody) (end of life)