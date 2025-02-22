#!/bin/sh
echo '[application]
env = "development"
port = 1304
log-level = "h2=info,debug"

[misc]
something = "http://localhost:8080"

[database]
pool_size = 100
port = 5432
name = "users"
host = "localhost"
user = "postgres"
password = "password"' > users.toml

echo '[application]
env = "development"
port = 1610
log-level = "h2=info,debug"

[misc]
max_query_results = 1000

[database]
pool_size = 100
port = 5432
name = "categories"
host = "localhost"
user = "postgres"
password = "password"' > categories.toml

docker run -d --network="host" -v ./users.toml:/users.toml ghcr.io/sellershut/users-service:master ./users-service --config-file "users.toml"
docker run -d --network="host" -v ./categories.toml:/categories.toml ghcr.io/sellershut/categories-service:master ./categories-service --config-file "categories.toml"
