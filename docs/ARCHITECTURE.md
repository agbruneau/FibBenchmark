# 🏗️ Architecture Technique

## Fibonacci Performance Benchmark Suite

> **Version**: 1.0.0  
> **Dernière mise à jour**: Janvier 2026  
> **Statut**: Production

---

## 📑 Table des matières

1. [Vue d'ensemble](#1-vue-densemble)
2. [Architecture du workspace](#2-architecture-du-workspace)
3. [Composants principaux](#3-composants-principaux)
4. [Diagrammes d'architecture](#4-diagrammes-darchitecture)
5. [Patterns et conventions](#5-patterns-et-conventions)
6. [API et interfaces](#6-api-et-interfaces)
7. [Flux de données](#7-flux-de-données)
8. [Performance et optimisations](#8-performance-et-optimisations)
9. [Gestion des erreurs](#9-gestion-des-erreurs)
10. [Tests et qualité](#10-tests-et-qualité)
11. [Décisions techniques](#11-décisions-techniques)
12. [Extensibilité](#12-extensibilité)
13. [Sécurité](#13-sécurité)
14. [Déploiement](#14-déploiement)

---

## 1. Vue d'ensemble

### 1.1 Objectif architectural

L'architecture du projet Fibonacci Benchmark Suite suit les principes de **modularité**, **séparation des responsabilités** et **performance** pour créer un écosystème cohérent d'outils de benchmarking.

### 1.2 Principes directeurs

| Principe                   | Description                                  | Application            |
| -------------------------- | -------------------------------------------- | ---------------------- |
| **Modularité**             | Chaque crate a une responsabilité unique     | 4 crates indépendantes |
| **Zero-cost abstractions** | Pas de surcoût runtime pour les abstractions | Traits, generics       |
| **Performance first**      | Optimisation sans sacrifier la lisibilité    | Algorithmes O(log n)   |
| **Documentation as code**  | Doc-tests exécutables                        | Exemples dans rustdoc  |
| **Fail fast**              | Erreurs détectées à la compilation           | Types stricts          |

### 1.3 Stack technologique

```
┌─────────────────────────────────────────────────────────────┐
│                    COUCHE PRÉSENTATION                       │
│  ┌─────────────┐  ┌─────────────┐  ┌─────────────┐  ┌─────────────┐  │
│  │   fib-cli   │  │ fib-profiler│  │   fib-viz   │  │  dashboard  │  │
│  │   (clap)    │  │  (pprof)    │  │  (plotly)   │  │ (Chart.js)  │  │
│  └──────┬──────┘  └──────┬──────┘  └──────┬──────┘  └──────┬──────┘  │
└─────────┼────────────────┼────────────────┼─────────────────┘
          │                │                │
          └────────────────┼────────────────┘
                           │
┌──────────────────────────┼──────────────────────────────────┐
│                    COUCHE MÉTIER                             │
│                    ┌─────┴─────┐                             │
│                    │ fib-core  │                             │
│                    │ (Rust+SIMD)                             │
│  ┌─────────────────┼───┬───────┼─────────────────┐          │
│  │    ┌────────────┴───│───────┴────────────┐    │          │
│  │    │         Algorithmes Fibonacci        │    │          │
│  │    │  ┌──────────┐ ┌──────────┐ ┌──────┐ │    │          │
│  │    │  │recursive │ │iterative │ │ SIMD │ │    │          │
│  │    │  └──────────┘ └──────────┘ └──────┘ │    │          │
│  │    │  ┌──────────┐ ┌───────────┐         │    │          │
│  │    │  │  matrix  │ │closed_form│         │    │          │
│  │    │  └──────────┘ └───────────┘         │    │          │
│  │    └─────────────────────────────────────┘    │          │
│  └───────────────────────────────────────────────┘          │
│                            │                                │
│                     ┌──────┴───────┐                        │
│                     │  Go Bridge   │                        │
│                     │    (FFI)     │                        │
│                     └──────────────┘                        │
└─────────────────────────────────────────────────────────────┘
                           │
┌──────────────────────────┼──────────────────────────────────┐
│                 COUCHE INFRASTRUCTURE                        │
│  ┌─────────────┐  ┌─────────────┐  ┌─────────────┐          │
│  │  Criterion  │  │   serde     │  │  num-bigint │          │
│  │ (benchmark) │  │   (json)    │  │  (précision)│          │
│  └─────────────┘  └─────────────┘  └─────────────┘          │
└─────────────────────────────────────────────────────────────┘
```

---

## 2. Architecture du workspace

### 2.1 Structure Cargo Workspace

```toml
# Cargo.toml (racine)
[workspace]
members = [
    "crates/fib-core",      # Bibliothèque principale
    "crates/fib-cli",       # Interface ligne de commande
    "crates/fib-profiler",  # Outils de profiling
    "crates/fib-viz",       # Visualisations
    "crates/fib-go",        # Bridge Go Integration
]
resolver = "2"
```

### 2.2 Hiérarchie des dépendances

```
fib-core (bibliothèque)
    └── num-bigint
    └── wide (SIMD)
    └── [dev] criterion, proptest

fib-cli (binaire)
    └── fib-core
    └── fib-go (optional)
    └── clap
    └── serde, serde_json
    └── criterion

fib-profiler (binaire)
    └── fib-core
    └── serde, serde_json
    └── [unix] pprof

fib-viz (binaire)
    └── fib-core
    └── plotly
    └── serde, serde_json

dashboard (web frontend)
    └── Chart.js 4.x
    └── chartjs-plugin-zoom
    └── Hammer.js (touch gestures)

fib-go (bibliothèque)
    └── [build] cc, bindgen
```

### 2.3 Profiles de compilation

```toml
[profile.release]
lto = true           # Link-Time Optimization
codegen-units = 1    # Meilleure optimisation
opt-level = 3        # Optimisation maximale

[profile.bench]
debug = true         # Symboles pour profiling
```

---

## 3. Composants principaux

### 3.1 fib-core

**Rôle**: Bibliothèque centrale contenant tous les algorithmes Fibonacci.

```
crates/fib-core/
├── Cargo.toml
├── src/
│   ├── lib.rs           # Point d'entrée, FibMethod enum
│   ├── recursive.rs     # Algorithmes récursifs
│   ├── iterative.rs     # Algorithmes itératifs
│   ├── matrix.rs        # Exponentiation matricielle
│   ├── closed_form.rs   # Formule de Binet
│   └── simd.rs          # Optimisations SIMD (AVX2/AVX512)
└── benches/
    └── fib_benchmarks.rs
```

**Responsabilités**:

- Implémenter les 6 algorithmes Fibonacci de base
- Fournir les implémentations SIMD optimisées
- Fournir une API unifiée via `FibMethod`
- Exposer les benchmarks Criterion
- Documenter les complexités et limites

**Exports publics**:

```rust
// Modules
pub mod recursive;
pub mod iterative;
pub mod matrix;
pub mod closed_form;
pub mod simd;

// Types
pub enum FibMethod { ... }

// Fonctions principales
pub use recursive::{fib_recursive, fib_recursive_memo};
pub use iterative::{fib_iterative, fib_iterative_branchless, fib_iterative_batch};
pub use matrix::{fib_matrix_fast, fib_matrix_modulo};
pub use closed_form::{fib_binet_f64, binet_error_analysis};
pub use simd::{fib_simd_u64, fib_simd_batch};
```

### 3.2 fib-cli

**Rôle**: Interface utilisateur en ligne de commande.

```
crates/fib-cli/
├── Cargo.toml
└── src/
    ├── main.rs
    └── commands/
        ├── mod.rs
        ├── calc.rs           # Calcul simple
        ├── compare.rs        # Comparaison algorithmes
        ├── bench.rs          # Lancer benchmarks
        ├── info.rs           # Informations algorithmes
        ├── sequence.rs       # Générer séquences
        ├── binet_analysis.rs # Analyse précision Binet
        ├── report.rs         # Génération rapports
        ├── simd.rs           # Démo SIMD
        └── compare_go.rs     # Comparaison Rust vs Go
```

**Responsabilités**:

- Parser les arguments avec clap
- Router vers les commandes appropriées
- Formater les sorties utilisateur
- Gérer les erreurs gracieusement

**Architecture de commandes**:

```rust
#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Calc { ... },
    Compare { ... },
    Bench { ... },
    Info { ... },
    Sequence { ... },
    BinetAnalysis { ... },
    Report { ... },
    Simd { ... },
    CompareGo { ... },
}
```

### 3.3 fib-profiler

**Rôle**: Outils de profiling et analyse de performance.

```
crates/fib-profiler/
├── Cargo.toml
└── src/
    └── main.rs
```

**Responsabilités**:

- Profiler les différentes méthodes
- Analyser la mémoire
- Comparer les temps d'exécution
- Générer des rapports

### 3.4 fib-viz

**Rôle**: Génération de visualisations et graphiques.

```
crates/fib-viz/
├── Cargo.toml
└── src/
    └── main.rs
```

**Responsabilités**:

- Générer des données CSV
- Créer des graphiques avec Plotly
- Exporter en SVG/HTML

### 3.5 dashboard

**Rôle**: Interface web interactive pour visualiser les résultats des benchmarks.

```
dashboard/
├── index.html           # Page principale
├── css/
│   └── styles.css       # Design system avec thèmes
├── js/
│   ├── app.js           # Logique principale
│   ├── charts.js        # Configuration Chart.js
│   └── theme.js         # Gestion sombre/clair
└── data/                # Données JSON des benchmarks
    ├── complexity_comparison.json
    ├── binet_accuracy.json
    └── golden_ratio_convergence.json
```

**Responsabilités**:

- Afficher un tableau de bord moderne et responsive
- Visualiser les données de benchmark avec des graphiques interactifs
- Permettre le zoom/pan sur les graphiques
- Offrir un toggle mode sombre/clair avec persistance
- Animer les métriques clés au scroll

**Fonctionnalités**:

- **Hero Section**: Présentation avec gradient animé
- **Bento Grid**: Métriques KPI avec animations count-up
- **Tableau Algorithmes**: Comparaison avec badges de complexité
- **Graphiques Interactifs**: Zoom, pan, tooltips enrichis
- **Theme Toggle**: Mode sombre/clair avec détection système

**Technologies utilisées**:

- Chart.js 4.x pour les graphiques
- chartjs-plugin-zoom pour zoom/pan
- Variables CSS pour le theming
- Intersection Observer pour les animations scroll
- localStorage pour la persistance du thème

### 3.6 fib-go

**Rôle**: Intégration et comparaison avec Go (FFI).

```
crates/fib-go/
├── Cargo.toml
├── build.rs             # Compilation du code Go
└── src/
    ├── lib.rs           # Bindings Rust
    └── fib.go           # Implémentation Go
```

**Responsabilités**:

- Compiler le code Go en bibliothèque statique
- Exposer les fonctions via C FFI
- Fournir des bindings sûrs pour Rust

---

## 4. Diagrammes d'architecture

### 4.1 Diagramme de composants

```
┌─────────────────────────────────────────────────────────────────────┐
│                         UTILISATEUR                                  │
└─────────────────────────────┬───────────────────────────────────────┘
                              │
                              ▼
┌─────────────────────────────────────────────────────────────────────┐
│                           fib-cli                                    │
│  ┌───────────────────────────────────────────────────────────────┐  │
│  │                        Commands                                │  │
│  │  ┌─────┐ ┌───────┐ ┌─────┐ ┌────┐ ┌────────┐ ┌─────────────┐  │  │
│  │  │calc │ │compare│ │bench│ │info│ │sequence│ │binet_analysis│ │  │
│  │  └──┬──┘ └───┬───┘ └──┬──┘ └──┬─┘ └────┬───┘ └──────┬──────┘  │  │
│  │     │        │        │       │        │            │             │  │
│  │  ┌──┴───┐ ┌──┴───┐ ┌──┴───────┴──┐     │            │             │  │
│  │  │report│ │ simd │ │ compare-go  │     │            │             │  │
│  │  └──────┘ └──────┘ └─────────────┘     │            │             │  │
│  └─────┼────────┼────────┼───────┼────────┼────────────┼─────────┘  │
└────────┼────────┼────────┼───────┼────────┼────────────┼────────────┘
         │        │        │       │        │            │
         └────────┴────────┴───────┴────────┴────────────┘
                              │
                              ▼
┌─────────────────────────────────────────────────────────────────────┐
│                          fib-core                                    │
│  ┌─────────────────────────────────────────────────────────────────┐│
│  │                        FibMethod                                 ││
│  │  ┌────────────┐ ┌────────────┐ ┌──────┐ ┌─────┐ ┌─────────────┐ ││
│  │  │ Recursive  │ │ Iterative  │ │Matrix│FastDouble│Binet│IterBranchless│││
│  │  └────────────┘ └────────────┘ └──────┘ └─────┘ └─────────────┘ ││
│  └─────────────────────────────────────────────────────────────────┘│
│                                                                      │
│  ┌─────────────────────────────────────────────────────────────────┐│
│  │                        Modules                                   ││
│  │  ┌─────────────┐ ┌─────────────┐ ┌──────────┐ ┌───────────────┐ ││
│  │  │recursive.rs │ │iterative.rs │ │ matrix.rs│ │closed_form.rs │ ││
│  │  │             │ │             │ │          │ │               │ ││
│  │  │fib_recursive│ │fib_iterative│ │fib_matrix│ │ fib_binet_f64 │ ││
│  │  │fib_rec_memo │ │fib_branchless││fib_modulo│ │binet_error    │ ││
│  │  │count_calls  │ │FibCache     │ │fib_doubl │ │find_limit     │ ││
│  │  │             │ │FibIterator  │ │Matrix2x2 │ │fib_ratio      │ ││
│  │  └─────────────┘ └─────────────┘ └──────────┘ └───────────────┘ ││
│  │  ┌─────────────┐                                                ││
│  │  │   simd.rs   │                                                ││
│  │  │             │                                                ││
│  │  │fib_simd_u64 │                                                ││
│  │  │fib_simd_bat │                                                ││
│  │  └─────────────┘                                                ││
│  └─────────────────────────────────────────────────────────────────┘│
└─────────────────────────────────────────────────────────────────────┘
```

### 4.2 Diagramme de séquence - Commande calc

```
Utilisateur          CLI              Commands::Calc        FibMethod         Algorithm
    │                 │                     │                   │                 │
    │ fib-bench calc  │                     │                   │                 │
    │ -n 100 -m matrix│                     │                   │                 │
    │────────────────>│                     │                   │                 │
    │                 │ parse args          │                   │                 │
    │                 │────────────────────>│                   │                 │
    │                 │                     │ method.parse()    │                 │
    │                 │                     │──────────────────>│                 │
    │                 │                     │                   │ FibMethod::Matrix│
    │                 │                     │<──────────────────│                 │
    │                 │                     │                   │                 │
    │                 │                     │ method.calculate(n)                 │
    │                 │                     │──────────────────>│                 │
    │                 │                     │                   │ fib_matrix_fast │
    │                 │                     │                   │────────────────>│
    │                 │                     │                   │                 │
    │                 │                     │                   │    result       │
    │                 │                     │                   │<────────────────│
    │                 │                     │      u128         │                 │
    │                 │                     │<──────────────────│                 │
    │                 │   format output     │                   │                 │
    │                 │<────────────────────│                   │                 │
    │  F(100) = ...   │                     │                   │                 │
    │<────────────────│                     │                   │                 │
    │                 │                     │                   │                 │
```

### 4.3 Diagramme de classes simplifié

```
┌───────────────────────────────────────────────────────────────┐
│                         FibMethod                              │
├───────────────────────────────────────────────────────────────┤
│ + Recursive                                                    │
│ + RecursiveMemo                                                │
│ + Iterative                                                    │
│ + IterativeBranchless                                          │
│ + Matrix                                                       │
│ + FastDoubling                                                 │
│ + Binet                                                        │
├───────────────────────────────────────────────────────────────┤
│ + calculate(&self, n: u64) -> u128                             │
│ + name(&self) -> &'static str                                  │
│ + time_complexity(&self) -> &'static str                       │
│ + space_complexity(&self) -> &'static str                      │
└───────────────────────────────────────────────────────────────┘
                              △
                              │ implements FromStr
                              │
┌───────────────────────────────────────────────────────────────┐
│                         Matrix2x2                              │
├───────────────────────────────────────────────────────────────┤
│ - data: [[u128; 2]; 2]                                         │
├───────────────────────────────────────────────────────────────┤
│ + new(data: [[u128; 2]; 2]) -> Self                            │
│ + identity() -> Self                                           │
│ + fibonacci_base() -> Self                                     │
│ + get(row: usize, col: usize) -> u128                          │
└───────────────────────────────────────────────────────────────┘
                              △
                              │ implements Mul<Self>
                              │
┌───────────────────────────────────────────────────────────────┐
│                      FibonacciCache                            │
├───────────────────────────────────────────────────────────────┤
│ - values: Vec<u128>                                            │
├───────────────────────────────────────────────────────────────┤
│ + new(max_n: u64) -> Self                                      │
│ + get(&self, n: u64) -> Option<u128>                           │
│ + max_n(&self) -> u64                                          │
└───────────────────────────────────────────────────────────────┘

┌───────────────────────────────────────────────────────────────┐
│                     FibonacciIterator                          │
├───────────────────────────────────────────────────────────────┤
│ - current: u128                                                │
│ - next: u128                                                   │
├───────────────────────────────────────────────────────────────┤
│ + new() -> Self                                                │
└───────────────────────────────────────────────────────────────┘
                              △
                              │ implements Iterator<Item = u128>
```

### 4.4 Diagramme d'états - Exponentiation matricielle

```
                    ┌──────────────┐
                    │    Start     │
                    │   n, base    │
                    └──────┬───────┘
                           │
                           ▼
                    ┌──────────────┐
                    │ result = I   │ (identité)
                    │ exp = n      │
                    └──────┬───────┘
                           │
                           ▼
              ┌────────────────────────┐
              │       exp > 0 ?        │
              └────────┬───────────────┘
                       │
            ┌──────────┴──────────┐
            │ oui                 │ non
            ▼                     ▼
    ┌───────────────┐     ┌─────────────────┐
    │  exp % 2 == 1 │     │ return result[0][1]
    └───────┬───────┘     └─────────────────┘
            │
     ┌──────┴──────┐
     │ oui         │ non
     ▼             ▼
┌────────────┐  ┌────────────┐
│ result =   │  │  (skip)    │
│ result*base│  │            │
└─────┬──────┘  └─────┬──────┘
      │               │
      └───────┬───────┘
              ▼
       ┌─────────────┐
       │ base = base²│
       │ exp = exp/2 │
       └──────┬──────┘
              │
              └─────────────────┐
                                │
                    ┌───────────┴────┐
                    │ (boucle while) │
                    └────────────────┘
```

---

## 5. Patterns et conventions

### 5.1 Patterns de conception utilisés

#### Builder Pattern (implicite via Default)

```rust
impl Default for FibonacciIterator {
    fn default() -> Self {
        Self::new()
    }
}
```

#### Strategy Pattern (via FibMethod enum)

```rust
impl FibMethod {
    pub fn calculate(&self, n: u64) -> u128 {
        match self {
            FibMethod::Recursive => fib_recursive(n),
            FibMethod::Iterative => fib_iterative(n),
            FibMethod::Matrix => fib_matrix_fast(n),
            FibMethod::FastDoubling => fib_doubling(n),
            // ...
        }
    }
}
```

#### Iterator Pattern

```rust
pub struct FibonacciIterator { ... }

impl Iterator for FibonacciIterator {
    type Item = u128;

    fn next(&mut self) -> Option<Self::Item> { ... }
}
```

### 5.2 Conventions de nommage

| Élément    | Convention      | Exemple          |
| ---------- | --------------- | ---------------- |
| Crates     | kebab-case      | `fib-core`       |
| Modules    | snake_case      | `closed_form`    |
| Types      | PascalCase      | `FibMethod`      |
| Fonctions  | snake_case      | `fib_iterative`  |
| Constantes | SCREAMING_SNAKE | `MAX_ACCURATE_N` |
| Traits     | PascalCase      | `Iterator`       |

### 5.3 Conventions de documentation

````rust
/// Description courte sur une ligne.
///
/// # Description détaillée
///
/// Explication plus longue si nécessaire.
///
/// # Arguments
///
/// * `n` - L'index Fibonacci à calculer
///
/// # Returns
///
/// Le n-ième nombre de Fibonacci
///
/// # Panics
///
/// Cette fonction panic si n > 186 (overflow u128).
///
/// # Examples
///
/// ```
/// use fib_core::iterative::fib_iterative;
/// assert_eq!(fib_iterative(10), 55);
/// ```
///
/// # Complexity
///
/// - Time: O(n)
/// - Space: O(1)
pub fn fib_iterative(n: u64) -> u128 { ... }
````

### 5.4 Structure des modules

```rust
//! Documentation du module (//!)
//!
//! Description détaillée du module.

// Imports standards
use std::ops::Mul;

// Imports externes
// (aucun dans fib-core)

// Imports internes
use crate::iterative::fib_iterative;

// Constantes publiques
pub const PHI: f64 = 1.618033988749895;

// Types publics
pub struct Matrix2x2 { ... }

// Implémentations
impl Matrix2x2 { ... }

// Fonctions publiques
pub fn fib_matrix_fast(n: u64) -> u128 { ... }

// Fonctions privées (helpers)
fn multiply_matrices(...) -> ... { ... }

// Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() { ... }
}
```

---

## 6. API et interfaces

### 6.1 API publique de fib-core

#### Module racine (lib.rs)

```rust
// Enum principal
pub enum FibMethod {
    Recursive,
    RecursiveMemo,
    Iterative,
    IterativeBranchless,
    Matrix,
    FastDoubling,
    Binet,
}

impl FibMethod {
    pub fn calculate(&self, n: u64) -> u128;
    pub fn name(&self) -> &'static str;
    pub fn time_complexity(&self) -> &'static str;
    pub fn space_complexity(&self) -> &'static str;
}

impl FromStr for FibMethod {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err>;
}
```

#### Module recursive

```rust
pub fn fib_recursive(n: u64) -> u128;
pub fn fib_recursive_memo(n: u64) -> u128;
pub fn count_recursive_calls(n: u64) -> u64;
```

#### Module iterative

```rust
pub fn fib_iterative(n: u64) -> u128;
pub fn fib_iterative_branchless(n: u64) -> u128;
pub fn fib_iterative_batch(ns: &[u64]) -> Vec<u128>;

pub struct FibonacciCache { ... }
impl FibonacciCache {
    pub fn new(max_n: u64) -> Self;
    pub fn get(&self, n: u64) -> Option<u128>;
    pub fn max_n(&self) -> u64;
}

pub struct FibonacciIterator { ... }
impl FibonacciIterator {
    pub fn new() -> Self;
}
impl Iterator for FibonacciIterator { ... }
```

#### Module matrix

```rust
pub struct Matrix2x2 { ... }
impl Matrix2x2 {
    pub fn new(data: [[u128; 2]; 2]) -> Self;
    pub fn identity() -> Self;
    pub fn fibonacci_base() -> Self;
    pub fn get(&self, row: usize, col: usize) -> u128;
}

pub fn fib_matrix_fast(n: u64) -> u128;
pub fn fib_matrix_modulo(n: u64, modulo: u128) -> u128;
pub fn fib_doubling(n: u64) -> u128;
pub fn fib_doubling(n: u64) -> u128;
```

#### Module closed_form

```rust
// Constantes
pub const PHI: f64;
pub const PSI: f64;
pub const SQRT_5: f64;
pub const MAX_ACCURATE_N: u64;

// Fonctions
pub fn fib_binet_f64(n: u64) -> f64;
pub fn fib_binet_rounded(n: u64) -> u128;
pub fn fib_binet_simplified(n: u64) -> f64;
pub fn binet_error_analysis(n: u64) -> (f64, f64);
pub fn find_binet_accuracy_limit() -> u64;
pub fn fibonacci_ratio(n: u64) -> f64;
pub fn convergence_to_phi(n: u64) -> f64;
```

### 6.2 Interface CLI

```bash
fib-bench <COMMAND>

Commands:
  calc            Calculer F(n) avec une méthode spécifique
  compare         Comparer toutes les méthodes
  bench           Lancer les benchmarks Criterion
  info            Afficher les informations sur les algorithmes
  sequence        Générer une séquence de Fibonacci
  binet-analysis  Analyser la précision de Binet
  report          Générer des rapports et visualisations
  simd            Démonstration des capacités SIMD
  compare-go      Comparer les performances Rust vs Go

Options:
  -h, --help     Afficher l'aide
  -V, --version  Afficher la version
```

#### Détail des commandes

```bash
# calc
fib-bench calc -n <N> [--method <METHOD>] [--time]

# compare
fib-bench compare -n <N> [--max-recursive <MAX>]

# bench
fib-bench bench [--filter <PATTERN>]

# info
fib-bench info [--method <METHOD|all>]

# sequence
fib-bench sequence [--count <COUNT>] [--start <START>]

# binet-analysis
fib-bench binet-analysis [--max-n <MAX_N>]

# report
fib-bench report --open

# simd
fib-bench simd -n 1000 --batch-size 16

# compare-go
fib-bench compare-go -n 1000
```

---

## 7. Flux de données

### 7.1 Flux de calcul Fibonacci

```
┌─────────────┐     ┌───────────────┐     ┌──────────────┐
│   Input     │     │   Processing  │     │    Output    │
│             │     │               │     │              │
│  n: u64     │────>│  FibMethod    │────>│ result: u128 │
│  method: str│     │  .calculate() │     │              │
└─────────────┘     └───────────────┘     └──────────────┘
```

### 7.2 Flux de benchmark

```
┌─────────────┐     ┌───────────────┐     ┌──────────────┐     ┌─────────────┐
│  Criterion  │     │   Algorithm   │     │  Statistics  │     │   Report    │
│   Runner    │     │   Execution   │     │  Collection  │     │  Generation │
│             │     │               │     │              │     │             │
│ warm-up     │────>│ iterations    │────>│ mean, std    │────>│ HTML/JSON   │
│ sampling    │     │ measurement   │     │ confidence   │     │ comparison  │
└─────────────┘     └───────────────┘     └──────────────┘     └─────────────┘
```

### 7.3 Flux de visualisation

```
┌─────────────┐     ┌───────────────┐     ┌──────────────┐
│  Benchmark  │     │  Data Parser  │     │    Chart     │
│   Results   │     │               │     │   Generator  │
│             │     │               │     │              │
│ target/     │────>│ CSV parsing   │────>│ Plotly       │
│ criterion/  │     │ aggregation   │     │ SVG/HTML     │
└─────────────┘     └───────────────┘     └──────────────┘
                           │
                           ▼
                    ┌─────────────┐
                    │   results/  │
                    │   csv/      │
                    │   reports/  │
                    └─────────────┘
```

---

## 8. Performance et optimisations

### 8.1 Optimisations par algorithme

| Algorithme | Optimisation        | Impact          |
| ---------- | ------------------- | --------------- |
| Recursive  | Memoization         | O(2^n) → O(n)   |
| Iterative  | Branchless loop     | 5-15% faster    |
| Matrix     | Fast exponentiation | O(n) → O(log n) |
| Fast Doubling | Doubling identities | O(n) → O(log n) |
| Binet      | Direct formula      | O(n) → O(1)     |

### 8.2 Optimisations de compilation

```toml
[profile.release]
lto = true           # +10-20% performance
codegen-units = 1    # Meilleure optimisation inter-procédurale
opt-level = 3        # Toutes les optimisations
```

### 8.3 Optimisations mémoire

| Structure               | Mémoire        | Notes                |
| ----------------------- | -------------- | -------------------- |
| `fib_iterative`         | 32 bytes stack | 2 × u128             |
| `Matrix2x2`             | 64 bytes stack | 4 × u128             |
| `FibonacciCache(100)`   | ~1.6 KB heap   | Vec<u128>            |
| `fib_recursive_memo(n)` | n × 16 bytes   | Allocation dynamique |

### 8.4 Caractéristiques de performance

```
┌────────────────────────────────────────────────────────────────────┐
│                    Performance Comparison                           │
├────────────────────────────────────────────────────────────────────┤
│                                                                     │
│  Temps (log scale)                                                  │
│    │                                                                │
│    │                                                                │
│  1s├─────────────────x Recursive (n=40)                             │
│    │                                                                │
│    │                                                                │
│ 1ms├                                                                │
│    │                                                                │
│    │                                                                │
│ 1µs├─────────────────────────────────────o Iterative (n=1000)       │
│    │     ────────────────────────────────● Matrix (n=1000)          │
│    │                                                                │
│100ns├─   ● Matrix (n=100)                                           │
│    │     o Iterative (n=100)                                        │
│    │                                                                │
│ 10ns├─   ■ Binet (all n)                                            │
│    │                                                                │
│    └────────────────────────────────────────────────────────────    │
│         10      100     1000    10000   n                           │
└────────────────────────────────────────────────────────────────────┘
```

### 8.5 Benchmarks Criterion

```rust
// Groupes de benchmarks
complexity_comparison   // Comparer les complexités
large_n                 // Scaling pour grands n
iterative_variants      // Comparer variantes itératives
batch_operations        // Opérations par lot
cache_vs_direct         // Cache vs calcul direct
modular_arithmetic      // Opérations modulo
```

---

## 9. Gestion des erreurs

### 9.1 Stratégie d'erreurs

| Situation       | Stratégie                   | Justification                 |
| --------------- | --------------------------- | ----------------------------- |
| n invalide      | Type système (u64)          | Impossible d'avoir n négatif  |
| Overflow        | Wrapping silencieux         | Comportement défini pour u128 |
| Method inconnue | `Result<FibMethod, String>` | Parsing utilisateur           |
| Binet imprécis  | Documentation               | Limitation connue             |

### 9.2 Types d'erreurs

```rust
// fib-core n'expose pas d'erreurs (calculs purs)
// Les limites sont documentées

// fib-cli gère les erreurs utilisateur
impl FromStr for FibMethod {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "recursive" => Ok(FibMethod::Recursive),
            // ...
            _ => Err(format!("Unknown method: {}", s)),
        }
    }
}
```

### 9.3 Gestion dans la CLI

```rust
fn main() {
    let cli = Cli::parse();

    match run(cli) {
        Ok(()) => {}
        Err(e) => {
            eprintln!("❌ Error: {}", e);
            std::process::exit(1);
        }
    }
}
```

---

## 10. Tests et qualité

### 10.1 Stratégie de tests

```
┌─────────────────────────────────────────────────────────────────┐
│                      Pyramide de Tests                           │
├─────────────────────────────────────────────────────────────────┤
│                                                                  │
│                        ┌──────────┐                              │
│                       /│ E2E CLI  │\                             │
│                      / └──────────┘ \                            │
│                     /    (manuel)    \                           │
│                    /                  \                          │
│                   ┌────────────────────┐                         │
│                  /│  Doc-tests (18)    │\                        │
│                 / └────────────────────┘ \                       │
│                /  (exemples documentation) \                     │
│               /                              \                   │
│              ┌────────────────────────────────┐                  │
│             /│    Unit Tests (25)             │\                 │
│            / └────────────────────────────────┘ \                │
│           /    (tests de chaque fonction)        \               │
│          /                                        \              │
│         ┌──────────────────────────────────────────┐             │
│        /│         Property Tests (proptest)        │\            │
│       / └──────────────────────────────────────────┘ \           │
│      /       (invariants mathématiques)               \          │
└─────────────────────────────────────────────────────────────────┘
```

### 10.2 Tests unitaires

```rust
#[cfg(test)]
mod tests {
    use super::*;

    // Valeurs connues
    const FIRST_20_FIBS: [u128; 21] = [
        0, 1, 1, 2, 3, 5, 8, 13, 21, 34,
        55, 89, 144, 233, 377, 610, 987, 1597, 2584, 4181, 6765,
    ];

    #[test]
    fn test_base_cases() {
        assert_eq!(fib_iterative(0), 0);
        assert_eq!(fib_iterative(1), 1);
    }

    #[test]
    fn test_known_values() {
        for (n, expected) in FIRST_20_FIBS.iter().enumerate() {
            assert_eq!(fib_iterative(n as u64), *expected);
        }
    }

    #[test]
    fn test_large_value() {
        assert_eq!(fib_iterative(100), 354224848179261915075);
    }
}
```

### 10.3 Tests de propriétés

```rust
#[cfg(test)]
mod property_tests {
    use proptest::prelude::*;

    proptest! {
        #[test]
        fn fib_additive_property(n in 2u64..100) {
            // F(n) = F(n-1) + F(n-2)
            let fib_n = fib_iterative(n);
            let fib_n1 = fib_iterative(n - 1);
            let fib_n2 = fib_iterative(n - 2);
            prop_assert_eq!(fib_n, fib_n1 + fib_n2);
        }

        #[test]
        fn methods_agree(n in 0u64..50) {
            // Toutes les méthodes donnent le même résultat
            let iter = fib_iterative(n);
            let matrix = fib_matrix_fast(n);
            prop_assert_eq!(iter, matrix);
        }
    }
}
```

### 10.4 Doc-tests

````rust
/// Calcule le n-ième nombre de Fibonacci.
///
/// # Examples
///
/// ```
/// use fib_core::iterative::fib_iterative;
///
/// assert_eq!(fib_iterative(0), 0);
/// assert_eq!(fib_iterative(10), 55);
/// assert_eq!(fib_iterative(50), 12586269025);
/// ```
pub fn fib_iterative(n: u64) -> u128 { ... }
````

### 10.5 Métriques de qualité

| Métrique        | Cible | Actuel       |
| --------------- | ----- | ------------ |
| Tests passants  | 100%  | ✅ 100%      |
| Doc-tests       | 100%  | ✅ 100%      |
| Clippy warnings | 0     | ✅ 0         |
| Couverture      | > 80% | 🔄 À mesurer |

---

## 11. Décisions techniques

### 11.1 Journal des décisions (ADR)

#### ADR-001: Choix de u128 pour les résultats

**Contexte**: Les nombres de Fibonacci croissent exponentiellement.

**Décision**: Utiliser `u128` comme type de retour principal.

**Justification**:

- F(186) est le dernier Fibonacci qui tient dans u128
- Couvre largement les cas d'usage courants
- Performance native sans allocation

**Conséquences**:

- Limite à n ≤ 186 sans overflow
- BigInt disponible pour extension future

#### ADR-002: Architecture en workspace

**Contexte**: Projet multi-composants avec CLI, bibliothèque, outils.

**Décision**: Utiliser un Cargo workspace avec crates séparées.

**Justification**:

- Séparation claire des responsabilités
- Compilation incrémentale efficace
- Réutilisation de fib-core comme bibliothèque

**Conséquences**:

- Complexité légèrement accrue
- Gestion des versions par crate

#### ADR-003: pprof conditionnel à Unix

**Contexte**: pprof ne compile pas sur Windows.

**Décision**: Conditionner pprof avec `cfg(unix)`.

**Justification**:

- Permet la compilation sur toutes les plateformes
- Profiling reste disponible sur Unix/macOS

**Conséquences**:

- Fonctionnalités de profiling limitées sur Windows
- Documentation des limitations

#### ADR-004: Criterion pour les benchmarks

**Contexte**: Besoin de benchmarks statistiquement rigoureux.

**Décision**: Utiliser Criterion.rs.

**Justification**:

- Analyse statistique avancée
- Détection des régressions
- Rapports HTML

**Conséquences**:

- Dépendance dev importante
- Temps de benchmark plus long

### 11.2 Choix technologiques

| Choix     | Alternatives      | Raison du choix            |
| --------- | ----------------- | -------------------------- |
| clap 4.x  | structopt, argh   | Derive macros, complétions |
| Criterion | built-in bench    | Statistiques, rapports     |
| plotly    | gnuplot, plotters | Interactif, web-friendly   |
| u128      | BigInt            | Performance, simplicité    |

---

## 12. Extensibilité

### 12.1 Points d'extension

#### Ajouter un nouvel algorithme

1. Créer `crates/fib-core/src/nouveau.rs`
2. Ajouter au `lib.rs`:
   ```rust
   pub mod nouveau;
   pub use nouveau::fib_nouveau;
   ```
3. Étendre `FibMethod`:
   ```rust
   pub enum FibMethod {
       // ...
       Nouveau,
   }
   ```
4. Ajouter les benchmarks

#### Ajouter une commande CLI

1. Créer `crates/fib-cli/src/commands/nouvelle.rs`
2. Ajouter au `commands/mod.rs`
3. Étendre `Commands` enum
4. Ajouter au match dans `main.rs`

### 12.2 Extension: SIMD (Complétée)

Le support SIMD a été implémenté en Phase 8 pour optimiser les calculs par lots (batch).

```rust
// crates/fib-core/src/simd.rs
pub fn fib_simd_batch(ns: &[u64]) -> Vec<u128> {
    // Utilise le crate 'wide' pour abstraction SIMD
    // Supporte AVX2, AVX512, NEON, SSE automatiquement
}
```

### 12.3 Extension: FFI Go (Complétée)

Le bridge Go a été implémenté en Phase 7 pour comparer les performances entre les langages.

```rust
// crates/fib-go/src/lib.rs
extern "C" {
    fn FibonacciIterative(n: u64) -> u64;
    fn FibonacciMatrix(n: u64) -> u64;
}

pub fn compare_languages(n: u64) {
    // Mesure et compare les temps d'exécution Rust vs Go
}
```

---

## 13. Sécurité

### 13.1 Considérations de sécurité

| Risque                     | Mitigation                            |
| -------------------------- | ------------------------------------- |
| Integer overflow           | Types u128, wrapping_add documenté    |
| Stack overflow (recursion) | Limites documentées, memo recommandée |
| Denial of Service          | Limites sur n pour recursive          |
| Supply chain               | cargo-audit en CI                     |

### 13.2 Audit des dépendances

```bash
# Vérifier les vulnérabilités
cargo audit

# Dépendances minimales
cargo tree --depth 1
```

### 13.3 Garanties mémoire

- ✅ Aucun `unsafe` dans fib-core
- ✅ Pas d'allocation dynamique dans les fonctions core (sauf memo)
- ✅ Ownership Rust = pas de data races

---

## 14. Déploiement

### 14.1 Publication crates.io

```bash
# Vérification
cargo publish --dry-run -p fib-core

# Publication
cargo publish -p fib-core
cargo publish -p fib-cli
```

### 14.2 CI/CD Pipeline

```yaml
# .github/workflows/rust-check.yml
jobs:
  check:
    - cargo check
  test:
    - cargo test
  fmt:
    - cargo fmt --check
  clippy:
    - cargo clippy -- -D warnings
  docs:
    - cargo doc --no-deps
```

### 14.3 Release Process

1. Bump version dans Cargo.toml
2. Update CHANGELOG.md
3. Créer tag Git
4. GitHub Release avec notes
5. Publication crates.io (optionnel)

---

## 📎 Annexes

### A. Glossaire

| Terme                   | Définition                         |
| ----------------------- | ---------------------------------- |
| **φ (phi)**             | Nombre d'or ≈ 1.618                |
| **Binet**               | Formule close pour F(n)            |
| **Fast exponentiation** | Calcul de M^n en O(log n)          |
| **Memoization**         | Cache des résultats intermédiaires |

### B. Références

- [The Rust Book](https://doc.rust-lang.org/book/)
- [Criterion.rs](https://bheisler.github.io/criterion.rs/book/)
- [clap Documentation](https://docs.rs/clap/)
- [Fibonacci Numbers (Wikipedia)](https://en.wikipedia.org/wiki/Fibonacci_number)

---

<p align="center">
<strong>🦀 Fibonacci Performance Benchmark Suite</strong><br>
<em>Architecture technique v1.0</em>
</p>
