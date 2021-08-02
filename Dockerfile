FROM liuchong/rustup:nightly-onbuild

WORKDIR /code-runner
COPY . .

CMD ["cargo", "run"]
