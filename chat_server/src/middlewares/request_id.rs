use axum::extract::Request;
use axum::http::HeaderValue;
use axum::middleware::Next;
use axum::response::Response;
use tracing::warn;

const REQUEST_ID_HEADER: &str = "x-request-id";
pub async fn set_request_id(
    mut req: Request,
    next: Next,
) -> Response {
    //check x-request-id if not exist, generate one,else do nothing
    let req_id = match req.headers().get(REQUEST_ID_HEADER) {
        Some(v) => {
            Some(v.clone())
        },
        None => {
            let request_id = uuid::Uuid::now_v7().to_string();
            match HeaderValue::from_str(&request_id){ 
                Ok(v) => {
                    req.headers_mut().insert(REQUEST_ID_HEADER, v.clone() );
                    Some(v)
                },
                Err(e) => {
                    warn!("Failed to generate request id : {}", e);
                    None
                }
            }
        }
    };
    let mut response = next.run(req).await;
    if let Some(v) =  req_id {
        response.headers_mut().insert(REQUEST_ID_HEADER,v.clone());
    }
    response
   
}