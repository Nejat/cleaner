# Cleaner

Cleaner is a utility for cleaning up build artifacts in bulk.

It recursively looks for specific build folders, at a given `path`, for the following project types:

* Rust - `target`
* .Net - `bin`, `obj`
* Web - `node_modules`
* Angular - `.angular`,`.run`
* Typescript/Deno - 'dist'

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
Cleaner is a utility for cleaning up build artifacts in bulk

Usage: cleaner.exe <COMMAND>

Commands:
  builds     Manage build artifacts of supported platforms
  empties    Manage empty folders
  repos      Search through repos
  supported  Manage supported development platforms
  help       Print this message or the help of the given subcommand(s)

Options:
  -h, --help     Print help
  -V, --version  Print version
```

## Commands

### Builds

Manages build artifacts of configured platforms

```
Manage build artifacts of supported platforms

Usage: cleaner.exe builds <COMMAND>

Commands:
  list    List matching build artifacts
  remove  Remove matching build artifacts
  help    Print this message or the help of the given subcommand(s)

Options:
  -h, --help     Print help
  -V, --version  Print version
```

#### Builds List

List matching build artifacts

```
List matching build artifacts

Usage: cleaner.exe builds list [OPTIONS] [PATH]

Arguments:
  [PATH]
          Optionally specify target path, defaults to current folder

          [default: .]

Options:
  -t, --types <TYPES>
          Optionally specify supported development platform(s), defaults to "all"

          * use "supported" command to see a list of all supported development platforms

          [default: all]

  -h, --help
          Print help (see a summary with '-h')

  -V, --version
          Print version
```

#### Builds Remove

Remove matching build artifacts

```
Remove matching build artifacts

Usage: cleaner.exe builds remove [OPTIONS] [PATH]

Arguments:
  [PATH]
          Optionally specify target path, defaults to current folder

          [default: .]

Options:
  -t, --types <TYPES>
          Optionally specify supported development platform(s), defaults to "all"

          * use "supported" command to see a list of all supported development platforms

          [default: all]

  -y, --confirmed
          Executes remove action without confirmation, defaults to interactive confirmation

  -h, --help
          Print help (see a summary with '-h')

  -V, --version
          Print version
```

### Empties

Manages empty folders, skips hidden folder by default

```
Manage empty folders

Usage: cleaner.exe empties <COMMAND>

Commands:
  list    List matching empty folders
  remove  Remove matching empty folders
  help    Print this message or the help of the given subcommand(s)

Options:
  -h, --help     Print help
  -V, --version  Print version
```

#### Empties List

List matching empty folders

```
List matching empty folders

Usage: cleaner.exe empties list [OPTIONS] [PATH]

Arguments:
  [PATH]  Optionally specify target path, defaults to current folder [default: .]

Options:
  -s, --hidden   Includes empty hidden folders, i.e. folders that start with a '.'
  -h, --help     Print help
  -V, --version  Print version
```

#### Empties Remove

Remove matching empty folders

```
Remove matching empty folders

Usage: cleaner.exe empties remove [OPTIONS] [PATH]

Arguments:
  [PATH]  Optionally specify target path, defaults to current folder [default: .]

Options:
  -y, --confirmed  Executes remove action without confirmation, defaults to interactive confirmation
  -s, --hidden     Includes empty hidden folders, i.e. folders that start with a '.'
  -h, --help       Print help
  -V, --version    Print version
```

### Repos

Search through GIT repositories

```
Search through repos

Usage: cleaner.exe repos <COMMAND>

Commands:
  branched    List repositories not in master or main
  changes     List repositories with uncommitted changes
  detached    List detached repositories, HEAD
  error       List repositories with errors
  init        List repositories that are only initialized (unborn)
  list        List repositories
  local       List repositories with no remotes configured
  main        List repositories with a "Main" branch
  master      List repositories with a "Master" branch
  outdated    List outdated repos
  up-to-date  List repositories that are up-to-date
  help        Print this message or the help of the given subcommand(s)

Options:
  -h, --help     Print help
  -V, --version  Print version
```

#### Outdated

Search for Git Repositories that are not in sync with its remote(s)

```
List outdated repos

Usage: cleaner.exe repos outdated [OPTIONS] [PATH]

Arguments:
  [PATH]
          Optionally specify target path, defaults to current folder

          [default: .]

Options:
  -f, --filter <FILTER>
          Filters outdated repos

          [default: either]

          Possible values:
          - ahead:  Only include outdated repos that are ahead in commits of the remote
          - either: Either Ahead or Behind (Default Value)
          - behind: Only include outdated repos that are behind in commits of the remote

  -m, --main
          Only check repo's Main branch

  -h, --help
          Print help (see a summary with '-h')

  -V, --version
          Print version
```

#### Up To Date

Search for Git Repositories that are in sync with its remote(s)

```
List repositories that are up-to-date

Usage: cleaner.exe repos up-to-date [OPTIONS] [PATH]

Arguments:
  [PATH]  Optionally specify target path, defaults to current folder [default: .]

Options:
  -m, --main     Only check repo's Main branch
  -h, --help     Print help
  -V, --version  Print version
```

### Supported

Supported development platforms configuration

```
Manage supported development platforms

Usage: cleaner.exe supported <COMMAND>

Commands:
  list    List configured development platforms
  path    Show path of platform configuration file
  manage  Manage platform configuration
  reset   Reset platform configuration to default
  help    Print this message or the help of the given subcommand(s)

Options:
  -h, --help     Print help
  -V, --version  Print version
```

#### Supported List

Initial/Default list of supported platforms

```

Platform:          .Net
  Build Artifacts: bin & obj
  Matched On:      *.sln & *.csproj

Platform:          Rust
  Build Artifacts: target
  Matched On:      cargo.toml

Platform:          Web
  Build Artifacts: node_modules
  Matched On:      package.json

Platform:          Angular
  Build Artifacts: .angular & .run
  Matched On:      angular.json

Platform:          Typescript
  Build Artifacts: dist
  Matched On:      tsconfig.json

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