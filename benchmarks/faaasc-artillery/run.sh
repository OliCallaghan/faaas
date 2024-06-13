#!/bin/bash

set -e
set -o pipefail

RUN_ARTILLERY="docker run --rm -it -v .:/scripts artilleryio/artillery:latest run --quiet"

echo "Starting tests at $(date +%Y-%m-%dT%H:%M:%S%z)"

# Pets
cd suites/pets
echo "Running suites/pets"
$RUN_ARTILLERY --output /scripts/reports/adaptive.json /scripts/artillery.adaptive.yaml
$RUN_ARTILLERY --output /scripts/reports/http.json /scripts/artillery.http.yaml
$RUN_ARTILLERY --output /scripts/reports/local.json /scripts/artillery.local.yaml
$RUN_ARTILLERY --output /scripts/reports/proxy.json /scripts/artillery.proxy.yaml
cd -

# Warehouse order from supplier
cd suites/warehouse/order-from-supplier
echo "Running suites/warehouse/order-from-supplier"
$RUN_ARTILLERY --output /scripts/reports/adaptive.json /scripts/artillery.adaptive.yaml
$RUN_ARTILLERY --output /scripts/reports/http.json /scripts/artillery.http.yaml
$RUN_ARTILLERY --output /scripts/reports/local.json /scripts/artillery.local.yaml
$RUN_ARTILLERY --output /scripts/reports/proxy.json /scripts/artillery.proxy.yaml
cd -

# Warehouse pricing summary report generation
cd suites/warehouse/pricing-summary-report
echo "Running suites/warehouse/pricing-summary-report"
$RUN_ARTILLERY --output /scripts/reports/adaptive.json /scripts/artillery.adaptive.yaml
$RUN_ARTILLERY --output /scripts/reports/http.json /scripts/artillery.http.yaml
$RUN_ARTILLERY --output /scripts/reports/local.json /scripts/artillery.local.yaml
$RUN_ARTILLERY --output /scripts/reports/proxy.json /scripts/artillery.proxy.yaml
cd -

echo "Tests completed at $(date +%Y-%m-%dT%H:%M:%S%z)"
