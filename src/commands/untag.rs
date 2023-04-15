use crate::commands::get::Get;
use crate::commands::Command;
use clap::Args;

#[derive(Args)]
pub struct Untag {
    pub path: std::path::PathBuf,
}

impl Command for Untag {
    fn execute(&self, tree: &sled::Tree) -> Result<String, String> {
        // First, canonicize the path
        let path = self.path.canonicalize().unwrap();

        // Find all of the tags for the path
        // Reuse the code from get.rs
        let get = Get { path: path.clone() };
        let tags = get.execute(tree).unwrap();
        let tags = tags.split(" ");

        // Find all of the paths for the tags
        // Remove the path and update the value
        for tag in tags {
            for result in tree.iter() {
                let (key, value) = result.unwrap();
                let key = String::from_utf8(key.to_vec()).unwrap();
                let value = String::from_utf8(value.to_vec()).unwrap();
                if key == tag {
                    let paths = value.split(" ");
                    let mut new_value = String::new();
                    for p in paths {
                        if p != path.to_str().unwrap() {
                            new_value.push_str(p);
                            new_value.push_str(" ");
                        }
                    }
                    tree.insert(tag, new_value.to_string().as_bytes()).unwrap();
                }
            }
        }
        Ok("".to_string())
    }
}
