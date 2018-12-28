FROM ubuntu:xenial

RUN apt-get update && apt-get install -y git

RUN git clone git://git.openssl.org/openssl.git && \
    cd openssl && \
    ./config && \
    make && make install
