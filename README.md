# Version Control System (VCS) CLI

This is a command-line interface (CLI) for a Version Control System (VCS). It provides various functionalities for managing and tracking changes to files in a repository. The following sections describe the available commands and their usage.

## Contributors

This project is contributed to by the following individuals:

- Lawson Oliveira Lima
- Lucas Vitoriano de Queiroz Lira


## Getting Started

To use the program, you need to provide arguments when running the CLI. Make sure you have Rust installed and follow these steps:

1. Clone the repository: `git clone https://github.com/LawsonOliveira/VCS.git`
2. Navigate to the project directory: `cd VCS`
3. Build the project: `cargo build`
4. Run the program: `cd ./target/debug && ./main <command>`

## Available Commands

### init

Initializes a new VCS repository.

Usage: `cd ./target/debug && ./main init`

### add

Adds one or more files to the VCS for version control.

Usage: `cd ./target/debug && ./main add <file1> <file2> ...`

### remove

Removes one or more files from the staging area and delete the respectives files in add/contents.

Usage: `cd ./target/debug && ./main remove <file1> <file2> ...`

### commit

Creates a new commit with the specified comment.

Usage: `cd ./target/debug && ./main commit <comment>`

### delete_commit

Deletes a specific commit from the VCS history using the hash.

Usage: `cd ./target/debug && ./main delete_commit <commit_hash>`

### checkout

Changes the current version of the project to the specified commit hash.

Usage: `cd ./target/debug && ./main change_version <commit_hash>`

### create_branch

Creates a new branch in the VCS repository.

Usage: `cd ./target/debug && ./main create_branch <branch_name>`

### change_branch

Changes the current branch in the VCS repository.

Usage: `cd ./target/debug && ./main change_branch <branch_name>`

### delete_branch

Deletes a branch from the VCS repository.

Usage: `cd ./target/debug && ./main delete_branch <branch_name>`

### print

Prints the current state of the project files.

Usage: `cd ./target/debug && ./main print`

### log

Displays the commit history and associated information.

Usage: `cd ./target/debug && ./main log`


---








