# pz-hiring

Some browser and Node.js code for doing private hiring matching. Built on top of Gauss Lab's [phantom-zone](https://github.com/gausslabs/phantom-zone)

## How to use

To test the hiring query:
`cargo test --release -- --nocapture ni_hiring_query`

To build the web version of the project, you'll need to set the right interactive vs non-interactive feature for phantom-zone in your `Cargo.toml`. Then, include the file you'd like to convert in wasm in `lib.rs`. Then run:
`wasm-pack build --target web --out-dir ./folder`

To run the web version of the project, first run: `npm install -g http-server`. Then in the `/ni_hiring_web` directory, run `http-server`. Finally, go to `localhost:8080` in your browser while viewing the console.
