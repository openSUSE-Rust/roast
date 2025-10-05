# Roast

`roast` is a set of commandline utilities to create or extract tar archives.
Its focus is ease of use and very clear commandline flag and descriptions. It
also has a library called [libroast](./libroast), which you can use to integrate
with your software tools.

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

Roast contains the following binaries:

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

# Quick Start

**Compressing a file**

Determined by file extension

```bash
roast -t directory -f file.tar.zst
```

Invalid extensions won't work. This command will fail

```bash
roast -t directory -f file.zst
```

**Recompressing** from a file to another file. This command will produce `file.tar.xz`

```bash
recomprizz -t file.tar.zst -c xz
```

**Extracting**

```bash
raw -t file.tar.zst
```

with custom directory

```bash
raw -t file.tar.zst -d new_directory
```

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
remote repository. It uses `roast` under the hood.

### Naming and Versioning

> [!NOTE]
> The naming mechanism follows a certain logic:
> - get the last segment of the git URL e.g. <https://codeberg.org/Rusty-Geckos/roast.git>'s last segment is "roast". `.git` is removed.
> - get the recent git tag if there is, otherwise, get the number of commits since revision. Format it to follow <https://en.opensuse.org/openSUSE:Package_versioning_guidelines>.
>
> **The explanation is done in reverse as you read further below so you can easily grasp the logic**.

You can *rewrite* the "revision" part of the filename. Since versions in
a specfile should be in this format, `a.b.c`, where `a` must be a numeric
string while `b` and `c` can be alphanumeric, a revision such as a tag with
names like `v7.0.0` is not considered a valid version string, despite that
it obviously indicates a version.

> A **specfile or RPM specfile** is a packaging build "recipe", specifically tailored for RPM-based
> distributions such as openSUSE, and Fedora. It contains metadata and instructions of how
> a software is packaged into the distribution.

To make it a valid version string for a specfile, the `versionrewriteregex`
must have a value like `^v?(.*)` (cause sometimes, the developer forgets to add a letter "v").
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

Since `roast_scm` is intended to be an OBS Service, an example `_service` file for
this scenario will look like this.

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

> A service file is another kind of "recipe". It contains a set of services with options configured in XML format.
> Each service has parameters to pass values that change how the command behaves.

In case that it is impossible to create a valid version, you can hard-code it
using the `set-version` flag. There is also a `set-name` flag to hard-code
the filename. This will only rename the filename excluding the file extension.
The resulting hard-coded name and version will follow the format like this:
`<set-name>-<set-version>.tar.gz`, without the angled brackets.

If the set-name value is missing, then it will try to set the name from the **last
segment** of the Git URL. For example, the URL <https://codeberg.org/Rusty-Geckos/roast>
and <https://codeberg.org/Rusty-Geckos/roast.git> will both have a "roast" name.

If the set-version value is missing, it will try to either create a
version from a recent git tag. Otherwise, it will be just the number of
commits since revision with a format `0+git<N>` where `N` is the number of
commits, and without the angled brackets. The versioning format follows the
<https://en.opensuse.org/openSUSE:Package_versioning_guidelines>.

> [!NOTE]
> One can use `outfile` flag to hard code the FULL filename.

> [!WARNING]
> There are some projects that do not follow the convention that git tags
> are for versions e.g. [wezterm](https://github.com/wezterm/wezterm). If that's
> the case, your best option for setting the version in the output tarball's filename
> is to hard-code the version.

### Changelog generation

Optionally, you can pass a value to `changesgenerate`, either `true` or `false`.

If set to `true`, one must provide a value to `changesauthor`. This is to create a timestamp + author as a changelog
header. This contains a record of who recently modified the package sources. There is an optional `changesemail`
flag that you can use to pass an email address as well.

Just below the changelog header are the list of commit summaries from the git repository. The list starts from
the target revision until the most recent tag. If there is no tag at all, it starts from the target revision until
the first initial commit.

The resulting changelog filename is based on the resulting filename EXCLUDING THE VERSION part of the generated tarball e.g. `source-1.0.1.tar.zst`
will have a changelog filename of `source.changes`. You can hard-code a full filename by passing a value to
`changesoutfile`.

If the destination `.changes` file exists, the new changelog will be prepended with the old contents of the file.

## Raw - How it works

`raw` is an extractor utility. It detects the mime-type instead of basing it from a file extension
before it extracts the tarball archive.

## Recomprizz - How it works

`recomprizz` is a recompression utility. It utilises `roast` and `raw` under the hood. It extracts the
target tarball before it creates a new tarball of a different compression option e.g. `source.tar.gz`
to `source.tar.zst`.

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
> The behaviours might differ in `roast_scm.service` if roast was compiled with `obs` feature. This additional feature flag
> will update the version from a target specfile and makes the `--silent` flag do nothing, regardless of what value is passed.
> Since the `--silent` flag does nothing, the flag is not documented in the service files.
> All of the service files are assumed to have the executables to be compiled with `obs` flag.

Most of the flags in the service files maps with the output of following commands

- `raw -h`
- `recomprizz -h`
- `roast -h`
- `roast_scm -h`


# Compression Ratios

> [!IMPORTANT]
> The filesystem used here is **apfs**. I ran the following in an M2 Macbook Air
> with 8GB of RAM and 512GB of storage.

Let's take for example, a "vendor" directory containing files and subdirectories with a total size 290MB.

We got this size by doing the following inside the project directory.

```bash
cargo vendor
du -hs vendor
```

Then we ran the following commands to produce different kinds of tarballs.

```bash
roast -S false -t vendor -f vendor.tar
roast -S false -t vendor -f vendor.tar.zst
roast -S false -t vendor -f vendor.tar.xz
roast -S false -t vendor -f vendor.tar.gz
roast -S false -t vendor -f vendor.tar.bz
```

Running this command

```bash
du -sh vendor.tar*
```

will give us the resulting compressed sizes for the following compression algorithms.

```
281M    vendor.tar
 31M    vendor.tar.bz
 40M    vendor.tar.gz
 23M    vendor.tar.xz
 23M    vendor.tar.zst
```

Hence, here are the following compression ratios calculated
from the sizes.

- No compression: **1.03:1**
- bzip2: **9.35:1**
- gzip: **7.25:1**
- xz: **12.61:1**
- zstd: **12.61:1**

Therefore, to get the **best compressed size**, you either have to select **xz** or **zstd**. Other
compressions are also decent.

> [!NOTE]
> Those numbers are not based on real statistic so the best way to calculate the predicted compressed
> size is to take a chunk of data as a stream and calculate the size of the bytes during roasting.

# Reproducibility

This project does not and will not support reproducible builds as a feature. If
you submit a PR to enable those features, we may accept it but we will not
maintain or guarantee that it will continue to work in the future.

> [!NOTE]
> Although, reproducible or deterministic tarballs are fine to reduce checks,
> the idea of reproducibility for security is something the authors
> of this project don't believe in e.g. the Jia Tan incident can't be
> caught with reproducible builds. Although, the tainted sources can be identified using
> reproducible / deterministic generation of the tarballs, those tainted sources
> are **only known AFTER a successful malicious attempt**.

[github]: https://github.com/openSUSE-Rust/roast
[sourcehut]: https://git.sr.ht/~uncomfy/roast
[codeberg]: https://codeberg.org/Rusty-Geckos/roast

