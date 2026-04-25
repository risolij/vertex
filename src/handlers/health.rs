use axum::Json;

pub async fn ruok() -> Json<&'static str> {
    Json("i'm ok")
}
