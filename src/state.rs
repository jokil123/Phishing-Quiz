use std::{
    fs::{self, OpenOptions},
    path::Path,
    sync::{Arc, Mutex},
};

use std::io::Write;

pub struct AppState(Arc<Mutex<AppStateInner>>);

impl From<AppStateInner> for AppState {
    fn from(inner: AppStateInner) -> Self {
        AppState(Arc::new(Mutex::new(inner)))
    }
}

impl AppState {
    pub fn lock(&self) -> Option<std::sync::MutexGuard<AppStateInner>> {
        self.0.lock().ok()
    }
}

pub struct AppStateInner {
    pub valid_emails: Vec<String>,
    pub collected_emails: CollectedEmails,
}

pub struct CollectedEmails(Vec<String>);

impl CollectedEmails {
    pub fn new(emails: Vec<String>) -> Self {
        CollectedEmails(emails)
    }

    pub fn contains(&self, email: &String) -> bool {
        self.0.contains(email)
    }

    pub fn push(&mut self, email: String) {
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
