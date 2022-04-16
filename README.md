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
cleaner 0.9.1
Cleaner is a utility for cleaning up build artifacts in bulk

USAGE:
    cleaner <SUBCOMMAND>

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
cleaner-builds 0.9.1
Manage build artifacts of supported platforms

USAGE:
    cleaner builds <SUBCOMMAND>

OPTIONS:
    -h, --help       Print help information
    -V, --version    Print version information

SUBCOMMANDS:
    help      Print this message or the help of the given subcommand(s)
    list      List matching build artifacts
    remove    Remove matching build artifacts
```

#### Builds List

List matching build artifacts

```
cleaner-builds-list 0.9.1
List matching build artifacts

USAGE:
    cleaner builds list [OPTIONS] [PATH]

ARGS:
    <PATH>
            Optionally specify target path, defaults to current folder
            
            [default: .]

OPTIONS:
    -h, --help
            Print help information

    -t, --types <TYPES>
            Optionally specify supported development platform(s), defaults to "all"
            
            * use "supported" command to see a list of all supported development platforms
            
            [default: all]

    -V, --version
            Print version information
```

#### Builds Remove

Remove matching build artifacts

```
cleaner-builds-remove 0.9.1
Remove matching build artifacts

USAGE:
    cleaner builds remove [OPTIONS] [PATH]

ARGS:
    <PATH>
            Optionally specify target path, defaults to current folder
            
            [default: .]

OPTIONS:
    -h, --help
            Print help information

    -t, --types <TYPES>
            Optionally specify supported development platform(s), defaults to "all"
            
            * use "supported" command to see a list of all supported development platforms
            
            [default: all]

    -V, --version
            Print version information

    -y, --confirmed
            Executes remove action without confirmation, defaults to interactive confirmation
```

### Empties

Manages empty folders, skips hidden folder by default

```
cleaner-empties 0.9.1
Manage empty folders

USAGE:
    cleaner empties <SUBCOMMAND>

OPTIONS:
    -h, --help       Print help information
    -V, --version    Print version information

SUBCOMMANDS:
    help      Print this message or the help of the given subcommand(s)
    list      List matching empty folders
    remove    Remove matching empty folders
```

#### Empties List

List matching empty folders

```
cleaner-empties-list 0.9.1
List matching empty folders

USAGE:
    cleaner empties list [OPTIONS] [PATH]

ARGS:
    <PATH>    Optionally specify target path, defaults to current folder [default: .]

OPTIONS:
    -h, --help       Print help information
    -s, --hidden     Includes empty hidden folders, i.e. folders that start with a '.'
    -V, --version    Print version information
```

#### Empties Remove

Remove matching empty folders

```
cleaner-empties-remove 0.9.1
Remove matching empty folders

USAGE:
    cleaner empties remove [OPTIONS] [PATH]

ARGS:
    <PATH>    Optionally specify target path, defaults to current folder [default: .]

OPTIONS:
    -h, --help         Print help information
    -s, --hidden       Includes empty hidden folders, i.e. folders that start with a '.'
    -V, --version      Print version information
    -y, --confirmed    Executes remove action without confirmation, defaults to interactive
                       confirmation
```

### Supported

Supported development platforms configuration

```
cleaner-supported 0.9.1
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

#### Supported List

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
* [x] _list supported platforms_
* [x] _command to manage configuration list_
* [ ] _better handling of input cancelling_
* [ ] _will consider new logic for detecting build artifacts on a case by case bases, will consider pull requests_