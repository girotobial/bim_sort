# Changelog

All notable changes to bim_sort will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).


## [Unreleased]
### Added
- Support for Calculation Groups.
- Support for Privacy Levels in Data Sources.
- Support for PostgresSQL Data Sources
- Support for MySQL Data Sources.


## [0.1.2] - 2023-01-03
### Fixed
- Columns not including `sourceColumn` and `sortByColumn` invalidating the bim file.

## [0.1.1] - 2022-12-30
### Fixed
- Fields that were not included in the input were being output as `null` in the output
- Allow missing roles field in the model
- Empty measures field in tables, if not in the original file.

## [0.1.0] - 2022-12-30
### Added
- Command line interface for selecting a file
- Sorting of items in the bim file by name.