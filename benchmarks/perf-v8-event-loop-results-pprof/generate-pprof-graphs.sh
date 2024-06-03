#!/bin/bash

set -e
set -o pipefail

RESULTS_DIR=node_modules/@faaas/perf-v8-event-loop-experiment/results

pprof -dot -focus handleDeletePet -nodecount 12 -relative_percentages $RESULTS_DIR/local_wall/onHttpDeletePet/pprof-time.pb.gz > assets/faas-profile-pprof-delete-pet.dot

pprof -dot -focus handleGetPet -nodecount 12 -relative_percentages $RESULTS_DIR/local_wall/onHttpGetPet/pprof-time.pb.gz > assets/faas-profile-pprof-get-pet.dot

pprof -dot -focus handleGetPets -nodecount 12 -relative_percentages $RESULTS_DIR/local_wall/onHttpGetPets/pprof-time.pb.gz > assets/faas-profile-pprof-get-pets.dot

pprof -dot -focus handlePutPet -nodecount 12 -relative_percentages $RESULTS_DIR/local_wall/onHttpPutPet/pprof-time.pb.gz > assets/faas-profile-pprof-put-pet.dot
