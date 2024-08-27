# pz-hiring

A hiring phantom-zone application usable in a browser & in a node server, built in the non-interactive setting.

Rust server implementation & connection to JS apps will be added soon.

## How to use

### Testing

To test the hiring query:
`cargo test --release -- --nocapture ni_hiring_query`

### Web

To build the web version of the project, you'll need to set the non-interactive feature for phantom-zone in your `Cargo.toml` Then run: `wasm-pack build --target web --out-dir ./pz_hiring_browser`

To run the web version of the project, first run: `npm install -g http-server`. Then in the `/pz_hiring_browser` directory, run `http-server`. Finally, go to `localhost:8080` in your browser while viewing the console.

### Node

To build the node version, run `wasm-pack build --target nodejs --out-dir ./pz_hiring_node`.

To run the node version, you can then go to `./node_test`, run `npm i`, and then run `node index.js`.

You can use the functions in the `pz-hiring` package, which you can find at https://www.npmjs.com/package/pz-hiring.
