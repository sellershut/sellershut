use std::pin::Pin;
use std::{future::Future, ops::Deref};

#[cfg(not(target_arch = "wasm32"))]
use oauth2::HttpResponse;
use oauth2::{AsyncHttpClient, HttpClientError, HttpRequest, http};

#[derive(Clone)]
pub struct AuthHttpClient(reqwest::Client);

impl Deref for AuthHttpClient {
    type Target = reqwest::Client;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<reqwest::Client> for AuthHttpClient {
    fn from(value: reqwest::Client) -> Self {
        Self(value)
    }
}

impl<'c> AsyncHttpClient<'c> for AuthHttpClient {
    type Error = HttpClientError<reqwest::Error>;

    #[cfg(target_arch = "wasm32")]
    type Future = Pin<Box<dyn Future<Output = Result<HttpResponse, Self::Error>> + 'c>>;
    #[cfg(not(target_arch = "wasm32"))]
    type Future =
        Pin<Box<dyn Future<Output = Result<HttpResponse, Self::Error>> + Send + Sync + 'c>>;

    fn call(&'c self, request: HttpRequest) -> Self::Future {
        Box::pin(async move {
            let response = self
                .0
                .execute(request.try_into().map_err(Box::new)?)
                .await
                .map_err(Box::new)?;

            let mut builder = http::Response::builder().status(response.status());

            #[cfg(not(target_arch = "wasm32"))]
            {
                builder = builder.version(response.version());
            }

            for (name, value) in response.headers().iter() {
                builder = builder.header(name, value);
            }

            builder
                .body(response.bytes().await.map_err(Box::new)?.to_vec())
                .map_err(HttpClientError::Http)
        })
    }
}
