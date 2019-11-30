FROM rust:1.31

WORKDIR /usr/src/discord_bot

COPY . .

RUN cargo install --path

CMD ["discord_bot"]
