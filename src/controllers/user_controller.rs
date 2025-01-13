use mongodb::Database;
use mongodb::error::Error as MongoError;
use mongodb::bson::oid::ObjectId;

use crate::models::user_model::User;
use crate::services::user_service::UserService;

/// Controller for handling user-related operations
pub struct UserController;

impl UserController {
    /// Retrieves all users from the database
    ///
    /// # Arguments
    ///
    /// * `db` - Reference to the MongoDB database connection
    ///
    /// # Returns
    ///
    /// Returns a Result containing:
    /// * `Ok(Vec<User>)` - Vector of all users on success
    /// * `Err(MongoError)` - MongoDB error on failure
    pub async fn get_all_users(db: &Database) -> Result<Vec<User>, MongoError> {
        UserService::find_all(db).await
    }

    /// Finds a user by their ID
    ///
    /// # Arguments
    ///
    /// * `db` - Reference to the MongoDB database connection
    /// * `id` - ObjectId of the user to find
    ///
    /// # Returns
    ///
    /// Returns a Result containing:
    /// * `Ok(Some(User))` - The found user
    /// * `Ok(None)` - No user found with given ID
    /// * `Err(MongoError)` - MongoDB error on failure
    pub async fn get_user_by_id(db: &Database, id: ObjectId) -> Result<Option<User>, MongoError> {
        UserService::find_by_id(db, id).await
    }

    /// Creates a new user in the database
    ///
    /// # Arguments
    ///
    /// * `db` - Reference to the MongoDB database connection
    /// * `user` - User instance to create
    ///
    /// # Returns
    ///
    /// Returns a Result containing:
    /// * `Ok(User)` - The created user with assigned ID
    /// * `Err(MongoError)` - MongoDB error on failure
    pub async fn create_user(db: &Database, user: User) -> Result<User, MongoError> {
        UserService::create(db, user).await
    }

    /// Updates an existing user in the database
    ///
    /// # Arguments
    ///
    /// * `db` - Reference to the MongoDB database connection
    /// * `id` - ObjectId of the user to update
    /// * `user` - Updated user data
    ///
    /// # Returns
    ///
    /// Returns a Result containing:
    /// * `Ok(Some(User))` - The updated user
    /// * `Ok(None)` - No user found with given ID
    /// * `Err(MongoError)` - MongoDB error on failure
    pub async fn update_user(db: &Database, id: ObjectId, user: User) -> Result<Option<User>, MongoError> {
        UserService::update(db, id, user).await
    }

    /// Deletes a user from the database
    ///
    /// # Arguments
    ///
    /// * `db` - Reference to the MongoDB database connection
    /// * `id` - ObjectId of the user to delete
    ///
    /// # Returns
    ///
    /// Returns a Result containing:
    /// * `Ok(true)` - User was successfully deleted
    /// * `Ok(false)` - No user found with given ID
    /// * `Err(MongoError)` - MongoDB error on failure
    pub async fn delete_user(db: &Database, id: ObjectId) -> Result<bool, MongoError> {
        UserService::delete(db, id).await
    }
}
