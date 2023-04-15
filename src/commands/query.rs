use crate::commands::Command;
use clap::Args;

#[derive(Args)]
pub struct Query {
    pub tags: Vec<String>,
}

impl Command for Query {
    // Get all of the paths for a tag
    // Returned format:
    // path1
    // path2
    // path3
    fn execute(&self, tree: &sled::Tree) -> Result<String, String> {
        // Get the paths which exist for each tag
        // If the tag doesn't exist, return an empty string

        // We'll create a map of path to number of tags
        // We'll increment the count for each path which exists for each tag
        let mut map = std::collections::HashMap::new();
        for tag in &self.tags {
            let paths = tree.get(tag.as_bytes()).unwrap();
            if paths.is_some() {
                let paths = String::from_utf8(paths.unwrap().to_vec()).unwrap();
                for path in paths.split(" ") {
                    let count = map.entry(path.to_string()).or_insert(0);
                    *count += 1;
                }
            }
        }

        // If the count for a path is equal to the number of tags
        // Then the path exists for all of the tags
        // So we'll add it to the result
        let mut result = vec![];
        for (path, count) in map {
            if count == self.tags.len() {
                result.push(path);
            }
        }

        // Format the result
        let result = result.join("\n");
        Ok(result)
    }
}
