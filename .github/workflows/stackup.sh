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
