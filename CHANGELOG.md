# Change Log

## [Unreleased] - ReleaseDate

### Added

* Added associated constants.

### Changed

### Fixed

## [0.3.0] - 2026-01-12

### Changed
* Uses `Option` instead of `Result`, removed `OneBasedError`.
  This is due to the following reasons.
  1. `Result` mixes up two failure modes unnecessarily.
     `OneBasedU*::from_zero_based` never fails on zero,
     `OneBasedU*::from_one_based` never fails on MAX.
     However, using Result would mix up and might give false sense that
     two API might fail with both reasons.
  2. `Option<OneBased*>` is exactly the same as the underlying integer,
     while `Result<OneBased*, OneBasedError>` would be definitely larger.
     Using `Option` is therefore an optimization.
  3. Currently, `Option::unwrap` is `const` while `Result::unwrap` isn't.

## [0.2.2] - 2025-07-30

### Added

* Added `From` and `TryFrom` from/to `OneBasedUsize` type.

## [0.2.1] - 2025-07-29

### Added

* Added unsafe `from_*_based_unchecked` variants.
* Added `From` and `TryFrom` between `OneBasedU*` types.

### Changed

* All functions are now marked as `const`.

## [0.2.0] - 2025-07-28

### Changed

* Now the library is `no_std`.

## [0.1.0] - 2025-07-28

### Added

* First release.
