# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.2.0] - 2021-08-08

### Changed

- Replace `BTreeMap` with `BinaryHeap`, which is better in terms of cache locality.

## [0.1.0] - 2020-06-24

### Added

- A `BTreeMap`-based naive timer.
