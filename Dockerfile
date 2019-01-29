FROM alpine:3.8

WORKDIR /app
COPY target/release/paste_nc paste_nc

ENTRYPOINT [ "./paste_nc" ]