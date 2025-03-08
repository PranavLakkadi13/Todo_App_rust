## DB 

```shell
# to start the Db 
docker run --rm -p 5433:5432 -e "POSTGRES_PASSWORD=postgres" --name pg postgres:14

# optionally psql 
docker exec -it -u postgres pg psql 

```