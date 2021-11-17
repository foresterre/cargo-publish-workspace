# cargo-publish-workspace

⚠ This project is not under active development. It was developed when there were no satisfactory solutions for publishing complete Cargo workspaces available, and is completely tailored towards my projects; only containing the features I need. If you're looking for an alternative, I would suggest:
* https://github.com/crate-ci/cargo-release

## Goals

* Publish workspace packages to crates.io with a single command.
* Support the package versioning scheme where all packages have the same major and minor version number.
* Contribute to enhanced release automation, so releasing a new version takes less time, which enables us to release more often,
 and with smaller increments.

## Command & options

```
cargo-publish-workspace
Topologically publish a complete workspace

All arguments provided after two dashes (--) will be passed on to 'cargo publish'. This means, if
cargo publish-workspace itself doesn't support a flag related to publishing a cargo crate (yet), you
can still use this method. For example, you may use a custom registry with the following command
`cargo publish-workspace <..options> -- --registry <registry>`. The '--registry <registry>'
arguments will be passed to cargo publish. Note: some arguments are also passed on by cargo publish-
workspace, in which case, if also provided after the two dashes may be passed on twice. For example,
this would be the case if we would run: `cargo publish-workspace <...options> --no-verify -- --no-
verify`.

By default, a tag formatted `v<version>` (e.g. v1.0.3) will be created from the current working
directory.

USAGE:
    cargo publish-workspace [FLAGS] [OPTIONS] --new-version <new-version>

FLAGS:
        --dry-run
            Simulate running this program

    -h, --help
            Prints help information

        --no-git-tag
            Don't create a tag after publishing

            Tags are named after the new (workspace) version and prefixed with 'v'; for example
            'v1.2.0'.

        --no-verify
            Don't build the crate before publishing

    -V, --version
            Prints version information


OPTIONS:
        --manifest <manifest>
            The workspace manifest file, usually the root Cargo.toml [default: Cargo.toml]

        --new-version <new-version>
            The version to which all workspace crates will be updated

            Version must be semver compatible.

        --sleep <sleep>
            The amount of seconds to sleep between publishing of workspace packages

            Allows the crate registry index to update, which may be important since consecutive
            attempts to publish crates may contain the just published crate as dependency, and if
            the registry hasn't processed a crate dependency yet, publishing will fail. [default:
            15]


Issues, requests and questions can be submitted at: 'https://github.com/foresterre/cargo-publish-
workspace/issues', thanks!
```

## Examples

Update packages in workspace to `0.17.0`, without verifying before publish. Sleep 15 seconds between publishing of packages, so the crates.io registry has time to refresh.
```
cargo publish-workspace --sleep 15 --no-verify --new-version 0.17.0
```
