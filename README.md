# requirements

```
# tauri cli
[p]npm i -g tauri
cargo install --locked trunk
cargo install wasm-bindgen-cli
```

note: make sure cargo bin folder is in path (default `~/.cargo/bin`)

# develop

use tauri cli to start/build app:

```
tauri dev # start client/api watcher pipelines
tauri build # build client/api into binary
```
