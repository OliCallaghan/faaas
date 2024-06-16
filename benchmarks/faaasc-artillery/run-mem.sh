#!/bin/bash

set -e
set -o pipefail

RUN_ARTILLERY="docker run --rm -it -v .:/scripts artilleryio/artillery:latest run --quiet"

echo "Starting tests at $(date +%Y-%m-%dT%H:%M:%S%z)"

# Pets order from supplier
for dir in 128 256 512 1024; do
  cd suites/pets/$dir
  echo "Running suites/pets/$dir"
  $RUN_ARTILLERY --output /scripts/reports/adaptive.json /scripts/artillery.adaptive.yaml
  $RUN_ARTILLERY --output /scripts/reports/local.json /scripts/artillery.local.yaml
  $RUN_ARTILLERY --output /scripts/reports/proxy.json /scripts/artillery.proxy.yaml
  cd -
done

# Warehouse order from supplier
for dir in 128 256 512 1024; do
    cd suites/warehouse/order-from-supplier/$dir
    echo "Running suites/warehouse/order-from-supplier/$dir"
    $RUN_ARTILLERY --output /scripts/reports/adaptive.json /scripts/artillery.adaptive.yaml
    $RUN_ARTILLERY --output /scripts/reports/local.json /scripts/artillery.local.yaml
    $RUN_ARTILLERY --output /scripts/reports/proxy.json /scripts/artillery.proxy.yaml
    cd -
done

# Warehouse pricing summary report generation
for dir in 128 256 512 1024; do
    cd suites/warehouse/pricing-summary-report/$dir
    echo "Running suites/warehouse/pricing-summary-report/$dir"
    $RUN_ARTILLERY --output /scripts/reports/adaptive.json /scripts/artillery.adaptive.yaml
    $RUN_ARTILLERY --output /scripts/reports/local.json /scripts/artillery.local.yaml
    $RUN_ARTILLERY --output /scripts/reports/proxy.json /scripts/artillery.proxy.yaml
    cd -
done

echo "Tests completed at $(date +%Y-%m-%dT%H:%M:%S%z)"
