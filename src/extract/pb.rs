// use axum::body::HttpBody;
// use axum::extract::rejection::{HeadersAlreadyExtracted, InvalidJsonBody};
// use axum::extract::{FromRequest, RequestParts};
// use axum::http::header;
// use axum::BoxError;
// use bytes::Bytes;
// use serde::de::DeserializeOwned;
// use std::io::Error;

// #[derive(Debug, Clone, Copy, Default)]
// struct Pb<T>(pub T);
//
// #[async_trait]
// impl<T, B> FromRequest<B> for Pb<T>
// where
//     T: prost::Message,
//     B: HttpBody + Send,
//     B::Data: Send,
//     B::Error: Into<BoxError>,
// {
//     type Rejection = (std::io::Error);
//
//     async fn from_request(req: &mut RequestParts<B>) -> Result<Self, Self::Rejection> {
//         if pb_content_type(req)? {
//             let mut body_bytes = Bytes::from_request(req).await?;
//
//             // let value = serde_json::from_slice(&bytes).map_err(InvalidJsonBody::from_err)?;
//             let value = T::decode(&mut body_bytes).map_err()?;
//             Ok(Pb(value))
//         } else {
//             Err(Error {})
//         }
//     }
// }
// fn pb_content_type<B>(req: &RequestParts<B>) -> Result<bool, HeadersAlreadyExtracted> {
//     let content_type = if let Some(content_type) = req
//         .headers()
//         .ok_or_else(HeadersAlreadyExtracted::default)?
//         .get(header::CONTENT_TYPE)
//     {
//         content_type
//     } else {
//         return Ok(false);
//     };
//
//     let content_type = if let Ok(content_type) = content_type.to_str() {
//         content_type
//     } else {
//         return Ok(false);
//     };
//
//     let mime = if let Ok(mime) = content_type.parse::<mime::Mime>() {
//         mime
//     } else {
//         return Ok(false);
//     };
//
//     let is_pb_content_type = mime.type_() == "application"
//         && (mime.subtype() == "octet-stream"
//             || mime
//                 .suffix()
//                 .filter(|name| *name == "octet-stream")
//                 .is_some());
//
//     Ok(is_pb_content_type)
// }
