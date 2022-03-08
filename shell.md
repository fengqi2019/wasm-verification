
```
docker run --name wasm-verify --restart=always -d -it -v "$(pwd)":/home/rust/src -v $(pwd)/target/registry/x86_64-musl:/root/.cargo/registry messense/rust-musl-cross:x86_64-musl

docker exec wasm-verify cargo run --package mem-verification --release -- -y 0 -t 100

docker exec wasm-verify ./target/x86_64-unknown-linux-musl/release/mem-verification -y 0 -t 100 -r 0


```


