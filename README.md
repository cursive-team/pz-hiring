# pz-web

Compiltion of a few useful phantom-zone applications usable in a browser.

## How to use

To test the hiring query:
`cargo test --release -- --nocapture ni_hiring_query`

To build the web version of the project:
`wasm-pack build --target web --out-dir ./web`

To run the web version of the project, first run: `npm install -g http-server`. Then in the `/web` directory, run `http-server`. Finally, go to `localhost:8080` in your browser while viewing the console.

## Disclaimers

Right now everything (including server key aggregation) is happening in browser, in WASM, which means the e2e flow takes 4-5 minutes. Over the next few days, I will be moving most intensive parts to a server to improve overall time.
