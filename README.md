![Rust](https://github.com/gmlion/dotenvsubst/workflows/Rust/badge.svg)

# dotenvsubst #
*dotenvsubst* is a little command line utility to substitute environment variables

## HOW TO INSTALL ##
`cargo install dotenvsubst`. You need to have [Cargo](https://doc.rust-lang.org/cargo/getting-started/installation.html) installed.

## SYNOPSIS ##
dotenvsubst [ENVFILE]

## DESCRIPTION ##
Standard input is copied to standard output, with reference to environment variables of the form `${VARIABLE}` being replaced with corrisponding values read from the specified ENVFILE. If no ENVFILE is specified, the program looks for a .env file in the current directory.
The syntax for the ENVFILE is composed of variables on new lines in the form of `NAME=VALUE`.

## EXAMPLES ##
Using a custom .env_custom ENVFILE on bash:
`cat file.sh | dotenvsubst .env_custom > file_env_custom.sh`

Using default .env file in Powershell:
`Get-Content file.sh | dotenvsubst > file_env.sh`
