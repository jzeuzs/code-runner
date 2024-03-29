FROM liuchong/rustup:nightly-onbuild

WORKDIR /code-runner
COPY . .

ENV TINI_VERSION v0.19.0
ADD https://github.com/krallin/tini/releases/download/${TINI_VERSION}/tini /tini

RUN apt-get update && \
	apt-get install -y git build-essential neofetch curl && \
	git clone https://github.com/kangalioo/poise && \
	curl -sL https://deb.nodesource.com/setup_16.x | bash - && \
	apt-get install -y nodejs && \
	npm i -g yarn pm2 && \
	cd server && \
	yarn && \
	yarn build && \
	cd - && \
	cargo install loc && \
	cargo build --release && \
	chmod +x /tini

ENTRYPOINT ["/tini", "--"]
CMD ["bash", "run.sh"]
