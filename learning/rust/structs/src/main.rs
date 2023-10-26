use rand::Rng;

static IS_DEBUG: bool = false;

#[derive(Debug)] // That's for some cool debugging output with e.g. 'println!("{:#?}", user);'
struct User {
    id: usize,
    name: String,
    email: String,
}

impl User {
    /// new will create a new [User] for the given username and email
    fn new(name: String, email: String) -> User {
        let mut random_generator = rand::thread_rng();
        User {
            id: random_generator.gen_range(1337..=7331),
            name,
            email,
        }
    }

    /// as_author is a function to get the user as author, e.g. for a quote or GitHub commit
    fn as_author(&self) -> String {
        format!("{} <{}>", self.name, self.email)
    }
}

fn main() {
    // We initialize the vector of contributors
    let contributors_username_email: Vec<(String, String)> = vec![
        (String::from("Krypton"), String::from("root@krypton.ninja")),
        (String::from("Nyvil"), String::from("alexandros@nyvil.dev")),
        (String::from("Linus Torvalds"), String::from("torvalds@linux-foundation.org")),
    ];

    // We first create an empty vector
    let mut contributors: Vec<User> = vec![];

    // We iterate from 1 to 100 (included)
    for item in contributors_username_email {
        // If we are in verbose mode, we print some information
        if IS_DEBUG {
            println!("Adding user {} to the vector", item.0)
        }
        // We add a new user to the vector of users
        contributors.push(User::new(item.0, item.1));
    }

    // We print that vector of users in a beautiful way thanks to the ':#?' format
    println!("Vector: {:#?}", contributors);

    // We get a random user, that we will print along with it's ID and formatted as author
    let mut random_generator = rand::thread_rng();
    let random_user = &contributors[random_generator.gen_range(0..contributors.len())];
    println!("User #{}: {:#?} \nCommit Author: {}", random_user.id, random_user, random_user.as_author());
}