# pz-web

web compilation of a few useful phantom-zone applications

## Commands

To test the hiring query:
`cargo test --release -- --nocapture hiring_query`

To build the web version of the project:
`wasm-pack build --target web --out-dir ./web`
