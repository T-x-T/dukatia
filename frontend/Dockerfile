FROM node:16-alpine AS builder

WORKDIR /frontend

COPY . .
ENV API_HOST=http://backend:4000
RUN npm install
RUN npm run build

FROM node:16-alpine

COPY --from=builder /frontend/.output /app/frontend

ENV NODE_ENV=production

EXPOSE 3000

CMD [ "node", "/app/frontend/server/index.mjs" ]
