# Rust Axum Full Course - Web Development

This is my implementation of the full course from Jeremy Chone for the Axum web development framework in Rust published on his [youtube channel](https://www.youtube.com/@JeremyChone). The Axum framework is built on top of the hyper HTTP library. It is a very fast and efficient framework built on top of the async/await syntax in Rust. This course will cover everything you need to know to get started with Axum and build web applications in Rust.

## Links
- [Jeremy's github commit 0.7](https://github.com/jeremychone-channel/rust-axum-course/commit/52ded5e01efce0fc237280d9a5e6b8d7c1436d9c)
- [Jeremy's youtube video](https://youtu.be/XZtlD_m59sM?si=u3TSMyB8M-cRByhj)

## Instructions
```sh
# Terminal 1 - To run the server.
cargo watch -q -c -w src/ -w .cargo/ -x "run"

# Terminal 2 - To run the quick_dev.
cargo watch -q -c -w examples/ -x "run --example quick_dev"
```

## Notes
- Nice trick with the .cargo/config.toml file to set the default run command to print the debug! statements (note underscore vs. hypen in cargo.toml).
