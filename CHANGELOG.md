# Changelog

All notable changes to this project will be documented in this file.

## [10.1.0] - 2025-10-04

### Bug Fixes

- Replace unwrap with ? for better error handling in test functions [64195cd](https://codeberg.org/Rusty-Geckos/roast/commit/64195cd5a3a393b7764496ddbe1713cf8e8bb2fe)
- Simplify check for empty processed paths in get_all_files function [ebe27a0](https://codeberg.org/Rusty-Geckos/roast/commit/ebe27a03ec651280e6df7364a73d307473794b10)
- Ensure workdir is added to updated_paths if no processed paths are found [4780ab7](https://codeberg.org/Rusty-Geckos/roast/commit/4780ab7fcf9911a8033193560cb1b318fdfebdc8)

### Miscellaneous Tasks

- Add .vscode/ to .gitignore [19656ec](https://codeberg.org/Rusty-Geckos/roast/commit/19656ec73bc517ddd594d9d8278d1eaa2f80a603)

### Other

- Run cargo +nightly fmt [c3b224d](https://codeberg.org/Rusty-Geckos/roast/commit/c3b224dc5d15c35c20a73b13bfc04df2e6e1035b)

### Styling

- Run cargo format [6704064](https://codeberg.org/Rusty-Geckos/roast/commit/67040641221d3947c105ca4ac33be06a9c9cc003)
- Apply clippy single component path import fix [68e6426](https://codeberg.org/Rusty-Geckos/roast/commit/68e6426a8176e73060d9118e777549c2fea53ee4)
- Format imports for better readability in test_utils.rs [2ab3acd](https://codeberg.org/Rusty-Geckos/roast/commit/2ab3acd15bba9e598fa711ce32793460ec3699ca)

### Testing

- Add unit tests for directory copying functionality [cb6aef8](https://codeberg.org/Rusty-Geckos/roast/commit/cb6aef82d231e6e96613dc895ee123fda32033ec)

## [10.0.2] - 2025-09-24

### Bug Fixes

- Canonicalize path before stripping prefix [9fbbe45](https://codeberg.org/Rusty-Geckos/roast/commit/9fbbe45e5f57943ab3727b35e0081466d30a8e6f)

### Continuous Integrations

- Rename build.yml to ci.yml [059378f](https://codeberg.org/Rusty-Geckos/roast/commit/059378f6aea097338eae536e41a112d38446985c)

### Miscellaneous Tasks

- Bump to version 10.0.2 [8b863cc](https://codeberg.org/Rusty-Geckos/roast/commit/8b863ccbec6dd335ad3554e46403ef252cba0a1a)

### Other

- Apply clippy fixes [22d2691](https://codeberg.org/Rusty-Geckos/roast/commit/22d269136590c6ffc79a4553294e350473c0c4c3)

## [10.0.1] - 2025-06-25

### Bug Fixes

- `start_trace` should be passed [a267574](https://codeberg.org/Rusty-Geckos/roast/commit/a2675743abb34f0c4c4d3fcbb64686cf0de40ee2)

### Miscellaneous Tasks

- V10.0.1 [ff1d6d0](https://codeberg.org/Rusty-Geckos/roast/commit/ff1d6d0044899f8e46090d0900ae9e59c56b24d2)

## [10.0.0] - 2025-06-25

### Bug Fixes

- The renaming logic have caused the `outdir` to be useless. [23673dc](https://codeberg.org/Rusty-Geckos/roast/commit/23673dc940a83de23d95dc065db88e5ef09892b3)

### Documentation

- Update README.md [7337011](https://codeberg.org/Rusty-Geckos/roast/commit/7337011b7600263255550f95184bfee4eb895348)
- Add a section about reproducibility [91355df](https://codeberg.org/Rusty-Geckos/roast/commit/91355dfed43cbcfc6712cb162a4622481c29306d)

### Features

- If compiled with feature `obs`, make `--silent` flag do nothing. [704993f](https://codeberg.org/Rusty-Geckos/roast/commit/704993fcb8ba22d7012b4fe9108d01336094a989)
- Add silent flag [fbe8d61](https://codeberg.org/Rusty-Geckos/roast/commit/fbe8d61f205e24589a46469dd9262cb1e8c45712)

### Miscellaneous Tasks

- V10.0.0 [ba46b2d](https://codeberg.org/Rusty-Geckos/roast/commit/ba46b2dc0703e1ba17b2b723bb71b3bc3b042eb5)
- Allow `clippy::if_same_then_else`. [11940e1](https://codeberg.org/Rusty-Geckos/roast/commit/11940e1012ebe77055da00363479bc8dd3b6be06)

### Refactor

- Path_buf_filename -> filename [278836f](https://codeberg.org/Rusty-Geckos/roast/commit/278836fdfceeb9c6cc6be1242722828681121de1)

### Testing

- Add missing flag `--silent` [efad22a](https://codeberg.org/Rusty-Geckos/roast/commit/efad22a0d6f36068655202633a8463ff14331fbb)

## [9.0.0] - 2025-06-22

### Bug Fixes

- Add missing fields [b551337](https://codeberg.org/Rusty-Geckos/roast/commit/b5513373cc95e5dc6bc5a9f695cd3f9734988531)

### Dependencies

- Specify clap_complete version [474ede3](https://codeberg.org/Rusty-Geckos/roast/commit/474ede3ed6c187b4c4538434faacd5f691add2bd)
- Remove `walkdir` [770c048](https://codeberg.org/Rusty-Geckos/roast/commit/770c048462a37908e8ad1b760a0e106933aa896c)
- Add clap_complete [e15cdfc](https://codeberg.org/Rusty-Geckos/roast/commit/e15cdfc02562d0ab442063bff88c0602d8bcf3fd)

### Features

- Create a new binary name for `roast_scm`. [65e9dcf](https://codeberg.org/Rusty-Geckos/roast/commit/65e9dcf00d246af4ff7d376eaa5df9cfd6e106d6)
- Add new subcommand `generate-completions-for` using `clap_complete` [f6ab16b](https://codeberg.org/Rusty-Geckos/roast/commit/f6ab16b675fe9d0dc141a71678531d27d9fcfd42)

### Miscellaneous Tasks

- V9.0.0 [c3d248b](https://codeberg.org/Rusty-Geckos/roast/commit/c3d248b3f90c9ef556bdb9bef4722ec021082725)

### Other

- Update workspace default members [047dfe3](https://codeberg.org/Rusty-Geckos/roast/commit/047dfe353a2978cb3904d4ac2272264927d6922d)

### Refactor

- `copy_dir_all` should canonicalise the paths to original sources first. [9193ab7](https://codeberg.org/Rusty-Geckos/roast/commit/9193ab7504ab5e945e9eb5c2afca5401916fb396)

## [8.1.5] - 2025-06-19

### Bug Fixes

- Just add one newline lol [d237ec6](https://codeberg.org/Rusty-Geckos/roast/commit/d237ec6be30eb4c6e373a73987059461e6b776b8)

### Miscellaneous Tasks

- Bump to v8.1.5 [c2677a5](https://codeberg.org/Rusty-Geckos/roast/commit/c2677a51cc5e7eda213c3b1523712badd7c3a546)

## [8.1.4] - 2025-06-19

### Bug Fixes

- Finally properly fix adding the newline. [5815417](https://codeberg.org/Rusty-Geckos/roast/commit/58154175cfe07c9bd10673db1a45788f88bf3d32)

### Miscellaneous Tasks

- V8.1.4 [8bc664a](https://codeberg.org/Rusty-Geckos/roast/commit/8bc664a58f9f8d0645cb7816bd2dcb0f3900afbc)

## [8.1.3] - 2025-06-19

### Bug Fixes

- Add newline at the end of the file properly [f131e45](https://codeberg.org/Rusty-Geckos/roast/commit/f131e455890bf2162898685cdc53655cf905e140)

### Miscellaneous Tasks

- V8.1.3 [e540816](https://codeberg.org/Rusty-Geckos/roast/commit/e5408167c6cb2214d9fc3b826d618502b25f44fd)

## [8.1.2] - 2025-06-18

### Bug Fixes

- Update submodule logic if in case the submodule path does not exist. [987695b](https://codeberg.org/Rusty-Geckos/roast/commit/987695be719e62d857503a2b649722c72a04bf53)

### Miscellaneous Tasks

- Bump to v8.1.2 [3655a2a](https://codeberg.org/Rusty-Geckos/roast/commit/3655a2a9c03014b446db028a5c19bf223b0a5b3d)

### Other

- Run `cargo +nightly fmt` [aafe93e](https://codeberg.org/Rusty-Geckos/roast/commit/aafe93e697fb829b5f6add2beda5bd5d6065c4b8)

### Testing

- Switch test case from deno to river [eb5f1ff](https://codeberg.org/Rusty-Geckos/roast/commit/eb5f1ff4f08c666b145015441d6fa0d20734baff)
- Add deno and jay, they're known to contain submodules [d923e42](https://codeberg.org/Rusty-Geckos/roast/commit/d923e428f2b62697df0b2d267ded8a05c12b5286)

## [8.1.1] - 2025-06-15

### Documentation

- Renaming scheme is now "better" [abe7353](https://codeberg.org/Rusty-Geckos/roast/commit/abe735342639007f70634564e4978f654f9c4350)
- The `.changes` filename is based on the filename without the version part. [91a6658](https://codeberg.org/Rusty-Geckos/roast/commit/91a66583a4e4a1f74b15f5e339aa4427c93f4ef1)
- Clarify what the header is for [4db561a](https://codeberg.org/Rusty-Geckos/roast/commit/4db561a5bd54e61527a90411a908e9aa84c54bb3)
- Fix grammar here [03d2d42](https://codeberg.org/Rusty-Geckos/roast/commit/03d2d426faf87f49c0b214597add3ea5224f5ae9)
- Update README [d31b41a](https://codeberg.org/Rusty-Geckos/roast/commit/d31b41a541a5c323d5faf67dde82c5b79d4d68de)
- Fix README.md. `obs` feature flag only updates the version in the specfile. [cd865b8](https://codeberg.org/Rusty-Geckos/roast/commit/cd865b858a52cc3575a449613c2864cfaef469b5)

### Miscellaneous Tasks

- Bump to v8.1.1 [df1f31c](https://codeberg.org/Rusty-Geckos/roast/commit/df1f31cd04308302555f97f67650d1f20322df09)

## [8.1.0] - 2025-06-15

### Bug Fixes

- More clap fix workarounds. `changesauthor` is required if `changesgenerate` is set to true. [ef52077](https://codeberg.org/Rusty-Geckos/roast/commit/ef5207761a92349942303e04cc7dd8964abb0733)
- "prepend" -> "it prepends" [43e746d](https://codeberg.org/Rusty-Geckos/roast/commit/43e746dcbde4981f7d3d25727ae13b02fc0033d7)
- Word should be prepend and not append. [4545613](https://codeberg.org/Rusty-Geckos/roast/commit/4545613dcbc2585f7a17f96364dfc3b681a2fc4c)
- `requires_if` to `required_if_eq` [314d477](https://codeberg.org/Rusty-Geckos/roast/commit/314d4775aed252b92057fca3ed64a41ac4e092d2)

### Continuous Integrations

- Add a note for why tests are separated. [71b8c53](https://codeberg.org/Rusty-Geckos/roast/commit/71b8c533a0b6379a3608e74c1866de720c576657)

### Documentation

- Update README.md to reflect new behaviour [26fd6a7](https://codeberg.org/Rusty-Geckos/roast/commit/26fd6a750d9f406bd93df4190835804380b0b81e)
- Fix typos and inconsistencies in README.md [9642839](https://codeberg.org/Rusty-Geckos/roast/commit/96428394f127b7c2077337d2e2cb05f151e8a6c4)
- Update README [4d81469](https://codeberg.org/Rusty-Geckos/roast/commit/4d81469104009537c63d6e0243858f920eb06555)
- Update README.md [27c27c6](https://codeberg.org/Rusty-Geckos/roast/commit/27c27c661464515acab768c389c55879256ae870)
- Update README.md [8d08ddb](https://codeberg.org/Rusty-Geckos/roast/commit/8d08ddbbe87e86101f6cfe9061435e853f3299c2)

### Improvements

- Feature obs will just call `set_version_in_specfile`. [2cee767](https://codeberg.org/Rusty-Geckos/roast/commit/2cee7671bc5fd132c4ed31fd1df5ce351d42c54e)

### Miscellaneous Tasks

- Bump to v8.1.0 [8a7e57f](https://codeberg.org/Rusty-Geckos/roast/commit/8a7e57fab8e2350374ff0f3c19b5b7630fa91628)

### Other

- Run `cargo +nightly fmt` [75fe51a](https://codeberg.org/Rusty-Geckos/roast/commit/75fe51acfdd3f7d35d6ebdd12a14a52a2244308f)

## [8.0.0] - 2025-06-13

### Continuous Integrations

- Switch to leap image [2f0db72](https://codeberg.org/Rusty-Geckos/roast/commit/2f0db72888fa153c0d1f94c85ea5fcccb2ca1554)

### Documentation

- Update recomprizz.service file to match new features [7fb14d6](https://codeberg.org/Rusty-Geckos/roast/commit/7fb14d63abec47be50202e7bc9bbdbf490d267cc)

### Improvements

- Remove any usage of walkdir [6c3e4fe](https://codeberg.org/Rusty-Geckos/roast/commit/6c3e4fee0f78fb7c0f68971f61cc1498ad820913)

### Miscellaneous Tasks

- Bump to v8.0.0 [2ac2c36](https://codeberg.org/Rusty-Geckos/roast/commit/2ac2c36d7b63b8dfcd00343ea49e9b03f172929b)
- Make the warning clear if renaming fails because of wrong file extension [551fdba](https://codeberg.org/Rusty-Geckos/roast/commit/551fdbaba6bcc7835bdefce6189a69fb89686452)

### Refactor

- `renamepattern` should require `rename` [5b077d1](https://codeberg.org/Rusty-Geckos/roast/commit/5b077d1c23e4bd69d42568c182f7ba7df7a8df55)
- Rework recomprizz renaming logic [85a8be0](https://codeberg.org/Rusty-Geckos/roast/commit/85a8be0964a41e899052380a9b2831a4a5f9adb0)
- Rework recomprizz args fields to introduce renaming with regex. [8f85c4c](https://codeberg.org/Rusty-Geckos/roast/commit/8f85c4c107f9263e7964cba0d9c2121f1f8d0563)

### Removed

- Removal of walkdir crate since we are using standard library's fs module only. [9df3842](https://codeberg.org/Rusty-Geckos/roast/commit/9df38426a0b2816b8dc7b88c5f4cddf118233679)

## [7.2.4] - 2025-06-08

### Dependencies

- Remove semver crate [9d10a3b](https://codeberg.org/Rusty-Geckos/roast/commit/9d10a3bc86d109f630d5efe1a55265e2e01a8ca6)

### Miscellaneous Tasks

- Bump to v7.2.4 [a7ab846](https://codeberg.org/Rusty-Geckos/roast/commit/a7ab8468466c6ebf15126bfc39e813c014a2aa17)
- Add newline to the end of the file if file was not empty before. [1b6fbf0](https://codeberg.org/Rusty-Geckos/roast/commit/1b6fbf01202fb8175fe456f9e6cce0d65c5db599)
- Add newline at the end of the dot changes file [9509d75](https://codeberg.org/Rusty-Geckos/roast/commit/9509d75e75dd791873a8ec99e872d022f55779f3)
- Change short flag to U [ad486a9](https://codeberg.org/Rusty-Geckos/roast/commit/ad486a98245fb2f879991e7e424260950c7ae614)
- Change short flag to U [d9b4a92](https://codeberg.org/Rusty-Geckos/roast/commit/d9b4a92b2ea32fdc99c93852b63dd0cc2c88b332)

### Refactor

- Improve getting "number of changes since" logic [c75d73f](https://codeberg.org/Rusty-Geckos/roast/commit/c75d73f8953641bc46255a40348edf4c4c0852c3)

## [7.2.3] - 2025-06-03

### Bug Fixes

- Refactor so that after a deletion of tag does not cause an error [ec170b0](https://codeberg.org/Rusty-Geckos/roast/commit/ec170b07a56dcdffacc40cc245f797835430d594)

### Miscellaneous Tasks

- Bump to version 7.2.3 [fe44607](https://codeberg.org/Rusty-Geckos/roast/commit/fe44607bf8e91268601de2183f161ee2f55cf2a2)

## [7.2.2] - 2025-06-03

### Bug Fixes

- If there are no tags found, count the commits until the initial commit [4e66d51](https://codeberg.org/Rusty-Geckos/roast/commit/4e66d5103142a8e04c9f6fa0b47f007a763e9fb4)
- Changelog generation was broken. use revwalk to fix the issue [a70f99c](https://codeberg.org/Rusty-Geckos/roast/commit/a70f99cd3baabcecf607fe57ac7ff49f35ec2591)

### Miscellaneous Tasks

- Bump to version 7.2.2 [a8c8b39](https://codeberg.org/Rusty-Geckos/roast/commit/a8c8b3925bce645386c14f4034299cf4aa295c85)

## [7.2.0] - 2025-06-02

### Improvements

- Add a possibility to pass a custom workdir [f8163a9](https://codeberg.org/Rusty-Geckos/roast/commit/f8163a9b723485c946cb97cf1192181c13c1f2be)

### Miscellaneous Tasks

- Bump to version 7.2.0 [b5c1c63](https://codeberg.org/Rusty-Geckos/roast/commit/b5c1c6399aa75f370007713dd88afe68d0114de3)

### Testing

- Roast_scm_opts should have `None` as the first parameter [ee0c7f8](https://codeberg.org/Rusty-Geckos/roast/commit/ee0c7f80648d04417a471077b577e185e8425d9a)

## [7.1.2] - 2025-06-01

### Continuous Integrations

- Forgot to enable obs in the feature flag [3b4ce92](https://codeberg.org/Rusty-Geckos/roast/commit/3b4ce922313b45fbbc3b264114ffe5d37fd05018)

### Improvements

- Only update the changelog header if possible [47a50b6](https://codeberg.org/Rusty-Geckos/roast/commit/47a50b6ea5d7914596deaa03f48b39ed9c9c1112)

### Miscellaneous Tasks

- Bump to version 7.1.2 [1671e70](https://codeberg.org/Rusty-Geckos/roast/commit/1671e70bdace568021be6b71be035bbbfddfa00d)

## [7.1.1] - 2025-06-01

### Continuous Integrations

- Forgot to specify which test to run [108f6f6](https://codeberg.org/Rusty-Geckos/roast/commit/108f6f6a6a17920b23b62eac75a279cf5d799192)
- Include a test for feature `obs` [38702fc](https://codeberg.org/Rusty-Geckos/roast/commit/38702fc355f70d839c7c11bbda840b85d8cb0a30)

### Miscellaneous Tasks

- Bump to v7.1.1 [48c61a4](https://codeberg.org/Rusty-Geckos/roast/commit/48c61a49b8f905ff5bd19a2ebe45c391ba38178b)
- Update only once the version string. [c524422](https://codeberg.org/Rusty-Geckos/roast/commit/c5244228e99a135f766789c136fb7649409e67a8)

### Testing

- Test feature `obs` [6630e70](https://codeberg.org/Rusty-Geckos/roast/commit/6630e705d7ae8f48cd63ebc449f5a94828182c20)

## [7.1.0] - 2025-05-31

### Bug Fixes

- Date format now fixed [7653416](https://codeberg.org/Rusty-Geckos/roast/commit/7653416b46d21267273d2e3cc308423aef24aff1)

### Miscellaneous Tasks

- Bump to v7.1.0 [c6a727c](https://codeberg.org/Rusty-Geckos/roast/commit/c6a727c4bbc5aeebb0fc28bcc3599754e78a8151)

## [7.0.0] - 2025-05-31

### Bug Fixes

- Apply clippy suggestions [fec89b9](https://codeberg.org/Rusty-Geckos/roast/commit/fec89b94d7dc3361f0447a202c357017e9c1d740)
- Disable `obs` feature for now in tests [5612c60](https://codeberg.org/Rusty-Geckos/roast/commit/5612c60f1d7dd3f7e949e20bceba1be89cb6d810)
- Change alias for changesemail [71f0344](https://codeberg.org/Rusty-Geckos/roast/commit/71f034480d6db797d471689977ee9cddc220fbed)
- Add the last newline [5f52081](https://codeberg.org/Rusty-Geckos/roast/commit/5f52081ee54f2d03dd98060259fd85676bdbd4d3)
- `remote_checkout_branch` creating a branch from remote branch with incorrect name [a1117e6](https://codeberg.org/Rusty-Geckos/roast/commit/a1117e674b6b7cf4e30d1e5313bb70e94ff15c41)
- The refactor caused the commit hash to be skipped [ae78d56](https://codeberg.org/Rusty-Geckos/roast/commit/ae78d567143b8fde9718f24fd964e649b6bf50c6)
- `versionrewriteregex` should require `versionrewritepattern` [424450f](https://codeberg.org/Rusty-Geckos/roast/commit/424450ffff42e7c7c2e75a6a74e018446699676d)
- It should be prefixed with `g` [30ef828](https://codeberg.org/Rusty-Geckos/roast/commit/30ef82847ce326a9ed9ade24b78196c0467b7eb2)
- Version format improvements and fixes [cce9877](https://codeberg.org/Rusty-Geckos/roast/commit/cce987743305ef4b790603a6c563303e894c6781)
- Checkout should not error if branch is HEAD. sorting in revwalk should not be reversed. [fc74963](https://codeberg.org/Rusty-Geckos/roast/commit/fc74963bfec1fcdd9e1937e88b57bd4591c5fc93)
- Use set_head_detached since we want it to point to a commit [cd56eb1](https://codeberg.org/Rusty-Geckos/roast/commit/cd56eb1481d397ea5f6704bfcb7ef135c7c22246)
- Add condition if branch ref is the current HEAD [94db917](https://codeberg.org/Rusty-Geckos/roast/commit/94db91779686f13466e107e36ea7d82e87e18805)
- Set this to 0 [e2face4](https://codeberg.org/Rusty-Geckos/roast/commit/e2face471d5d59a4b0795356b02abcb4b6fce6cc)

### Continuous Integrations

- Seems it needs to be more specific where with this blob for *.rs [cc88697](https://codeberg.org/Rusty-Geckos/roast/commit/cc886971a6194b933d5c50659bfcbbb9f5b71d1c)
- Test if CI runs with this config [bec4cda](https://codeberg.org/Rusty-Geckos/roast/commit/bec4cdac882bb2548ce7138c9ec7c262c8018a56)

### Dependencies

- Add hifitime [1961921](https://codeberg.org/Rusty-Geckos/roast/commit/19619217343a7df56784e2a86237b8a3bd2f78d4)
- Add regex crate [4a09a12](https://codeberg.org/Rusty-Geckos/roast/commit/4a09a124834a11b36a1b7f42449aaf82b5f2dce1)
- Add semver [0391031](https://codeberg.org/Rusty-Geckos/roast/commit/039103158e3d84b6b631161b4ca664556d0b1ce7)

### Documentation

- Update README [8e8bfac](https://codeberg.org/Rusty-Geckos/roast/commit/8e8bfac1a89be6436ffd83ff91296dba74f7053c)
- Update roast_scm.service file [d6fe7f7](https://codeberg.org/Rusty-Geckos/roast/commit/d6fe7f7eb6ab48d85a624dd7f9be310444c7903c)
- Document the `obs` feature in Cargo.toml [244fce7](https://codeberg.org/Rusty-Geckos/roast/commit/244fce73fa1b8cf3d98f902301fe2e463e109ec5)
- Update wording [4801bd5](https://codeberg.org/Rusty-Geckos/roast/commit/4801bd566e39939d7011ed13d178abb35731e814)
- Add where the final filename is based on. [d0603f4](https://codeberg.org/Rusty-Geckos/roast/commit/d0603f4dce6322724c22bfbb34e233631cfbcaa9)
- Update README [5afb085](https://codeberg.org/Rusty-Geckos/roast/commit/5afb085748117589b691d71d6e2e985845a81cd4)

### Features

- Append email if `changesemail` passed. [9c32533](https://codeberg.org/Rusty-Geckos/roast/commit/9c32533696cd514c6356ad6583b34590f28a41d1)
- Add `changesemail` field for cli [b270d12](https://codeberg.org/Rusty-Geckos/roast/commit/b270d126fe0abfbf3a3d5abff7ca5a89b8d14c8c)
- Implement update version in specfile [c6ac215](https://codeberg.org/Rusty-Geckos/roast/commit/c6ac21592f6ddd088a53ae958560453be29a3ba3)
- Cli has new fields, `set_name` and `set_version` for obs feature [26cdf5b](https://codeberg.org/Rusty-Geckos/roast/commit/26cdf5b5ab8fc50bb23807ce67857827baa068e6)
- Changelog generation is now implemented. [c38c412](https://codeberg.org/Rusty-Geckos/roast/commit/c38c41255bca306d8f7ffcc329d9aa8f175c5194)
- Add new cli arguments and fieldnames for `RoastScmArgs` related to `changesgenerate` [d607682](https://codeberg.org/Rusty-Geckos/roast/commit/d607682f40ec05268a26dc6c48e3a6a0f2669c80)
- `versionrewriteregex` and `versionrewritepattern` implemented [25609d8](https://codeberg.org/Rusty-Geckos/roast/commit/25609d825ed57409b8521a80d871c0e0e35cf59c)
- Initial versionformat prefix [e962f55](https://codeberg.org/Rusty-Geckos/roast/commit/e962f5552be9d2b50f9af69c90dc2e2b314ade4b)

### Improvements

- Replace dumb lookup by reading config instead [39d4ed5](https://codeberg.org/Rusty-Geckos/roast/commit/39d4ed5a3b1a03be248665b6620624b9d75904e8)
- If there is no changelog, warn the user [4c802e5](https://codeberg.org/Rusty-Geckos/roast/commit/4c802e50903b2fd0ef011ad0930783c71397a9d4)
- Roast_scm now is able to generate some changelog [02f35f3](https://codeberg.org/Rusty-Geckos/roast/commit/02f35f3b481bbe14fabede2283405b877ee873b9)
- Describe string is implemented alongside improved log messages [5fe08a8](https://codeberg.org/Rusty-Geckos/roast/commit/5fe08a8faaea8289bce1864db911cd6992f68e8d)

### Miscellaneous Tasks

- Bump to v7.0.0 [89d185b](https://codeberg.org/Rusty-Geckos/roast/commit/89d185b2163cc1bd27cf56e0f47ec25edc098722)
- Cleanup and whether to disable or enable code for `obs` feature [8949e89](https://codeberg.org/Rusty-Geckos/roast/commit/8949e89ff36ef2760192e6aa0e534f1e4ff800ca)
- Add optional obs feature in roast-cli [3edb410](https://codeberg.org/Rusty-Geckos/roast/commit/3edb410c13aa985c175af3b0a3459b63550f681b)
- Add optional obs feature [5ab9259](https://codeberg.org/Rusty-Geckos/roast/commit/5ab9259045c9256dbb24df51309d9c024645445f)
- Apply `trim()` first for strings before checking if they're empty [0e16546](https://codeberg.org/Rusty-Geckos/roast/commit/0e1654650b5c88aa45960aa494f6fd7a8c3582a3)
- `split('\n')` -> `lines()` [1be8ca2](https://codeberg.org/Rusty-Geckos/roast/commit/1be8ca23230df87f1c499a0f2b5a56f0309522ab)
- Use the `to_extension()` method [3223c05](https://codeberg.org/Rusty-Geckos/roast/commit/3223c05ac4234a0b6712541312c2085073bc3cf0)
- Change this to an iterator [3b1fae3](https://codeberg.org/Rusty-Geckos/roast/commit/3b1fae311bd6660180cc3ca12d6dce2212341971)
- Change wording [6b21e0b](https://codeberg.org/Rusty-Geckos/roast/commit/6b21e0b3a2a022342ba433ba973ffc7eab477f30)
- Cleanup code for feature, changelog generation. [6baf190](https://codeberg.org/Rusty-Geckos/roast/commit/6baf190ea76d60e2b0d19131c5dc465603c39e79)
- Add initial logic for `changesgenerate` feature [9da0a35](https://codeberg.org/Rusty-Geckos/roast/commit/9da0a35914425d38ff449cc49c9a67cae8bfb2c4)
- More cleanup [f4f73f2](https://codeberg.org/Rusty-Geckos/roast/commit/f4f73f2a16eee8720680856797a6e659574d1ec7)
- Add cliff.toml for changelog generation config [4ef65e5](https://codeberg.org/Rusty-Geckos/roast/commit/4ef65e5cd9b890144909e899f0b6c7232c152609)
- Cleanup code [d6a99ae](https://codeberg.org/Rusty-Geckos/roast/commit/d6a99aed964a63ecc26d1644cdd5efaa6f69faa4)
- Make it clear what depth was set in the warnings [a0783bf](https://codeberg.org/Rusty-Geckos/roast/commit/a0783bf7a886c8728f146e7c9d23a95479ea70f4)

### Other

- Run `cargo +nightly fmt` [001690c](https://codeberg.org/Rusty-Geckos/roast/commit/001690c2d8aeccf29b456843da855df1617de3f1)
- Remove needless borrow [019eefa](https://codeberg.org/Rusty-Geckos/roast/commit/019eefaec397a3340b511757355984b0d8aaa3d3)
- Cleanup visibility and privacy of certain functions [bd5c585](https://codeberg.org/Rusty-Geckos/roast/commit/bd5c58564cb4e1d03658b56ded44e5498f444943)
- Run `cargo +nightly fmt` [e75b446](https://codeberg.org/Rusty-Geckos/roast/commit/e75b44675c0bfe25ba45c6627cf74abfe8bbe219)
- Run `cargo +nightly fmt` [40be8df](https://codeberg.org/Rusty-Geckos/roast/commit/40be8dfa81f6322451f9e1507bd88092e2780e4c)
- Generate changelog improvements [5e3922b](https://codeberg.org/Rusty-Geckos/roast/commit/5e3922b7b0d87c0948479f6345c98ef86a3ed1fe)
- Add `to_extension` method for Compression [42c21f8](https://codeberg.org/Rusty-Geckos/roast/commit/42c21f8119076ee0ee847192066f81941328fde4)
- Remove redundant & [1416f00](https://codeberg.org/Rusty-Geckos/roast/commit/1416f008b0a47a4e6fe27313d244d09a812de967)
- Initial refactor of the cloning process and finding the revision [77d8a34](https://codeberg.org/Rusty-Geckos/roast/commit/77d8a34b0540d20c76ecb67ed4ea654e7460886e)
- Run \`cargo +nightly fmt\` [8e6002b](https://codeberg.org/Rusty-Geckos/roast/commit/8e6002bb3f86d0ce80ac1e51af717972605da5ae)

### Performance

- Do a dumb revision lookup to check if a revision is a branch [dfb8b1a](https://codeberg.org/Rusty-Geckos/roast/commit/dfb8b1a109b51dded07b98cb57f9891f0f1f22c7)

### Refactor

- Put set_version logic inside `map` as well. create separate function for changelog file generation. [2ac4023](https://codeberg.org/Rusty-Geckos/roast/commit/2ac402329cc6711f57dae7be874a78480ec60801)
- Move changelog file generator code inside `map` [dc497fb](https://codeberg.org/Rusty-Geckos/roast/commit/dc497fb9d3c3b1b80a9867a2198198cd3decf459)
- Use `any` since it returns at the first true, otherwise, false. [e267807](https://codeberg.org/Rusty-Geckos/roast/commit/e26780730c93f10cb66933f9c59be6bcac4d1105)
- Use iterators over for loops [112edf7](https://codeberg.org/Rusty-Geckos/roast/commit/112edf737865071aa2737e45de801086d6f420cc)
- More cleanup. remove redundant logic [2b4fd3f](https://codeberg.org/Rusty-Geckos/roast/commit/2b4fd3f25c01429712ac97abdc514ca84e108584)

### Testing

- Add tests for roast scm [767f03a](https://codeberg.org/Rusty-Geckos/roast/commit/767f03aa5da191581bc208a2489050435c0abf8a)

## [6.1.1] - 2025-05-19

### Bug Fixes

- Return Ok(()) [a25a239](https://codeberg.org/Rusty-Geckos/roast/commit/a25a2392b2a7b4c23f7016741de3712403e50c1e)
- Return type should be Result<Option<PathBuf>> [d06516d](https://codeberg.org/Rusty-Geckos/roast/commit/d06516d1e400dde7e3d8d51ba6d0195a389e75d6)
- Return type should be Result<Option<PathBuf>> [5bf6fcb](https://codeberg.org/Rusty-Geckos/roast/commit/5bf6fcbea47c4bc049a56bbbe1eb80318a0e807f)
- Ensure that the outdir exists [fc1bfd6](https://codeberg.org/Rusty-Geckos/roast/commit/fc1bfd66bd1cba0a78b3d7dca01fe34c814a6b5f)
- Use the correct return type inside the map. [c092a37](https://codeberg.org/Rusty-Geckos/roast/commit/c092a377c5263538002f8e21e0e1cd4b64f0e5b5)
- Import correctly `std::path::PathBuf`. [87d7091](https://codeberg.org/Rusty-Geckos/roast/commit/87d709117a427f69fcae770b0d295b1fcb1652b2)

### Documentation

- Fix typo in README.md [f765b6f](https://codeberg.org/Rusty-Geckos/roast/commit/f765b6fb74a3356c0886ddbed4806d2ec438f79f)
- This is roast_scm. this file was copied from recomprizz so it was overlooked accidentally [e5c515c](https://codeberg.org/Rusty-Geckos/roast/commit/e5c515ca7a830e308eed733ce35ad97198a19ab1)

### Improvements

- Allow returning an `Option<PathBuf>` if is-temporary is set to `false`. [78c12cd](https://codeberg.org/Rusty-Geckos/roast/commit/78c12cdcfacdc997593168c91b2442b2ed7bde85)

### Miscellaneous Tasks

- V6.1.1 [b8b1be9](https://codeberg.org/Rusty-Geckos/roast/commit/b8b1be9855a2a8836131359bfa9c16b96fab082a)

## [6.1.0] - 2025-05-17

### Bug Fixes

- It should be evaluated if it's false [e7253a7](https://codeberg.org/Rusty-Geckos/roast/commit/e7253a7897a2d699d9b62c1f6e538c3278853235)
- Add let binding [7afed69](https://codeberg.org/Rusty-Geckos/roast/commit/7afed69ceb32620fe95255215c67699201a1ac27)
- Remove unused key in roast-cli's Cargo.toml [f1be55a](https://codeberg.org/Rusty-Geckos/roast/commit/f1be55a326d6e4ee24f9da3acb6d9f1de034c193)

### Continuous Integrations

- Only run ci for certain file changes [5ea534b](https://codeberg.org/Rusty-Geckos/roast/commit/5ea534b16ec0e1a81e7faa889f454296af97465b)
- We are only using one workflow anyway for everything [1053f02](https://codeberg.org/Rusty-Geckos/roast/commit/1053f024049777c71e4952b93306993d41b4aee9)

### Documentation

- Add and improve doc comments [0c1e4d1](https://codeberg.org/Rusty-Geckos/roast/commit/0c1e4d14ed5481b5f35b2d68c2f8266c9f3acd0e)
- Improvements on how everything works [45b3add](https://codeberg.org/Rusty-Geckos/roast/commit/45b3add19fe4abe30ae3658ce95eb1f2042d7a6f)
- Add roast_scm.service [ecc4154](https://codeberg.org/Rusty-Geckos/roast/commit/ecc415466fbe948ea43e8481b276f279584a2a26)

### Features

- Is-temporary flag now used. [293a52c](https://codeberg.org/Rusty-Geckos/roast/commit/293a52c5e1091949c385dec7009bcc56b0db9ba6)

### Improvements

- Feature is-temporary now have clearer messages if directory was not deleted. [5f20d1d](https://codeberg.org/Rusty-Geckos/roast/commit/5f20d1ddce8a463e339c1a775b4b954cc4b462c7)

### Miscellaneous Tasks

- V6.1.0 [7c37334](https://codeberg.org/Rusty-Geckos/roast/commit/7c37334626b3c9f981ea4b1755c84d8da9fbd1cc)
- Version bump dependencies [ba634ca](https://codeberg.org/Rusty-Geckos/roast/commit/ba634ca90d97c09effdb4a9e425d902558f926f1)
- Begin 6.1.0 cycle [43eb90e](https://codeberg.org/Rusty-Geckos/roast/commit/43eb90e7d66a2144edff670e9d856a0df0c1af42)

### Other

- Apply clippy fixes [89dec89](https://codeberg.org/Rusty-Geckos/roast/commit/89dec8917165742fc451783dd5e9f571a86a5253)
- Set the is-temporary with ArgAction::Set [f16c09c](https://codeberg.org/Rusty-Geckos/roast/commit/f16c09cd5b73e695cf730b1ff97d8b230f7b8b59)
- Run `cargo +nightly fmt` [b02b9c4](https://codeberg.org/Rusty-Geckos/roast/commit/b02b9c443402b31210f24d77b14cc4dccf144c8d)
- Apply clippy fixes [d31aa42](https://codeberg.org/Rusty-Geckos/roast/commit/d31aa42338fcfa9ac328486cdc53cb1c8f2582b8)

## [6.0.0] - 2025-05-17

### Continuous Integrations

- Add nightly toolchain. for fmt only. [f8ee10c](https://codeberg.org/Rusty-Geckos/roast/commit/f8ee10ccfd76a1a7051a94fc9d01d8e1152520d5)
- Remove aarch64 target for now [859786d](https://codeberg.org/Rusty-Geckos/roast/commit/859786d3833022d01906508131b3a331b5177185)
- Prepare woodpecker build [bb5c2b0](https://codeberg.org/Rusty-Geckos/roast/commit/bb5c2b0c397173834390df286fa61e8954222eec)
- Auto close prs. contribute to https://codeberg.org/Rusty-Geckos/roast instead. [21caa59](https://codeberg.org/Rusty-Geckos/roast/commit/21caa594829c5a0c11531c8808dab8e6cabb0bd4)

### Dependencies

- Add url crate [11c2fa1](https://codeberg.org/Rusty-Geckos/roast/commit/11c2fa1c292c7cbad65130d2f168c5c911ec954b)

### Documentation

- Add link to roast-cli [CI SKIP] [da48332](https://codeberg.org/Rusty-Geckos/roast/commit/da483324362cb34182378b7727c3ac2f29be939a)
- Add README.md for each crate in the workspace. [1abc9d2](https://codeberg.org/Rusty-Geckos/roast/commit/1abc9d2b4d8123d944e2f764291e2504c487519b)
- Update ci badge links in README [5107dbf](https://codeberg.org/Rusty-Geckos/roast/commit/5107dbff79a216cb008b760f4aa74825a01bcc5b)

### Features

- Finalise feature and provide improvements [2bab074](https://codeberg.org/Rusty-Geckos/roast/commit/2bab074ef4e70eb5588b69a741772734a607947d)

### Improvements

- Incremental improvements on roast scm logic [7dbe1ee](https://codeberg.org/Rusty-Geckos/roast/commit/7dbe1ee5b943c072cd85495387ec3ccf629dd7e8)

### Miscellaneous Tasks

- V6.0.0 [ff9ee7d](https://codeberg.org/Rusty-Geckos/roast/commit/ff9ee7d0d58ef9f59530de27ff5f78cccf59d576)
- Adjust spacing [726c01f](https://codeberg.org/Rusty-Geckos/roast/commit/726c01ff8e826e442d367e04fccd7e98e83c813b)
- Add a bit of spacing [9b58b7d](https://codeberg.org/Rusty-Geckos/roast/commit/9b58b7d521af1f886e1e3e3397f15daaa80d2360)
- Add git2 [1567e40](https://codeberg.org/Rusty-Geckos/roast/commit/1567e403d3981fce25a7c20694985984d02b49b8)
- Initial code for roast scm [9970254](https://codeberg.org/Rusty-Geckos/roast/commit/9970254b0d4f95b5355a51def907e964c2bc5eaf)
- Update URLs [066ad5e](https://codeberg.org/Rusty-Geckos/roast/commit/066ad5e1ec31c9ed8e49414735e7319d2800417d)
- Update structure and boilerplate logic [d3f6b16](https://codeberg.org/Rusty-Geckos/roast/commit/d3f6b1649ce231cd86a1515e144bac61273636f1)
- Add git2-rs as dependency [f356d73](https://codeberg.org/Rusty-Geckos/roast/commit/f356d73df51cb2c80ea8a9e485a8759d75afd3b0)
- Make libroast a workspace dependency [f63df58](https://codeberg.org/Rusty-Geckos/roast/commit/f63df582d30561ee2c0e1fea9e26be0919ff9cb9)

### Other

- RoastScmArgs' outfile field is set to Option<PathBuf> [07079bf](https://codeberg.org/Rusty-Geckos/roast/commit/07079bf3ded48e25b6a9200ef28cb8fd8ccecd2e)
- Allow roast scm to become a binary executable [096fe82](https://codeberg.org/Rusty-Geckos/roast/commit/096fe82cb4eb6d46ce2e6ad92be22b298c9be07a)
- Start including roast scm to roast cli [0ff1097](https://codeberg.org/Rusty-Geckos/roast/commit/0ff1097ee6985972f615e24edbdaa0475265bebe)
- Improve struct definition for RoastScmArgs [d954b2e](https://codeberg.org/Rusty-Geckos/roast/commit/d954b2e9ead263863b8b2fe635992bbc06235954)
- Write initial clone logic [90e4c17](https://codeberg.org/Rusty-Geckos/roast/commit/90e4c17f595dd801039e4456b69a866bff37c795)
- Import to operations module [4a77f9f](https://codeberg.org/Rusty-Geckos/roast/commit/4a77f9fd852b48d7388e55ab105730f972ae2bc8)
- Run `cargo +nightly fmt` [ceca8da](https://codeberg.org/Rusty-Geckos/roast/commit/ceca8da4261f987644c02adb5171894ff3067a33)
- Run cargo +nightly fmt [e28dc26](https://codeberg.org/Rusty-Geckos/roast/commit/e28dc266d301b866baec10b9f90eece795d1864e)
- Add struct for RoastScmArgs [8268a17](https://codeberg.org/Rusty-Geckos/roast/commit/8268a171dad16621ca3dea572d50305542e46a95)
- Update to Rust 2024 edition [c888538](https://codeberg.org/Rusty-Geckos/roast/commit/c88853897f164a23d6ee66bf8bac6495bdde3630)
- Use git cliff to generate changelog [94ed82e](https://codeberg.org/Rusty-Geckos/roast/commit/94ed82eae523b4513c151eae23b89504149b4a0a)
- Run cargo +nightly fmt [dfc556b](https://codeberg.org/Rusty-Geckos/roast/commit/dfc556b5ac078e48bb5d0a86bf0006df734b33d2)

### Refactor

- Remove git2 and any code related to it [edddf73](https://codeberg.org/Rusty-Geckos/roast/commit/edddf73af93869b2c5993cae36fad7929e008c59)

### Removed

- Filename should be with underscores [e883dbc](https://codeberg.org/Rusty-Geckos/roast/commit/e883dbccb6f9b5cfb863794e7601df73cb8d8b1f)
- Filenames should be with underscores. [0d207c8](https://codeberg.org/Rusty-Geckos/roast/commit/0d207c8f055a03a097b661558c3586fd0df0d691)
- Move over this feature and focus on cloning only [77f28fc](https://codeberg.org/Rusty-Geckos/roast/commit/77f28fc66dc9103a7da373460874a48506f2ac0a)

## [5.1.7] - 2024-11-27

### Bug Fixes

- Added trace feature [d6b3dd0](https://codeberg.org/Rusty-Geckos/roast/commit/d6b3dd03dc37fbb5f6da05a1f0c9d84847ead8d8)

### Continuous Integrations

- Add filtering so it only runs when `.rs` and `Cargo.toml` change [7f320ac](https://codeberg.org/Rusty-Geckos/roast/commit/7f320ac721a33e6255129eab8f1c1fe871480a09)

### Dependencies

- Update clap features to use [002602a](https://codeberg.org/Rusty-Geckos/roast/commit/002602a75426815ee70f4056f143f1add7318f04)

### Features

- Add zstd as another alias for zst [66b09d5](https://codeberg.org/Rusty-Geckos/roast/commit/66b09d50dbf8ecd0d1c1dce3f041596b266f72d4)

### Miscellaneous Tasks

- V5.1.7 [b466750](https://codeberg.org/Rusty-Geckos/roast/commit/b46675053d196a8b8ccb66627c29c6eb69499d68)

### Other

- Improve error message here [994e70d](https://codeberg.org/Rusty-Geckos/roast/commit/994e70d5e875eebd6b71f87ad19480cbe7586ea0)

## [5.1.6] - 2024-11-02

### Bug Fixes

- Revert 8977c6741364c6d25fb33408d8b4232d835a768b [cd70cfb](https://codeberg.org/Rusty-Geckos/roast/commit/cd70cfb4a30b632ed2274a3b8a5c4c47006da7a8)

### Miscellaneous Tasks

- Release v5.1.6 [5c8db7d](https://codeberg.org/Rusty-Geckos/roast/commit/5c8db7db68c1deafd370066ed65a3221448b6af7)

## [5.1.5] - 2024-11-02

### Miscellaneous Tasks

- Release v5.1.5 [730d454](https://codeberg.org/Rusty-Geckos/roast/commit/730d454cbc0e9cfe0edbb4fb82ac9cf97c0575f2)

## [5.1.4] - 2024-11-02

### Miscellaneous Tasks

- Release v5.1.4 [2466a6b](https://codeberg.org/Rusty-Geckos/roast/commit/2466a6bd9771fce2e62b211018d076c27bd4d91d)

## [5.1.3] - 2024-11-02

### Bug Fixes

- Just use an empty "" if strip fails [8977c67](https://codeberg.org/Rusty-Geckos/roast/commit/8977c6741364c6d25fb33408d8b4232d835a768b)

### Documentation

- Removed warning. ensured reproducibility. [b4f81ad](https://codeberg.org/Rusty-Geckos/roast/commit/b4f81ad615b63c9ee6b035ff65db8a6638b6cb54)

### Miscellaneous Tasks

- Release v5.1.3 [d311c5b](https://codeberg.org/Rusty-Geckos/roast/commit/d311c5b5a14ada92d0634cb1772982f45519ff74)
- Set resolver to 2 and enforce strict linting rules [4146991](https://codeberg.org/Rusty-Geckos/roast/commit/4146991f0c6824f785a333ad2dd51c684d2b42af)

### Other

- Rectify the needless pass by value [38778cb](https://codeberg.org/Rusty-Geckos/roast/commit/38778cb77f7b13a07f090ba9182d4c6f0dcf67af)

## [5.1.2] - 2024-11-01

### Miscellaneous Tasks

- Release 5.1.2 [249fa27](https://codeberg.org/Rusty-Geckos/roast/commit/249fa279eacd5464fd2d4a51fe4891fc3c69ce08)

### Other

- Improvements on how we sort files and directories [037aa62](https://codeberg.org/Rusty-Geckos/roast/commit/037aa626e656df2c888ad04f5dc00daba41bcdd1)

## [5.1.0] - 2024-11-01

### Documentation

- Add important difference between ADDED and INCLUDED [ci skip] [03aa7e0](https://codeberg.org/Rusty-Geckos/roast/commit/03aa7e073c99d50dd8ed55c4ddc753995b3929d9)
- Improve wording [ci skip] [945ff95](https://codeberg.org/Rusty-Geckos/roast/commit/945ff956056c91a0ac6ccad825e111208aa79941)
- Fix grammar [ci skip] [0fa8911](https://codeberg.org/Rusty-Geckos/roast/commit/0fa89113a0f81ba143c18fb077790f27f76276aa)
- Explain how the path behaviour works [4c7d96a](https://codeberg.org/Rusty-Geckos/roast/commit/4c7d96a6232884b90257a5c64b314440145bc0b1)

### Miscellaneous Tasks

- Release 5.1.0 [fe0d2e4](https://codeberg.org/Rusty-Geckos/roast/commit/fe0d2e4f191842eea27f85e4fd60019b91416e6f)

### Performance

- Add rayon to parallelise copying operations [150437b](https://codeberg.org/Rusty-Geckos/roast/commit/150437be47b8843e66297a1f4df001375aeec2df)

## [5.0.0] - 2024-11-01

### Bug Fixes

- Resolved some edge-cases with additional paths and included paths [1480677](https://codeberg.org/Rusty-Geckos/roast/commit/1480677c7925949d70397d44b6d1b8ad15f7aed3)
- Reimplement adding of archive files [60708fb](https://codeberg.org/Rusty-Geckos/roast/commit/60708fb088e244f88ce901eb4bb4bbb3fe5a599c)
- Avoid duplicating entries [7a154a5](https://codeberg.org/Rusty-Geckos/roast/commit/7a154a5dfa24186eea72a56223a74107edc35b42)
- Do not consider temporary directory as hidden [38acc76](https://codeberg.org/Rusty-Geckos/roast/commit/38acc76cba59b35dfcb066ad3852d877307360bd)
- Just use ends_with to check if it's a valid file extension [3faa703](https://codeberg.org/Rusty-Geckos/roast/commit/3faa7030a1308d60859774159d6ec98304afb420)

### Documentation

- Update README on CLI help [8f7a931](https://codeberg.org/Rusty-Geckos/roast/commit/8f7a9314229d8579bc685bd98b2018f21de1f3ce)

### Features

- Hidden file and gitignore finally correctly implemented [87ae4ab](https://codeberg.org/Rusty-Geckos/roast/commit/87ae4aba83219d414195848061d35da554b5984c)

### Improvements

- Also filter_paths for each element in additional_paths [bcfcaa8](https://codeberg.org/Rusty-Geckos/roast/commit/bcfcaa87f0a868c286e00a22e03e9696d530be50)

### Miscellaneous Tasks

- Release 5.0.0 [55f888a](https://codeberg.org/Rusty-Geckos/roast/commit/55f888ad2130341b29d5b360630a3eca9ab06257)

### Other

- Remove unused imports [9a7ff2d](https://codeberg.org/Rusty-Geckos/roast/commit/9a7ff2d0c8caadc31bddb3cca8c028503da16c71)
- Finalise flags. begin cycle [c32873d](https://codeberg.org/Rusty-Geckos/roast/commit/c32873d41a74f9393e9bf1a5d12dce163c68a23b)
- Begin refactor cycle [ci skip] [a438974](https://codeberg.org/Rusty-Geckos/roast/commit/a43897400d1ed1561dad1c7c03ae4fe02f487e50)
- Set to trace level for filter_paths [450e6a2](https://codeberg.org/Rusty-Geckos/roast/commit/450e6a22f6cf41ae11f18c2265e9ac8a5d76f2a7)

### Refactor

- Improve the logic handling for adding, excluding and including [d5beb40](https://codeberg.org/Rusty-Geckos/roast/commit/d5beb405218f43e1c1da430e952097398c6f29ec)

## [4.5.0] - 2024-10-20

### Bug Fixes

- Actually implement the fix for ef1e6f857e48821198d720d092bc7087af762f2a [1c93654](https://codeberg.org/Rusty-Geckos/roast/commit/1c936546166b693c293aa8cbf1e970e70dac3103)

### Documentation

- Update README and include instructions regarding renaming [af59fb4](https://codeberg.org/Rusty-Geckos/roast/commit/af59fb44c49215b5865a1f9bdb257439e195187a)

### Features

- Add glob support to all [95721c8](https://codeberg.org/Rusty-Geckos/roast/commit/95721c84f7e16db3ae22869e3aa4262d20b49c33)
- Add glob support [168e3e2](https://codeberg.org/Rusty-Geckos/roast/commit/168e3e2747e85573fd950d4e8a39751df3d28edf)

### Other

- V4.5.0 [4696fd5](https://codeberg.org/Rusty-Geckos/roast/commit/4696fd538320dceac0aa324505a1d1caaa724901)
- Update tests and update paths code [b515454](https://codeberg.org/Rusty-Geckos/roast/commit/b51545400aef55cd167c7edff589139706b6fc64)

## [4.2.0] - 2024-10-20

### Bug Fixes

- Filename should leave out version part alone [ef1e6f8](https://codeberg.org/Rusty-Geckos/roast/commit/ef1e6f857e48821198d720d092bc7087af762f2a)

### Other

- V4.2.0 [23bae24](https://codeberg.org/Rusty-Geckos/roast/commit/23bae242c77c647f34834d4eb7866b1ac4d007bd)

## [4.1.0] - 2024-10-20

### Other

- V4.1.0 [68d1209](https://codeberg.org/Rusty-Geckos/roast/commit/68d12093cca9603c5473a98b664e618e3c8c5140)
- Allow to explicitly tell "true" or "false" using ArgAction::Set and add our service file [355c734](https://codeberg.org/Rusty-Geckos/roast/commit/355c73417cfd228e83a3fb0d44168b85ba2bd0e3)

## [4.0.0] - 2024-10-20

### Bug Fixes

- Additional paths variable should only be a collection of files and not directories [a3ae9cb](https://codeberg.org/Rusty-Geckos/roast/commit/a3ae9cb77015017e4b6cb43babad93df271fb62e)
- Apply clippy lints [ec8cfcd](https://codeberg.org/Rusty-Geckos/roast/commit/ec8cfcdf2a29c1fa9d2ba82076d0987ec90a5c2b)

### Documentation

- Fix warning msg [165a4e0](https://codeberg.org/Rusty-Geckos/roast/commit/165a4e05c13ae4aa750dad663143c39268a52383)
- Fix warning msg [8245f89](https://codeberg.org/Rusty-Geckos/roast/commit/8245f89e09fd49f4dacc696e1d68966986fe82d1)
- Add a warning regarding reproducibility [a96208f](https://codeberg.org/Rusty-Geckos/roast/commit/a96208f850bde47d232cfb73553ed2f5608337ce)

### Library Updates

- Move over copy_dir_all as a common utility [31aefaf](https://codeberg.org/Rusty-Geckos/roast/commit/31aefafa17ba6c01f6f25beab1191cd82ec5a6cc)

### Other

- V4.0.0 [3f60b70](https://codeberg.org/Rusty-Geckos/roast/commit/3f60b700900956321ccb71a4b12a20f8c624e741)
- Raaaaaaaaaaaaawwwwwwww [6227781](https://codeberg.org/Rusty-Geckos/roast/commit/622778115ae58c626f403ec2c81749a0065925e5)
- Set to false by default [0f4338b](https://codeberg.org/Rusty-Geckos/roast/commit/0f4338bb6d8a8a7732fbec898d30a28c1609664d)
- Set logic where and when to start properly [78f980a](https://codeberg.org/Rusty-Geckos/roast/commit/78f980ada30bbc167f2ff4ea24c3241e26eba50a)
- Initial implementation [c145a64](https://codeberg.org/Rusty-Geckos/roast/commit/c145a64d082edce7fa5d95f29bee767e9318b37f)
- Add recomprizz args [ce8d443](https://codeberg.org/Rusty-Geckos/roast/commit/ce8d44394c010c3dff2fb7a181ace45f8b44e2ca)
- Move logic as cli stubs [cc825b7](https://codeberg.org/Rusty-Geckos/roast/commit/cc825b769c8b8bddc4eb43918b4ff1531cb7cc70)

### Refactor

- Remove tracing crate unused imports [2b3f918](https://codeberg.org/Rusty-Geckos/roast/commit/2b3f918123d5766b4b9010aeb9b2da5b34b37082)
- Improve field naming and description [806fdce](https://codeberg.org/Rusty-Geckos/roast/commit/806fdce58d99784764d5d389c98068457fc9f2f6)
- Move mostly to libroast [aca2e9c](https://codeberg.org/Rusty-Geckos/roast/commit/aca2e9c98102fcfc118f530bc354f7bd8a88542b)
- Canonicalize paths [527f9af](https://codeberg.org/Rusty-Geckos/roast/commit/527f9af7108f8731347c547749781b5f8e8eaf87)
- Canonicalize filter [6e25a38](https://codeberg.org/Rusty-Geckos/roast/commit/6e25a3844e318b28e461fc40e2aef79d86cc45ea)

### Removed

- Cliff.toml and git-cliff is an overengineered changelog generator [a101a64](https://codeberg.org/Rusty-Geckos/roast/commit/a101a64cc9101842fd2654fc68122ab88a3bdba1)

### Testing

- Use copy_dir_all as part of lib now instead [729114a](https://codeberg.org/Rusty-Geckos/roast/commit/729114a5fc0ab1613032f6b2c8ce4ab2e4943ba0)

## [3.3.1] - 2024-10-19

### Bug Fixes

- Ci yaml config fix. best format x2 [d805ac6](https://codeberg.org/Rusty-Geckos/roast/commit/d805ac626008faf70ddf2c4e57d945558aaa2d1e)
- Ci yaml config fix. best format [a4c66cd](https://codeberg.org/Rusty-Geckos/roast/commit/a4c66cdfb314288ea0938f0cb85f758344bcb5c5)

### Continuous Integrations

- Install a c compiler. clang preferred [f0c9922](https://codeberg.org/Rusty-Geckos/roast/commit/f0c992283298288d68fcabc4a39a008b805ddd0b)
- Rename workflow [97cdc83](https://codeberg.org/Rusty-Geckos/roast/commit/97cdc83433fcee463bd7c7553d7462bd4964aff5)

### Miscellaneous Tasks

- Release v3.3.1 [9b24494](https://codeberg.org/Rusty-Geckos/roast/commit/9b244942050d57127ca3e57d807785c6f91fb4d9)

### Testing

- This should be two separate files [1dd8fe3](https://codeberg.org/Rusty-Geckos/roast/commit/1dd8fe3bc5ee5eb9462698073e1863d0ba2c9b2b)
- Add library tests + ci tests [6b72c36](https://codeberg.org/Rusty-Geckos/roast/commit/6b72c36969f96ea2fdae440a0223780f7142ff68)

## [3.3.0] - 2024-10-15

### Documentation

- Update README [fc07e13](https://codeberg.org/Rusty-Geckos/roast/commit/fc07e13764fea55447a718cf5ea2ba5e0bcb38e2)

### Features

- Support uncompressed tarballs with tar extension [5ae79b2](https://codeberg.org/Rusty-Geckos/roast/commit/5ae79b28c7312119db0c5be6c5408de9ae10600c)

### Other

- Add required keys and prepare to publish [4a0978e](https://codeberg.org/Rusty-Geckos/roast/commit/4a0978e59987179c1a941fb13ef89de735c82935)
- Add repository key value [e2e3404](https://codeberg.org/Rusty-Geckos/roast/commit/e2e34043e3c1d834f06eb8aba91abfdac633627d)
- Add repository key value [ae67fc0](https://codeberg.org/Rusty-Geckos/roast/commit/ae67fc049e2305097a9ec7cdeede281cc32e0e4f)

### Refactor

- Cleanup raw binary log output [199ea64](https://codeberg.org/Rusty-Geckos/roast/commit/199ea64458ea9c5520dd2dfafce91b2ee75c7a3c)
- Properly set preserve root [f739fa2](https://codeberg.org/Rusty-Geckos/roast/commit/f739fa27f4ab706cfbcfe768c63c1f0681979283)

## [3.2.2] - 2024-10-12

### Bug Fixes

- Properly delete temporary directories [3532c94](https://codeberg.org/Rusty-Geckos/roast/commit/3532c94ecfa06df847992da9b11b37a7623a696e)

### Miscellaneous Tasks

- V3.2.2 [9f5905c](https://codeberg.org/Rusty-Geckos/roast/commit/9f5905c1900b885556c951af54a73dcbf206b776)

## [3.2.1] - 2024-10-12

### Miscellaneous Tasks

- V3.2.1 [3dedba2](https://codeberg.org/Rusty-Geckos/roast/commit/3dedba27b8fca0275caa6b5ada2110a69667ae24)

### Other

- Improve description [92bd4f2](https://codeberg.org/Rusty-Geckos/roast/commit/92bd4f2dfdd9d9913d1c001a51fbb14db6155827)

## [3.2.0] - 2024-10-12

### Other

- Update lockfile [66d1036](https://codeberg.org/Rusty-Geckos/roast/commit/66d10363f98cf841f2b6bb7e1da437dfcc9faef7)

## [3.1.1] - 2024-10-12

### Improvements

- Add Display trait for Compression and Error trait for UnsupportedFormat [ff2cbda](https://codeberg.org/Rusty-Geckos/roast/commit/ff2cbda9aea9ff8dbe9845a311c9391ec2458abd)

## [3.1.0] - 2024-10-12

### Miscellaneous Tasks

- Bump version to 3.1.0 [39275ea](https://codeberg.org/Rusty-Geckos/roast/commit/39275ea90a130eb1906c81da2febda4960fc5090)

## [3.0.0] - 2024-10-12

### Miscellaneous Tasks

- Bump version to 3.0.0 [4bd5194](https://codeberg.org/Rusty-Geckos/roast/commit/4bd519434ba9aff0d4a6065d287e0cb1318455e7)

## [2.0.0] - 2024-10-12

### Features

- Add ability to extract supported file formats [ef3fcfa](https://codeberg.org/Rusty-Geckos/roast/commit/ef3fcfa05471ffbbcb0492921ffaac6dd9a22b53)
- Add is_supported_format function [c72c956](https://codeberg.org/Rusty-Geckos/roast/commit/c72c9563f944eb941b5fac6c432b2468ec42fa98)

### Other

- Refactor and put only one return keyword for if-else block [a418441](https://codeberg.org/Rusty-Geckos/roast/commit/a418441241bd925d1ea447dbef3468a673165cea)
- Apply trait Debug for UnsupportedFormat [6117e67](https://codeberg.org/Rusty-Geckos/roast/commit/6117e6734e3a162c817b87a887c95c5f5c56f1a1)
- Remove unnecessary consts [1b16d09](https://codeberg.org/Rusty-Geckos/roast/commit/1b16d0963205aa02460b7c19b940b935e145faae)
- Add Display trait to namespace and slightly change the error message [487877a](https://codeberg.org/Rusty-Geckos/roast/commit/487877a14b3169eb5f8a38c503734b9ac937a259)
- Use new format with just format command [4457a32](https://codeberg.org/Rusty-Geckos/roast/commit/4457a322de21f18daa5b0f62c644a0f4b4203844)
- Use inspect_err instead of map_err [773ec1b](https://codeberg.org/Rusty-Geckos/roast/commit/773ec1b9c5e61110d6d2aceb5888d721e6198888)
- Use inspect_err when map_err returns the original item [ecad202](https://codeberg.org/Rusty-Geckos/roast/commit/ecad2026fbd344a42c0f472b92a789a4e1b1fc43)
- Use new format with just format command [8586cd0](https://codeberg.org/Rusty-Geckos/roast/commit/8586cd0c2628a600ba5e440c5bbc0ece3243fbcf)

## [1.1.0] - 2024-09-07

### Features

- Add preserve-root and properly handle extra files using tempfile crate [1dc9554](https://codeberg.org/Rusty-Geckos/roast/commit/1dc95548bd871bd296aecb91317e0b9c00690a5a)

## [1.0.0] - 2024-09-07


