use dotenv_codegen::dotenv;
use serde::{de::DeserializeOwned, Serialize};
use crate::types::{ErrorInfo, Error};
use serde_json::{from_str, Value};

const API_PROXY: &str = dotenv!("BASE_PROXY");
// const API_PROXY: &str = "http://localhost:8080/api";

pub async fn request<B, T>(method: reqwasm::http::Method, url: String, body: B) -> Result<T, Error>
where
    T: DeserializeOwned + 'static + std::fmt::Debug,
    B: Serialize + std::fmt::Debug,
{
    let allow_body = match method {
        reqwasm::http::Method::POST => true,
        reqwasm::http::Method::PUT => true,
        _ => false
    };
    let url = format!("{}{}", API_PROXY, url);
    let mut builder = reqwasm::http::Request::new(&url).header("Content-type", "application/json").method(method);

    if allow_body {
        builder = builder.body(serde_json::to_string(&body).unwrap());
    }

    let response = builder.send().await;


    if let Ok(data) = response {
        if data.ok() {
            log::info!("get_all data {:#?}", &data);
            // let raw_data = data.text().await.unwrap();
            // log::info!("get_all raw_data {:#?}", &raw_data);
            // let _json = data.json().await;
            // log::info!("get_all _json {:#?}", &_json);
            let data: Result<T, _> = data.json::<T>().await;
            // let data: Result<T, _> = data.json::<T>().await;
            if let Ok(data) = data {
                log::info!("ok --- {:#?}", data);
                Ok(data)
            } else {
                log::info!("error --- {:#?}", data);
                Err(Error::DeserializeError)
            }
        } else {
            match data.status() {
                401 => Err(Error::Unauthorized),
                403 => Err(Error::Forbidden),
                404 => Err(Error::NotFound),
                500 => Err(Error::InternalServerError),
                422 => {
                    let data: Result<ErrorInfo, _> = data.json::<ErrorInfo>().await;
                    if let Ok(data) = data {
                        Err(Error::UnprocessableEntity(data))
                    } else {
                        Err(Error::DeserializeError)
                    }
                }
                _ => Err(Error::RequestError),
            }
        }
    } else {
                log::info!("error");
        Err(Error::RequestError)
    }
}

/// Delete request
pub async fn request_delete<T>(url: String) -> Result<T, Error>
where
    T: DeserializeOwned + 'static + std::fmt::Debug,
{
    request(reqwasm::http::Method::DELETE, url, ()).await
}

/// Get request
pub async fn request_get<T>(url: String) -> Result<T, Error>
where
    T: DeserializeOwned + 'static + std::fmt::Debug,
{
    request(reqwasm::http::Method::GET, url, ()).await
}

/// Post request with a body
pub async fn request_post<B, T>(url: String, body: B) -> Result<T, Error>
where
    T: DeserializeOwned + 'static + std::fmt::Debug,
    B: Serialize + std::fmt::Debug,
{
    request(reqwasm::http::Method::POST, url, body).await
}

/// Put request with a body
pub async fn request_put<B, T>(url: String, body: B) -> Result<T, Error>
where
    T: DeserializeOwned + 'static + std::fmt::Debug,
    B: Serialize + std::fmt::Debug,
{
    request(reqwasm::http::Method::PUT, url, body).await
}

/// Set limit for pagination
pub fn limit(count: u32, p: u32) -> String {
    let offset = if p > 0 { p * count } else { 0 };
    format!("limit={}&offset={}", count, offset)
}