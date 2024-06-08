#!/bin/bash

set -e
set -o pipefail

X_DIR=$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" &> /dev/null && pwd)
R_DIR=$X_DIR/../..

echo "Gateway directory: $X_DIR"
echo "Root directory: $R_DIR"

# Change to root directory
cd $R_DIR

docker build -t postgres_faaas_engine -f pkg/postgres-faaas-engine/Dockerfile .
docker tag postgres_faaas_engine:latest 177684222297.dkr.ecr.eu-west-2.amazonaws.com/postgres_faaas_engine:latest
docker push 177684222297.dkr.ecr.eu-west-2.amazonaws.com/postgres_faaas_engine:latest
