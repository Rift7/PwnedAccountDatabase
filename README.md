# Pwned Account Database

The Pwned Account Database is a tool for storing and searching a database of compromised email accounts and passwords. It allows users to easily compile a self hosted database from any password lists found while using their OSINT tactics.

## Features

* Import email and password data from text files
* Search the database for specific email addresses, email domains, or passwords
* Query the database from a web browser
* GUI application with a search box and output box

## Requirements

* A current installation of Rust

## Usage

To host your own Pwned Account Database, follow these steps:

Clone this repository to your local machine:

    git clone https://github.com/<username>/pwned-account-database.git

Navigate to the repository directory:

    cd pwned-account-database

Import the email and password data from one or more text files:

    cargo run -- import <file1> [<file2> ...]

Run the GUI application to search the database:

    cargo run -- gui

Alternatively, you can use the web server to query the database from a web browser:

    cargo run -- server

Then, open a web browser and go to [`http://127.0.0.1:8080/{query}`](http://127.0.0.1:8080/{query}), replacing `{query}` with the search query you want to use.

## License

This project is licensed under the MIT License - see the [LICENSE](https://github.com/Rift7/PwnedAccountDatabase/blob/main/LICENSE) file for details.

## Acknowledgments

This project was inspired by [';--have i been pwned?](https://haveibeenpwned.com/)
