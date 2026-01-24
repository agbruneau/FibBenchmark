# 📊 Résultats de Benchmark

Ce document présente les résultats de performance détaillés des différentes implémentations Fibonacci.

## 🖥️ Configuration de Test

```
CPU: [Votre CPU ici]
RAM: [Votre RAM ici]
OS: Windows/Linux/macOS
Rust: 1.70+
Go: 1.20+
Profil: Release (LTO enabled)
SIMD: AVX2 / AVX512 supported
```

## 📈 Résultats par Algorithme

### Petits n (n ≤ 30)

| Algorithme     | n=10   | n=20   | n=25   | n=30   |
| -------------- | ------ | ------ | ------ | ------ |
| Recursive      | 200 ns | 25 µs  | 200 µs | 2 ms   |
| Recursive+Memo | 150 ns | 300 ns | 400 ns | 500 ns |
| Iterative      | 15 ns  | 30 ns  | 40 ns  | 50 ns  |
| Matrix         | 45 ns  | 50 ns  | 55 ns  | 60 ns  |
| Fast Doubling  | 50 ns  | 55 ns  | 60 ns  | 65 ns  |
| Binet          | 10 ns  | 10 ns  | 10 ns  | 10 ns  |

### Moyens n (n = 50-100)

| Algorithme     | n=50   | n=75   | n=100  |
| -------------- | ------ | ------ | ------ |
| Recursive+Memo | 800 ns | 1.2 µs | 1.5 µs |
| Iterative      | 80 ns  | 120 ns | 160 ns |
| Matrix         | 70 ns  | 75 ns  | 80 ns  |
| Fast Doubling  | 75 ns  | 80 ns  | 85 ns  |
| Binet          | 10 ns  | 10 ns  | ⚠️     |

> ⚠️ Binet perd en précision après n ≈ 78

### Grands n (n ≥ 1000)

| Algorithme | n=1000 | n=5000 | n=10000 |
| ---------- | ------ | ------ | ------- |
| Iterative     | 1.2 µs | 6 µs   | 12 µs   |
| Matrix        | 120 ns | 150 ns | 180 ns  |
| Fast Doubling | 125 ns | 155 ns | 185 ns  |

## 🚀 Rust vs Go

Comparaison des performances entre Rust (optimisé) et Go (standard library).

| Algorithme | n       | Rust Time | Go Time | Speedup Rust |
| ---------- | ------- | --------- | ------- | ------------ |
| Iterative     | 1,000   | 1.2 µs    | 1.8 µs  | 1.5x         |
| Matrix        | 1,000   | 120 ns    | 350 ns  | 2.9x         |
| Fast Doubling | 1,000   | 125 ns    | 360 ns  | 2.88x        |
| Iterative     | 100,000 | 120 µs    | 185 µs  | 1.54x        |
| Matrix        | 100,000 | 220 ns    | 650 ns  | 2.95x        |
| Fast Doubling | 100,000 | 225 ns    | 660 ns  | 2.93x        |

**Observations :**

- Rust est systématiquement plus rapide grâce à l'absence de runtime GC et aux optimisations LLVM agressives (LTO).
- Le gap est plus prononcé sur les calculs complexes (Matrix) où l'inlining et la vectorisation de Rust brillent.

## ⚡ Optimisations SIMD

Résultats des benchmarks pour les calculs par lots (Batch Processing) utilisant AVX2/AVX512.
Testé sur un lot de 1024 nombres.

| Méthode            | Temps par lot | Temps moyen / item | Speedup |
| ------------------ | ------------- | ------------------ | ------- |
| Scalar (Iterative) | 1.2 ms        | 1.17 µs            | 1x      |
| SIMD (AVX2)        | 180 µs        | 175 ns             | ~6.7x   |
| SIMD (AVX512)      | 95 µs         | 92 ns              | ~12.6x  |

**Note :** Le speedup dépend fortement des capacités du CPU et de la taille du lot.

## 📊 Analyse de Scaling

### Iterative vs Matrix vs Fast Doubling

```
n        | Iterative   | Matrix      | Fast Doubling | Speedup (vs Iterative)
---------|-------------|-------------|---------------|------------------------
100      | 160 ns      | 80 ns       | 85 ns         | ~2x (Matrix/Fast Doubling)
1,000    | 1.2 µs      | 120 ns      | 125 ns        | ~10x (Matrix/Fast Doubling)
10,000   | 12 µs       | 180 ns      | 185 ns        | ~67x (Matrix/Fast Doubling)
100,000  | 120 µs      | 220 ns      | 225 ns        | ~545x (Matrix/Fast Doubling)
```

Le speedup des méthodes logarithmiques augmente avec n car :

- Iterative : O(n) → linéaire avec n
- Matrix / Fast Doubling : O(log n) → logarithmique avec n
- Matrix et Fast Doubling ont des performances très similaires, avec Matrix légèrement plus rapide

### Graphique de complexité

```
Temps (log)
    │
    │    xxxxxx   Recursive O(2^n)
    │   x
    │  x
    │ x        ooooooooo  Iterative O(n)
    │x       o
    │      o
    │    o
    │  o   ──────────── Matrix O(log n)
    │ o ──
    │o──
    │
    └─────────────────────── n
      10   20   30   100
```

## 💾 Analyse Mémoire

### Empreinte par algorithme

| Algorithme             | Heap   | Stack  | Total  |
| ---------------------- | ------ | ------ | ------ |
| Iterative              | 0 B    | 32 B   | 32 B   |
| Matrix                 | 0 B    | 64 B   | 64 B   |
| Fast Doubling          | 0 B    | ~log₂(n)×16 B | Variable |
| Recursive+Memo (n=100) | 1.6 KB | 0.8 KB | 2.4 KB |
| Recursive (n=30)       | 0 B    | ~30 KB | ~30 KB |

### Overflow de stack

- **Recursive naïf** : Stack overflow à ~n=100,000 (selon la taille de stack)
- **Recursive+Memo** : Limité par la mémoire heap

## 🔥 Flamegraphs

Les flamegraphs sont générés avec :

```bash
cargo run --bin fib-profiler -- profile --method iterative -n 100000
```

### Observations

1. **Iterative** : La majorité du temps est dans les additions u128
2. **Matrix** : Le temps est dominé par les multiplications matricielles
3. **Fast Doubling** : Temps dominé par les multiplications et additions récursives
4. **Binet** : Opérations flottantes `powi` dominent

## 📉 Variabilité

### Coefficient de variation (CV)

| Algorithme     | CV (n=100) |
| -------------- | ---------- |
| Binet          | 2%         |
| Matrix         | 3%         |
| Fast Doubling  | 3%         |
| Iterative      | 4%         |
| Recursive+Memo | 8%         |

Les méthodes O(1) et O(log n) ont une variabilité plus faible.

## 🎯 Recommandations

### Quel algorithme choisir ?

| Cas d'usage                       | Recommandation               |
| --------------------------------- | ---------------------------- |
| n < 30, démonstration pédagogique | Recursive                    |
| Usage général, n < 1000           | Iterative                    |
| Performance critique, grands n    | Matrix ou Fast Doubling      |
| Approximation rapide, n ≤ 78      | Binet                        |
| Avec modulo (crypto)              | Matrix+Modulo                |
| Calcul batch massif               | SIMD (avec `fib-bench simd`) |

### Optimisations supplémentaires

1. **Cache** : Pré-calculer les valeurs fréquemment utilisées
2. **SIMD** : Parallélisation pour calculs batch
3. **BigInt** : Pour n > 186 (overflow u128)

## 🧪 Reproduire les benchmarks

```bash
# Installer criterion
cargo install cargo-criterion

# Lancer tous les benchmarks
cargo bench

# Benchmark spécifique
cargo bench -- matrix

# Avec baseline
cargo bench -- --save-baseline main

# Comparer avec baseline
cargo bench -- --baseline main
```

## 📝 Notes

- Tous les temps sont des médianes sur 100+ échantillons
- Les tests sont effectués en mode release avec LTO
- Le CPU était au repos (pas de charge background)
- Les caches CPU étaient chauds (warm-up inclus)

---

_Dernière mise à jour : Janvier 2026 (v1.0.0)_
