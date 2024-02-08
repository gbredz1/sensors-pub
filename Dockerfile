FROM debian:bookworm-slim AS final

ARG TARGETPLATFORM
ARG APP_V
ARG RELEASE_URI=https://github.com/gbredz1/sensors-pub/releases/download
ARG UID=1000

RUN ["/bin/bash", "-c", ": ${APP_V:?APP_V arg needs to be set.}"]

RUN apt update \
    && apt install -y wget tini \
    && apt clean

RUN adduser \
    --disabled-password \
    --gecos "" \
    --home "/nonexistent" \
    --shell "/sbin/nologin" \
    --no-create-home \
    --uid "${UID}" \
    appuser
USER appuser

WORKDIR /app

RUN case "$TARGETPLATFORM" in \
    linux/arm/v6) FILE=linux-armv6-gnueabihf.tar.gz;; \
    linux/arm/v7) FILE=linux-armv7-gnueabihf.tar.gz;; \
    linux/arm64) FILE=linux-arm64-gnu.tar.gz;; \
    linux/386) FILE=linux-i686-gnu.tar.gz ;; \
    linux/amd64) FILE=linux-x86_64-gnu.tar.gz;; \
    *) echo "unsupported platform ${TARGETPLATFORM}"; exit 1 ;; \
    esac \
    && FILENAME="sensors-pub-${APP_V}-${FILE}" \
    && wget -qO- "${RELEASE_URI}/${APP_V}/${FILENAME}" | tar xzf - --strip-components 1  

ENTRYPOINT ["/usr/bin/tini", "--", "./sensors-pub"]