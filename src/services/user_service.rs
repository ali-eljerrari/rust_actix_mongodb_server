use mongodb::Database;
use mongodb::error::Error as MongoError;
use mongodb::bson::{doc, oid::ObjectId};
use futures::stream::TryStreamExt;
use crate::models::user_model::User;

/// Service layer for handling user-related business logic and database operations
pub struct UserService;

impl UserService {
    /// Retrieves all users from the database
    ///
    /// # Arguments
    ///
    /// * `db` - Reference to the MongoDB database connection
    ///
    /// # Returns
    ///
    /// Returns a Result containing:
    /// * `Ok(Vec<User>)` - Vector of all users found
    /// * `Err(MongoError)` - MongoDB error on failure
    pub async fn find_all(db: &Database) -> Result<Vec<User>, MongoError> {
        let collection = db.collection::<User>("users");
        let mut cursor = collection.find(doc! {}).await?;
        
        let mut users = Vec::new();
        while let Some(user) = cursor.try_next().await? {
            users.push(user);
        }
        
        Ok(users)
    }

    /// Finds a user by their ID
    ///
    /// # Arguments
    ///
    /// * `db` - Reference to the MongoDB database connection
    /// * `id` - The ObjectId of the user to find
    ///
    /// # Returns
    ///
    /// Returns a Result containing:
    /// * `Ok(Some(User))` - The found user
    /// * `Ok(None)` - No user found with the given ID
    /// * `Err(MongoError)` - MongoDB error on failure
    pub async fn find_by_id(db: &Database, id: ObjectId) -> Result<Option<User>, MongoError> {
        let collection = db.collection::<User>("users");
        collection.find_one(doc! { "_id": id }).await
    }

    /// Creates a new user in the database
    ///
    /// # Arguments
    ///
    /// * `db` - Reference to the MongoDB database connection
    /// * `user` - The User instance to create
    ///
    /// # Returns
    ///
    /// Returns a Result containing:
    /// * `Ok(User)` - The created user with assigned ID
    /// * `Err(MongoError)` - MongoDB error on failure or if user not found after creation
    pub async fn create(db: &Database, mut user: User) -> Result<User, MongoError> {
        let collection = db.collection::<User>("users");
        
        // Ensure the user is active by default
        user.is_active = true;
        
        let insert_result = collection.insert_one(user).await?;
        
        let id = insert_result
            .inserted_id
            .as_object_id()
            .expect("Failed to get inserted id");
            
        Self::find_by_id(db, id)
            .await?
            .ok_or_else(|| MongoError::from(std::io::Error::new(
                std::io::ErrorKind::NotFound,
                "Created user not found"
            )))
    }

    /// Updates an existing user in the database
    ///
    /// # Arguments
    ///
    /// * `db` - Reference to the MongoDB database connection
    /// * `id` - The ObjectId of the user to update
    /// * `user` - The User instance containing updated fields
    ///
    /// # Returns
    ///
    /// Returns a Result containing:
    /// * `Ok(Some(User))` - The updated user
    /// * `Ok(None)` - No user found with the given ID
    /// * `Err(MongoError)` - MongoDB error on failure
    pub async fn update(db: &Database, id: ObjectId, user: User) -> Result<Option<User>, MongoError> {
        let collection = db.collection::<User>("users");
        
        // Create update document
        let update = doc! {
            "$set": {
                "name": user.name,
                "email": user.email,
                "password": user.password,
                "is_active": user.is_active
            }
        };

        let result = collection
            .update_one(doc! { "_id": id }, update)
            .await?;

        if result.modified_count == 1 {
            Self::find_by_id(db, id).await
        } else {
            Ok(None)
        }
    }

    /// Deletes a user from the database
    ///
    /// # Arguments
    ///
    /// * `db` - Reference to the MongoDB database connection
    /// * `id` - The ObjectId of the user to delete
    ///
    /// # Returns
    ///
    /// Returns a Result containing:
    /// * `Ok(true)` - User was successfully deleted
    /// * `Ok(false)` - No user found with the given ID
    /// * `Err(MongoError)` - MongoDB error on failure
    pub async fn delete(db: &Database, id: ObjectId) -> Result<bool, MongoError> {
        let collection = db.collection::<User>("users");
        let result = collection
            .delete_one(doc! { "_id": id })
            .await?;
            
        Ok(result.deleted_count == 1)
    }
} 