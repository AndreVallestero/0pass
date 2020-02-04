// main.rs
// 0pass: A cross platform CLI password manager written in Rust intended as a backend for native GUI clients
// Andre Vallestero
// 2020-01-30

mod commands;

use clap::{Arg, App, SubCommand};
use commands::*;

fn main() {
    let matches = App::new(env!("CARGO_PKG_NAME"))
        .version(env!("CARGO_PKG_VERSION"))
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .arg(Arg::with_name("password")
            .required(true))
        .subcommand(SubCommand::with_name("view")
            .about("View a label and its fields")
            .arg(Arg::with_name("label")
                .required(true)))
        .subcommand(SubCommand::with_name("create")
            .about("Creates new labels which store fields")
            .arg(Arg::with_name("label")
                .required(true))
            .arg(Arg::with_name("fields")
                .min_values(1)))
        .subcommand(SubCommand::with_name("delete")
            .about("Deletes existing labels")
            .arg(Arg::with_name("label")
                .required(true)))
        .subcommand(SubCommand::with_name("list")
            .about("Lists all labels"))
        .subcommand(SubCommand::with_name("passwd")
            .about("Changes the master password")
            .arg(Arg::with_name("new_password")
                .required(true))
            .arg(Arg::with_name("verify_new_password")
                .required(true)))
        .get_matches();

    // Require password for commands
    match matches.value_of("password") {
        None => println!("A password is required for each command"),
        Some(password) => {
            // Match commands
            match matches.subcommand() {
                ("view", Some(sub_matches)) => view_command(&password.to_string(), sub_matches),
                ("create", Some(sub_matches)) => create_command(&password.to_string(), sub_matches),
                ("delete", Some(sub_matches)) => delete_command(&password.to_string(), sub_matches),
                ("list", Some(_)) => list_command(&password.to_string()),
                ("passwd", Some(sub_matches)) => passwd_command(&password.to_string(), sub_matches),
                _ => println!("Invalid command")
            }
        }
    }
}
