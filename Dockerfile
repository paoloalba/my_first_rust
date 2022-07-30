FROM python:3.10.5-slim-bullseye as base

RUN apt update && apt upgrade
RUN apt install build-essential -y
RUN apt install curl -y

RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y

RUN pip install maturin

# WORKDIR /rust_inst

# COPY ./docker_helpers /rust_inst
# RUN chmod a+x ./setup_rust.sh
# RUN ./setup_rust.sh

COPY ./rust_app /rust_app
WORKDIR /rust_app
RUN chmod a+x ./build_install_rust.sh
RUN ./build_install_rust.sh

COPY ./app /app
WORKDIR /app

FROM base as debug

FROM base as prod

CMD ["python", "main.py"]
