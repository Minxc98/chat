use axum::response::IntoResponse;

pub(crate) async fn sign_in_handler() -> impl IntoResponse  {
    "sign_in".to_string()
}

pub(crate) async fn sign_up_handler() -> impl IntoResponse  {
    "sign_up".to_string()
}