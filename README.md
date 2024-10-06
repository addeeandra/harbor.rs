<p align="center">
    <h1 align="center">Harbor.rs</h1>
    <p align="center">Running Docker Environment</p>
</p>

------

## Usage

Usage instruction here.

## Basic Commands

To be available.

> [!WARNING]
> Basic commands are not available yet.

#### Show help

```bash
./harbor --help # or [ -h ]
```

#### Initialize Environment

```bash
# generating docker-compose.yml for general-purpose
./harbor init

# generating docker-compose.yml file for laravel environment that include `artisan` command
./harbor init laravel

# shows harbor environment information
./harbor info
```

#### Start and Stop

```bash
# start environment
./harbor start # or [ up ] which equivalent to `docker compose up -d`

# restart environment
./harbor restart # which equivalent to `docker compose restart`

# stop and remove all containers
./harbor stop # or [ down ] which equivalent to `docker compose down`
```

#### Container's Command

```bash
# equivalent to `docker compose exec -it <container> <command>`
./harbor exec <container> <command>

# equivalent to `docker compose exec -it <container> bash`
./harbor ssh <container>
```

#### Laravel Command

Environment specific commands.

```bash
# running `php` command
./harbor php <command>

# running `composer` command
./harbor composer <command> # or [ comp ]

# running `artisan` command
./harbor artisan <command> # or [ art ]

# running `npm` command
./harbor npm <command>

# running `yarn` command
./harbor yarn <command>

# running `npx` command
./harbor npx <command>
```