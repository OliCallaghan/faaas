FROM node:20 AS base
ARG PNPM_VERSION=8.15.6
ENV PNPM_HOME=/usr/local/bin
RUN corepack enable && corepack prepare pnpm@${PNPM_VERSION} --activate
WORKDIR /app

FROM base AS builder
RUN pnpm add -g turbo
COPY . .
RUN turbo prune @faaas/perf-v8-event-loop-db --docker

FROM base AS migration
COPY --from=builder /app/out/json .
COPY --from=builder /app/out/pnpm-lock.yaml .
RUN pnpm install --frozen-lockfile

COPY --from=builder /app/out/full .
WORKDIR /app/benchmarks/perf-v8-event-loop-db
CMD pnpm run db:generate && pnpm run db:push
