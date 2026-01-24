# 🚀 Guide de Démarrage Rapide

Ce guide vous aidera à démarrer rapidement avec la Fibonacci Benchmark Suite.

## 📋 Prérequis

### Installation de Rust

Si Rust n'est pas installé sur votre système :

```bash
# Linux/macOS
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Windows
# Télécharger et exécuter rustup-init.exe depuis https://rustup.rs/
```

Vérifiez l'installation :

```bash
rustc --version
cargo --version
```

## 🔧 Installation du projet

### 1. Cloner le repository

```bash
git clone https://github.com/agbru/FibBenchmark.git
cd FibBenchmark
```

### 2. Compiler le projet

```bash
# Compilation en mode debug (rapide)
cargo build

# Compilation en mode release (optimisé)
cargo build --release
```

### 3. Exécuter les tests

```bash
cargo test
```

## 🎮 Utilisation de la CLI

### Commandes de base

#### Calculer un nombre de Fibonacci

```bash
# Syntaxe de base
cargo run --bin fib-bench -- calc -n 50

# Avec une méthode spécifique
cargo run --bin fib-bench -- calc -n 100 --method matrix

# Avec affichage du temps
cargo run --bin fib-bench -- calc -n 100 --method iterative --time
```

Méthodes disponibles :
- `recursive` - O(2^n), pour démonstration seulement
- `recursive_memo` - O(n) avec memoization
- `iterative` - O(n), recommandé par défaut
- `matrix` - O(log n), optimal pour grands n
- `binet` - O(1), précis jusqu'à n ≤ 78

#### Comparer les algorithmes

```bash
# Comparaison pour n = 30
cargo run --bin fib-bench -- compare -n 30

# Limiter le récursif à n = 25
cargo run --bin fib-bench -- compare -n 40 --max-recursive 25
```

#### Générer une séquence

```bash
# 20 premiers nombres
cargo run --bin fib-bench -- sequence --count 20

# À partir de n = 10
cargo run --bin fib-bench -- sequence --count 10 --start 10
```

#### Informations sur les algorithmes

```bash
# Tous les algorithmes
cargo run --bin fib-bench -- info --method all

# Un algorithme spécifique
cargo run --bin fib-bench -- info --method matrix
```

#### Analyse de Binet

```bash
cargo run --bin fib-bench -- binet-analysis --max-n 100
```

## 📊 Exécuter les Benchmarks

### Benchmarks complets

```bash
cargo bench
```

Les résultats sont sauvegardés dans `target/criterion/`.

### Filtrer les benchmarks

```bash
# Seulement la comparaison de complexité
cargo bench -- complexity_comparison

# Seulement les grands n
cargo bench -- large_n
```

### Visualiser les résultats

Ouvrez le rapport HTML :

```bash
# Linux
xdg-open target/criterion/report/index.html

# macOS
open target/criterion/report/index.html

# Windows
start target/criterion/report/index.html
```

## 📚 Utilisation comme bibliothèque

### Ajouter la dépendance

Dans votre `Cargo.toml` :

```toml
[dependencies]
fib-core = { path = "path/to/FibBenchmark/crates/fib-core" }
```

### Exemples de code

```rust
use fib_core::{iterative, matrix, FibMethod};

fn main() {
    // Méthode itérative
    let fib_50 = iterative::fib_iterative(50);
    println!("F(50) = {}", fib_50);

    // Méthode matricielle
    let fib_100 = matrix::fib_matrix_fast(100);
    println!("F(100) = {}", fib_100);

    // Avec l'enum FibMethod
    let method = FibMethod::Matrix;
    println!("Algorithme: {}", method.name());
    println!("Complexité: {}", method.time_complexity());
    println!("F(100) = {}", method.calculate(100));

    // Cache pour requêtes répétées
    let cache = iterative::FibonacciCache::new(100);
    println!("F(50) depuis cache = {:?}", cache.get(50));

    // Itérateur sur la suite
    let premiers_10: Vec<u128> = iterative::FibonacciIterator::new()
        .take(10)
        .collect();
    println!("10 premiers: {:?}", premiers_10);
}
```

## 🛠️ Outils additionnels

### Profiler

```bash
cargo run --bin fib-profiler
```

### Visualisation

```bash
cargo run --bin fib-viz
```

Les fichiers CSV sont générés dans `results/`.

### Dashboard Web Interactif

Le projet inclut un dashboard web moderne pour visualiser les résultats :

```bash
# Option 1: Ouvrir directement
start dashboard/index.html  # Windows
open dashboard/index.html   # macOS
xdg-open dashboard/index.html  # Linux

# Option 2: Avec serveur local (recommandé)
cd dashboard
python -m http.server 8080
# Ouvrir http://localhost:8080
```

**Fonctionnalités du dashboard** :

- Mode sombre/clair avec toggle (raccourci: Ctrl+Shift+L)
- Graphiques interactifs avec zoom et pan
- Métriques animées au scroll
- Responsive design pour mobile/desktop
- Tooltips enrichis avec détails contextuels

## ❓ Dépannage

### Erreur de compilation

```bash
# Nettoyer et recompiler
cargo clean
cargo build
```

### Tests qui échouent

```bash
# Tests verbeux
cargo test -- --nocapture
```

### Benchmarks trop longs

Réduisez la taille des échantillons dans les fichiers bench, ou filtrez :

```bash
cargo bench -- small_n
```

## 📖 Prochaines étapes

1. Lisez [MATHEMATICS.md](../../MATHEMATICS.md) pour la théorie
2. Explorez [BENCHMARKS.md](../../BENCHMARKS.md) pour les résultats
3. Consultez la [documentation des algorithmes](../math/)

---

**Besoin d'aide ?** Ouvrez une issue sur GitHub !
