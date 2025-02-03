use clap::{Parser, Subcommand};
use nfa::{NFAError, NoteManager};
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "nfa")]
#[command(about = "A simple note-taking application", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,

    #[arg(group = "input")]
    content: Option<String>,
}

#[derive(Subcommand)]
enum Commands {
    New {
        #[arg(short, long)]
        title: String,

        #[arg(short, long)]
        content: String,
    },
    List,
    Show {
        id: String,
    },
    Delete {
        id: String,
    },
    Update {
        #[arg(short, long)]
        id: String,
        #[arg(short, long)]
        title: Option<String>,
        #[arg(short, long)]
        content: Option<String>,
    },
}

fn get_db_path() -> PathBuf {
    let mut path = dirs::home_dir().expect("Could not find home directory");
    path.push(".nfa");
    std::fs::create_dir_all(&path).expect("Could not create directory");
    path
}

fn main() -> Result<(), NFAError> {
    let cli = Cli::parse();
    let manager = NoteManager::new(get_db_path().to_str().unwrap())?;

    match (cli.command, cli.content) {
        (None, Some(content)) => {
            if content.is_empty() {
                println!("Error: Content cannot be empty");
                return Ok(());
            }

            let title = {
                let default_title = content.chars().take(10).collect::<String>();
                if content.len() > 10 {
                    format!("{}...", default_title)
                } else {
                    default_title
                }
            };

            let note = manager.create_note(title, content)?;
            println!("Created note with ID: {}", note.id);
        }
        (Some(command), None) => match command {
            Commands::New { title, content } => {
                let note = manager.create_note(title, content)?;
                println!("Created note with ID: {}", note.id);
            }

            Commands::List => {
                let notes = manager.list_notes()?;
                if notes.is_empty() {
                    println!("No notes found");
                } else {
                    for note in notes {
                        println!("ID: {}", note.id);
                        println!("Title: {}", note.title);
                        println!("Content: {}", note.content);
                        println!("---");
                    }
                }
            }

            Commands::Show { id } => match manager.get_note(&id) {
                Ok(note) => {
                    println!("Title: {}", note.title);
                    println!("Content: {}", note.content);
                    println!("Created: {:?}", note.created_at);
                    println!("Updated: {:?}", note.updated_at);
                }
                Err(NFAError::NoteNotFound) => {
                    println!("Note not found");
                }
                Err(e) => return Err(e),
            },

            Commands::Delete { id } => match manager.delete_note(&id) {
                Ok(_) => println!("Note deleted successfully"),
                Err(NFAError::NoteNotFound) => println!("Note not found"),
                Err(e) => return Err(e),
            },

            Commands::Update { id, title, content } => {
                match manager.update_note(&id, title, content) {
                    Ok(_note) => println!("Note updated successfully"),
                    Err(NFAError::NoteNotFound) => println!("Note not found"),
                    Err(e) => return Err(e),
                }
            }
        },

        (None, None) => {
            println!("No content provided. Use --help for usage information.");
        }
        (Some(_), Some(_)) => {
            // This should never happen due to conflicts_with attribute
            unreachable!("Cannot use both command and content");
        }
    }

    Ok(())
}
