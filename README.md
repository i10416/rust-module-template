## About

My general purpose rust project example.

```
.
├── Cargo.toml
├── README.md
├── app_grpc_api: grpc api
│   ├── Cargo.toml
│   └── src
│       └── main.rs
├── app_http_api: web api
│   ├── Cargo.toml
│   └── src
│       └── main.rs
├── app_cli: std io/err handling
│   ├── Cargo.toml
│   └── src
│       └── main.rs
├── app_core: core logic
│   ├── Cargo.toml
│   └── src
│       └── lib.rs
├── app_macros: macros
│   ├── Cargo.toml
│   └── src
│       └── lib.rs
├── rust-toolchain
└── src: nope
    └── main.rs
```


Rustで設定項目(もしくは設定ファイル)をどう扱うか
- https://zenn.dev/j5ik2o/scraps/6283e0230cb92f
## run main in submodule

```sh
cargo run -p app_cli
cargo run -p app_http_api
cargo run -p app_grpc_api
```

## build module

```sh
cargo build -p app_cli
cargo build -p app_http_api
cargo build -p app_grpc_api
```



