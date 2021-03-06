FROM rustlang/rust:nightly AS builder

ADD Cargo.toml /repo/
ADD src /repo/src
WORKDIR /repo

RUN cargo build --release

FROM node AS js-builder

ADD ui /home/node/app
RUN cd /home/node/app && npm install . && npm run-script build

FROM centos
COPY --from=builder /repo/target/release/lunchbox /usr/local/bin/lunchbox
COPY --from=js-builder /home/node/app/public /static
EXPOSE 80
ENTRYPOINT ["/usr/local/bin/lunchbox"]

