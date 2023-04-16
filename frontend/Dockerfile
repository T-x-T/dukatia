FROM node:19-alpine

WORKDIR /frontend
ENV NODE_ENV=production

COPY . .

RUN npm install
RUN npm run build

EXPOSE 3000

CMD [ "npm", "start" ]
