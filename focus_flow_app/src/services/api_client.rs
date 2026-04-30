use dioxus::prelude::*;
use reqwest::Client;
use reqwest::StatusCode;
use serde::{de::DeserializeOwned, Serialize};
use std::fmt;

#[derive(Debug, Clone)]
pub enum ApiError {
    Network(String),
    InternalServerError(String),
    BadRequest(String),
    Unauthorized,
    Deserialization(String),
    Unknown(String),
}

impl fmt::Display for ApiError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Network(msg) => write!(f, "Network error: {}", msg),
            Self::InternalServerError(msg) => write!(f, "Server error: {}", msg),
            Self::BadRequest(msg) => write!(f, "Bad request: {}", msg),
            Self::Unauthorized => write!(f, "Unauthorized"),
            Self::Deserialization(msg) => write!(f, "Deserialization error: {}", msg),
            Self::Unknown(msg) => write!(f, "Unknown error: {}", msg),
        }
    }
}

impl From<reqwest::Error> for ApiError {
    fn from(err: reqwest::Error) -> Self {
        if err.is_timeout() || err.is_connect() {
            return ApiError::Network("Connection failed".to_string());
        }
        if err.is_decode() {
            return ApiError::Deserialization(err.to_string());
        }
        ApiError::Unknown(err.to_string())
    }
}

#[derive(Clone, Debug)]
pub struct ApiClient {
    client: Client,
    base_url: String,
    auth_token: Option<String>,
}

impl ApiClient {
    pub fn new(base_url: &str) -> Self {
        let mut headers = reqwest::header::HeaderMap::new();
        headers.insert(
            reqwest::header::ACCEPT,
            reqwest::header::HeaderValue::from_static("application/json"),
        );

        debug!("Initializing ApiClient with base_url: {}", base_url);
        Self {
            client: Client::builder()
                .default_headers(headers)
                .build()
                .unwrap_or_default(),
            base_url: base_url.to_string(),
            auth_token: None,
        }
    }

    pub fn set_base_url(&mut self, url: &str) {
        self.base_url = url.to_string();
    }

    pub fn set_auth_token(&mut self, token: Option<String>) {
        self.auth_token = token;
    }

    fn apply_auth(&self, request: reqwest::RequestBuilder) -> reqwest::RequestBuilder {
        match &self.auth_token {
            Some(token) => request.bearer_auth(token),
            None => request,
        }
    }

    fn build_url(&self, endpoint: &str, path_params: Option<&[(&str, &str)]>) -> String {
        let mut path = endpoint.to_string();
        if let Some(params) = path_params {
            for (key, value) in params {
                path = path.replace(&format!("{{{}}}", key), value);
            }
        }
        format!("{}{}", self.base_url, path)
    }

    async fn handle_response<R>(&self, response: reqwest::Response) -> Result<R, ApiError>
    where
        R: DeserializeOwned,
    {
        let status = response.status();

        if status.is_success() {
            return response.json::<R>().await.map_err(ApiError::from);
        }

        match status {
            StatusCode::UNAUTHORIZED => Err(ApiError::Unauthorized),
            s if s.is_client_error() => {
                let text = response.text().await.unwrap_or_default();
                Err(ApiError::BadRequest(text))
            }
            s if s.is_server_error() => {
                let text = response.text().await.unwrap_or_default();
                Err(ApiError::InternalServerError(text))
            }
            _ => Err(ApiError::Unknown(status.to_string())),
        }
    }

    pub async fn get<T>(
        &self,
        endpoint: &str,
        path_params: Option<&[(&str, &str)]>,
        query: Option<&[(&str, &str)]>,
    ) -> Result<T, ApiError>
    where
        T: DeserializeOwned,
    {
        let url = self.build_url(endpoint, path_params);
        let mut request = self.apply_auth(self.client.get(&url));
        if let Some(q) = query {
            request = request.query(q);
        }
        let response = request.send().await.map_err(ApiError::from)?;
        self.handle_response(response).await
    }

    pub async fn post<B, R>(
        &self,
        endpoint: &str,
        path_params: Option<&[(&str, &str)]>,
        query: Option<&[(&str, &str)]>,
        body: &B,
    ) -> Result<R, ApiError>
    where
        B: Serialize + ?Sized,
        R: DeserializeOwned,
    {
        let url = self.build_url(endpoint, path_params);
        let json_body = serde_json::to_string(body).unwrap_or_default();
        info!("Sending Body: {}", json_body);
        debug!("Request URL: {}", url);
        let mut request = self.apply_auth(self.client.post(&url).json(body));
        if let Some(q) = query {
            request = request.query(q);
        }
        let response = request.send().await.map_err(ApiError::from)?;
        self.handle_response(response).await
    }

    pub async fn put<B, R>(
        &self,
        endpoint: &str,
        path_params: Option<&[(&str, &str)]>,
        query: Option<&[(&str, &str)]>,
        body: &B,
    ) -> Result<R, ApiError>
    where
        B: Serialize + ?Sized,
        R: DeserializeOwned,
    {
        let url = self.build_url(endpoint, path_params);
        let mut request = self.apply_auth(self.client.put(&url).json(body));
        if let Some(q) = query {
            request = request.query(q);
        }
        let response = request.send().await.map_err(ApiError::from)?;
        self.handle_response(response).await
    }

    pub async fn delete<T>(
        &self,
        endpoint: &str,
        path_params: Option<&[(&str, &str)]>,
        query: Option<&[(&str, &str)]>,
    ) -> Result<T, ApiError>
    where
        T: DeserializeOwned,
    {
        let url = self.build_url(endpoint, path_params);
        let mut request = self.apply_auth(self.client.delete(&url));
        if let Some(q) = query {
            request = request.query(q);
        }
        let response = request.send().await.map_err(ApiError::from)?;
        self.handle_response(response).await
    }
}
