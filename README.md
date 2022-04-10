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
cleaner 0.2.0
Cleaner is a utility for cleaning up build artifacts in bulk

USAGE:
    cleaner <SUBCOMMAND>

OPTIONS:
    -h, --help       Print help information
    -V, --version    Print version information

SUBCOMMANDS:
    builds       Manage build artifacts of supported platforms
    help         Print this message or the help of the given subcommand(s)
    supported    List supported development platforms
```

## Commands

### Builds

```
> cleaner builds
cleaner-builds 0.2.0
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
            Executes remove action automatically confirmed, i.e. non-interactively,
            defaults to interactive

SUBCOMMANDS:
    help
            Print this message or the help of the given subcommand(s)
    list
            List corresponding folders, default subcommand
    remove
            Remove corresponding folders
```

### Supported

```shell
> cleaner supported
```

Lists all supported development platforms

## Supported Platforms

```

Platform: .Net
  Build Artifacts: bin, obj
  Matched On: sln, csproj

Platform: Rust
  Build Artifacts: target
  Matched On: cargo.toml

Platform: Web
  Build Artifacts: node_modules
  Matched On: package.json

```

_\* this version has a static list of supported development platforms, see [road map](#road-map) for upcoming features_

## Road Map

* [ ] _configuration to support custom list of development platforms_
* [ ] _new commands to manage configuration list_
* [ ] _will consider new logic for detecting build artifacts on a case by case bases, will consider pull requests_