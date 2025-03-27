//Retrieves a joke

use exitfailure::ExitFailure;
use reqwest::Url;
use serde_derive::{Deserialize, Serialize};
//use std::env;


#[derive(Serialize, Deserialize, Debug)]
struct Joke {
    #[serde(rename = "type")]
    tp: String,
    setup: String,
    punchline: String,
    id: i32,
}

impl Joke {
    async fn get() -> Result<Self, ExitFailure> {
	let url = format!("https://official-joke-api.appspot.com/random_joke");
	let url = Url::parse(&*url)?;
	let res = reqwest::get(url).await?.json::<Joke>().await?;

	Ok(res)
    }
}

#[tokio::main]
async fn main() -> Result<(), ExitFailure> {

    let jk = Joke::get().await?;

    println!("{} {}", jk.setup, jk.punchline);

    Ok(())
}
