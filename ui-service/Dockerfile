FROM node:lts-alpine as builder

# Create app directory
WORKDIR /app

# Install app dependencies
COPY package*.json ./

RUN npm install

# Copy app source code
COPY . .

# Build static assets
RUN npm run build


FROM node:lts-alpine
WORKDIR /app
COPY --from=builder /app/.next ./.next
COPY --from=builder /app/public ./public
COPY --from=builder /app/node_modules ./node_modules
COPY --from=builder /app/package.json ./
CMD ["npm", "run", "start"]