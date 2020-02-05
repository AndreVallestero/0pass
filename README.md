# 0pass
A cross platform CLI password manager written in Rust intended as a backend for native GUI clients

## Features
- Data is encrypted with AES-256-CBC 
- 0 unwraps and unsafe blocks
- Small ~600kb standalone windows binary (ik, not *that* small)

## Usage
`0pass [password] create [label] "[field1]" "[field2]" "[field3]"...`
- Creates new labels which store fields
- [password] current master password
- [label] ID that represents the fields of data
- [fieldX] string field(s) to be stored

`0pass [password] delete [label]`
- Deletes existing labels
- [password] current master password
- [label] ID that represents the fields of data

`0pass --help`
- Prints help menu

`0pass [password] list`
- Lists all labels
- [password] current master password

`0pass [password] passwd [new_password] [verify_new_password]`
- Changes the master password
- [password] current master password
- [new_password] new master password
- [verify_new_password] new master password

`0pass [password] view [label]`
- View a label and its fields
- [password] current master password
- [label] ID that represents the fields of data

## TODO:
- (CRITICAL) Use a proper key derivation function rather than SHA256
- Add a timeout where no password is needed. Extending the current timeout will require a password