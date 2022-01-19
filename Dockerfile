FROM rust

WORKDIR /src
COPY . .

RUN cargo install --path .

CMD ["learning_actix"]