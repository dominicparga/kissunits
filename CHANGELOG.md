# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog][keepachangelog], and this project adheres to [Semantic Versioning][semver].


## Table of contents

1. [Unreleased](#unreleased)
1. [v1.1.0](#v1.1.0)
    1. [v1.0.3](#v1.0.3)
        1. [v1.0.2](#v1.0.2)
        1. [v1.0.1](#v1.0.1)
        1. [v1.0.0](#v1.0.0)
1. [v0.0.0](#v0.0.0)


## [Unreleased][github/self/unreleased] <a name="unreleased"></a>

### Added <a name="unreleased/added"></a>

\-


### Changed <a name="unreleased/changed"></a>

\-


### Deprecated <a name="unreleased/deprecated"></a>

\-


### Removed <a name="unreleased/removed"></a>

- All functions `new(...)` (-> breaking change).


### Fixed <a name="unreleased/fixed"></a>

\-


### Security <a name="unreleased/security"></a>

\-


## [v1.1.0][github/self/v1.1.0] <a name="v1.1.0"></a>

### Added <a name="v1.1.0/added"></a>

- Add division of same unit (e.g. `3 m / 1 m = 3`).


## [v1.0.3][github/self/v1.0.3] <a name="v1.0.3"></a>

### Changed <a name="v1.0.3/changed"></a>

- Change style of __LICENSE__ and write it with `markdown`.


## [v1.0.2][github/self/v1.0.2] <a name="v1.0.2"></a>

### Changed <a name="v1.0.2/changed"></a>

- Change `Cargo.toml`-keywords, referencing coordinates, haversine, and general names (`kmph` becomes `velocity`).


### Fixed <a name="v1.0.2/fixed"></a>

- Build before publishing in GitHub-action.
  When deploying, to get the current version from `Cargo`, a `Cargo.lock` has to be built, which doesn't happen when publishing.


## [v1.0.1][github/self/v1.0.1] <a name="v1.0.1"></a>

### Removed <a name="v1.0.1/removed"></a>

- Remove __`Cargo.lock`__ from version-control since this repo has no binaries.


### Fixed <a name="v1.0.1/fixed"></a>

- Fix `README.md`__
  - Fix __failing `nightly`-badge__
  - __Format example-code__
  - Fix __hardcoded docs-link__


## [v1.0.0][github/self/v1.0.0] <a name="v1.0.0"></a>

### Added <a name="v1.0.0/added"></a>

- Move all the code from the module `units` from __[osmgraphing][github/dominicparga/osmgraphing]__ to here.
- Add __metrics__ `Kilometers`, `Meters`, `KilometersPerHour`, `MetersPerSecond`, `Hours`, `Minutes`, `Seconds`.
- Add __example__ for basic usage.


### Changed <a name="v1.0.0/changed"></a>

- Update __`README.md`__.
- Update __license__.


### Deprecated <a name="v1.0.0/deprecated></a>

- __Documentation__ should be nicer.
- `README.md` shows failing `nightly`-build.
- `README.md` has example-code not formatted properly.
- `README.md` has hardcoded version 0 for docs.


### Removed <a name="unreleased/removed"></a>

- Remove __`benchmarks`__.
- Remove __`binaries`__.


## [v0.0.0][github/self/v0.0.0] <a name="v0.0.0"></a>

### Added <a name="v0.0.0/added"></a>

- Initialize repository.


[github/dominicparga/osmgraphing]: https://github.com/dominicparga/osmgraphing
[keepachangelog]: https://keepachangelog.com/en/
[semver]: https://semver.org/

[github/self/unreleased]: https://github.com/dominicparga/kissunits/compare/v1.1.0...HEAD
[github/self/v1.1.0]: https://github.com/dominicparga/kissunits/compare/v1.0.3...v1.1.0
[github/self/v1.0.3]: https://github.com/dominicparga/kissunits/compare/v1.0.2...v1.0.3
[github/self/v1.0.2]: https://github.com/dominicparga/kissunits/compare/v1.0.1...v1.0.2
[github/self/v1.0.1]: https://github.com/dominicparga/kissunits/compare/v1.0.0...v1.0.1
[github/self/v1.0.0]: https://github.com/dominicparga/kissunits/compare/v0.0.0...v1.0.0
[github/self/v0.0.0]: https://github.com/dominicparga/kissunits/releases/tag/v0.0.0
