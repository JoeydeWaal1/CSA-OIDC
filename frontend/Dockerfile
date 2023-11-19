FROM node:18 as build

WORKDIR /app
COPY package*.json ./
RUN npm install
COPY . .

ARG AUTHORITY
ENV REACT_APP_AUTHORITY $AUTHORITY
ARG CLIENT_ID
ENV REACT_APP_CLIENT_ID $CLIENT_ID

RUN npm run build
FROM nginx:alpine
COPY --from=build /app/build /usr/share/nginx/html
COPY ./config.conf /etc/nginx/nginx.conf

EXPOSE 3000

CMD ["nginx", "-g", "daemon off;"]
