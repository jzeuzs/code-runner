FROM liuchong/rustup:nightly-onbuild

WORKDIR /code-runner
COPY . .

ENV TINI_VERSION v0.19.0
ADD https://github.com/krallin/tini/releases/download/${TINI_VERSION}/tini /tini

RUN apt-get update && \
	apt-get install -y git build-essential neofetch && \
	git clone https://github.com/kangalioo/poise && \
	cargo install loc && \
	cargo build --release && \
	chmod +x /tini

ENTRYPOINT ["/tini", "--"]
CMD ["./target/release/code-runner"]
