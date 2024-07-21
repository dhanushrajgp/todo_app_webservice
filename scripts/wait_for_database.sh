#!/bin/bash

# Navigate to the directory containing the docker-compose.yml file
cd ..

# Start the Docker services
docker-compose up -d

# Wait for PostgreSQL to be ready
until docker run --rm --network host --env POSTGRES_PASSWORD=${POSTGRES_PASSWORD} postgres:12.19 pg_isready -h localhost -U ${POSTGRES_USER} -d ${POSTGRES_DB}
do
  echo "Waiting for postgres..."
  sleep 2
done

echo "PostgreSQL is now ready."

# (Optional) Stop the Docker services if needed
# docker-compose down
