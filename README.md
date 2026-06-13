# sellershut

## compose stuff

```sh
docker compose up -d

docker exec -it sellershut-vault-1 vault operator init

docker exec -it sellershut-vault-1 vault operator unseal <unseal-key-1>
docker exec -it sellershut-vault-1 vault operator unseal <unseal-key-2>
docker exec -it sellershut-vault-1 vault operator unseal <unseal-key-3>

docker exec -it sellershut-vault-1 vault login <root-token>

docker exec -it sellershut-vault-1 vault status

docker exec -it sellershut-vault-1 vault secrets enable -path=secret kv-v2

docker logs -f sellershut-vault-1
```
