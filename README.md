working thru:
 
 [Let's make a simple authentication server in Rust with Warp](https://blog.joco.dev/posts/warp_auth_server_tutorial)
 
 to build a single-file Docker image of this toy server, I followed:
 
 [Building Minimal Docker Containers for Rust Applications](https://blog.semicolonsoftware.de/building-minimal-docker-containers-for-rust-applications/)
 
statically building with musl:
 
 ```shell script
$ alias rust-musl-builder='docker run --rm -it -v "$(pwd)":/home/rust/src ekidd/rust-musl-builder'
$ rust-musl-builder cargo build --release
```

building the docker image using the Dockerfile in the repo:
```shell script
docker build -t warp_auth_server .
```

running the built image:
```shell script
docker run --rm --name warp_auth_server -p 3030:3030 warp_auth_server
```