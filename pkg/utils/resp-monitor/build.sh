#!/bin/bash

poetry export --without-hashes --only lambda-resp-monitor -f requirements.txt -o requirements.txt

docker build -t 177684222297.dkr.ecr.eu-west-2.amazonaws.com/faaas_monitor:latest .
docker push 177684222297.dkr.ecr.eu-west-2.amazonaws.com/faaas_monitor:latest
