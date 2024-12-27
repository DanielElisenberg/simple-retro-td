# Simple Retro TD
A simple retro tower defence game. Made to learn the 
[bevy game engine](https://bevyengine.org) and 
[the ECS pattern](https://en.wikipedia.org/wiki/Entity_component_system).

## Play the game
Try the WASM build on [github pages](https://danielelisenberg.github.io/simple-retro-td)

## Built with
- [Language: Rust](https://www.rust-lang.org)
- [Game engine: Bevy](https://bevyengine.org)
- [Audio plugin: Kira](https://github.com/NiklasEi/bevy_kira_audio)
- [Built for WASM with: WASM-bindgen](https://github.com/rustwasm/wasm-bindgen)

## Notes
Bevy and ECS was an incredibly fun way to develop a game. Felt really productive
once I got more familiar with it. Not a great way to learn rust. For anyone 
looking to learn rust while developing games I would recommend starting with
[macroquad](https://macroquad.rs).

## Build
### Local
To build locally simply clone the repository and ```cargo run``` or ```cargo run --release```

### WASM
- Build with: ```cargo build --target wasm32-unknown-unknown --release```
- Generate javascript bindings: ```wasm-bindgen target/wasm32-unknown-unknown/release/simple-retro-td.wasm --outdir dist --target web```
- Make an [index.html](https://github.com/DanielElisenberg/simple-retro-td/blob/gh-pages/index.html) in the `dist` directory
- Serve with a simple webserver: ```cd dist && python -m http.server 8000```

