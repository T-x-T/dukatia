FROM node:16-alpine AS builder

ARG NUXT_PUBLIC_COMMIT
ARG NUXT_PUBLIC_BRANCH

WORKDIR /frontend
COPY . .
RUN npm install
RUN npm run build

FROM node:16-alpine

COPY --from=builder /frontend/.output /app/frontend
ENV NODE_ENV=production
ENV NUXT_PUBLIC_COMMIT=${NUXT_PUBLIC_COMMIT}
ENV NUXT_PUBLIC_BRANCH=${NUXT_PUBLIC_BRANCH}
EXPOSE 3000

CMD [ "node", "/app/frontend/server/index.mjs" ]
