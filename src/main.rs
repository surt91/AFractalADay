#![feature(closure_to_fn_coercion)]

extern crate json;
extern crate twitter_api as twitter;
extern crate oauth_client as oauth;

#[macro_use] extern crate itertools;
mod newton_fractal;

use oauth::Token;
use std::fs::File;
use std::io::prelude::*;

use newton_fractal::NewtonFractal;

struct Auth {
    pub consumer_key: String,
    pub consumer_secret: String,
    pub access_token_key: String,
    pub access_token_secret: String
}

fn read_token(filename: &str) -> Auth {
    let mut file = File::open("keys_and_secrets.json").unwrap();
    let mut raw = String::new();
    file.read_to_string(&mut raw).unwrap();

    let obj = json::parse(&raw).unwrap();

    Auth { consumer_key: obj["consumer_key"].to_string(),
           consumer_secret: obj["consumer_secret"].to_string(),
           access_token_key: obj["access_token_key"].to_string(),
           access_token_secret: obj["access_token_secret"].to_string() }
}

fn twitter_update(auth: Auth) {
    let consumer = Token::new(auth.consumer_key, auth.consumer_secret);
    let access = Token::new(auth.access_token_key, auth.access_token_secret);

    // twitter::update_status(&consumer, &access, &status).unwrap();
}

fn main() {
    let a = NewtonFractal::new(|z| z.powf(4.) + z.sin() + 15.);

    a.render("test.png");
}
