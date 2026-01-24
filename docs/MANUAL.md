# 📘 Manuel d'Utilisation - Fibonacci Benchmark Suite

Bienvenue dans le manuel d'utilisation de la suite de benchmark Fibonacci. Ce guide vous accompagnera dans l'installation, l'utilisation et la compréhension des outils fournis.

## 📋 Table des matières

1. [Installation](#-installation)
2. [Prise en main rapide](#-prise-en-main-rapide)
3. [Guide des commandes](#-guide-des-commandes)
4. [Fonctionnalités avancées](#-fonctionnalités-avancées)
5. [Dépannage](#-dépannage)

---

## 🚀 Installation

### Prérequis

- **Rust** (1.70 ou plus récent) : [Installer Rust](https://rustup.rs/)
- **Go** (1.20+, optionnel pour `compare-go`) : [Installer Go](https://go.dev/)
- **Environnement Unix** (Linux/macOS) recommandé pour le profiling avancé (flamegraphs).

### Compilation

Le projet utilise un workspace Cargo standard.

```bash
# 1. Cloner le repository
git clone https://github.com/agbru/FibBenchmark.git
cd FibBenchmark

# 2. Compiler en mode release (optimisé)
cargo build --release

# 3. Vérifier l'installation
cargo run --bin fib-bench -- --version
```

---

## ⚡ Prise en main rapide

Une fois compilé, vous pouvez utiliser l'outil via `cargo run --bin fib-bench -- <COMMANDE>` ou directement via l'exécutable dans `target/release/fib-bench`.

### Exemples courants

```bash
# Calculer le 50ème nombre de Fibonacci
cargo run --bin fib-bench -- calc -n 50

# Comparer tous les algorithmes pour n=1000
cargo run --bin fib-bench -- compare -n 1000

# Lancer un benchmark complet (Criterion)
cargo run --bin fib-bench -- bench
```

---

## 📖 Guide des commandes

L'outil principal `fib-bench` dispose de plusieurs sous-commandes :

### 1. `calc` - Calculateur simple

Calcule F(n) avec une méthode spécifique.

```bash
fib-bench calc -n <N> [OPTIONS]
```

**Options :**

- `-n, --number <N>` : Le nombre à calculer.
- `-m, --method <METHOD>` : Algorithme à utiliser (`iterative` (défaut), `recursive`, `matrix`, `fast_doubling`, `binet`).
- `-t, --time` : Affiche le temps d'exécution.

### 2. `compare` - Comparateur d'algorithmes

Compare les performances de tous les algorithmes pour un `n` donné.

```bash
fib-bench compare -n <N>
```

**Note :** Pour les grands `n`, l'algorithme récursif sera automatiquement désactivé pour éviter les temps d'attente infinis.

### 3. `bench` - Benchmarks rigoureux

Lance la suite de benchmarks Criterion pour des mesures statistiques précises.

```bash
fib-bench bench -f [FILTRE]
```

**Options :**
- `-f, --filter <nom>`: Filtre les benchmarks par nom.

Les rapports HTML sont générés dans `target/criterion/report/index.html`.

### 4. `info` - Informations techniques

Affiche les détails sur les algorithmes (complexité, description).

```bash
fib-bench info --method <all|nom>
```

### 5. `sequence` - Générateur de suite

Génère une séquence de nombres de Fibonacci.

```bash
fib-bench sequence --count 20 --start 0
```

### 6. `binet-analysis` - Analyse de précision

Analyse la précision de la formule de Binet (approximation flottante) par rapport au calcul entier exact.

```bash
fib-bench binet-analysis --max-n 100
```

### 7. `simd` - Démonstration SIMD

Démontre les gains de performance du traitement par lots avec SIMD (AVX2/AVX512).

```bash
# Calculer un lot de nombres
fib-bench simd --batch 10,100,1000

# Comparer avec la version scalaire
fib-bench simd --batch 10,100,1000 --compare

# Voir les infos SIMD
fib-bench simd --info
```

### 8. `compare-go` - Rust vs Go

Compare les performances de l'implémentation Rust face à une implémentation Go compilée (via FFI).

```bash
fib-bench compare-go -n 10000 --iterations 100
```

### 9. `report` - Génération de rapports

Génère le rapport HTML complet des benchmarks.

```bash
fib-bench report --input results --output results
```
*Note: Cette commande ne lance pas automatiquement le navigateur. Ouvrez le fichier `index.html` dans le dossier de sortie.*

### 10. `memory` - Analyse Mémoire

Analyse l'allocation mémoire pour différents algorithmes.

```bash
fib-bench memory -n 1000 --method matrix
```

---

## 🔍 Fonctionnalités avancées

### Profiling (Unix uniquement)

Utilisez `fib-profiler` pour générer des flamegraphs.

```bash
# Profiler l'algorithme itératif
cargo run --bin fib-profiler -- profile --method iterative -n 100000
```

Le fichier `.svg` sera généré dans le dossier courant.

### Visualisation

Utilisez `fib-viz` pour générer des graphiques comparatifs.

```bash
cargo run --bin fib-viz
```

Les graphiques interactifs HTML seront dans `results/reports/`.

---

## ❓ Dépannage

### Problème : Stack Overflow

**Cause** : Utilisation de l'algorithme récursif avec n > 100,000.
**Solution** : Utilisez `iterative` ou `matrix` pour les grands nombres, ou augmentez la taille de la pile (`RUST_MAX_STACK_SIZE`).

### Problème : Overflow u128

**Cause** : Calcul de F(n) pour n > 186.
**Solution** : Le type `u128` est limité à ~3.4e38. Pour de plus grands nombres, utilisez `fib_matrix_modulo` dans le code (pas exposé directement en CLI pour éviter la confusion).

### Problème : `compare-go` échoue

**Cause** : Go non installé ou `CGO_ENABLED` non configuré.
**Solution** : Installez Go et assurez-vous que `go build` fonctionne.

### Problème : SIMD non détecté

**Cause** : CPU incompatible ou compilation sans flags natifs.
**Solution** : Compilez avec `RUSTFLAGS="-C target-cpu=native" cargo build --release`.

---

<p align="center">
  <em>Besoin d'aide supplémentaire ? Ouvrez une <a href="https://github.com/agbru/FibBenchmark/issues">issue sur GitHub</a>.</em>
</p>
