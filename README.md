# `thought_loop`

I've been wanting to build/write my own blog for a while. While I might be succumbing to the [law of the instrument](https://en.wikipedia.org/wiki/Law_of_the_instrument), given how much I enjoy learning the language, I figured why not do it in rust?

this is currently built using rocket 0.4, may switch to 0.5 fairly soon. 

## Getting started

1. set rust to nightly: `rustup override set nightly`
2. create a .env file with the database credentials in the top level project directory. [see diesel getting started](https://diesel.rs/guides/getting-started)
3. run with `cargo run`
4. naviaget to [locally running instance](http://localhost:8000)