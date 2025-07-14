FROM node:18-alpine as builder
WORKDIR /app
COPY package*.json ./
RUN npm ci
COPY . .

# Build args for API URLs
ARG REACT_APP_API_URL
ARG REACT_APP_SERVE_URL
ENV REACT_APP_API_URL=${REACT_APP_API_URL}
ENV REACT_APP_SERVE_URL=${REACT_APP_SERVE_URL}

# Remove the NODE_OPTIONS from here and use the build command directly
RUN npm run build

FROM nginx:alpine
COPY --from=builder /app/build /usr/share/nginx/html
COPY nginx.conf.template /etc/nginx/templates/default.conf.template

# Environment variables for nginx
ENV API_URL=http://localhost:8081
ENV SERVE_URL=http://localhost:8080

EXPOSE 80
CMD ["nginx", "-g", "daemon off;"]