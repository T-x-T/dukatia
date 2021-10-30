FROM node:alpine

# Set workdir
WORKDIR /app

# Bundle APP files
COPY . .

# Install app dependencies
ENV NPM_CONFIG_LOGLEVEL warn
RUN npm install --production

# expose port
EXPOSE 4000

CMD [ "npm", "run", "prod" ]
