FROM rust:1.63

WORKDIR /myapp
COPY . .
ADD https://files.grouplens.org/datasets/movielens/ml-10m.zip /myapp/data

CMD ["cargo run"]