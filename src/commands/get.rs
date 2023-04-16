use crate::commands::Command;
use clap::Args;

#[derive(Args)]
#[command(
    name = "get",
    about = "Get all tags for a path",
    long_about = r#"
Get all tags for a path. 
Example: 
    ## Tag a path with multiple tags ##
    optitag tag /home/user/file.txt tag1 tag2 tag3
    ## Get all of the tags for a path ##
    optitag get /home/user/file.txt
    tag1 tag2 tag3
    "#
)]
pub struct Get {
    #[arg(help = "The path to get tags for")]
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
