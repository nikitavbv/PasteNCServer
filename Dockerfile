FROM frolvlad/alpine-glibc

WORKDIR /app
COPY target/release/paste_nc /app/paste_nc

ENTRYPOINT [ "/app/paste_nc" ]