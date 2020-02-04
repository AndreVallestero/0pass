// commands.rs
// 0pass commands

mod data;
use data::*;

pub fn view_command(password:&String, matches:&clap::ArgMatches<>) {
    match read_data(password) {
        Ok(passbook) => {
            match matches.value_of("label") {
                Some(label) => {
                    let mut found = false;
                    for entry in passbook.entries.iter() {
                        if entry.label == label {
                            found = true;
                            println!("{}: {:?}", entry.label, entry.fields);
                        }
                    }
                    if !found {
                        println!("No such label was found")
                    }
                },
                None => println!("No search label given")
            }
        },
        Err(_) => println!("Error reading passbook")
    }
}

pub fn create_command(password:&String, matches:&clap::ArgMatches<>) {
    match read_data(password) {
        Ok(mut passbook) => {
            match matches.value_of("label") {
                Some(label) => {
                    match matches.values_of("fields") {
                        Some(fields) => {
                            // Create new entry
                            let mut new_entry = Entry::default();
                            new_entry.label = label.to_string();
                            for field in fields {
                                new_entry.fields.push(field.to_string());
                            }
                            
                            // Append to passbook
                            passbook.entries.push(new_entry);
                            
                            // Write changes
                            match write_data(password, passbook) {
                                Ok(_) => return,
                                Err(_) => println!("Error writing passbook")
                            }
                        },
                        None => println!("No fields were given")
                    }
                },
                None => println!("No label was given")
            }
        },
        Err(_) => println!("Error reading passbook")
    }
}

pub fn delete_command(password:&String, matches:&clap::ArgMatches<>) {
    match read_data(password) {
        Ok(mut passbook) => {
            match matches.value_of("label") {
                Some(label) => {
                    // Search for entry with matching label
                    let mut found = false;
                    for i in 0..passbook.entries.len() {
                        if passbook.entries[i].label == label {
                            // Remove entry
                            passbook.entries.remove(i);
                            found = true;
                            break
                        }
                    }
                    
                    if found {
                        // Write changes
                        match write_data(password, passbook) {
                            Ok(_) => return,
                            Err(_) => println!("Error writing passbook")
                        }
                    } else {
                        println!("No such label was found")
                    } 
                },
                None => println!("No label was given")
            }
        },
        Err(_) => println!("Error reading passbook")
    }
}

pub fn list_command(password:&String) {
    match read_data(password) {
        Ok(passbook) => {
            if passbook.entries.len() == 0 {
                println!("Passbook is empty");
            } else {
                for entry in passbook.entries.iter() {
                    println!("{}", entry.label);
                }
            }
        },
        Err(_) => println!("Error reading passbook")
    }
}

pub fn passwd_command(password:&String, matches:&clap::ArgMatches<>) {
    match read_data(password) {
        Ok(passbook) => {
            match matches.value_of("new_password") {
                Some(new_password) => {
                    match matches. value_of("verify_new_password") {
                        Some(verify_password) => {
                            if new_password == verify_password {
                                match write_data(&new_password.to_string(), passbook){
                                    Ok(_) => return,
                                    Err(_) => println!("Error writing passbook")
                                }
                            } else {
                                println!("New password was not the same as verification password, password change failed")
                            }
                        },
                        None => println!("No verification password was given")
                    }
                },
                None => println!("No new password was given")
            }
        },
        Err(_) => println!("Error reading passbook")
    }
}