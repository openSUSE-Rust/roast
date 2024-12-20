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

> [!NOTE]
> When using `recomprizz`, files with filenames like `package-1.2.3.tar.gz` will have
> the number parts of their names preserved i.e. `package-1.2.3.tar.gz` -> `package-1.2.3.tar.zst`.
> However, filenames with letters after the numbers will be removed especially for version part
> of the filenames are tagged as `alpha` or `beta`. For example, `package-1.2.3.alpha.tar.gz` will
> turn into `package-1.2.3.tar.zst`. This is a limitation with the renaming logic. The solution is
> to use the `-R` or `--rename` flag to hardcode the new name. So a command like
> ```
> recomprizz -t package-1.2.3.alpha.tar.gz -R package-1.2.3.alpha
> ```
> should fix the issue.

## Reason of existence

I am trying to split the logic from [obs-service-cargo](https://github.com/openSUSE-Rust/obs-service-cargo).
Not only is this a library, it also contains binaries that extract and decompress tarballs or create
tarballs with the available highest level compression for supported compression formats.

Plus, it has the comfort of being a simple `tar` alternative.

# How to install the binaries

Roast contains to binaries
- `roast`
- `raw`
- `recomprizz`

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

# Service files are in the following with descriptions.

- [recomprizz.service](./recomprizz.service)
- [roast.service](./roast.service)
- [raw.service](./raw.service)

It maps when you run the following commands
- `roast -h`
- `raw -h`
- `recomprizz -h`

[github]: https://github.com/openSUSE-Rust/roast
[sourcehut]: https://git.sr.ht/~uncomfy/roast
[codeberg]: https://codeberg.org/Rusty-Geckos/roast
~

