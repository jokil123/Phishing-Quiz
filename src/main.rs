#[macro_use]
extern crate rocket;

use rocket::fs::relative;
use rocket::fs::FileServer;
use rocket::http::Status;
use rocket::State;
use phishing_quiz::state::AppState;
use phishing_quiz::state::AppStateInner;
use phishing_quiz::state::CollectedEmails;
use phishing_quiz::util::load_emails;

#[put("/submit?<email>")]
fn submit(state: &State<AppState>, email: String) -> Status {
    let email = email.to_lowercase();

    let mut state = state.lock().unwrap();

    println!("Email: {}", email);

    println!("Valid emails: {:?}", state.valid_emails);

    if !state.valid_emails.contains(&email) {
        return Status::BadRequest;
    };

    if state.collected_emails.contains(&email) {
        return Status::BadRequest;
    };

    state.collected_emails.push(email);

    Status::Ok
}

#[launch]
fn rocket() -> _ {
    let state: AppState = AppStateInner {
        valid_emails: load_emails("valid_emails.txt"),
        collected_emails: CollectedEmails::new(load_emails("collected_emails.txt")),
    }
    .into();

    rocket::build()
        .manage(state)
        .mount("/", routes![submit])
        .mount("/", FileServer::from(relative!("static")))
}
