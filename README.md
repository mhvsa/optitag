# Optitag
## Optitag Program for Tidying Information with Tags And Grouping 

Optitag is a command-line tool written in Rust for tagging files with metadata. It allows you to assign tags to files, and then search and filter files based on those tags.

## Usage

To get started with Optitag, you can install it using `cargo`, the Rust package manager:

```bash
cargo install optitag
```

Once installed, you can use the following commands to interact with Optitag:

### Tag a file
```bash
optitag tag /path/to/file.txt tag1 tag2 tag3
```

This command will assign the tags `tag1`, `tag2`, and `tag3` to the file `/path/to/file.txt`.

### Get all tags for a file

```bash
optitag get /path/to/file.txt
```

This command will display all the tags associated with the file `/path/to/file.txt`.

### Query files by tag

```bash
optitag query tag1 tag2
```

This command will display a list of all the files that have been tagged with both `tag1` and `tag2`.

### Remove tags from a file

```bash
optitag untag /path/to/file.txt tag1 tag2
```

This command will remove the `tag1`and the `tag2` tags from the file `/path/to/file.txt`.

### Clear the database

```bash
optitag clear
```

This command will clear all tags associated with all files.

### Help

```bash
optitag help
```

This command will display Optitag's help menu.

## Dependencies

Optitag is built in Rust, so you'll need a Rust toolchain installed on your system to build it. To install Rust, visit the [Rust website](https://www.rust-lang.org/tools/install) and follow the instructions.

## Known Issues and Future Development

Optitag is a work in progress and hasn't been tested in a real-world setting, so there may be bugs or limitations that still need to be addressed. 

In the future, we plan to add more query options and improvements to how the data is stored.

## License and Contributions

Optitag is licensed under the MIT License, and outside contributors are welcome to submit issues, pull requests, or contact the author directly via email.

## Documentation

No additional resources or documentation available at this time.

