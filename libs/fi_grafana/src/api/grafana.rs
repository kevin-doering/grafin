use serde::Serialize;

pub struct GrafanaClient {
    pub client: reqwest::Client,
    pub api: String,
    pub token: String,
}

impl GrafanaClient {
    pub fn new(client: reqwest::Client, api: String, token: String) -> Self {
        Self {
            client,
            api,
            token,
        }
    }

    pub async fn post<T: Serialize + ?Sized>(&self, resource: &str, request_data: &T) -> Result<reqwest::Response, reqwest::Error> {
        Ok(self.client
            .post(format!("{}{}", self.api, resource))
            .header("Accept", "application/json")
            .header("Content-Type", "application/json")
            .header("Authorization", format!("Bearer {}", self.token))
            .json(request_data)
            .send()
            .await?)
    }

    pub async fn get(&self, resource: &str) -> Result<reqwest::Response, reqwest::Error> {
        Ok(self.client
            .get(format!("{}{}", self.api, resource))
            .header("Accept", "application/json")
            .header("Content-Type", "application/json")
            .header("Authorization", format!("Bearer {}", self.token))
            .send()
            .await?)
    }

    pub async fn put<T: Serialize + ?Sized>(&self, resource: &str, request_data: &T) -> Result<reqwest::Response, reqwest::Error> {
        Ok(self.client
            .put(format!("{}{}", self.api, resource))
            .header("Accept", "application/json")
            .header("Content-Type", "application/json")
            .header("Authorization", format!("Bearer {}", self.token))
            .json(request_data)
            .send()
            .await?)
    }

    pub async fn del(&self, resource: &str) -> Result<reqwest::Response, reqwest::Error> {
        Ok(self.client
            .delete(format!("{}{}", self.api, resource))
            .header("Accept", "application/json")
            .header("Content-Type", "application/json")
            .header("Authorization", format!("Bearer {}", self.token))
            .send()
            .await?)
    }

    pub async fn query<T: Serialize + ?Sized>(&self, resource: &str, params: &T) -> Result<reqwest::Response, reqwest::Error> {
        Ok(self.client
            .get(format!("{}{}", self.api, resource))
            .header("Accept", "application/json")
            .header("Content-Type", "application/json")
            .header("Authorization", format!("Bearer {}", self.token))
            .query(params)
            .send()
            .await?)
    }
}