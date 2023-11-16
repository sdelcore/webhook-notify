use reqwest::Error;
use std::env;

#[tokio::main]
async fn main() -> Result<(), Error> {
    // collect the command line arguments
    let args: Vec<String> = env::args().collect();

    // ensure there are enough parameters
    if args.len() < 3 {
        println!("Usage: program <title> <message>");
        return Ok(());
    }

    let title = &args[1];
    let message = &args[2];

    let webhook_url = match env::var("WEBHOOK_URL") {
        Ok(url) => url,
        Err(_) => {
            println!("WEBHOOK_URL environment variable not set");
            return Ok(());
        }
    };

    //Prepare the data to send
    let data = serde_json::json!({
        "title": title,
        "message": message,
    });

    let client = reqwest::Client::new();
    let res = client.post(&webhook_url)
        .header(reqwest::header::CONTENT_TYPE, "application/json")
        .body(data.to_string())
        .send()
        .await?;

    println!("Status: {:?}", res.status());
    Ok(())
}
