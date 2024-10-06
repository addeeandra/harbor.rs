<p align="center">
    <h1 align="center">Harbor.rs</h1>
    <p align="center">CLI Command to Generate Docker Compose</p>
</p>

------

## Usage

Harbor helps you generate `docker-compose.yml` in single command.

## Basic Commands

```bash
./harbor <args> -- [<image-tags>, ...]
```

#### Show Helps

```bash
./harbor --help # or [ -h ]
```

#### Generate

```bash
# generating docker-compose.yml with mysql and redis
./harbor -- [<image-tags>, ...]
```

```bash
# generating docker-compose.yml with mysql and redis
./harbor -- mysql redis

# generating docker-compose.yml with postgresql and redis
./harbor -- pgsql redis

# generating docker-compose.yml with mariadb, redis and mailpit
./harbor -- mariadb redis mailpit
```

#### Other Commands

```bash
# compose generation will fail if `docker-compose.yml` exists
# use `--force` or `-f` to replace existing file
./harbor --force -- mysql redis

# environment variables are shown only at end execution
# use `--env` or `-e` to generate `.env.harbor` file
./harbor --include-env -- mysql redis

# default output file is `docker-compose.yml`
# use `--output` or `-o` to define custom output file name
./harbor --output=docker-compose.dev.yml -- mysql redis
```

## Available Images

|  Tag | Image |
| --- | --- |
| mysql | mysql/mysql-server:8.0 |
| pgsql | postgres:15 |
| mariadb | mariadb:11 |
| redis | redis:alpine |
| mailpit | axllent/mailpit:latest |
