# 💾 Analyse Mémoire des Algorithmes Fibonacci

Ce document détaille les profils mémoire de chaque algorithme Fibonacci implémenté.

## 📊 Comparaison Globale

| Algorithme      | Heap   | Stack      | Complexité Spatiale | Stack Safe |
| --------------- | ------ | ---------- | ------------------- | ---------- |
| **Iterative**   | 0 B    | 32 B       | O(1)                | ✅         |
| **Branchless**  | 0 B    | 32 B       | O(1)                | ✅         |
| **Matrix**      | 0 B    | 64 B       | O(log n)            | ✅         |
| **Doubling**    | 0 B    | 48 B       | O(log n)            | ✅         |
| **Binet**       | 0 B    | 24 B       | O(1)                | ✅         |
| **Memoization** | n×16 B | 0.8 KB     | O(n)                | ✅         |
| **Recursive**   | 0 B    | ~1 KB/call | O(n)                | ❌         |

---

## 🔍 Analyse Détaillée par Algorithme

### Iterative & Branchless - O(1) Mémoire

```
┌─────────────────────────────┐
│  STACK (32 bytes)           │
├─────────────────────────────┤
│  a: u128       (16 bytes)   │
│  b: u128       (16 bytes)   │
└─────────────────────────────┘
```

**Avantages:**

- Aucune allocation heap
- Empreinte mémoire constante
- Pas de risque de stack overflow

**Utilisation:**

```rust
pub fn fib_iterative(n: u64) -> u128 {
    let (mut a, mut b) = (0u128, 1u128);
    for _ in 0..n {
        (a, b) = (b, a + b);
    }
    a
}
```

---

### Matrix - O(log n) Stack

```
┌─────────────────────────────┐
│  STACK (~64 bytes + appels) │
├─────────────────────────────┤
│  matrix: [4 × u128]  (64 B) │
│  Frames récursifs (log n)   │
└─────────────────────────────┘
```

**Profil stack pour différents n:**

| n       | Frames | Stack Utilisé |
| ------- | ------ | ------------- |
| 100     | 7      | ~450 B        |
| 1,000   | 10     | ~640 B        |
| 10,000  | 14     | ~900 B        |
| 100,000 | 17     | ~1.1 KB       |

---

### Recursive Naïf - ⚠️ Danger Stack Overflow

```
┌─────────────────────────────┐
│  STACK (croissance O(n))    │
├─────────────────────────────┤
│  Frame 1: ret_addr, args    │
│  Frame 2: ret_addr, args    │
│  ...                        │
│  Frame n: ret_addr, args    │
└─────────────────────────────┘
```

**Limites de stack:**

| n       | Stack Estimé | Risque       |
| ------- | ------------ | ------------ |
| 30      | ~30 KB       | 🟢 Safe      |
| 1,000   | ~1 MB        | 🟡 Attention |
| 10,000  | ~10 MB       | 🔴 Overflow  |
| 100,000 | ~100 MB      | 💀 Crash     |

> ⚠️ **Ne pas utiliser** pour n > 30 en production.

---

### Memoization - O(n) Heap

```
┌─────────────────────────────┐
│  HEAP (Vec<u128>)           │
├─────────────────────────────┤
│  [0] = 0                    │
│  [1] = 1                    │
│  [2] = 1                    │
│  ...                        │
│  [n] = F(n)                 │
└─────────────────────────────┘
     Total: n × 16 bytes
```

**Utilisation mémoire par n:**

| n         | Heap   | Overhead      |
| --------- | ------ | ------------- |
| 100       | 1.6 KB | +1 KB headers |
| 1,000     | 16 KB  | +1 KB headers |
| 10,000    | 160 KB | +1 KB headers |
| 1,000,000 | 16 MB  | +1 KB headers |

---

## 📈 Allocation Patterns

### Pattern Idéal: Zero Allocation

```rust
// ✅ Pas d'allocation - utilisation directe de registres/stack
pub fn fib_matrix_fast(n: u64) -> u128 {
    // Matrices sur le stack uniquement
}
```

### Pattern À Éviter: Allocation Par Appel

```rust
// ❌ Allocation à chaque appel
fn fib_memo(n: u64) -> u128 {
    let mut memo = vec![0u128; (n + 1) as usize];  // Allocation!
    // ...
}

// ✅ Réutilisation de buffer
struct FibCache {
    cache: Vec<u128>,
}

impl FibCache {
    fn get(&mut self, n: u64) -> u128 {
        // Réutilise le buffer existant
    }
}
```

---

## 🧮 Recommandations par Cas d'Usage

| Cas d'Usage               | Algorithme  | Raison                             |
| ------------------------- | ----------- | ---------------------------------- |
| Embedded/microcontrôleur  | Iterative   | Mémoire O(1) minimale              |
| Serveur haute performance | Matrix      | O(log n) temps, O(1) heap          |
| Calculs répétitifs        | Memoization | Amortissement O(1) après précalcul |
| Enseignement              | Recursive   | Lisibilité (avec avertissement)    |
| n ≤ 78, approximation     | Binet       | O(1) temps et mémoire              |

---

## 📊 Outils de Profiling Mémoire

### Valgrind Massif (Linux)

```bash
valgrind --tool=massif ./target/release/fib-bench calc -n 10000 -m matrix
ms_print massif.out.*
```

### Heaptrack (Linux)

```bash
heaptrack ./target/release/fib-bench calc -n 10000 -m memo
heaptrack_gui heaptrack.*.gz
```

### Windows Performance Analyzer

Pour Windows, utiliser:

- Dr. Memory
- Visual Studio Memory Profiler
- Windows Performance Analyzer (WPA)

---

## 📝 Notes Importantes

1. **u128 vs BigInt**: Au-delà de F(186), utiliser `num-bigint` augmente significativement la mémoire
2. **Cache L1/L2**: Les algorithmes O(1) tiennent dans le cache L1 (typiquement 32 KB)
3. **Alignment**: Les `u128` sont alignés sur 16 bytes pour performance optimale

---

_Dernière mise à jour: Janvier 2026_
