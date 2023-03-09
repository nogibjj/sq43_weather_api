use reqwest::Result;
use std::time::Duration;
use reqwest::ClientBuilder;

#[tokio::main]
async fn main() -> Result<()> {
    let user = "sqing73";
    let request_url = "https://weatherapi-com.p.rapidapi.com/current.json?q=48.8567%2C2.3508";
    println!("{}", request_url);

    let timeout = Duration::new(5, 0);
    let client = ClientBuilder::new().timeout(timeout).build()?;
    // let mut headers = HeaderMap::new();
    let response = client
        .get(request_url)
        .header("X-RapidAPI-Key", "62029c271cmsh91bad56c294759ep16c88bjsn412348323fac")
        .header("X-RapidAPI-Host", "weatherapi-com.p.rapidapi.com")
        .send()
        .await?;
    println!("Success! {:?}", response);


    Ok(())
}