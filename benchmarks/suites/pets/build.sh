#!/bin/bash

set -e
set -o pipefail

faaasc --adaptor=@faaas/aws-http-adaptor src/handler.ts gen/http/handler.ts
faaasc --adaptor=@faaas/aws-adaptor src/handler.ts gen/mq/handler.ts

esbuild gen/http/handler.ts --platform=node --bundle --outdir=dist/http
esbuild gen/mq/handler.ts --platform=node --bundle --outdir=dist/mq

zip -j -r dist/http/handler.zip dist/http/handler.js
zip -j -r dist/mq/handler.zip dist/mq/handler.js
