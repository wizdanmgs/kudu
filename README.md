# kudu

A simple CLI todo list application written in Rust. Data is stored locally in a JSON file.

## Installation

Make sure you have `rustc` and `cargo` in your system.

Clone this project, go to the project directory, and then install with this command:

```bash
cargo install --path .
```

Now you can use `kudu`.

## Usage

Add a new todo item

```bash
kudu add "Go to the grocery store"
kudu add "Clean the bathub"
```

List all todo item

```bash
kudu list
```

Mark a todo item as completed

```bash
kudu done 1
```

Delete a todo item

```bash
kudu delete 2
```
