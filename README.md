# Rust Wasm Template

This is a GitHub template project used to generate new Rust WebAssembly (Wasm) projects. It provides a basic setup for building Rust applications that can run in a web browser using WebAssembly. The template includes configuration files and a simple example code to get you started quickly.

## Contents

The project contains the following files:

* cargo.toml: The project's configuration file, which manages dependencies and project metadata.
* index.html: An HTML file that loads the Wasm module and provides a canvas element for rendering.
* index.js: A JavaScript file that imports and runs the Wasm module.
* package.json: A Node.js package configuration file that defines build and serve scripts.
* webpack.config.js: The configuration file for Webpack, a module bundler used to bundle JavaScript and Wasm code.
* src/lib.rs: The main Rust source code file, which includes a simple example that logs a message to the console.

## Usage

To create a new Rust Wasm project using this template, you can follow these steps:

1. Click on the "Use this template" button on the GitHub repository page to create a new repository based on this template.
2. Clone the newly created repository to your local machine.
3. Install the required dependencies by running the following command:
```shell
npm install
```
4. Build the project using Webpack by running the following command:
```shell
npm run build
```
5. Start a local development server by running the following command:
```shell
npm run serve
```
This will launch a development server that serves the project files. You can access the application in your web browser by visiting http://localhost:8080.
6. Customize the project by modifying the Rust code in the src/lib.rs file and the HTML code in the index.html file. You can also modify the project's dependencies and build configuration in the cargo.toml, package.json, and webpack.config.js files as needed.
7. Deploy the project to a web server of your choice by hosting the contents of the dist directory.

## License

This project is licensed under the MIT License. Feel free to use it as a starting point for your own Rust Wasm projects.

## Acknowledgments

This template is built using the following libraries and tools:

* [Rust](https://www.rust-lang.org/): The programming language used for writing the Wasm module.
* [wasm-bindgen](https://github.com/rustwasm/wasm-bindgen): A Rust library for interacting with JavaScript and Web APIs from Wasm.
* [web-sys](https://github.com/rustwasm/wasm-bindgen/tree/main/crates/web-sys): A Rust library providing high-level access to Web APIs.
* [Webpack](https://webpack.js.org/): A module bundler used to bundle JavaScript and Wasm code.
* [@wasm-tool/wasm-pack-plugin](https://github.com/wasm-tool/wasm-pack-plugin): A Webpack plugin for building and optimizing Wasm code.
* [html-webpack-plugin](https://github.com/jantimon/html-webpack-plugin): A Webpack plugin for generating HTML files.
* [text-encoding](https://www.npmjs.com/package/text-encoding): A JavaScript library for encoding and decoding text.
* [webpack-dev-server](https://webpack.js.org/configuration/dev-server/): A development server for Webpack.

Feel free to consult their respective documentation for more information on how to use them effectively.
