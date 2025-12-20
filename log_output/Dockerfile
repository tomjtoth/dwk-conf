FROM alpine

RUN apk add util-linux

WORKDIR /app

COPY . .

ENTRYPOINT ["/bin/sh", "/app/main.sh"]
