# Roast

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

## CLI Help

```
roast 3.3.1 - Archiver with high-level compression

roast [OPTIONS] --target <TARGET> --outfile <OUTFILE>

Options:
  -t, --target <TARGET>
          Target directory to archive. This will be set as the root directory of the archive.
  -a, --additional-paths <ADDITIONAL_PATHS>
          Additional paths such as files or directories to add to the archive. Their parent directory will be put next to the target directory.
  -o, --outfile <OUTFILE>
          Output file of the generated archive with path.
  -p, --preserve-root
          Preserve root directory instead of only archiving relative paths. DEFAULT: false.
  -r, --reproducible
          Allow reproducibility for Reproducible Builds. DEFAULT: false.
  -h, --help
          Print help (see more with '--help')
  -V, --version
          Print version
```

```
raw 3.3.1 - Raw extractor and decompressor

raw [OPTIONS] --target <TARGET>

Options:
  -t, --target <TARGET>  Target tarball file to extract and decompress.
  -o, --outdir <OUTDIR>  Output path of extracted archive. DEFAULT: current directory if omitted.
  -h, --help             Print help (see more with '--help')
  -V, --version          Print version
```

```
recomprizz 3.3.1 - Recompress to other compression formats

recomprizz [OPTIONS] --target <TARGET>

Options:
  -t, --target <TARGET>            Target tarball file to extract and recompress.
  -o, --outdir <OUTDIR>            Output directory of recompressed archive. DEFAULT: current directory if omitted.
  -c, --compression <COMPRESSION>  Compression to use. [default: zst] [possible values: gz, xz, zst, bz2, not]
  -R, --rename <RENAME>            Use this flag if you want a new filename to use ignoring the new file extension. Omitting this flag will just fallback to basename.
  -r, --reproducible               Allow reproducibility for Reproducible Builds. DEFAULT: false.
  -h, --help                       Print help (see more with '--help')
  -V, --version                    Print version
```

