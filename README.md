# sdl/axum_server

## Getting Started

Download links:

SSH clone URL: <ssh://git@git.jetbrains.space/soltedev/sdl/axum_server.git>

HTTPS clone URL: <https://git.jetbrains.space/soltedev/sdl/axum_server.git>

These instructions will get you a copy of the project up and running on your local machine for development and testing purposes.

## Prerequisites

What things you need to install the software and how to install them.

```sh
Examples
```

## Development

Backend

```cargo watch -q -c -w src/ -x run```

client

```cargo watch -q -c -w tests/ -x "test -q quick_dev -- --nocapture"```

## Deployment

Add additional notes about how to deploy this on a production system.

## Resources

Add links to external resources for this project, such as CI server, bug tracker, etc.

## Starting the DB

```sh
# Start postgresql server docker image:
docker run --rm --name pg -p 5432:5432  -e POSTGRES_PASSWORD=welcome  postgres:15

# (optional) To have a psql terminal on pg. 
# In another terminal (tab) run psql:
docker exec -it -u postgres pg psql

# (optional) For pg to print all sql statements.
# In psql command line started above.
ALTER DATABASE postgres SET log_statement = 'all';
```

## Dev (REPL)

> NOTE: Install cargo watch with `cargo install cargo-watch`.

```sh
# Terminal 1 - To run the server.
cargo watch -q -c -w src/ -w .cargo/ -x "run"

# Terminal 2 - To run the quick_dev.
cargo watch -q -c -w examples/ -x "run --example quick_dev"
```

## Unit Test (REPL)

```sh
cargo watch -q -c -x "test -- --nocapture"

# Specific test with filter.
cargo watch -q -c -x "test model::task::tests::test_create -- --nocapture"
```

## Dev

```sh
# Terminal 1 - To run the server.
cargo run

# Terminal 2 - To run the tests.
cargo run --example quick_dev
```

## Unit Test

```sh
cargo test -- --nocapture

cargo watch -q -c -x test model::task::tests::test_create -- --nocapture
```
