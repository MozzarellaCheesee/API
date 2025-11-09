// use actix_web::{get, web, HttpResponse, Responder};
// use crate::structs::{AppState, AuthInput};
// 
// #[get("/api/auth/confirm")]
// pub async fn confirm_get(
//     _user_data: web::Json<AuthInput>,
//     app_state: web::Data<AppState<'_>>,
// ) -> impl Responder {
//     let _conn = app_state.pool.get().await.expect("Не удалось получить соединение");
//     HttpResponse::Ok()
// }