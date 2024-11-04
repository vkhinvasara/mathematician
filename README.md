

## About

This project is a WebAssembly package for basic mathematical operations, written in Rust and compiled to WebAssembly using `wasm-pack`. It provides a simple API for performing addition, subtraction, multiplication, and division.

## 🚀 Getting Started
#### Installation
```sh
npm install methematician
```
#### Usage
Here is an example of how to use the Methematician class in a JavaScript project:
```js
import { Methematician } from 'methematician';

async function run() {
    const m = Methematician.new(10, 5);
    console.log("10 + 5 =", m.add());
    console.log("10 - 5 =", m.sub());
    console.log("10 * 5 =", m.mul());
    console.log("10 / 5 =", m.div());
}

run();
```