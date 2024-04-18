#!/bin/bash

set -e
set -o pipefail

# Determine experiment and root directories
X_DIR=$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" &> /dev/null && pwd)
R_DIR=$X_DIR/../..

echo "Experiment directory: $X_DIR"
echo "Root directory: $R_DIR"

# Change to root directory
cd $R_DIR

# Configure log output
if [[ $* == *--flag* ]]
then
    LOG_DEST=/dev/null
else
    LOG_DEST=$X_DIR/local.log
fi

experiment () {
    # Get experiment type
    local TYPE=$1

    echo "Running $TYPE experiment"

    # Define Docker compose command
    local COMPOSE="docker compose --env-file $X_DIR/.env -f $X_DIR/docker-compose.base.yaml -f $X_DIR/docker-compose.$TYPE.yaml --project-directory=$R_DIR"

    echo "Using $COMPOSE"
    echo "Writing logs to $LOG_DEST"

    # Cleanup previous compose runs
    $COMPOSE down --volumes &> $LOG_DEST

    # Startup DB and run migrations
    echo "Startup DB and run migrations"
    $COMPOSE up --build -d database migrate &> $LOG_DEST
    $COMPOSE wait migrate &> $LOG_DEST

    echo "Start artillery"
    $COMPOSE run -v $X_DIR/results/local_$TYPE/reports:/reports -d --build artillery &> $LOG_DEST

    # Add latency
    echo "Simulate network latency"
    $COMPOSE exec database tc qdisc add dev eth0 root netem delay 20ms 30ms
    $COMPOSE exec on_http_delete_pet tc qdisc add dev eth0 root netem delay 20ms 30ms
    $COMPOSE exec on_http_get_pet tc qdisc add dev eth0 root netem delay 20ms 30ms
    $COMPOSE exec on_http_get_pets tc qdisc add dev eth0 root netem delay 20ms 30ms
    $COMPOSE exec on_http_put_pet tc qdisc add dev eth0 root netem delay 20ms 30ms

    echo "Wait for artillery"
    $COMPOSE wait artillery &> $LOG_DEST

    if [ $TYPE = "io" ] || [ $TYPE = "time" ]
    then
        echo "Killing webservers"
        $COMPOSE exec on_http_delete_pet /app/terminate.sh
        $COMPOSE exec on_http_get_pet /app/terminate.sh
        $COMPOSE exec on_http_get_pets /app/terminate.sh
        $COMPOSE exec on_http_put_pet /app/terminate.sh
    fi

    $COMPOSE stop &> $LOG_DEST

    echo "Copy results from containers"
    $COMPOSE cp on_http_delete_pet:/app/perf/. $X_DIR/results/local_$TYPE/onHttpDeletePet &> $LOG_DEST
    $COMPOSE cp on_http_get_pet:/app/perf/. $X_DIR/results/local_$TYPE/onHttpGetPet &> $LOG_DEST
    $COMPOSE cp on_http_get_pets:/app/perf/. $X_DIR/results/local_$TYPE/onHttpGetPets &> $LOG_DEST
    $COMPOSE cp on_http_put_pet:/app/perf/. $X_DIR/results/local_$TYPE/onHttpPutPet &> $LOG_DEST

    # Generate HTML report
    echo "Generate HTML artillery report"
    docker run -v $X_DIR/results/local_$TYPE/reports:/home/node/artillery/reports artilleryio/artillery:2.0.9 report -o reports/report.html reports/report.json
}

experiment "cpu"
experiment "io"
experiment "time"
experiment "wall"
