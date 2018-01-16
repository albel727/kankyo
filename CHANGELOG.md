# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog] and this project adheres to
[Semantic Versioning].

## [0.2.0] - 2018-01-16

### Added

- Add `snapshot` function [c:a7c58a9]

### Changed

- Remove `Error` enum [c:f9c2511]

### Misc.

- Fix `Cargo.toml` `repository` URL [c:5228098]
- Add `Cargo.toml` `homepage` URL [c:0d21f95]

## [0.1.1] - 2017-12-06

### Added

- Add `utils::unload_from_parsed_lines` [c:00f4a67], [c:d5a98cd]

### Fixed

- Fix `utils::parse_line` allowing whitespace [c:f6c50fb]
- Fix comment parsing [c:4186a71]

### Misc.

- Fix duplicate wording in Error docs [c:099a152]
- Remove unnecessary reference [c:ff878d4]
- Fix tests in Travis script [c:a566851]
- Fix test compilation on rustc 1.13.0 [c:5777ae4]
- Add some benchmarks [c:5afaebe], [c:c40d0cc]
- Slightly improve `utils::set_variables` performance [c:3466a13]
- Improve performance of `unload_from_reader` [c:ae8d2c5]
- Fix some documentation links [c:108556f]
- Improve the documentation [c:2eb35ed]
- Add more tests [c:6b1c4d2]

## 0.1.0 - 2017-10-25

### Added

- Initial release

[c:099a152]: https://github.com/zeyla/kankyo/commit/099a152485c4b511ff50096ece5c3fb8fb1bf57f
[c:0d21f95]: https://github.com/zeyla/kankyo/commit/0d21f95c5f10d5c8f680e890ab96161cd1189531
[c:00f4a67]: https://github.com/zeyla/kankyo/commit/00f4a673715c7076464dafb430215ed5da999cba
[c:108556f]: https://github.com/zeyla/kankyo/commit/108556f531f6c22ff44f59b4380db3f02281607e
[c:2eb35ed]: https://github.com/zeyla/kankyo/commit/2eb35edd28ba1e95d3e49c0799e0748b9b45af8e
[c:3466a13]: https://github.com/zeyla/kankyo/commit/3466a13f87e24f16f48dab39c07e4b225fc0f8e0
[c:4186a71]: https://github.com/zeyla/kankyo/commit/4186a7159dd0375db8a6ebc360b0b798fdc063f1
[c:5228098]: https://github.com/zeyla/kankyo/commit/5228098b7adabceb44b7970350f3f002697d8177
[c:5777ae4]: https://github.com/zeyla/kankyo/commit/5777ae419e15b6d6df247dc860a932f5a3e00560
[c:5afaebe]: https://github.com/zeyla/kankyo/commit/5afaebe9c02aa09211c145dd24237c2db3905806
[c:6b1c4d2]: https://github.com/zeyla/kankyo/commit/6b1c4d29d476c3e0fccfbdb19c101f3d5ec5ad90
[c:a566851]: https://github.com/zeyla/kankyo/commit/a5668516ddacae9eeedcb4c81a9b187fdff955f6
[c:a7c58a9]: https://github.com/zeyla/kankyo/commit/a7c58a9d959ec0c3aaf87b43b68cb45773257a72
[c:ae8d2c5]: https://github.com/zeyla/kankyo/commit/ae8d2c59b5920920e3b2562c3dfcd209590fc20e
[c:c40d0cc]: https://github.com/zeyla/kankyo/commit/c40d0ccd2636107e69b835a70ecdd9ea717c0503
[c:d5a98cd]: https://github.com/zeyla/kankyo/commit/d5a98cda9442e92ae6e757d40db0bc96ef54ba0b
[c:f6c50fb]: https://github.com/zeyla/kankyo/commit/f6c50fb3afa0cc3037d55bb9321d65546858df81
[c:f9c2511]: https://github.com/zeyla/kankyo/commit/f9c251113f32f1061c90fdb9cc79dc3c9cee7e34
[c:ff878d4]: https://github.com/zeyla/kankyo/commit/ff878d4cf9ceb1e4713f1b3630f410277f1d5792

[0.2.0]: https://github.com/zeyla/kankyo/compare/v0.1.1...v0.2.0
[0.1.1]: https://github.com/zeyla/kankyo/compare/v0.1.0...v0.1.1

[Keep a Changelog]: http://keepachangelog.com/en/1.0.0/
[Semantic Versioning]: http://semver.org/spec/v2.0.0.html
