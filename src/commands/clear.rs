use crate::commands::Command;
use clap::Args;
use std::io::prelude::*;

#[derive(Args)]
#[command(
    name = "clear",
    about = "Clear the database",
    long_about = r#"
Clear the database.
Example:
    ## Pupulate the database ##
    optitag tag /home/user/file.txt tag1 tag2 tag3
    
    ## Test that the database is populated ##
    optitag get /home/user/file.txt
    tag1 tag2 tag3

    ## Clear the database ##
    optitag clear
    Database cleared

    ## Test that the database is empty ##
    optitag get /home/user/file.txt
    ## No output ##
    "#
)]
pub struct Clear {}

impl Command for Clear {
    // Ask the user if they want to clear the database
    // If yes, clear the database
    fn execute(&self, tree: &sled::Tree) -> Result<String, String> {
        std::io::stdout()
            .write_all(b"Are you sure you want to clear the database? [y/N] ")
            .unwrap();
        std::io::stdout().flush().unwrap();
        let mut buffer = [0u8; 1];
        std::io::stdin().read_exact(&mut buffer).unwrap();
        let input = String::from_utf8(buffer.to_vec()).unwrap();
        if input == "y" || input == "Y" {
            tree.clear().unwrap();
            Ok("Database cleared".to_string())
        } else {
            Ok("Aborted".to_string())
        }
    }
}
