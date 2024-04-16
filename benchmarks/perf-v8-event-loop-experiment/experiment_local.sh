#!/bin/bash

set -e

# Determine experiment and root directories
X_DIR=$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" &> /dev/null && pwd)
R_DIR=$X_DIR/../..

echo "Experiment directory: $X_DIR"
echo "Root directory: $R_DIR"

# Change to root directory
cd $R_DIR

# Define Docker compose command
COMPOSE="docker compose --env-file $X_DIR/.env -f $X_DIR/docker-compose.yaml --project-directory=$R_DIR"

# Cleanup previous compose runs
$COMPOSE down --volumes &> /dev/null

# Startup DB and run migrations
echo "Startup DB and run migrations"
$COMPOSE up --build -d database migrate &> /dev/null
$COMPOSE wait migrate &> /dev/null

echo "Run artillery"
$COMPOSE run --build artillery &> /dev/null
$COMPOSE stop &> /dev/null

echo "Copy results from containers"
$COMPOSE cp on_http_delete_pet:/app/perf/. $X_DIR/results/local/onHttpDeletePet &> /dev/null
$COMPOSE cp on_http_get_pet:/app/perf/. $X_DIR/results/local/onHttpGetPet &> /dev/null
$COMPOSE cp on_http_get_pets:/app/perf/. $X_DIR/results/local/onHttpGetPets &> /dev/null
$COMPOSE cp on_http_put_pet:/app/perf/. $X_DIR/results/local/onHttpPutPet &> /dev/null
