FROM liuchong/rustup:nightly-onbuild

WORKDIR /code-runner
COPY . .

RUN apt-get update && \
	apt-get install -y git build-essential && \
	git clone https://github.com/kangalioo/poise && \
	cargo build --release

CMD ["./target/release/code-runner"]
