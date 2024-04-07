# rm-rs

A cross-platform alternative to `rm`, written in Rust for native performance.

## Installation

### With npm

```shell
npm install --save-dev rm-rs
# OR
npx rm-rs
```

### With pnpm

```shell
pnpm install --save-dev rm-rs
# OR
pnpm dlx rm-rs
```

### With cargo

```shell
cargo install rm-rs
```

## Usage examples

```shell
# Delete some files
rm-rs file1 file2 ...

# Delete an empty directory
rm-rs -d directory

# Delete a directory and all its children
rm-rs -r directory

# Delete a file, no errors if it doesn't exist
rm-rs -f file
```

### Help

```console
$ rm-rs --help

Usage: rm-rs [OPTIONS] <FILES>...

Arguments:
  <FILES>...

Options:
  -d          Attempt to remove directories as well as other types of files
  -r          Recursively remove directories and the files they contain. This implies the -d option
  -f          Ignore "file not found" errors
  -h, --help  Print help
```
