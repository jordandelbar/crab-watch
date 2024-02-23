# Use an official Node.js runtime as a base image
FROM node:14-alpine as build-stage

# Set the working directory inside the container
WORKDIR /app

# Copy package.json and package-lock.json to the working directory
COPY package*.json ./

# Install project dependencies
RUN npm install

# Copy the rest of the application code
COPY . .

# Build the application
RUN npm run build

# Use a smaller, production-ready base image
FROM nginx:alpine

# Copy the built app from the build stage to the nginx web root directory
COPY --from=build-stage /app/dist /usr/share/nginx/html

# Expose port 80 for the Nginx server
EXPOSE 80

# Command to start the Nginx server
CMD ["nginx", "-g", "daemon off;"]
