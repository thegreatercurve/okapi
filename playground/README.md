# Okapi Playground

A web-based GUI for testing the the Okapi Parser

### Installation

The playground has a local dependency on the WASM package in `crates/okapi_wasm`. To build the WebAssembly modules to JavaScript run the below:

```shell
cd ./crates/okapi_wasm

npm install -g wasm-pack

wasm-pack build --target web
```

Once, the WebAssembly module has been built, `cd` back into this folder and run the below:

```shell
npm install
```

### Run

The below should open a web app at `http://localhost:3000/`:

```shell
npm run dev
```
