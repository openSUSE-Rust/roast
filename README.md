# Roast

Create archive tarballs and roast them!

## Reason of existence

I am trying to split the logic from [obs-service-cargo](https://github.com/openSUSE-Rust/obs-service-cargo).
Not only is this a library, it also contains binaries that extract and decompress tarballs or create
tarballs with the available highest level compression for supported compression formats.

Plus, it has the comfort of being a simple `tar` alternative.

# How to install the binaries

Roast contains to binaries
- `roast`
- `raw`
- 

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

