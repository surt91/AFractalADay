# A Fractal A Day

[![Build Status](https://travis-ci.org/surt91/AFractalADay.svg?branch=master)](https://travis-ci.org/surt91/AFractalADay)

This is the Twitter bot [@AFractalADay](https://twitter.com/AFractalADay),
which tweets pictures of random fractals.

Documentation at [surt91.github.io/AFractalADay](https://surt91.github.io/AFractalADay/).

## Setup

This program is pure rust with an optional dependency on Imagemagick and optipng
which will be called through the shell.

Just use cargo: `cargo run --release --features="binaries"`

**Important:** Do not forget to put in valid keys and secrets in `keys_and_secrets.json`.

## Dependencies

*   Imagemagick (optional)
*   optipng (optional)
