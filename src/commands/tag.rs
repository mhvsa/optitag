use crate::commands::Command;
use clap::Args;

#[derive(Args)]
#[command(
    name = "tag",
    about = "Tag a path",
    long_about = r#"
Tag a path.
Example: 
    ## Tag a path with multiple tags ##
    optitag tag /home/user/file.txt tag1 tag2 tag3

    ## Get all of the tags for a path ##
    optitag get /home/user/file.txt
    tag1 tag2 tag3
    "#
)]
pub struct Tag {
    #[arg(help = "The path to tag")]
    pub path: std::path::PathBuf,
    #[arg(help = "The tags to add to the path")]
    pub tags: Vec<String>,
}

impl Command for Tag {
    fn execute(&self, tree: &sled::Tree) -> Result<String, String> {
        let mut tags = self.tags.clone();
        // Use the set to remove duplicates
        let mut set = std::collections::HashSet::new();
        for tag in tags {
            set.insert(tag);
        }
        tags = set.into_iter().collect::<Vec<String>>();

        // Get the full path of the file
        let cannoincal_path = self.path.clone().canonicalize().unwrap();
        let full_path = cannoincal_path.to_str().unwrap();

        // For each tag, insert the path into the value
        // If the key doesn't exist, create it
        // If the key exists, append the path to the value
        for tag in tags {
            let paths = tree.insert(tag.as_bytes(), full_path.as_bytes()).unwrap();

            // Push the new path to the existing paths if there were any
            // Skip duplicates with a set
            if paths.is_some() {
                let mut set = std::collections::HashSet::new();
                let existing_paths = String::from_utf8(paths.unwrap().to_vec()).unwrap();
                for path in existing_paths.split(" ") {
                    set.insert(path);
                }
                set.insert(full_path);
                let paths = set.into_iter().collect::<Vec<&str>>().join(" ");
                tree.insert(tag.as_bytes(), paths.as_bytes()).unwrap();
            }
        }
        Ok("".to_string())
    }
}
