pub(crate) mod expenses;

use rocket::http::{Cookie, CookieJar, Status};
use rocket::serde::json::Json;
use crate::auth::AuthenticatedUser;
use crate::models::user::{InsertableUser, User};
use crate::services::user_service;

#[get("/<id>")]
pub fn get_user_by_id_route(id: i32, _authenticated_user: AuthenticatedUser) -> Result<Json<User>, (Status, String)> {
    match user_service::get_user_essentials_by_id(&id) {
            Ok(user) => {
                Ok(Json(user))
            }
            Err(status) => Err(status),
    }
}

#[get("/email/<email>")]
pub fn get_user_by_email_route(email: &str, _authenticated_user: AuthenticatedUser) -> Result<Json<User>, (Status, String)> {
    match user_service::get_user_essentials_by_email(&email) {
        Ok(user) => {
            Ok(Json(user))
        }
        Err(status) => Err(status),
    }
}



#[post("/", format = "application/json", data = "<insertable_user>")]
pub fn create_user(insertable_user: Json<InsertableUser>,  cookies: &CookieJar<'_>) -> Result<Json<User>, (Status, String)> {
    let insertable_user_entity = insertable_user.into_inner();
    match user_service::create_user(&insertable_user_entity) {
        Ok((user, token)) => {
            let mut cookie = Cookie::new("token", token);
            cookie.set_http_only(true);
            cookie.set_same_site(rocket::http::SameSite::Lax);
            cookies.add(cookie);
            Ok(Json(user))
        }
        Err(status) => Err(status),
    }
}