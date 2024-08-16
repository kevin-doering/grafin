use std::env;

use clap::Parser;
use dotenvy::dotenv;

use fi_grafana::api::grafana::GrafanaClient;
use fi_grafana::cli::shell::{Cli, handle_add, handle_del, handle_get, handle_set, RequestMethod};

#[tokio::main]
async fn main() {
    dotenv().expect(".env file not found");
    let cli = Cli::parse();
    let api = env::var("GRAFANA_API_PATH").expect("GRAFANA_API_PATH env var not found");
    let token = env::var("SERVICE_ACCOUNT_TOKEN").expect("SERVICE_ACCOUNT_TOKEN env var not found");
    let client = GrafanaClient::new(reqwest::Client::new(), api, token);
    match cli.method {
        RequestMethod::Add(request) => {
            handle_add(&client, request).await;
        }
        RequestMethod::Get(request) => {
            handle_get(&client, request).await;
        }
        RequestMethod::Set(request) => {
            handle_set(&client, request).await;
        }
        RequestMethod::Del(request) => {
            handle_del(&client, request).await;
        }
    }
}
