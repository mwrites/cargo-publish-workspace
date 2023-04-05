# cargo publish-workspace

This tool extends Cargo to allow you to publish a workspace, a rust mono repo.

## Installation

Ensure that you have a fairly recent version of rust/cargo installed. On Ubuntu you would also need to install `libssl-dev` and `pkg-config` packages.

```console,ignore
$ cargo install cargo-publish-workspace
```

## Usage
```console
$ cargo publish-workspace --help
Usage: cargo publish-workspace [OPTIONS] --crate-prefix <CRATE_PREFIX> [-- <cargo-publish-args>...]

Arguments:
  [cargo-publish-args]...  Additional arguments to pass to 'cargo publish'

Options:
  -p, --crate-prefix <CRATE_PREFIX>                    The prefix of the crates to publish, e.g. 'my-repo-crate-'
      --dry-run                                        Run without publishing, same as --show-order
      --show-order                                     Only display the order of crates to be published
      --target-version <TARGET_VERSION>                Specify the version to use instead of CI_TAG environment variable
      --aligned-versions-only                          Verify that every Cargo.toml version are aligned with the version to publish
      --token <TOKEN>                                  Specify the token to use instead of CRATES_IO_TOKEN environment variable
      --exclude <EXCLUDE>                              Crates to exclude and not modify (arg can be supplied multiple times)
      --verify-upload-retries <VERIFY_UPLOAD_RETRIES>  The number of retries to attempt when verifying the upload of a crate [default: 30]
  -h, --help                                           Print help
  -V, --version                                        Print version
```

## Examples

Publish
```console
$ cd my-mono-repo
$ cargo publish-workspace --target-version 1.0.0 --token CRATES_IO_TOKEN --crate-prefix PREFIX
```

Dry run and show order of crates to be published
```console
$ cd my-mono-repo
$ cargo publish-workspace --crate-prefix PREFIX --show-order
```
```console
    Finished show dependencies order
0. mat-clockwork-utils
1. mat-clockwork-cron
2. mat-clockwork-thread-program-v1
3. mat-clockwork-network-program
```

## Publishing From GitHub Action
This tool has been made to work with a CI such as GitHub Action.
Make sure to setup the GitHub secrets variable for `CRATES_IO_TOKEN` with the appropriate value.
- Version will be inferred from the $CI_TAG environment variable
- Token will be inferred from the $CRATES_IO_TOKEN environment variable
```console
cargo publish-workspace --crate-prefix PREFIX
```

## License

Apache-2.0/MIT
