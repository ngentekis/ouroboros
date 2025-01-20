FROM rust:latest

COPY . /usr/app
WORKDIR /usr/app

RUN cargo install --path .

EXPOSE 8000

CMD ["ouroboros"]
