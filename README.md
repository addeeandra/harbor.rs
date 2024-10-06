<p align="center">
    <h1 align="center">Harbor.rs</h1>
    <p align="center">Generate Docker Compose</p>
</p>

------

## Usage

Harbor helps you generate `docker-compose.yml` with single command.

## Basic Commands

```bash
./harbor --<command> <args>
```

#### Show Helps

```bash
./harbor --help
```

#### Generate

```bash
# generating docker-compose.yml with mysql and redis
./harbor --ship [<image-tags> ...]
```

```bash
# generating docker-compose.yml with mysql and redis
./harbor --ship mysql redis

# generating docker-compose.yml with postgresql and redis
./harbor --ship pgsql redis

# generating docker-compose.yml with mariadb, redis and mailpit
./harbor --ship mariadb redis mailpit
```

## Available Images

#### List of Tags

|  Tag | Image |
| --- | --- |
| mysql | mysql/mysql-server:8.0 |
| pgsql | postgres:15 |
| mariadb | mariadb:11 |
| redis | redis:alpine |
| mailpit | axllent/mailpit:latest |
