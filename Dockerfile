FROM alpine:3.8

WORKDIR /app
COPY target/release/paste_nc /app/paste_nc

RUN ls /app

ENTRYPOINT [ "/app/paste_nc" ]