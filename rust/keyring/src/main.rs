use keyring::Entry;
use rand::Rng;

struct App {
    keyring_entry: Entry,
}

impl App {
    fn new() -> Self {
        let mut rng = rand::thread_rng();
        let keyring_entry = Entry::new("keyring_test_rs", "krypton").unwrap();
        keyring_entry
            .set_password(rng.gen::<usize>().to_string().as_str())
            .unwrap();
        App { keyring_entry }
    }

    fn get_password(&self) -> String {
        self.keyring_entry.get_password().unwrap()
    }
}

impl Drop for App {
    fn drop(&mut self) {
        self.keyring_entry.delete_credential().unwrap()
    }
}

fn main() {
    let app = App::new();
    println!("Got password {}", app.get_password());
}
