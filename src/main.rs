mod commands;
use clap::{ Parser, Subcommand };
use commands::{ Clear, Command, Get, Query, Tag, Untag };

#[derive(Parser)]
#[command(author, version, author, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    Tag(Tag),
    Get(Get),
    Query(Query),
    Untag(Untag),
    Clear(Clear),
}

fn main() {
    let config_dir = dirs::config_dir().unwrap();
    let tree = sled::open(config_dir.join("optitag/state")).unwrap();

    let cli = Cli::parse();
    match &cli.command {
        Some(Commands::Tag(tag)) => {
            tag.execute(&tree).unwrap();
        }
        Some(Commands::Get(get)) => {
            println!("{}", get.execute(&tree).unwrap());
        }
        Some(Commands::Query(query)) => {
            println!("{}", query.execute(&tree).unwrap());
        }
        Some(Commands::Untag(untag)) => {
            untag.execute(&tree).unwrap();
        }
        Some(Commands::Clear(clear)) => {
            println!("{}", clear.execute(&tree).unwrap());
        }
        None => {
            println!("No command");
        }
    }
}
