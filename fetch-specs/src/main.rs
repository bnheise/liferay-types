use anyhow::Ok;
use apis::API_LIST;
use dotenv::dotenv;
use liferay_client::LiferayClient;
use log::info;
use std::env;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;

mod apis;
mod liferay_client;

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    if env::var("RUST_LOG").is_err() {
        env::set_var("RUST_LOG", "info")
    }
    env_logger::init();

    dotenv().ok();

    info!("Fetching all headless api specs...");
    let client = LiferayClient::from_env()?;
    for (api, version) in API_LIST.iter() {
        info!("Retrieving spec for {api} {version}...");
        let def = client
            .get(format!(
                "http://localhost:8080/o/{api}/{version}/openapi.json"
            ))
            .send()
            .await?
            .text()
            .await?;
        info!("Spec received.");

        info!("Writing spec field...");
        let mut file = File::create(PathBuf::from(format!("../specs/{api}-{version}.json")))?;
        write!(file, "{def}")?;
        info!("Done.");
    }

    info!("Finished fetching all headless api specs...");
    Ok(())
}
