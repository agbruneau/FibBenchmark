# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [1.1.0] - 2026-01-07

### Added

- **Dashboard Web Interactif** (`dashboard/`):
  - Design dark gradient moderne avec effets glassmorphism
  - Toggle mode sombre/clair avec persistance localStorage
  - Detection automatique des preferences systeme (`prefers-color-scheme`)
  - Graphiques interactifs avec Chart.js (zoom, pan, tooltips enrichis)
  - Metriques animees (count-up) au scroll avec Intersection Observer
  - Navigation sticky avec effet blur
  - Hero section avec gradient anime
  - Bento grid pour les KPI
  - Tableau comparatif des algorithmes avec badges de complexite
  - Support responsive (mobile/tablette/desktop)
  - Raccourci clavier Ctrl+Shift+L pour toggle theme

### Changed

- Mise a jour de la documentation README avec section dashboard
- Mise a jour de ARCHITECTURE.md avec composant dashboard
- Mise a jour du guide de demarrage rapide

---

## [1.0.0] - 2026-01-04

### Added

- **Core Algorithms**: Implemented 6 Fibonacci calculation methods:
  - Recursive (Naïve & Memoized)
  - Iterative (Standard & Branchless)
  - Matrix Exponentiation (Fast O(log n))
  - Binet Formula (O(1) approximation)
- **CLI Tool (`fib-cli`)**:
  - `calc`: Calculate F(n) with various methods.
  - `compare`: Compare execution time of all methods.
  - `bench`: Run Criterion benchmarks.
  - `info`: Display complexity info for algorithms.
  - `sequence`: Generate Fibonacci sequences.
  - `report`: Generate HTML visual reports.
- **Profiling (`fib-profiler`)**:
  - Flamegraph support (Unix) with `pprof`.
  - Memory usage tracking with custom allocator.
- **Visualization (`fib-viz`)**:
  - Interactive Plotly charts (SVG/HTML).
  - Automatic complexity comparison graphs.
- **Documentation**:
  - Comprehensive `README.md`.
  - Math theory documentation (`docs/math/`).
  - Benchmark analysis (`docs/BENCHMARKS.md`).

### Changed

- Standardized error handling across all crates.
- Optimized matrix multiplication for performance.
- Improved CLI output formatting.

### Fixed

- Addressed potential overflows in `u128` calculations (documented limits).
- Accurate timing measurements for micro-benchmarks.

## [0.1.0] - 2026-01-03

### Added

- Initial release for internal testing.
- Basic implementation of recursive and iterative algorithms.
