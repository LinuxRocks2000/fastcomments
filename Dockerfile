FROM rust:1.86-bullseye as build

COPY . .

RUN cargo build --release

FROM debian:bullseye-slim

RUN mkdir static
COPY --from=build /target/release/fastcomments .
COPY --from=build /static static

RUN mkdir /db/

ENV DATABASE_PATH="/db/fastcomments.db"
CMD ["./fastcomments"]