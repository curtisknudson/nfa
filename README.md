# NFA (Notes For Anon)

A simple, fast, and efficient command-line note-taking application written in Rust.

## Features

- Create, read, update, and delete notes
- List all notes
- Persistent storage using sled database
- Simple and intuitive command-line interface
- Secure storage in user's home directory

## Installation

Install using cargo:

```bash
cargo install nfa
```

### Creating a new note

```bash
# Quick note (automatically generates title from content)
nfa "This is a quick note"

# Structured note (requires both title and content flags)
nfa new -t "My Title" -c "This is the content"
```

### Listing all notes

```bash
nfa list
```

### Showing a specific note

```bash
nfa show <note-id>
```

### Updating a note

```bash
nfa update --id <note-id> --title "New Title" --content "Updated content"
```

### Deleting a note

```bash
nfa delete <note-id>
```

## Storage

Notes are stored in `~/.nfa/` directory using the sled database engine.

## Development

### Prerequisites

- Rust 1.56 or higher
- Cargo

### Building from source

```bash
git clone https://github.com/curtisknudson/nfa
cd nfa
cargo build --release
```

### Running tests

```bash
cargo test
```

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## Author

Curtis Knudson
