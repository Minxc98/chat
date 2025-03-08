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
            v.as_bytes().to_vec()
        },
        None => {
            let request_id = uuid::Uuid::now_v7().to_string();
            match request_id.parse() { 
                Ok(v) => {
                    req.headers_mut().insert(
                        REQUEST_ID_HEADER,
                        v,
                    );
                },
                Err(e) => {
                    warn!("Failed to generate request id : {}", e);
                }
            }
            request_id.as_bytes().to_vec()
        }
    };
    let mut response = next.run(req).await;
    match HeaderValue::from_bytes(&req_id) {
        Ok(v) => {
            response.headers_mut().insert(
                REQUEST_ID_HEADER,
                v,
            );
        },
        Err(e) => {
            warn!("Failed to set request id : {}", e);
        }
    }
    response
   
}