use axum::extract::{FromRequestParts, Request, State};
use axum::http::{HeaderValue, StatusCode};
use axum::middleware::Next;
use axum::response::{IntoResponse, Response};
use axum_extra::headers::Authorization;
use axum_extra::headers::authorization::Bearer;
use crate::AppState;
use axum_extra::TypedHeader;
use tracing::warn;

pub async fn verify_token(
    State(state): State<AppState>,
    mut req: Request,
    next: Next,
) -> Response {
    let (mut parts,body) = req.into_parts();
    match TypedHeader::<Authorization<Bearer>>::from_request_parts(&mut parts,&state).await {
        Ok(TypedHeader(Authorization(bearer))) => {
            let token = bearer.token();
            state
        }
        Err(e)=>{
            let msg = format!("Failed to verify token : {}", e);
            warn!(msg);
            return (StatusCode::UNAUTHORIZED, msg).into_response()
        }
    }
    todo!()
    

}