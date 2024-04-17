FROM node:20 AS base
ARG PNPM_VERSION=8.15.6
ENV PNPM_HOME=/usr/local/bin
RUN corepack enable && corepack prepare pnpm@${PNPM_VERSION} --activate
WORKDIR /app

FROM base AS builder
RUN pnpm add -g turbo
COPY . .
RUN turbo prune @faaas/perf-v8-event-loop --docker

FROM base AS installer
COPY --from=builder /app/out/json .
COPY --from=builder /app/out/pnpm-lock.yaml .
RUN pnpm install --frozen-lockfile

COPY --from=builder /app/out/full .
RUN pnpm run build --filter=@faaas/perf-v8-event-loop...

FROM base AS runner
RUN apt-get update && apt-get -y install \
    linux-perf \
    sudo
RUN mkdir -p /app/perf
RUN npm install -g 0x pprof-it

COPY --from=installer /app/benchmarks/perf-v8-event-loop/node_modules/@faaas/perf-v8-event-loop-db/node_modules/.prisma/client-benchmark-perf-v8-event-loop-db/schema.prisma .
COPY --from=installer /app/benchmarks/perf-v8-event-loop/node_modules/@faaas/perf-v8-event-loop-db/node_modules/.prisma/client-benchmark-perf-v8-event-loop-db/libquery_engine-* .
COPY --from=installer /app/benchmarks/perf-v8-event-loop/dist .
