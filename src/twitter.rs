extern crate json;
extern crate tweetust;
extern crate std;

use std::error::Error;
use std::fs;
use std::io;
use std::io::prelude::*;
use self::tweetust::*;

type Client<'a> = TwitterClient<OAuthAuthenticator<'a>, DefaultHttpHandler<conn::DefaultHttpsConnector>>;

struct Auth {
    pub consumer_key: String,
    pub consumer_secret: String,
    pub access_token_key: String,
    pub access_token_secret: String
}

fn read_token() -> io::Result<Auth> {
    let mut file = fs::File::open("keys_and_secrets.json")?;
    let mut raw = String::new();
    file.read_to_string(&mut raw)?;
    let obj = json::parse(&raw).expect("'keys_and_secrets.json' contains no valid json!");

    Ok(
        Auth { consumer_key: obj["consumer_key"].to_string(),
               consumer_secret: obj["consumer_secret"].to_string(),
               access_token_key: obj["access_token_key"].to_string(),
               access_token_secret: obj["access_token_secret"].to_string() }
    )
}

fn create_client<'a>() -> Client<'a> {
    load_config_file().expect("failed to load config from file")
}

fn load_config_file<'a>() -> io::Result<Client<'a>> {
    let auth = read_token()?;

    Ok(TwitterClient::new(
        OAuthAuthenticator::new(auth.consumer_key, auth.consumer_secret, auth.access_token_key, auth.access_token_secret),
        DefaultHttpHandler::with_https_connector().unwrap()
    ))
}

pub fn tweet_image(text: &str, image_filename: &str) -> Result<(), Box<Error>> {
    let client = create_client();

    let mut image_file = fs::File::open(image_filename.trim())?;
    let file_len = image_file.metadata()?.len();

    let mut buffer = Vec::new();
    // read the whole file
    image_file.read_to_end(&mut buffer)?;
    let mut io_buf = io::Cursor::new(&buffer);

    let init_res = client.media()
                         .upload_init_command(file_len, "image/png")
                         .media_category("tweet_image")
                         .execute()?;

    client.media().upload_append_command(init_res.object.media_id, 0)
          .media(&mut io_buf)
          .execute()?;

    client.media()
          .upload_finalize_command(init_res.object.media_id)
          .execute()?;

    // seems to always result in a parse error: why?
    client.statuses()
          .update(text)
          .media_ids(Some(init_res.object.media_id))
          .execute();

    Ok(())
}
