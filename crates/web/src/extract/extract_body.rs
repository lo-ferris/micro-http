use crate::body::OptionReqBody;
use crate::extract::{Form, FromRequest, Json};
use crate::RequestContext;
use async_trait::async_trait;
use bytes::Bytes;
use http_body_util::BodyExt;
use serde::Deserialize;
use micro_http::protocol::ParseError;

#[async_trait]
impl FromRequest for Bytes {
    type Output<'any> = Bytes;

    async fn from_request(_req: &RequestContext, body: OptionReqBody) -> Result<Self::Output<'static>, ParseError> {
        body.apply(|b| async { b.collect().await.map(|c| c.to_bytes()) }).await
    }
}

#[async_trait]
impl FromRequest for String {
    type Output<'any> = String;

    async fn from_request(req: &RequestContext, body: OptionReqBody) -> Result<Self::Output<'static>, ParseError> {
        let bytes = Bytes::from_request(req, body).await?;
        // todo: using character to decode
        match String::from_utf8(bytes.into()) {
            Ok(s) => Ok(s),
            Err(_) => Err(ParseError::invalid_body("request body is not utf8")),
        }
    }
}

#[async_trait]
impl<T> FromRequest for Form<T>
where
    T: for<'de> Deserialize<'de> + Send,
{
    type Output<'r> = Form<T>;

    async fn from_request<'r>(req: &'r RequestContext, body: OptionReqBody) -> Result<Self::Output<'r>, ParseError> {
        let bytes = Bytes::from_request(req, body).await?;
        serde_urlencoded::from_bytes::<'_, T>(&bytes)
            .map(|t| Form(t))
            .map_err(|e| ParseError::invalid_body(e.to_string()))
    }
}

#[async_trait]
impl<T> FromRequest for Json<T>
    where
        T: for<'de> Deserialize<'de> + Send,
{
    type Output<'r> = Json<T>;

    async fn from_request<'r>(req: &'r RequestContext, body: OptionReqBody) -> Result<Self::Output<'r>, ParseError> {
        let bytes = Bytes::from_request(req, body).await?;
        serde_json::from_slice::<'_, T>(&bytes)
            .map(|t| Json(t))
            .map_err(|e| ParseError::invalid_body(e.to_string()))
    }
}
