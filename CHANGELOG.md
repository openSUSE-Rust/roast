# Changelog

All notable changes to this project will be documented in this file.

## [unreleased]

### 🚜 Refactor

- Remove git2 and any code related to it

### 📚 Documentation

- Update ci badge links in README

### ⚙️ Miscellaneous Tasks

- Make libroast a workspace dependency
- Add git2-rs as dependency
- Update structure and boilerplate logic
- Auto close prs. contribute to https://codeberg.org/Rusty-Geckos/roast instead.
- Update URLs
- Prepare woodpecker build
- Remove aarch64 target for now
- Add nightly toolchain. for fmt only.

### Format

- Run cargo +nightly fmt

## [5.1.7] - 2024-11-27

### 🚀 Features

- Add zstd as another alias for zst

### 🐛 Bug Fixes

- Added trace feature

### ⚙️ Miscellaneous Tasks

- Add filtering so it only runs when `.rs` and `Cargo.toml` change
- *(release)* V5.1.7

### Dep

- Update clap features to use

### Minor

- Improve error message here

## [5.1.6] - 2024-11-02

### 🐛 Bug Fixes

- Revert 8977c6741364c6d25fb33408d8b4232d835a768b

### ⚙️ Miscellaneous Tasks

- Release v5.1.6

## [5.1.5] - 2024-11-02

### ⚙️ Miscellaneous Tasks

- Release v5.1.5

## [5.1.4] - 2024-11-02

### ⚙️ Miscellaneous Tasks

- Release v5.1.4

## [5.1.3] - 2024-11-02

### 🐛 Bug Fixes

- Just use an empty "" if strip fails

### 📚 Documentation

- Removed warning. ensured reproducibility.

### ⚙️ Miscellaneous Tasks

- Set resolver to 2 and enforce strict linting rules
- Release v5.1.3

### Clippy

- Rectify the needless pass by value

## [5.1.2] - 2024-11-01

### ⚙️ Miscellaneous Tasks

- Release 5.1.2

### Minor

- Improvements on how we sort files and directories

## [5.1.0] - 2024-11-01

### 📚 Documentation

- Explain how the path behaviour works
- Fix grammar [ci skip]
- Improve wording [ci skip]
- Add important difference between ADDED and INCLUDED [ci skip]

### ⚡ Performance

- Add rayon to parallelise copying operations

### ⚙️ Miscellaneous Tasks

- Release 5.1.0

## [5.0.0] - 2024-11-01

### 🚀 Features

- Hidden file and gitignore finally correctly implemented

### 🐛 Bug Fixes

- Just use ends_with to check if it's a valid file extension
- Do not consider temporary directory as hidden
- Avoid duplicating entries
- Reimplement adding of archive files
- Resolved some edge-cases with additional paths and included paths

### 🚜 Refactor

- Improve the logic handling for adding, excluding and including

### 📚 Documentation

- Update README on CLI help

### ⚙️ Miscellaneous Tasks

- Release 5.0.0

### Cli

- Finalise flags. begin cycle

### Clippy

- Remove unused imports

### Improvement

- Also filter_paths for each element in additional_paths

### Logging

- Set to trace level for filter_paths

### Major

- Begin refactor cycle [ci skip]

## [4.5.0] - 2024-10-20

### 🚀 Features

- Add glob support
- Add glob support to all

### 🐛 Bug Fixes

- Actually implement the fix for ef1e6f857e48821198d720d092bc7087af762f2a

### 📚 Documentation

- Update README and include instructions regarding renaming

### Minor

- Update tests and update paths code

### Release

- V4.5.0

## [4.2.0] - 2024-10-20

### 🐛 Bug Fixes

- Filename should leave out version part alone

### Release

- V4.2.0

## [4.1.0] - 2024-10-20

### Cli

- Allow to explicitly tell "true" or "false" using ArgAction::Set and add our service file

### Release

- V4.1.0

## [4.0.0] - 2024-10-20

### 🐛 Bug Fixes

- Apply clippy lints
- Additional paths variable should only be a collection of files and not directories

### 🚜 Refactor

- Canonicalize filter
- Canonicalize paths
- Move mostly to libroast
- Improve field naming and description
- Remove tracing crate unused imports

### 📚 Documentation

- Add a warning regarding reproducibility
- Fix warning msg
- Fix warning msg

### 🧪 Testing

- Use copy_dir_all as part of lib now instead

### Cli

- Move logic as cli stubs
- Add recomprizz args

### Lib

- Move over copy_dir_all as a common utility

### Minor

- Raaaaaaaaaaaaawwwwwwww

### Recomprizz

- Initial implementation

### Release

- V4.0.0

### Remove

- Cliff.toml and git-cliff is an overengineered changelog generator

### Reproducibility

- Set to false by default

### Tracing

- Set logic where and when to start properly

## [3.3.1] - 2024-10-19

### 🐛 Bug Fixes

- Ci yaml config fix. best format
- Ci yaml config fix. best format x2

### 🧪 Testing

- Add library tests + ci tests
- This should be two separate files

### ⚙️ Miscellaneous Tasks

- Rename workflow
- Install a c compiler. clang preferred
- Release v3.3.1

## [3.3.0] - 2024-10-15

### 🚀 Features

- Support uncompressed tarballs with tar extension

### 🚜 Refactor

- Properly set preserve root
- Cleanup raw binary log output

### 📚 Documentation

- Update README

### Publish

- Add repository key value
- Add repository key value
- Add required keys and prepare to publish

## [3.2.2] - 2024-10-12

### 🐛 Bug Fixes

- Properly delete temporary directories

### ⚙️ Miscellaneous Tasks

- *(release)* V3.2.2

## [3.2.1] - 2024-10-12

### ⚙️ Miscellaneous Tasks

- *(release)* V3.2.1

### Cli

- Improve description

## [3.2.0] - 2024-10-12

### Cargo

- Update lockfile

## [3.1.1] - 2024-10-12

### Improvement

- Add Display trait for Compression and Error trait for UnsupportedFormat

## [3.1.0] - 2024-10-12

### ⚙️ Miscellaneous Tasks

- *(release)* Bump version to 3.1.0

## [3.0.0] - 2024-10-12

### ⚙️ Miscellaneous Tasks

- *(release)* Bump version to 3.0.0

## [2.0.0] - 2024-10-12

### 🚀 Features

- Add is_supported_format function
- Add ability to extract supported file formats

### Clippy

- Use inspect_err when map_err returns the original item

### Consts

- Remove unnecessary consts

### Format

- Use new format with just format command
- Use new format with just format command

### Minor

- *(refactor)* Use inspect_err instead of map_err
- Add Display trait to namespace and slightly change the error message
- Apply trait Debug for UnsupportedFormat
- Refactor and put only one return keyword for if-else block

## [1.1.0] - 2024-09-07

### 🚀 Features

- Add preserve-root and properly handle extra files using tempfile crate

<!-- generated by git-cliff -->
