# 🔬 Fibonacci Performance Benchmark Suite

[![Rust](https://img.shields.io/badge/rust-1.70%2B-orange.svg)](https://www.rust-lang.org/)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)

> Un écosystème complet de benchmarking des algorithmes Fibonacci en Rust, avec analyses de complexité, visualisations et documentation mathématique rigoureuse.

## ✨ Caractéristiques

- **6 algorithmes Fibonacci** avec différentes complexités temporelles
- **Optimisations SIMD** (AVX2/AVX512) pour le traitement par lots
- **Comparaison Cross-Language** avec implémentation Go via FFI
- **Benchmarking Criterion** pour des mesures précises et statistiquement rigoureuses
- **CLI complète** avec 9 commandes pour calculs, comparaisons et analyses
- **Profiling avancé** (Flamegraphs) et analyse mémoire
- **Visualisations** graphiques et rapports HTML
- **Dashboard interactif** moderne avec mode sombre/clair
- **Documentation exhaustive** et Manuel Utilisateur

## 📊 Algorithmes Implémentés

| Algorithme          | Temps    | Espace | Cas d'usage              |
| ------------------- | -------- | ------ | ------------------------ |
| Récursif naïf       | O(2ⁿ)    | O(n)   | Démonstration uniquement |
| Récursif + Mémo     | O(n)     | O(n)   | Petits n avec cache      |
| Itératif            | O(n)     | O(1)   | Usage général            |
| Itératif branchless | O(n)     | O(1)   | Optimisation micro       |
| Matriciel           | O(log n) | O(1)   | Grands n                 |
| Fast Doubling       | O(log n) | O(log n) | Grands n (alternative) |
| Binet               | O(1)     | O(1)   | Approximation (n ≤ 78)   |
| SIMD Batch          | O(n/k)   | O(k)   | Calculs massifs par lots |

## 🚀 Installation

### Prérequis

- Rust 1.70+ ([rustup](https://rustup.rs/))
- Cargo (inclus avec Rust)
- (Optionnel) Go 1.20+ pour `compare-go`

### Compilation

```bash
# Cloner le repository
git clone https://github.com/agbru/FibBenchmark.git
cd FibBenchmark

# Compiler en mode release
cargo build --release

# Exécuter les tests (avec couverture étendue)
cargo test --workspace

# Lancer les benchmarks
cargo bench
```

## 🛠️ Utilisation

Voir le [**Manuel Utilisateur**](docs/MANUAL.md) pour un guide complet.

### CLI Tool

Le projet fournit une interface en ligne de commande complète via `fib-bench` :

```bash
# Calculer F(n)
cargo run --bin fib-bench -- calc -n 100

# Comparer toutes les méthodes
cargo run --bin fib-bench -- compare -n 1000

# Démonstration SIMD (comparaison scalaire vs vectorielle)
cargo run --bin fib-bench -- simd --batch 10,100,1000 --compare

# Comparer avec Go
cargo run --bin fib-bench -- compare-go -n 10000

# Générer le rapport complet (output dans results/)
cargo run --bin fib-bench -- report

# Ouvrir le dashboard interactif
# (ouvrir dashboard/index.html dans un navigateur)
```

**Commandes disponibles :**

- `calc`, `compare`, `bench`, `info`, `sequence`, `binet-analysis`, `report`, `simd`, `compare-go`, `memory`

### Comme bibliothèque

```rust
use fib_core::{iterative, matrix, FibMethod};

// Calcul simple
let fib_100 = iterative::fib_iterative(100);
assert_eq!(fib_100, 354224848179261915075);

// Méthode matricielle pour grands n
let fib_1000 = matrix::fib_matrix_fast(1000);

// Via l'enum FibMethod
let method = FibMethod::Matrix;
let result = method.calculate(100);
```

## 📁 Structure du Projet

```
FibBenchmark/
├── Cargo.toml                    # Workspace root
├── README.md                     # Ce fichier
├── LICENSE                       # MIT License
├── rust-toolchain.toml           # Version Rust
│
├── crates/
│   ├── fib-core/                 # 🧮 Bibliothèque principale
│   │   ├── src/
│   │   │   ├── lib.rs            # Point d'entrée + FibMethod enum
│   │   │   ├── recursive.rs     # O(2^n) + O(n) mémorisé
│   │   │   ├── iterative.rs     # O(n) + branchless + cache
│   │   │   ├── matrix.rs        # O(log n) + Fast Doubling
│   │   │   └── closed_form.rs   # O(1) Binet + analyse
│   │   └── benches/
│   │       └── fib_benchmarks.rs # Benchmarks Criterion
│   │
│   ├── fib-cli/                  # 🖥️ Interface CLI
│   │   └── src/
│   │       ├── main.rs
│   │       └── commands/
│   │           ├── calc.rs
│   │           ├── compare.rs
│   │           ├── bench.rs
│   │           ├── info.rs
│   │           ├── sequence.rs
│   │           ├── memory.rs
│   │           ├── simd.rs
│   │           └── binet_analysis.rs
│   │
│   ├── fib-go/                   # 🐹 Pont FFI vers Go
│   │   ├── src/lib.rs            # Interface Rust (FFI)
│   │   ├── go/fib.go             # Implémentation Go
│   │   └── build.rs              # Script de build Go
│   │
│   ├── fib-profiler/             # 📊 Outil de profiling
│   │   └── src/main.rs
│   │
│   └── fib-viz/                  # 📈 Visualisations
│       └── src/main.rs
│
├── dashboard/                    # 🎨 Dashboard Web Interactif
│   ├── index.html                # Page principale
│   ├── css/styles.css            # Design system (dark/light)
│   └── js/
│       ├── app.js                # Application principale
│       ├── charts.js             # Graphiques Chart.js
│       └── theme.js              # Toggle sombre/clair
│
├── docs/                         # 📚 Documentation complète
│   ├── ARCHITECTURE.md           # Architecture technique détaillée
│   ├── BENCHMARKS.md             # Résultats et analyses de performance
│   ├── MANUAL.md                 # Manuel Utilisateur
│   ├── MATHEMATICS.md            # Théorie mathématique complète
│   ├── PLANNING.md               # Planification et roadmap
│   ├── math/
│   │   ├── fibonacci_theory.md
│   │   ├── matrix_method.md
│   │   └── binet_formula.md
│   ├── performance/
│   │   └── optimization_techniques.md
│   └── usage/
│       └── getting_started.md
│
└── target/                       # Artifacts de compilation (gitignored)
```

## 📈 Benchmarks

Le projet utilise [Criterion.rs](https://github.com/bheisler/criterion.rs) pour des benchmarks statistiquement rigoureux avec détection de régressions.

### Exécution des benchmarks

```bash
# Tous les benchmarks
cargo bench

# Filtrer par nom de groupe
cargo bench -- complexity_comparison

# Benchmark spécifique
cargo bench -- matrix

# Avec baseline pour comparaison
cargo bench -- --save-baseline main
cargo bench -- --baseline main

# Via CLI
cargo run --bin fib-bench -- bench
```

### Groupes de benchmarks

Le projet inclut 6 groupes de benchmarks Criterion :

1. **complexity_comparison** - Comparaison des complexités algorithmiques
2. **large_n** - Scaling pour grands n
3. **iterative_variants** - Comparaison des variantes itératives
4. **batch_operations** - Opérations par lot
5. **cache_vs_direct** - Cache vs calcul direct
6. **modular_arithmetic** - Opérations modulo

### Rapports

Les rapports HTML détaillés sont générés dans `target/criterion/report/index.html` après chaque exécution.

### Dashboard Interactif

Un dashboard web moderne est disponible dans `dashboard/index.html` avec :

- **Design dark gradient** moderne et élégant
- **Toggle mode sombre/clair** avec persistance localStorage
- **Graphiques interactifs** (zoom, pan, tooltips enrichis)
- **Métriques clés** animées et responsive
- **Comparaison d'algorithmes** avec badges de complexité

Pour visualiser le dashboard :

```bash
# Option 1: Ouvrir directement dans le navigateur
start dashboard/index.html  # Windows
open dashboard/index.html   # macOS
xdg-open dashboard/index.html  # Linux

# Option 2: Serveur local (recommandé pour éviter les restrictions CORS)
cd dashboard
python -m http.server 8080
# Puis ouvrir http://localhost:8080
```

### Résultats typiques

Voir [**BENCHMARKS.md**](docs/BENCHMARKS.md) pour des résultats détaillés. Exemples :

```
complexity_comparison/matrix/100      time: [45 ns 46 ns 47 ns]
complexity_comparison/iterative/100   time: [120 ns 122 ns 125 ns]

large_n/matrix/10000               time: [180 ns 185 ns 190 ns]
large_n/iterative/10000              time: [12 µs 12.5 µs 13 µs]
```

Le speedup de la méthode matricielle augmente avec n (O(log n) vs O(n)).

## 📚 Documentation

Le projet inclut une documentation exhaustive organisée en plusieurs sections :

### Guides

- [**MANUEL.md**](docs/MANUAL.md) - Manuel d'utilisation complet 📘
- [**ARCHITECTURE.md**](docs/ARCHITECTURE.md) - Architecture technique
- [**BENCHMARKS.md**](docs/BENCHMARKS.md) - Résultats de performance
- [**PLANNING.md**](docs/PLANNING.md) - Historique du projet

### Mathématiques et Théorie

- [**MATHEMATICS.md**](docs/MATHEMATICS.md) - Théorie mathématique

### Documentation générée

```bash
# Générer la documentation complète
cargo doc --open

# Documentation pour un crate spécifique
cargo doc -p fib-core --open
```

## 🧪 Tests

La couverture de tests a été maximisée pour garantir la robustesse fonctionnelle et technique.

```bash
# Exécuter tous les tests (Workspace complet)
cargo test --workspace

# Tests d'intégration CLI
cargo test -p fib-cli --test integration_tests

# Tests de débordement et BigInt (fib-core)
cargo test -p fib-core --test overflow_tests

# Tests d'interopérabilité Go
cargo test -p fib-go

# Tests d'un crate spécifique
cargo test -p fib-core
```

## 🎯 État du Projet

**Version actuelle :** 1.1.0 🎉

Projet complet et finalisé avec toutes les phases optionnelles (Go, SIMD) implémentées.

## 🤝 Contribution

Les contributions sont les bienvenues !

1. Fork le projet
2. Créer une branche (`git checkout -b feature/amazing-feature`)
3. S'assurer que les tests passent : `cargo test`
4. Vérifier le formatage : `cargo fmt --check`
5. Vérifier les lints : `cargo clippy -- -D warnings`
6. Commit les changements (`git commit -m 'Add amazing feature'`)
7. Push (`git push origin feature/amazing-feature`)
8. Ouvrir une Pull Request

### Standards de code

- Formatage : `cargo fmt`
- Linting : `cargo clippy -- -D warnings`
- Tests : Tous les tests doivent passer
- Documentation : Doc-tests pour les exemples publics

## 🔍 Exemples d'Utilisation

### Calcul simple

```rust
use fib_core::{iterative, matrix, FibMethod};

// Calcul direct
let fib_100 = iterative::fib_iterative(100);
assert_eq!(fib_100, 354224848179261915075);

// Via enum
let method = FibMethod::Matrix;
let result = method.calculate(1000);
```

### Comparaison de méthodes

```rust
use fib_core::FibMethod;

let n = 50;
let methods = [
    FibMethod::Iterative,
    FibMethod::Matrix,
    FibMethod::Binet,
];

for method in methods {
    let result = method.calculate(n);
    println!("{}: {} (complexity: {})",
        method.name(),
        result,
        method.time_complexity()
    );
}
```

### Cache pour calculs répétés

```rust
use fib_core::iterative::FibonacciCache;

let mut cache = FibonacciCache::new(100);
// Le cache peut être réutilisé pour plusieurs calculs
```

## 🛡️ Limitations et Notes

- **u128 overflow** : F(186) est le dernier Fibonacci qui tient dans u128 (utilisez le mode BigInt automatique via CLI pour n > 186)
- **Binet précision** : Limité à n ≤ 78 pour une précision exacte
- **Récursif naïf** : Extrêmement lent pour n > 35, à utiliser uniquement à des fins pédagogiques
- **Stack overflow** : La récursion peut causer un stack overflow pour n > 100,000 (selon la taille de stack)

## 🙏 Remerciements

- [Criterion.rs](https://github.com/bheisler/criterion.rs) pour le framework de benchmarking statistiquement rigoureux
- [clap](https://github.com/clap-rs/clap) pour l'excellente bibliothèque CLI
- La communauté Rust pour les outils et le support

## 📜 Licence

Ce projet est sous licence MIT. Voir [LICENSE](LICENSE) pour plus de détails.

---

<p align="center">
  Fait avec ❤️ et 🦀<br>
  <em>Un projet démontrant l'excellence en ingénierie Rust</em>
</p>
