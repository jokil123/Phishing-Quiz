#[macro_use]
extern crate rocket;

use std::fs;
use std::fs::OpenOptions;
use std::io::Read;
use std::io::Write;
use std::sync::Arc;
use std::sync::Mutex;

use rocket::fs::relative;
use rocket::fs::FileServer;
use rocket::http::Status;
use rocket::State;

use std::path::Path;

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
        collected_emails: CollectedEmails(load_emails("collected_emails.txt")),
    }
    .into();

    rocket::build()
        .manage(state)
        .mount("/", routes![submit])
        .mount("/", FileServer::from(relative!("static")))
}

struct AppState(Arc<Mutex<AppStateInner>>);

impl From<AppStateInner> for AppState {
    fn from(inner: AppStateInner) -> Self {
        AppState(Arc::new(Mutex::new(inner)))
    }
}

impl AppState {
    fn lock(&self) -> Option<std::sync::MutexGuard<AppStateInner>> {
        self.0.lock().ok()
    }
}

struct AppStateInner {
    valid_emails: Vec<String>,
    collected_emails: CollectedEmails,
}

struct CollectedEmails(Vec<String>);

impl CollectedEmails {
    fn contains(&self, email: &String) -> bool {
        self.0.contains(email)
    }

    fn push(&mut self, email: String) {
        self.0.push(email.clone());

        if !Path::new("collected_emails.txt").exists() {
            fs::File::create("collected_emails.txt").unwrap();
            println!("Created file: {}", "collected_emails.txt");
        };

        let mut file = OpenOptions::new()
            .write(true)
            .append(true)
            .open("collected_emails.txt")
            .unwrap();

        writeln!(file, "{}\r", email).unwrap();
    }
}

fn load_emails(f_name: &str) -> Vec<String> {
    if !Path::new(f_name).exists() {
        fs::File::create(f_name).unwrap();
        println!("Created file: {}", f_name);
    };

    let file = fs::read_to_string(f_name).unwrap();

    let strings = file
        .split("\n")
        .into_iter()
        .map(|s| s.to_owned().replace("\r", ""))
        .collect::<Vec<String>>();

    strings
}
