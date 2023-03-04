use mongodb::{options::ClientOptions, Client};
use phishing_quiz::entities::user::User;

#[tokio::main]
async fn main() {
    // Parse a connection string into an options struct.
    let mut client_options = ClientOptions::parse("mongodb://localhost:27017")
        .await
        .unwrap();

    // Manually set an option.
    client_options.app_name = Some("My App".to_string());

    // Get a handle to the deployment.
    let client = Client::with_options(client_options).unwrap();

    // List the names of the databases in that deployment.
    for db_name in client.list_database_names(None, None).await.unwrap() {
        println!("{}", db_name);
    }

    let typed_collection = client.database("phishing_quiz").collection::<User>("users");

    let user = User {
        email: "joshua.lung@student.htldornbirn.at".to_string(),
        phished: true,
        form_result: Some(phishing_quiz::entities::user::FormResult {
            phishing_likelihood: 1,
            phished_before: false,
        }),
        ..Default::default()
    };

    typed_collection.insert_one(user, None).await.unwrap();
}
