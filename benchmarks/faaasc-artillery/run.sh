#!/bin/bash

set -e
set -o pipefail

RUN_ARTILLERY="docker run --rm -it -v ${PWD}:/scripts artilleryio/artillery:latest run"

$RUN_ARTILLERY /scripts/suites/pets/artillery.adaptive.yaml
$RUN_ARTILLERY /scripts/suites/pets/artillery.local.yaml
$RUN_ARTILLERY /scripts/suites/pets/artillery.proxy.yaml
