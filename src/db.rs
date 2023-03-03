use mongodb::{options::ClientOptions, Client};
use crate::entities::user::User;

struct DbConnection {
    client: Client,
}

impl DbConnection {
    pub async fn new() -> Self {
        todo!()
    }

    pub async fn get_users(&self) -> Vec<User> {
        todo!()
    }

    pub async fn new_user(&self, user: User) -> Result<(), ()> {
        todo!()
    }

    pub async fn update_user(&self, user: User) -> Result<(), ()> {
        todo!()
    }
}
