# Learn.ai Rust Backend

Handles part of the Learn.ai backend. Responsible for calculating engagement scores for the largest face in each image.

## Overview

- Sends binary image via POST request endpoint
- Is sent through the Azure Face API
- Parses all face detection data to Rust structs
- Calculates an engagement score from many different features

## Engagement Score calculation

![Engagement score](https://i.imgur.com/C3gUl0E.png)
![Head yaw normalization](https://i.imgur.com/rEmVIMD.png)
![Head pitch normalization](https://i.imgur.com/PBJuhVY.png)
![Emotion normalization graph](https://i.imgur.com/ZYbeVfa.png)
![Emotion normalization](https://i.imgur.com/odsZBSG.png)

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