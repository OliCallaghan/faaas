#!/bin/bash

set -e
set -o pipefail

generate_files() {
    local filename="$1"

    faaasc --adaptor=@faaas/aws-http-adaptor "src/${filename}.ts" "gen/http/${filename}.ts"
    faaasc --adaptor=@faaas/aws-adaptor "src/${filename}.ts" "gen/mq/${filename}.ts"

    esbuild "gen/http/${filename}.ts" --platform=node --bundle --outfile="dist/${filename}/http/handler.js"
    esbuild "gen/mq/${filename}.ts" --platform=node --bundle --outfile="dist/${filename}/mq/handler.js"

    zip -j -r "dist/${filename}/http/handler.zip" "dist/${filename}/http/handler.js"
    zip -j -r "dist/${filename}/mq/handler.zip" "dist/${filename}/mq/handler.js"
}

generate_files "order-from-supplier"
generate_files "pricing-summary-report"
generate_files "revenue-pred"
