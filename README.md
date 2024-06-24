# Rust Axum Full Production Course - Web Development

This is my implementation of the full course from Jeremy Chone for the Axum web development framework in Rust published on his [youtube channel](https://www.youtube.com/@JeremyChone). The Axum framework is built on top of the hyper HTTP library. It is a very fast and efficient framework built on top of the async/await syntax in Rust. This course will cover everything you need to know to get started with Axum and build web applications in Rust.

## Links
- [Jeremy's github commit 0.7](https://github.com/jeremychone-channel/rust-axum-course/commit/52ded5e01efce0fc237280d9a5e6b8d7c1436d9c)
- [Jeremy's youtube video](https://youtu.be/XZtlD_m59sM?si=u3TSMyB8M-cRByhj)

## Starting the DB
```sh
# Start the postgresql server docker image:
docker run --rm --name pg -p 5432:5432 \
    -e POSTGRES_PASSWORD=welcome \
    postgres:latest
```

```sh
# (optional) To have a psql terminal:
# In another terminal (tab) run psql
docker exec -it -u postgres pg psql
```

## Unit Test (watch)

```sh
cargo watch -q -c -x "test -- --nocapture"

# specific test with filter
cargo watch -q -c -x "test model::task::tests::test_create -- --nocapture --test test_get_user"
```

## Development

```sh
# Terminal 1 - To run the server.
cargo watch -q -c -w src/ -w .cargo/ -x "run"

# Terminal 2 - To run the quick_dev.
cargo watch -q -c -w examples/ -x "run --example quick_dev"
```

## Notes
- Nice trick with the .cargo/config.toml file to set the default run command to print the debug! statements (note underscore vs. hypen in cargo.toml).
- Specifying the duration unit in a variable name is a good practice. Example: `SESSION_DURATION_MINUTES=30` vs. `SESSION_DURATION=30`
- Strategy for Error Handling: Key modules or submodules will have their own `error.rs` and we export them in `mod.rs`.  A good example of this approach is in `crates/libs/lib-auth/pwd/scheme`. But, `crates/libs/lib-auth/pwd` also has its own `error.rs`
- Similar to the previous bullet, some crates may also require a `config.rs`.  See `crates/libs/lib-auth/config` for an example.
