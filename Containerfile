FROM docker.io/node:23-alpine as web-builder

WORKDIR /opt/app-root

ENV PNPM_HOME="/pnpm"
ENV PATH="$PNPM_HOME:$PATH"
RUN corepack enable
RUN apk add just

COPY web/ web/
COPY Justfile Justfile

RUN just install
RUN just build-frontend

FROM docker.io/rust:1-alpine as rust-builder

WORKDIR /opt/app-root

RUN apk add just musl-dev

COPY server/ server/
COPY Justfile Justfile

RUN just build-backend

FROM docker.io/alpine as final-image

WORKDIR /opt/app-root

COPY --from=web-builder /opt/app-root/web /opt/app-root/web
COPY --from=rust-builder /opt/app-root/server/target/release/server /opt/app-root/server

# Need to update the server to serve the frontend
CMD ["./server"]
