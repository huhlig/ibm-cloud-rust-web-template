FROM rust

WORKDIR /usr/src/ibm-cloud-rust-web-template
COPY . .

RUN cargo install

CMD ["ibm-cloud-rust-web-template"]