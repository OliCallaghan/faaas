#!/bin/bash

set -e
set -o pipefail

RUN_ARTILLERY="docker run --rm -it -v .:/scripts artilleryio/artillery:latest run"

echo "Starting tests at $(date +%Y-%m-%dT%H:%M:%S%z)"

# Pets
cd suites/echoer
echo "Running suites/echoer in adaptive mode"
$RUN_ARTILLERY --output /scripts/reports/adaptive.json /scripts/artillery.adaptive.yaml
cd -

echo "Tests completed at $(date +%Y-%m-%dT%H:%M:%S%z)"
