FROM liuchong/rustup:nightly-onbuild

WORKDIR /code-runner
COPY . .

RUN git clone https://github.com/kangalioo/poise

CMD ["cargo", "run"]
