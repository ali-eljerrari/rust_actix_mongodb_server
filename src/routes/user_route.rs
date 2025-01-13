/// User routes module
///
/// This module contains all the route handlers for user-related endpoints.
/// It provides REST API endpoints for CRUD operations on users.
use actix_web::{get, post, put, delete, web, HttpResponse, Responder};
use mongodb::bson::oid::ObjectId;
use crate::models::user_model::User;
use crate::AppState;
use crate::controllers::user_controller::UserController;

/// Get all users
///
/// Returns a list of all users in the system.
///
/// # Returns
///
/// * `200 OK` - List of users as JSON
/// * `500 Internal Server Error` - If database operation fails
#[get("/users")]
pub async fn get_users(state: web::Data<AppState>) -> impl Responder {
    match UserController::get_all_users(&state.db).await {
        Ok(users) => HttpResponse::Ok().json(users),
        Err(_) => HttpResponse::InternalServerError().json(serde_json::json!({
            "error": "Failed to fetch users"
        }))
    }
}

/// Get user by ID
///
/// Returns a single user matching the provided ID.
///
/// # Arguments
///
/// * `id` - User ID as path parameter
///
/// # Returns
///
/// * `200 OK` - User details as JSON
/// * `400 Bad Request` - If ID format is invalid
/// * `404 Not Found` - If user doesn't exist
/// * `500 Internal Server Error` - If database operation fails
#[get("/users/{id}")]
pub async fn get_user_by_id(
    state: web::Data<AppState>,
    path: web::Path<String>
) -> impl Responder {
    let user_id = match ObjectId::parse_str(path.into_inner()) {
        Ok(id) => id,
        Err(_) => return HttpResponse::BadRequest().json(serde_json::json!({
            "error": "Invalid user ID format"
        }))
    };

    match UserController::get_user_by_id(&state.db, user_id).await {
        Ok(Some(user)) => HttpResponse::Ok().json(user),
        Ok(None) => HttpResponse::NotFound().json(serde_json::json!({
            "error": "User not found"
        })),
        Err(_) => HttpResponse::InternalServerError().json(serde_json::json!({
            "error": "Failed to fetch user"
        }))
    }
}

/// Create new user
///
/// Creates a new user with the provided details.
///
/// # Arguments
///
/// * `user` - User details in request body as JSON
///
/// # Returns
///
/// * `201 Created` - Created user details as JSON
/// * `500 Internal Server Error` - If database operation fails
#[post("/users")]
pub async fn create_user(
    state: web::Data<AppState>,
    user: web::Json<User>
) -> impl Responder {
    match UserController::create_user(&state.db, user.into_inner()).await {
        Ok(created_user) => HttpResponse::Created().json(created_user),
        Err(_) => HttpResponse::InternalServerError().json(serde_json::json!({
            "error": "Failed to create user"
        }))
    }
}

/// Update existing user
///
/// Updates an existing user with the provided details.
///
/// # Arguments
///
/// * `id` - User ID as path parameter
/// * `user` - Updated user details in request body as JSON
///
/// # Returns
///
/// * `200 OK` - Updated user details as JSON
/// * `400 Bad Request` - If ID format is invalid
/// * `404 Not Found` - If user doesn't exist
/// * `500 Internal Server Error` - If database operation fails
#[put("/users/{id}")]
pub async fn update_user(
    state: web::Data<AppState>,
    path: web::Path<String>,
    user: web::Json<User>
) -> impl Responder {
    let user_id = match ObjectId::parse_str(path.into_inner()) {
        Ok(id) => id,
        Err(_) => return HttpResponse::BadRequest().json(serde_json::json!({
            "error": "Invalid user ID format"
        }))
    };

    match UserController::update_user(&state.db, user_id, user.into_inner()).await {
        Ok(Some(updated_user)) => HttpResponse::Ok().json(updated_user),
        Ok(None) => HttpResponse::NotFound().json(serde_json::json!({
            "error": "User not found"
        })),
        Err(_) => HttpResponse::InternalServerError().json(serde_json::json!({
            "error": "Failed to update user"
        }))
    }
}

/// Delete user
///
/// Deletes an existing user from the system.
///
/// # Arguments
///
/// * `id` - User ID as path parameter
///
/// # Returns
///
/// * `200 OK` - Success message as JSON
/// * `400 Bad Request` - If ID format is invalid
/// * `404 Not Found` - If user doesn't exist
/// * `500 Internal Server Error` - If database operation fails
#[delete("/users/{id}")]
pub async fn delete_user(
    state: web::Data<AppState>,
    path: web::Path<String>
) -> impl Responder {
    let user_id = match ObjectId::parse_str(path.into_inner()) {
        Ok(id) => id,
        Err(_) => return HttpResponse::BadRequest().json(serde_json::json!({
            "error": "Invalid user ID format"
        }))
    };

    match UserController::delete_user(&state.db, user_id).await {
        Ok(true) => HttpResponse::Ok().json(serde_json::json!({
            "message": "User deleted successfully"
        })),
        Ok(false) => HttpResponse::NotFound().json(serde_json::json!({
            "error": "User not found"
        })),
        Err(_) => HttpResponse::InternalServerError().json(serde_json::json!({
            "error": "Failed to delete user"
        }))
    }
}
