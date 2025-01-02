use crate::{user_to_document, User};
use bson;
use mongodb::{error::Error, results::InsertOneResult, Collection};
pub struct UserService<'a> {
    collection: Collection<&'a User>,
}
impl<'a> UserService<'a> {
    pub fn new(collection: Collection<&'a User>) -> UserService {
        UserService { collection }
    }
    pub async fn create(&self, user: &User) -> Result<InsertOneResult, Error> {
        self.collection
            .insert_one(bson::Bson::Document(user_to_document(user)))
            .await
    }
}
