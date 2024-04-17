#!/bin/bash

set -e

# Determine experiment and root directories
X_DIR=$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" &> /dev/null && pwd)
R_DIR=$X_DIR/../..

echo "Experiment directory: $X_DIR"
echo "Root directory: $R_DIR"

# Change to root directory
cd $R_DIR

experiment () {
    # Get experiment type
    local TYPE=$1

    echo "Running $TYPE experiment"

    # Define Docker compose command
    local COMPOSE_CPU="docker compose --env-file $X_DIR/.env -f $X_DIR/docker-compose.base.yaml -f $X_DIR/docker-compose.$TYPE.yaml --project-directory=$R_DIR"

    # Cleanup previous compose runs
    $COMPOSE_CPU down --volumes &> /dev/null

    # Startup DB and run migrations
    echo "Startup DB and run migrations"
    $COMPOSE_CPU up --build -d database migrate &> /dev/null
    $COMPOSE_CPU wait migrate &> /dev/null

    echo "Run artillery"
    $COMPOSE_CPU run --build artillery &> /dev/null
    $COMPOSE_CPU stop &> /dev/null

    echo "Copy results from containers"
    $COMPOSE_CPU cp on_http_delete_pet:/app/perf/. $X_DIR/results/local_$TYPE/onHttpDeletePet &> /dev/null
    $COMPOSE_CPU cp on_http_get_pet:/app/perf/. $X_DIR/results/local_$TYPE/onHttpGetPet &> /dev/null
    $COMPOSE_CPU cp on_http_get_pets:/app/perf/. $X_DIR/results/local_$TYPE/onHttpGetPets &> /dev/null
    $COMPOSE_CPU cp on_http_put_pet:/app/perf/. $X_DIR/results/local_$TYPE/onHttpPutPet &> /dev/null
}

experiment "cpu"
experiment "wall"
