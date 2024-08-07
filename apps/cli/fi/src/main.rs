use std::env;

use clap::Parser;
use dotenvy::dotenv;

use fi_grafana::api::grafana::GrafanaClient;
use fi_grafana::cli::shell::{CrudRequest, FI, handle_del, handle_get, handle_post, handle_put};

#[tokio::main]
async fn main() {
    dotenv().expect(".env file not found");

    let cli = FI::parse();
    let api = env::var("GRAFANA_API_PATH").expect("GRAFANA_API_PATH env var not found");
    let token = env::var("SERVICE_ACCOUNT_TOKEN").expect("SERVICE_ACCOUNT_TOKEN env var not found");
    let client = GrafanaClient::new(reqwest::Client::new(), api, token);
    match cli.method {
        CrudRequest::Add(request) => {
            handle_post(&client, request).await;
        }
        CrudRequest::Get(request) => {
            handle_get(&client, request).await;
        }
        CrudRequest::Set(request) => {
            handle_put(&client, request).await;
        }
        CrudRequest::Del(request) => {
            handle_del(&client, request).await;
        }
    }
}
