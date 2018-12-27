FROM ubuntu:xenial

RUN apt install git

RUN git clone git://git.openssl.org/openssl.git && \
    cd openssl && \
    ./config && \
    make && make install
