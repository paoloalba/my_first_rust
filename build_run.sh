#!/bin/bash

export registry=myregistry.io
export versionNumber=beta

export dockerfile_src=$1
export dockerfile_target=$2

export repositoryName=my_first_rust

export PERMANENT_STORAGE=./permanent_storage
export APP_PATH=./app
export RUST_APP_PATH=./rust_app

docker-compose --project-name ${dockerfile_target} build

# docker run -it --rm --name my-running-app ${registry}/${repositoryName}:${dockerfile_target}_${versionNumber}

docker-compose --project-name ${dockerfile_target} up
docker-compose --project-name ${dockerfile_target} down
