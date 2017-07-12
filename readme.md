# A Fractal A Day

[![Build Status](https://travis-ci.org/surt91/AFractalADay.svg?branch=master)](https://travis-ci.org/surt91/AFractalADay)

This is the Twitter bot [@AFractalADay](https://twitter.com/AFractalADay),
which tweets pictures of random fractals.

<blockquote class="twitter-tweet" data-lang="en"><p lang="en" dir="ltr">Fractal Flame: &#39;Horseshoe&#39; Variation, 5 affine transformations <a href="https://t.co/5UShXwtSw8">pic.twitter.com/5UShXwtSw8</a></p>&mdash; randomFractals (@AFractalADay) <a href="https://twitter.com/AFractalADay/status/881103896374124544">1. Juli 2017</a></blockquote>
<script async src="//platform.twitter.com/widgets.js" charset="utf-8"></script>

Documentation at [surt91.github.io/AFractalADay](https://surt91.github.io/AFractalADay/).

## Setup

This program is pure rust with an optional dependency on Imagemagick and optipng
which will be called through the shell.

Just use cargo: `cargo run --release`

**Important:** Do not forget to put in valid keys and secrets in `keys_and_secrets.json`.

## Dependencies

    * Imagemagick (optional)
    * optipng (optional)
