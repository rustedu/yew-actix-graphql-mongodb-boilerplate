use wasm_bindgen::{JsValue, __rt::IntoJsResult};
use dotenv_codegen::dotenv;
use serde::{de::DeserializeOwned, Serialize};
use crate::types::{ErrorInfo, Error};

const API_PROXY: &str = dotenv!("BASE_PROXY");

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
let a = JsValue::from_serde(&body).unwrap();
// log::info!("{:?}", a.into());
log::info!("{:?}", a);
        builder = builder.body(a);
    }

    let response = builder.send().await;

    log::info!("{:?}", response);

    if let Ok(data) = response {
    log::info!("{:?}", data.ok());
        if data.ok() {
            let data: Result<T, _> = data.json::<T>().await;
    log::info!("{:?}", data);
            if let Ok(data) = data {
                log::debug!("Response: {:?}", data);
                Ok(data)
            } else {
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

/// build all kinds of http request: post/get/delete etc.
// pub async fn request_by_reqwest<B, T>(method: reqwest::Method, url: String, body: B) -> Result<T, Error>
// where
//     T: DeserializeOwned + 'static + std::fmt::Debug,
//     B: Serialize + std::fmt::Debug,
// {
//     let allow_body = method == reqwest::Method::POST || method == reqwest::Method::PUT;
//     let url = format!("{}{}", API_ROOT, url);
//     log::info!("{}{}", method, url);
//     let mut builder = reqwest::Client::new()
//         .request(method, url)
//         .header("Content-Type", "application/json");
//     // if let Some(token) = get_token() {
//     //     builder = builder.bearer_auth(token);
//     // }

//     if allow_body {
//         builder = builder.json(&body);
//     }

//     let response = builder.send().await;

//     if let Ok(data) = response {
//         if data.status().is_success() {
//             let data: Result<T, _> = data.json::<T>().await;
//             if let Ok(data) = data {
//                 log::debug!("Response: {:?}", data);
//                 Ok(data)
//             } else {
//                 Err(Error::DeserializeError)
//             }
//         } else {
//             match data.status().as_u16() {
//                 401 => Err(Error::Unauthorized),
//                 403 => Err(Error::Forbidden),
//                 404 => Err(Error::NotFound),
//                 500 => Err(Error::InternalServerError),
//                 422 => {
//                     let data: Result<ErrorInfo, _> = data.json::<ErrorInfo>().await;
//                     if let Ok(data) = data {
//                         Err(Error::UnprocessableEntity(data))
//                     } else {
//                         Err(Error::DeserializeError)
//                     }
//                 }
//                 _ => Err(Error::RequestError),
//             }
//         }
//     } else {
//                 log::info!("error");
//         Err(Error::RequestError)
//     }
// }

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