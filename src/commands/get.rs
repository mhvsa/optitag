use crate::commands::Command;
use clap::Args;

#[derive(Args)]
pub struct Get {
    pub path: std::path::PathBuf,
}

impl Command for Get {
    // Get all of the tags for a path
    // Returned format: "tag1 tag2 tag3"
    fn execute(&self, tree: &sled::Tree) -> Result<String, String> {
        let path = self.path.clone().canonicalize().unwrap();
        let path = path.to_str().unwrap();
        // Iterate over all of the keys
        // If the value contains the path, add the key to the result
        let mut tags = vec![];
        for result in tree.iter() {
            let (key, value) = result.unwrap();
            let key = String::from_utf8(key.to_vec()).unwrap();
            let value = String::from_utf8(value.to_vec()).unwrap();
            if value.contains(path) {
                tags.push(key);
            }
        }
        let result = tags.join(" ");
        Ok(result)
    }
}
