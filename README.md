# pz-web

Compiltion of a few useful phantom-zone applications usable in a browser.

## How to use

To test the hiring query:
`cargo test --release -- --nocapture ni_hiring_query`

To build the web version of the project:
`wasm-pack build --target web --out-dir ./web`

To run the web version of the project, first run:

`npm install -g http-server`

and then in the `/web` directory, run

`http-server`

and go to `localhost:8080` in your browser while viewing the console.
