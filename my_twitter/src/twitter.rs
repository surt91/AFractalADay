extern crate json;
extern crate egg_mode;
extern crate tokio;

use std::error::Error;
use std::fs;
use std::io;
use std::io::prelude::*;
// use self::e::*;

use self::tokio::runtime::current_thread::block_on_all;

use self::egg_mode::media::{UploadBuilder, media_types};
use self::egg_mode::tweet::DraftTweet;
use self::egg_mode::{Token, KeyPair};

fn read_token() -> io::Result<Token> {
    let mut file = fs::File::open("keys_and_secrets.json")?;
    let mut raw = String::new();
    file.read_to_string(&mut raw)?;
    let obj = json::parse(&raw).expect("'keys_and_secrets.json' contains no valid json!");

    let con_token = KeyPair::new(
        obj["consumer_key"].to_string(),
        obj["consumer_secret"].to_string()
    );
    let access_token = KeyPair::new(
        obj["access_token_key"].to_string(),
        obj["access_token_secret"].to_string()
    );
    let token = Token::Access {
                    consumer: con_token,
                    access: access_token,
    };

    Ok(token)
}

pub fn tweet_image(text: &str, image_filename: &str) -> Result<(), Box<dyn Error>> {
    let mut image_file = fs::File::open(image_filename.trim())?;
    let mut buffer = Vec::new();
    image_file.read_to_end(&mut buffer)?;

    let token = read_token()?;

    let builder = UploadBuilder::new(buffer, media_types::image_png());
    let media_handle = block_on_all(builder.call(&token))?;

    let draft = DraftTweet::new(text)
                           .media_ids(&[media_handle.id]);

    block_on_all(draft.send(&token))?;

    Ok(())
}
