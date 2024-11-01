# Roast

<p align="center>
  <a href="https://github.com/openSUSE-Rust/roast/actions/workflows/ci.yml"><img src="https://github.com/openSUSE-Rust/roast/actions/workflows/ci.yml/badge.svg?branch=main" /></a>
  <a href="https://build.opensuse.org/package/show/Archiving/roast" target="_blank"><img src="https://build.opensuse.org/projects/Archiving/packages/roast/badge.svg?type=default" /></a>
  <br />
  <a href="https://crates.io/crates/roast-cli/"><img src="https://img.shields.io/crates/v/roast-cli?style=flat&logo=Rust&label=roast-cli"></a>
  <a href="https://crates.io/crates/libroast/"><img src="https://img.shields.io/crates/v/libroast?style=flat&logo=Rust&label=libroast"></a>
</p>

Create archive tarballs and roast them!

> [!WARNING]
> When using any of the tools, **reproducibility** is limited because file metadata depends
> on the environment even if we use reproducibility flag. It's only reproducible
> if the environment is close to what we expect e.g. directory name length.
> This is to avoid confusion when recompressing a file to the same compression format i.e.
> `vendor.tar.zst` to `vendor2.tar.zst`. **You should expect that they won't have the same
> sha256 hash**. However, _you should expect that `vendor2.tar.zst` and `vendor3.tar.zst`
> have the same hash if they came from the same source `vendor.tar.zst`_.
> The fix could be to try to match the length of the directory name of the temporary directory
> as obs-service-cargo. I have not tested that yet though ☺️

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

One thing to note about the path behaviours is there higher precedence over files than directories.

- If a **directory is INCLUDED while EXCLUDED**, it is, therefore, **IGNORED**.
- If a **file is INCLUDED but it is WITHIN an EXCLUDED directory**, it is,
therefore, **ADDED with the directory created if directory (new parent of
the file) does not exist**.
- If a **directory is ADDED i.e. from outside but resulting destination should
be EXCLUDED**, it is, therefore, **ADDED**.

> [!NOTE]
> The rationale for this is the user might intended to do this i.e. use another
source instead or add only a set of number of files i.e. ignoring the
top-level most directory of the source itself!

## CLI Help

```
roast 4.7.0 - Archiver with high-level compression

roast [OPTIONS] --target <TARGET> --outfile <OUTFILE>

Options:
  -t, --target <TARGET>
          Target directory to archive. This will be set as the root directory of the archive. Supports globbing.
  -i, --include <INCLUDE>
          Additional paths such as files or directories in the target directory to include to the archive. Their parent directory will be put next to the target directory's work directory. The work directory is based on the preserve root option. This is different from `--additional_paths`. Useful to override excluded directories. ⚠️ Careful if the archive has whether preserved root set when it was created.
  -E, --exclude <EXCLUDE>
          Additional paths such as files or directories from within target directory's work directory to exclude when generating the archive.
  -A, --additional-paths <ADDITIONAL_PATHS>
          Additional paths such as files or directories to add to the archive. Their parent directory will be put next to the target directory. This is different from `--include`. Optionally, one can add a path to a directory inside the archive e.g. `-A some/file/to/archive,put/where/in/archive`. If directory does not exist, it will be created.
  -f, --outfile <OUTFILE>
          Output file of the generated archive with path.
  -d, --outdir <OUTDIR>
          Output path of extracted archive.
  -p, --preserve-root <PRESERVE_ROOT>
          Preserve root directory instead of only archiving relative paths. [default: false] [possible values: true, false]
  -r, --reproducible <REPRODUCIBLE>
          Allow reproducibility for Reproducible Builds. [default: false] [possible values: true, false]
  -g, --ignore-git <IGNORE_GIT>
          Whether to ignore git related metadata, files and directories. [default: true] [possible values: true, false]
  -I, --ignore-hidden <IGNORE_HIDDEN>
          Whether to ignore hidden directories and files or what we call dotfiles. Does not affect `--ignore-git`. [default: false] [possible values: true, false]
  -h, --help
          Print help (see more with '--help')
  -V, --version
          Print version

Maintained by Soc Virnyl Estela <contact@uncomfyhalomacro.pl>.
```

```
raw 4.7.0 - Raw extractor and decompressor

raw [OPTIONS] --target <TARGET>

Options:
  -t, --target <TARGET>  Target tarball file to extract and decompress. Supports globbing.
  -d, --outdir <OUTDIR>  Output directory of extracted archive.
  -h, --help             Print help (see more with '--help')
  -V, --version          Print version

Maintained by Soc Virnyl Estela <contact@uncomfyhalomacro.pl>.
```

```
recomprizz 4.7.0 - Recompress to other compression formats

recomprizz [OPTIONS] --target <TARGET>

Options:
  -t, --target <TARGET>
          Target tarball file to extract and recompress. Supports globbing.
  -i, --include <INCLUDE>
          Additional paths such as files or directories in the target directory to include to the archive. Their parent directory will be put next to the target directory's work directory. The work directory is based on the preserve root option. This is different from `--additional_paths`. Useful to override excluded directories.
  -E, --exclude <EXCLUDE>
          Additional paths such as files or directories from within target directory's work directory to exclude when generating the archive. ⚠️ Careful if the archive has whether preserved root set when it was created.
  -A, --additional-paths <ADDITIONAL_PATHS>
          Additional paths such as files or directories to add to the archive. Their parent directory will be put next to the target directory. This is different from `--include`. Optionally, one can add a path to a directory inside the archive e.g. `-A some/file/to/archive,put/where/in/archive`. If directory does not exist, it will be created.
  -d, --outdir <OUTDIR>
          Output directory of recompressed archive.
  -c, --compression <COMPRESSION>
          Compression to use. [default: zst] [possible values: gz, xz, zst, bz2, not]
  -R, --rename <RENAME>
          Use this flag if you want a new filename to use ignoring the new file extension. Omitting this flag will just fallback to basename.
  -r, --reproducible <REPRODUCIBLE>
          Allow reproducibility for Reproducible Builds. [default: false] [possible values: true, false]
  -g, --ignore-git <IGNORE_GIT>
          Whether to ignore git related metadata, files and directories. [default: true] [possible values: true, false]
  -I, --ignore-hidden <IGNORE_HIDDEN>
          Whether to ignore hidden directories and files or what we call dotfiles. Does not affect `--ignore-git`. [default: false] [possible values: true, false]
  -h, --help
          Print help (see more with '--help')
  -V, --version
          Print version

Maintained by Soc Virnyl Estela <contact@uncomfyhalomacro.pl>.
```

