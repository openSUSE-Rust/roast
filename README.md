# Roast

Create archive tarballs and roast them!

> [!WARNING]
> When using any of the tools, **reproducibility** is limited because file metadata depends
> on the environment even if we use reproducibility flag. It's only reproducible
> if the environment is close to what we expect e.g. directory name length.
> This is to avoid confusion when recompressing a file to the same compression format i.e.
> `vendor.tar.zst` to `vendor2.tar.zst`. **You should expect that they won't have the same
> sha256 hash. However, you should expect that `vendor2.tar.zst` and `vendor3.tar.zst`
> have the same hash if they came from the same source `vendor.tar.zst`.
> The fix could be to match the length of the directory name of the temporary directory
> as obs-service-cargo. I have not tested that yet though ☺️

## Reason of existence

I am trying to split the logic from [obs-service-cargo](https://github.com/openSUSE-Rust/obs-service-cargo).
Not only is this a library, it also contains binaries that extract and decompress tarballs or create
tarballs with the available highest level compression for supported compression formats.

Plus, it has the comfort of being a simple `tar` alternative.

# How to install the binaries

Roast contains to binaries
- `roast`
- `raw`

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

## CLI

```
roast [OPTIONS] --target <TARGET> --outpath <OUTPATH>

Options:
  -t, --target <TARGET>
          Target directory to archive. This will be set as the root directory of the archive.
  -a, --additional-paths <ADDITIONAL_PATHS>
          Additional paths such as files or directories to add to the archive. Their parent directory will be put next to the target directory.
  -o, --outpath <OUTPATH>
          Output file of the tarball with path.
  -p, --preserve-root
          Preserve root directory instead of only archiving relative paths.
  -r, --reproducible
          Allow reproducibility for Reproducible Builds 🥴
  -h, --help
          Print help (see more with '--help')
  -V, --version
          Print version
```

```
raw [OPTIONS] --target <TARGET>

Options:
  -t, --target <TARGET>    Target tarball file to extract and decompress.
  -o, --outpath <OUTPATH>  Output path of extracted archive. DEFAULT is current directory if omitted.
  -h, --help               Print help (see more with '--help')
  -V, --version            Print version
```

```
roast-cli 3.3.1 - Recompress to other compression formats

recomprizz [OPTIONS] --target <TARGET> --compression <COMPRESSION>

Options:
  -t, --target <TARGET>            Target tarball file to extract and recompress.
  -o, --outpath <OUTPATH>          Output path of recompressed archive. DEFAULT: current directory if omitted.
  -c, --compression <COMPRESSION>  Compression to use. [possible values: gz, xz, zst, bz2, not]
  -R, --rename <RENAME>            Use this flag if you want a new filename to use ignoring the new file extension. Omitting this flag will just fallback to basename.
  -r, --reproducible               Allow reproducibility for Reproducible Builds. DEFAULT: false
  -h, --help                       Print help (see more with '--help')
  -V, --version                    Print version
```

