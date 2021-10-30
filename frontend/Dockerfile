FROM node:alpine

WORKDIR /frontend
ENV NODE_ENV=prod

COPY . .

RUN npm install --production
RUN npm run build

EXPOSE 3000

CMD [ "npm", "start" ]
