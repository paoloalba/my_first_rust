#!/bin/bash

export registry=myregistry.io
export versionNumber=beta

export dockerfile_src="Dockerfile"
export dockerfile_target=$1

export repositoryName=my_first_rust

export PERMANENT_STORAGE=./permanent_storage

docker-compose --project-name ${dockerfile_target} build
docker-compose --project-name ${dockerfile_target} up
docker-compose --project-name ${dockerfile_target} down