FROM ubuntu:xenial

RUN git clone git://git.openssl.org/openssl.git && \
    cd openssl && \
    ./config && \
    make && make install
