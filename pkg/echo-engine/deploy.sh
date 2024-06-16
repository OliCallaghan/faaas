#!/bin/bash

set -e
set -o pipefail

X_DIR=$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" &> /dev/null && pwd)
R_DIR=$X_DIR/../..

echo "Gateway directory: $X_DIR"
echo "Root directory: $R_DIR"

# Change to root directory
cd $R_DIR

docker build -t echo_engine -f pkg/echo-engine/Dockerfile .
docker tag echo_engine:latest 177684222297.dkr.ecr.eu-west-2.amazonaws.com/echo_engine:latest
docker push 177684222297.dkr.ecr.eu-west-2.amazonaws.com/echo_engine:latest
