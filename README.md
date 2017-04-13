## rust-wasm-docker
[![Docker Automated build](https://img.shields.io/docker/automated/jrottenberg/ffmpeg.svg?style=flat)](https://hub.docker.com/r/yasuumi/rust-wasm-docker/)

### Usage

・First, Run Container
```
docker run --rm -it -v $(pwd):/rust-wasm-docker -p 8000:8000  yasuumi/rust-wasm-docker
```

・Second, Build & Run Server in Container

```
cd ./rust-wasm-docker
cargo build --bin hello --target wasm32-unknown-emscripten --release
cp ./target/wasm32-unknown-emscripten/release/deps/hello-[a-z0-9]*.wasm ./tmp/hello.wasm
cp ./target/wasm32-unknown-emscripten/release/hello.js ./tmp/hello.js
python -m SimpleHTTPServer
```

・Third, Open http://localhost:8000/examples/hello.html & Check Console

・Additional, example of call js from rust with https://github.com/hajifkd/emval-rs
