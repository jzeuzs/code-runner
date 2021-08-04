FROM node:alpine

WORKDIR /code-runner-server
COPY . .

ENV TINI_VERSION v0.19.0
ADD https://github.com/krallin/tini/releases/download/${TINI_VERSION}/tini /tini

RUN yarn

CMD ["yarn", "start"]
