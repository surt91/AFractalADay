# A Fractal A Day

[![Build Status](https://travis-ci.org/surt91/AFractalADay.svg?branch=master)](https://travis-ci.org/surt91/AFractalADay)

This is a Twitter bot, which can tweet pictures of random fractals.

## Setup

This program is pure rust with an optional dependency on Imagemagick and optipng
which will be called through the shell.

Just use cargo: `cargo run --release`

**Important:** Do not forget to put in valid keys and secrets in `keys_and_secrets.json`.

## Dependencies

    * Imagemagick (optional)
    * optipng (optional)
