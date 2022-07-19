FROM python:3.10.5-slim-bullseye as base

RUN apt update && apt upgrade
RUN apt install build-essential -y
RUN apt install curl -y

RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y

WORKDIR /rust_inst

COPY ./docker_helpers /rust_inst
RUN chmod a+x ./build_rust.sh

FROM base as debug

CMD ["./build_rust.sh"]

FROM base as prod

RUN ./build_rust.sh

COPY ./app /app
WORKDIR /app

CMD ["python", "main.py"]
