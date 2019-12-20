FROM scratch

ADD target/x86_64-unknown-linux-musl/release/warp_auth_server /
EXPOSE 3030

CMD ["/warp_auth_server"]