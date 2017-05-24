extern crate json;
extern crate tweetust;
extern crate std;

use std::error::Error;
use std::fmt;
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
    let obj = json::parse(&raw).unwrap();

    Ok(
        Auth { consumer_key: obj["consumer_key"].to_string(),
               consumer_secret: obj["consumer_secret"].to_string(),
               access_token_key: obj["access_token_key"].to_string(),
               access_token_secret: obj["access_token_secret"].to_string() }
    )
}

fn create_client<'a>() -> Client<'a> {
    load_config_file().unwrap()
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

    let init_res = client.media()
                         .upload_init_command(file_len, "image/png")
                         .media_category("tweet_image")
                         .execute()?;

    print!("\n{:?}\n\n", init_res);

    let mut buffer = Vec::new();
    // read the whole file
    image_file.read_to_end(&mut buffer)?;
    let mut io_buf = io::Cursor::new(&buffer);

    client.media().upload_append_command(init_res.object.media_id, 0)
          .media(&mut io_buf)
          .execute()?;

    let finalize_res = client.media()
                             .upload_finalize_command(init_res.object.media_id)
                             .execute()?;
    println!("\n{:?}", finalize_res);


    if let Some(models::ProcessingInfo { mut check_after_secs, .. }) = finalize_res.object.processing_info {
        while let Some(x) = check_after_secs {
            std::thread::sleep(std::time::Duration::from_secs(x as u64));

            let status_res = client.media().upload_status_command(init_res.object.media_id).execute()?;
            println!("\n{:?}", status_res);

            check_after_secs = status_res.object.processing_info.check_after_secs;
        }
    }

    // write_and_flush(format_args!("\nTweet: "));
    // let mut status = String::new();
    // io::stdin().read_line(&mut status).unwrap();

    println!(
        "\n{:?}",
        client.statuses()
              .update(text)
              .media_ids(Some(init_res.object.media_id))
              .execute()?
    );

    Ok(())
}
