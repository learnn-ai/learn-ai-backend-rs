# Learn.ai Rust Backend

Handles part of the Learn.ai backend. Responsible for calculating engagement scores for the largest face in each image.

## Features

- POST requests an image to the endpoint
- Sends to the Azure Face API
- Parses response to helpful, reusable Rust struct
- Implements helpful functions such as calculating an engagement score for a face

## Building

Before you are able to run the server, you must create a file called `config.rs` and add this code to it:

```rust
pub static KEY: &'static str = "<Azure Face API key here>";
pub static ENDPOINT: &'static str = "<Azure Face API endpoint here>";
```

Afterwards, place the `config.rs` under the `src` folder.

Now you use Cargo, the Rust package manager, to build the executable. Once you have installed Cargo, clone this repository, add your `config.rs`, and run:

```
cargo build --release
```

the resulting executable will be under `target/release/`.