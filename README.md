# Cleaner

Cleaner is a utility for cleaning up build artifacts in bulk.

It recursively looks for specific build folders, at a given `path`, for the following project types:

* Rust - `target`
* .Net - `bin`, `obj` 
* Web - `node_modules`

## Install

```shell
cargo install --git https://github.com/nejat/cleaner.git
```

_\* requires rust and cargo - [Rust Install Instructions](https://rustup.rs/)_

## Usage

```
cleaner <path> [-forreals]
```

* `path` - required parameter that specifies to *clean*
* `-forreals` - optional switch that deletes all build artifacts found<br>
 *without the switch* `cleaner` *only lists the found build artifacts* 

