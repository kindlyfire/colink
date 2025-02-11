FROM imbios/bun-node:1.2.2-22.13.1-alpine AS base
WORKDIR /usr/src/app

COPY . .
RUN bun install --frozen-lockfile
RUN bun run build
ENV PORT=80 HOST=0.0.0.0
EXPOSE 80
ENTRYPOINT [ "bun", "--bun", ".output/server/index.mjs" ]