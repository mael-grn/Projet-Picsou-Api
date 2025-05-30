use rocket::http::Status;
use rocket::serde::json::Json;
use crate::auth::AuthenticatedUser;
use crate::services::group_service;
use crate::models::group::{Group, GroupWithUser, UpdatableGroup};

pub(crate) mod expenses;
pub(crate) mod users;
pub(crate) mod balance;

/**
Ajoute un groupe après avoir vérifié que l'utilisateur est bien le propriétaire du groupe.
*/
#[post("/", format = "application/json", data = "<group>")]
pub fn create_group(group: Json<GroupWithUser>, authenticated_user: AuthenticatedUser) -> Result<Json<Group>, (Status, String)> {
    let group_with_user = group.into_inner();
    match group_service::create_group(&group_with_user, &authenticated_user) {
        Ok(group) => {
            Ok(Json(group))
        }
        Err(status) => Err(status),
    }
}

/**
Récupère le groupes si l'utilisateur authentifié et membre de celui ci.
*/

#[get("/<group_id>", format = "application/json")]
pub fn get_group_by_id(group_id: i32, authenticated_user: AuthenticatedUser) -> Result<Json<Group>, (Status, String)> {
    match group_service::get_group_by_id(&group_id, &authenticated_user) {
        Ok(group) => {
            Ok(Json(group))
        }
        Err(status) => Err(status),
    }
}

/**
Supprime le groupe si l'utilisateur authentifié est le propriétaire du groupe.
*/
#[delete("/<group_id>", format = "application/json")]
pub fn delete_group(group_id: i32, authenticated_user: AuthenticatedUser) -> Result<String, (Status, String)> {
    match group_service::delete_group(&group_id, &authenticated_user) {
        Ok(message) => Ok(message),
        Err(status) => Err(status),
    }
}

/**
Met à jour le groupe si l'utilisateur authentifié est le propriétaire ou l'admin du groupe.
*/
#[put("/<group_id>", format = "application/json", data = "<group>")]
pub fn modify_group(group_id: i32, group: Json<UpdatableGroup>, authenticated_user: AuthenticatedUser) -> Result<Json<Group>, (Status, String)> {
    let group_to_update = group.into_inner();
    match group_service::modify_group_service(&group_id, &group_to_update, &authenticated_user) {
        Ok(updated_group) => Ok(Json(updated_group)),
        Err(status) => Err(status),
    }
}