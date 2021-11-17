# A Fractal A Day

<a href="https://twitter.com/AFractalADay/status/962651103325310976" target="_blank"><img align="right" width="256" height="256" alt="4 Möbius transformations with 7-fold rotational symmetry" src="extra/example_fractal.webp"></a>

[![Build Status](https://travis-ci.org/surt91/AFractalADay.svg?branch=master)](https://travis-ci.org/surt91/AFractalADay)

This is the Twitter bot [@AFractalADay](https://twitter.com/AFractalADay),
which tweets pictures of random fractals.

Two blog articles describing the fractals and -- more importantly -- showing example fractals
can be found [here](https://blog.schawe.me/randomFractals.html) and [here](https://blog.schawe.me/more-fractals.html).

Documentation at [surt91.github.io/AFractalADay](https://surt91.github.io/AFractalADay/).

## Setup

This program is pure rust with an optional dependency on Imagemagick and optipng
which will be called through the shell.

Just use cargo: `cargo run --release --features="binaries"`

**Important:** Do not forget to put in valid keys and secrets in `keys_and_secrets.json`.

## Dependencies

*   Imagemagick (optional)
*   optipng (optional)
