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
name = "postgres"
host = "localhost"
user = "postgres"
password = "password"' > users.toml

docker run -d --network="host" -v ./users.toml:/users.toml ghcr.io/sellershut/users-service:master ./users-service --config-file "users.toml"
