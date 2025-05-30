use crate::schema::groups_users::id_user;
use rocket::http::Status;
use projet_picsou_api::establish_connection;
use crate::models::group::{Group, InsertableGroup, UpdatableGroup};
use crate::schema::groups::dsl::{groups, id};
use diesel::prelude::*;
use crate::schema::groups_users::dsl::{groups_users, id as groups_users_id, status};
pub fn get_group_by_id(groups_id: &i32) -> Result<Group, (Status, String)> {
    let conn = &mut establish_connection();

    match groups.filter(id.eq(groups_id)).first::<Group>(conn) {
        Ok(group) => Ok(group), //If group is found, we return the first element
        Err(diesel::result::Error::NotFound) => Err((Status::NotFound, "This group does not exist".to_string())),
        Err(_) => Err((Status::InternalServerError, "An internal server error occurred while querying the database".to_string())),
    }
}

pub fn insert_group_transac(conn: &mut PgConnection, group: &InsertableGroup) -> Result<Group, (Status, String)> {
    match diesel::insert_into(groups).values(group).get_result::<Group>(conn) {
        Ok(group) => Ok(group),
        Err(_) => Err((Status::InternalServerError, "An internal server error occurred while querying the database".to_string())),
    }
}

pub fn insert_group(group: &InsertableGroup) -> Result<Group, (Status, String)> {
    let conn = &mut establish_connection();
    insert_group_transac(conn, group)
}

pub fn delete_group(group_id: &i32) -> Result<String, (Status, String)> {
    let conn = &mut establish_connection();
    match diesel::delete(groups.filter(id.eq(group_id))).execute(conn) {
        Ok(0) => Err((Status::NotFound, "Group not found".to_string())),
        Ok(_) => Ok("Group deleted successfully".to_string()),
        Err(_) => Err((Status::InternalServerError, "An internal server error occurred while deleting the group".to_string())),
    }
}

pub fn update_user_status_in_group(user_group_id: &i32, user_group_status: &i32) -> Result<String, (Status, String)> {
    let conn = &mut establish_connection();

    match diesel::update(groups_users)
        .filter(groups_users_id.eq(user_group_id))
        .set(status.eq(user_group_status))
        .execute(conn) {
        Ok(0) => Err((Status::NotFound, "Group user not found".to_string())),
        Ok(_) => Ok("Group user status updated successfully".to_string()),
        Err(_) => Err((Status::InternalServerError, "An internal server error occurred while updating the group user status".to_string())),
    }
}

pub fn get_all_groups_user_repository(user_id: &i32) -> Result<Vec<Group>, (Status, String)> {
    let conn = &mut establish_connection();
    
    match groups_users
        .inner_join(groups)
        .filter(id_user.eq(user_id))
        .select(groups::all_columns())
        .load::<Group>(conn)  {
        Ok(found_groups) => Ok(found_groups),
        Err(_) => Err((Status::NotFound, "Group user not found".to_string())),
    }
}

pub fn update_group(group_id: &i32, group: &UpdatableGroup) -> Result<Group, (Status, String)> {
    let conn = &mut establish_connection();

    match diesel::update(groups.filter(id.eq(group_id)))
        .set(group)
        .get_result::<Group>(conn) {
        Ok(updated_group) => Ok(updated_group),
        Err(diesel::result::Error::NotFound) => Err((Status::NotFound, "Group not found".to_string())),
        Err(_) => Err((Status::InternalServerError, "An internal server error occurred while updating the group".to_string())),
    }
}