# Roast

Read-only mirrors are available on [GitHub][github] and [sourcehut][sourcehut].

The main repository is on [codeberg][codeberg], which is where the issue tracker is found and where contributions are accepted.

<a href="https://codeberg.org/Rusty-Geckos/roast" target="_blank"><img alt="Join Us Now on Codeberg" src="./advocacy/join-us-now-on-blue-on-white.png" height="60" /></a>
<a href="https://codeberg.org" target="_blank"><img alt="Support and Promote Codeberg" src="./advocacy/support-and-promote-blue-on-white.png" height="60" /></a>

<p align="center>
  <a href="https://ci.codeberg.org/repos/13976"><img src="https://ci.codeberg.org/api/badges/13976/status.svg" /></a>
  <a href="https://build.opensuse.org/package/show/Archiving/roast" target="_blank"><img src="https://build.opensuse.org/projects/Archiving/packages/roast/badge.svg?type=percent" /></a>
  <br />
  <a href="https://crates.io/crates/roast-cli/"><img src="https://img.shields.io/crates/v/roast-cli?style=flat&logo=Rust&label=roast-cli"></a>
  <a href="https://crates.io/crates/libroast/"><img src="https://img.shields.io/crates/v/libroast?style=flat&logo=Rust&label=libroast"></a>
</p>

Create archive tarballs and roast them!

# How to install the binaries

Roast contains to binaries
- `raw`
- `recomprizz`
- `roast`
- `roast_scm`

## Cargo

**From source**:
```bash
cargo install --git https://github.com/openSUSE-Rust/roast
```

**From crates.io**:
```bash
cargo install roast-cli
```

Both commands pull from source. The only difference is that the first one
obviously relies on git.

## Roast - How it works

There are three path behaviours in Roast.
- excluded paths
- additional paths
- included paths

Excluded paths and included paths are **within** the source or target
directory. For example. If we are going to archive the `roast-cli` directory
here, declared paths in the `--exclude` and `--include` paths are relative
to the top-most level directory of the source or target directory e.g. `src/bin/roast.rs`
points to `roast-cli/src/bin/roast.rs`.

One thing to note about the path behaviours is the higher precedence over files than directories.

- If a **directory is INCLUDED while EXCLUDED**, it is, therefore, **IGNORED**.
- If a **file is INCLUDED but it is WITHIN an EXCLUDED directory**, it is,
therefore, **ADDED with the directory created if directory (new parent of
the file) does not exist**.
- If a **directory is ADDED i.e. from outside but resulting destination should
be EXCLUDED**, it is, therefore, **ADDED**.

> [!IMPORTANT]
> ADDED != INCLUDED. ADDED can either point to any path. INCLUDED always points WITHIN
> the top-most level directory of the source or target directory.

> [!NOTE]
> The reasoning behind the **third point** is that the user may have intended to
use a different source or to include only a specific set of files, thereby
ignoring the top-level directory of the original source.

As of now, the output file's filename MUST INCLUDE the extension. We might want to change this behaviour
in the future where a user will only provide the filename without indicating the extension since
the extension should be based on the compression option.

## Roast SCM - How it works

`roast_scm` is an extended utility of `roast`. Its purpose is to create tarballs from a
remote repository. The behaviour is similar to `roast` but only at some point.

### With OBS Feature enabled

> [!NOTE]
> This feature is to be used only for [OBS](https://openbuildservice.org/) and only affects
> the `roast_scm` library and binary.
>
> Do remember that `raw`, `roast`, and `recomprizz` can be used for OBS even without the feature flag.

If `roast-cli` was compiled with `obs` feature, you can *rewrite* the "revision" part
of the filename. Since versions in a specfile should be in this format, `a.b.c`, where
`a` must be a numeric string while `b` and `c` can be alphanumeric, a revision such
as a tag with names like `v7.0.0` is not considered a valid version string, despite
that it obviously indicates a version.

To make it a valid version string for a specfile, the `versionrewriteregex`
must have a value like `^v?(.*)` (cause sometimes, the developer forgots to add a letter "v").
Then rearrange the string based on the regex by indicating the capture groups. You can pass
this pattern of rearrangement to replace the old string value to `versionrewritepattern`.
The value for `versionrewritepattern` is "$1".

> [!IMPORTANT]
> This regex replacement format for the `--versionrewritepattern` is based on the
> [regex crate](https://docs.rs/crate/regex/latest)
> [example](https://docs.rs/regex/latest/regex/struct.Regex.html#example-10).
>
> Capture groups are denoted by `$` and a number based on their position
> from other capture groups starting from the left-most side of the string.

Since `roast_scm` is intended to be an OBS Service,
an example `_service` file for this scenario will look like this.

```xml
<services>
  <service name="roast_scm" mode="manual">
    <param name="src">https://github.com/openSUSE-Rust/obs-service-cargo</param>
    <param name="versionrewriteregex">^v?(.*)</param>
    <param name="versionrewritepattern">${1}</param>
    <param name="revision">v7.0.0</param>
  </service>
</services>
```

In case that it is impossible to create a valid version, you can hard-code it
using the `set-version` flag. There is also a `set-name` flag to hard-code
the filename. This will only rename the filename excluding the file extension.

> [!NOTE]
> One can use `outfile` flag to hard code the FULL filename.

#### Changelog generation

Optionally, you can pass a value to `changesgenerate`, either `true` or `false`.

If set to `true`, one must provide a value to `changesauthor`. This is to create
a timestamp + author as a changelog header. This contains a record of who generated
the tarball. There is an optional `changesemail` flag that you can use to pass
an email address as well.

Just below the changelog header are the list of commit summaries from the git
repository. The list starts from the target revision until the most recent
tag. If there is no tag at all, it starts from the target revision until
the first initial commit.

The resulting changelog filename is based on the resulting filename of the
generated tarball e.g. `source.tar.zst` will have a changelog filename of
`source.changes`. You can hard-code a full filename by passing a value to
`changesoutfile`.

If the destination `.changes` file exists, the new changelog will be prepended
with the old contents of the file.

## Raw - How it works

`raw` is an extractor utility. It detects the mime-type instead of basing it from a file extension
before it extracts the tarball archive.

## Recomprizz - How it works

`recomprizz` is a recompression utility. It utilises `roast` and `raw` under the hood. It extracts the
target tarball before it creates a new tarball of a different compression option e.g. `source.tar.gz`
to `source.tar.zst`. The renaming scheme is too dumb and simple though, and not perfect—see note below.

You might want to _rename_ the resulting output file with `recomprizz`. There are two flags you should
know:
- `--rename`
- `--renamepattern`

The `--rename` flag can be used to either hard-code a filename or use a valid regex which can be used
for `--renamepattern`.

The `--renamepattern` should be a string that contains the *capture groups* based on the regex you
passed to `--rename`.

For example, you want to rename `roast.tar.zst` to `raw.tar.zst`, then
you can just hard-code it by just passing "raw" to `--rename`. In another
example, you want to rename `source.tar.zst` to `marvelous-source.tar.zst`,
then you must first pass a valid regex to `--rename` like `(.*).tar.zst`,
and the `--renamepattern` should be `marvelous-${1}.tar.zst`. Of course,
since you are going to use some form of shell like bash to run the commands,
you must escape `$` like so -> `\${1}`.

> [!IMPORTANT]
> This regex replacement format for the `--renamepattern` is based on the
> [regex crate](https://docs.rs/crate/regex/latest)
> [example](https://docs.rs/regex/latest/regex/struct.Regex.html#example-10).
>
> Capture groups are denoted by `$` and a number based on their position
> from other capture groups starting from the left-most side of the string.

The difference between hard-coded vs regex is that when hard-coded, you just
need to pass a desired name EXCLUDING the file extensions. However, if the
target file has no file extension, the recompressed output file will have
a file extension based on the compression option which defaults to `.tar.zst`.

If `--rename` has a regex, then `--renamepattern` should have a value. **The
constructed regex should encompass the whole filename** e.g. a filename of `vendor.tar.zst`
with a `--rename` regex value of `(.*)` and `--compression` of "gz" will have an output
filename of `vendor.tar.zst.tar.gz`. Hence, be careful on how you construct your regex.


> [!WARNING]
> If you accidentally pass a string with no regex to `--rename` flag
> and then pass a string as well with `--renamepattern`, the rename might result
> in an undesirable output. That is NOT A BUG.

> [!IMPORTANT]
> Since `recomprizz` can be used without the `--rename` flag, filenames that
> do not follow the usual file extensions with their supported formats
> will be forced to retain its old filename but with a new file extension
> based on the value of the compression option e.g.
> a target file `vendor.tar.wrong.ext` will have an output file
> `vendor.tar.wrong.ext.tar.gz`. A zstd compressed tarball with filename
> `vendor.tar.gz` that is recompressed as a gz file will have an output filename
> of `vendor.tar.gz.tar.gz`.
>
> Files with the correct file extension and mime-type will have a desired
> output filename.

# Service files are in the following with descriptions.

- [raw.service](./raw.service)
- [recomprizz.service](./recomprizz.service)
- [roast_scm.service](./roast_scm.service)
- [roast.service](./roast.service)

> [!NOTE]
> The options might differ in `roast_scm.service` since those options only exist if the `obs` feature was enabled. These flags or options are
> - `set-name`
> - `set-version`
> - `versionrewriteregex`
> - `versionrewritepattern`

It maps when you run the following commands
- `raw -h`
- `recomprizz -h`
- `roast -h`
- `roast_scm -h`

[github]: https://github.com/openSUSE-Rust/roast
[sourcehut]: https://git.sr.ht/~uncomfy/roast
[codeberg]: https://codeberg.org/Rusty-Geckos/roast

