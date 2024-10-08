# rs-web-filesys
This is a simple while strong online file system build with rust. 


## Front

```bash
pnpm install
pnpm dev # development
pnpm build # build
```

## Backend 

```bash
cargo run --release 
 ./target/release/backend  --host 0.0.0.0 --port 8080 --upload-dir ./data
```

(mixed compile 2 linux)
```bash
brew install FiloSottile/musl-cross/musl-cross
ln -s /usr/local/bin/x86_64-linux-musl-gcc /usr/local/bin/musl-gcc
rustup target add x86_64-unknown-linux-musl
cargo build --release --target x86_64-unknown-linux-musl

```

Ouput as following:
~~~
) ./target/release/backend  --host 0.0.0.0 --port 8080 --upload-dir ./data
Server starting at http://0.0.0.0:8080
[2024-08-18T11:15:05Z INFO  actix_server::builder] starting 10 workers
[2024-08-18T11:15:05Z INFO  actix_server::server] Actix runtime found; starting in Actix runtime
[2024-08-18T11:15:05Z INFO  actix_server::server] starting service: "actix-web-service-0.0.0.0:8080", workers: 10, listening on: 0.0.0.0:8080
~~~