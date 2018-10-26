# Draco

> Draco is a Rust library for building client side web applications with Web
> Assembly.

Draco implements a Redux and Elm inspired architecture. The core of a Draco
application consists of two functions:

- `render`: returns a description of what should be rendered on the screen.
  This description is efficiently applied to the browser's DOM with a minimal
  number of patches using Virtual DOM diffing.
- `update`: receives a message and updates the state of the application. The
  application is re-rendered after every update.

## Getting Started

There's a starter crate available [here][starter].

To run it, clone the repository:

    $ git clone https://github.com/utkarshkukreti/draco-starter
    $ cd draco-starter

and follow the instructions in [its README][starter].

## Examples

> [Live Demo](https://draco-examples.netlify.com/)

There's not a lot of documentation present right now. If you want to learn
more, the best way is to read the source code of [the examples](./examples).

We recommend starting with [Hello World](./examples/hello_world.rs), followed
by [Counter](./examples/counter.rs), and then
[Counters](./examples/counters.rs).

To build the examples, you'll need Ruby and
[wasm-bindgen-cli][wasm-bindgen-cli] installed.

    $ cd /path/to/this/repo
    $ rake

Now start an HTTP server of your choice <sup>[1](#http-server)</sup> and open
`target/examples/index.html` in your browser to run the examples.

<sup>[<span id="http-server">1</span>] Python 2/3's built in HTTP Server (and
possibly others) does not work as browsers require `.wasm` files to be served
with a MIME type of `application/wasm` which they do not do. Try
[serve](https://www.npmjs.com/package/serve) if your HTTP server of choice
does not work.</sup>

[starter]: https://github.com/utkarshkukreti/draco-starter
[wasm-bindgen-cli]: https://rustwasm.github.io/wasm-bindgen/whirlwind-tour/basic-usage.html
