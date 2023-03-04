use crate::entities::user::User;
use mongodb::{options::ClientOptions, Client};

const DB_NAME: &str = "phishing_quiz";
const CONNECTION_STRING: &str = "mongodb://localhost:27017";

struct DbConnection {
    client: Client,
}

impl DbConnection {
    pub async fn new() -> Self {
        let mut client_options = ClientOptions::parse(CONNECTION_STRING).await.unwrap();

        client_options.app_name = Some("Phishing Quiz".to_string());

        DbConnection {
            client: Client::with_options(client_options).unwrap(),
        }
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

    pub async fn is_htl_email(&self, email: String) -> bool {
        todo!()
    }
}
