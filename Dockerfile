FROM rustlang/rust:nightly AS builder

ADD . /repo
WORKDIR /repo

RUN cargo build --release

FROM centos
COPY --from=builder /repo/target/release/lunchbox /usr/local/bin/lunchbox
ENTRYPOINT ["/usr/local/bin/lunchbox"]

