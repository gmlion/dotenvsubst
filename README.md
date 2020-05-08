![Rust](https://github.com/gmlion/dotenvsubst/workflows/Rust/badge.svg)

# dotenvsubst #
*dotenvsubst* is a little command line utility to substitute environment variables

## HOW TO INSTALL ##
`cargo install dotenvsubst`. You need to have [Cargo](https://doc.rust-lang.org/cargo/getting-started/installation.html) installed.

## SYNOPSIS ##
dotenvsubst [ENVFILE]

## DESCRIPTION ##
Standard input is copied to standard output, with reference to environment variables of the form `${VARIABLE}` being replaced with corresponding values read from the specified ENVFILE. If no ENVFILE is specified, the program looks for a .env file in the current directory.  If the environment variable is not found, the text is kept intact.
The syntax for the ENVFILE is composed of variables on new lines in the form of `NAME=VALUE`.

## EXAMPLES ##
Using a custom .env_custom ENVFILE on bash:
`cat file.sh | dotenvsubst .env_custom > file_env_custom.sh`

Using default .env file in Powershell:
`Get-Content file.sh | dotenvsubst > file_env.sh`

As an example of a specific use case, you can create specific Dockerfiles from a template and .env files
`cat Dockerfile.template | dotensubst .env.development > Dockerfile.development`
`cat Dockerfile.template | dotensubst .env.production > Dockerfile.production`
