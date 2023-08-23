# Commands used for app lifecycle

## Docker

### Build
To build docker image: build a docker image tagged as "zero2prod" according to teh recipe 
specified in `Dockerfile`

```
docker build --tag zero2prod --file Dockerfile .
```

### Run
```
docker run zero2prod
```

### Kill Running Image (with -p flag) to Launch It Again With Command
```
docker run -p 8000:8000 zero2prod
```

### Healthcheck
```
curl http://127.0.0.1:8000/health_check
```

## To prepare sqlx to work in offline mode (static compilation)
It must be invoked as a cargo subcommand
All options after `--` are passed to cargo itself
We need to point it at our library since it contains 
all our SQL queries.
```
cargo sqlx prepare -- --lib
```