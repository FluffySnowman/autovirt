use std;

pub fn get_autovirt_data_dir() {
    let user_home_path = std::env::var("HOME");

    println!("Getting current user $HOME path...");

    if user_home_path.is_ok() {
        println!("User $HOME path: {:?}", user_home_path);
    } else {
        eprintln!("Could not find user $HOME path");
    }

        println!("{:#?}", format!(r"The stringified home path I think: {:?}", user_home_path));

}

pub fn create_autovirt_data_dir() {
}

