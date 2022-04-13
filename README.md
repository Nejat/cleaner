# Cleaner

Cleaner is a utility for cleaning up build artifacts in bulk.

It recursively looks for specific build folders, at a given `path`, for the following project types:

* Rust - `target`
* .Net - `bin`, `obj` 
* Web - `node_modules`

## Install

```shell
cargo install cleaner
```

 or

```shell
cargo install --git https://github.com/nejat/cleaner.git
```

_\* requires rust and cargo - [Rust Install Instructions](https://rustup.rs/)_

## Usage

```
cleaner 0.6.0
Cleaner is a utility for cleaning up build artifacts in bulk

USAGE:
    cleaner.exe <SUBCOMMAND>

OPTIONS:
    -h, --help       Print help information
    -V, --version    Print version information

SUBCOMMANDS:
    builds       Manage build artifacts of supported platforms
    empties      Manage empty folders
    help         Print this message or the help of the given subcommand(s)
    supported    Manage supported development platforms
```

## Commands

### Builds

Manages build artifacts of configured platforms

```
> cleaner builds
cleaner-builds 0.6.0
Manage build artifacts of supported platforms

USAGE:
    cleaner builds [OPTIONS] [SUBCOMMAND]

OPTIONS:
    -h, --help
            Print help information

    -p, --path <PATH>
            Optionally specify target path, defaults to current folder

            [default: .]

    -t, --types <TYPES>
            Optionally specify supported development platform(s), defaults to "all"

            * use "supported" command to see a list of all supported
            development platforms

            [default: all]

    -V, --version
            Print version information

    -y, --confirmed
            Executes remove action non-interactively,, defaults to interactive

SUBCOMMANDS:
    help
            Print this message or the help of the given subcommand(s)
    list
            List matching folders, default subcommand
    remove
            Remove matching folders
```


### Empties

Manages empty folders, skips hidden folder by default

```
> cleaner empties
cleaner-empties 0.6.0
Manage empty folders

USAGE:
    cleaner empties [OPTIONS] [SUBCOMMAND]

OPTIONS:
    -h, --help           Print help information
    -p, --path <PATH>    Optionally specify target path, defaults to current folder [default: .]
    -s, --hidden         Includes empty hidden folders, i.e. folders that start with a '.'
    -V, --version        Print version information
    -y, --confirmed      Executes remove action non-interactively,, defaults to interactive

SUBCOMMANDS:
    help      Print this message or the help of the given subcommand(s)
    list      List matching folders, default subcommand
    remove    Remove matching folders
```

### Supported

Lists all configured development platforms

```
cleaner-supported 0.6.0
Manage supported development platforms

USAGE:
    cleaner.exe supported [SUBCOMMAND]

OPTIONS:
    -h, --help       Print help information
    -V, --version    Print version information

SUBCOMMANDS:
    help      Print this message or the help of the given subcommand(s)
    list      List configured development platforms
    manage    Manage platform configuration
    path      Show path of platform configuration file
    reset     Reset platform configuration to default
```

## Supported Platforms

Initial/Default list of supported platforms

```

Platform: .Net
  Build Artifacts: bin & obj
  Matched On: *.sln & *.csproj

Platform: Rust
  Build Artifacts: target
  Matched On: cargo.toml

Platform: Web
  Build Artifacts: node_modules
  Matched On: package.json

```
_* deleting or resetting configuration will re-create this list_

## Road Map

* [x] list projects with build artifacts
* [x] remove build artifacts from projects
* [x] list empty folders
* [x] remove empty folders
* [x] configuration to support custom list of development platforms
* [x] wild card support for matching files used to determine platform type 
* [x] _path command to show path of supported configuration json_
* [x] _reset command to revert supported configuration json to default_
* [ ] _new command to manage configuration list_
* [ ] _will consider new logic for detecting build artifacts on a case by case bases, will consider pull requests_