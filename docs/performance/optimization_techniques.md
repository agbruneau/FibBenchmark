# ⚡ Techniques d'Optimisation

Ce document détaille les techniques d'optimisation utilisées dans les différentes implémentations.

## 🎯 Optimisations du compilateur

### Profile Release

```toml
[profile.release]
lto = true           # Link-Time Optimization
codegen-units = 1    # Meilleure optimisation, compilation plus lente
opt-level = 3        # Optimisation maximale
```

### Inlining

```rust
#[inline]
fn fib_helper(a: u128, b: u128) -> u128 {
    a + b
}

#[inline(always)]  // Force l'inlining
fn critical_path() { ... }
```

## 🔄 Optimisations algorithmiques

### Branchless code

Éviter les branches conditionnelles dans les boucles critiques :

```rust
// Avec branches
fn fib_branched(n: u64) -> u128 {
    match n {
        0 => 0,
        1 => 1,
        _ => { /* loop */ }
    }
}

// Sans branches dans la boucle
fn fib_branchless(n: u64) -> u128 {
    let (mut a, mut b) = (0u128, 1u128);
    for _ in 0..n {
        let temp = a + b;
        a = b;
        b = temp;
    }
    a
}
```

### Loop unrolling

Dérouler les boucles pour réduire l'overhead :

```rust
// Boucle standard
for i in 0..n {
    process(i);
}

// Déroulée par 4
let mut i = 0;
while i + 4 <= n {
    process(i);
    process(i + 1);
    process(i + 2);
    process(i + 3);
    i += 4;
}
while i < n {
    process(i);
    i += 1;
}
```

## 💾 Optimisations mémoire

### Éviter les allocations

```rust
// Mauvais: allocation à chaque appel
fn fib_memo(n: u64) -> u128 {
    let mut memo = vec![0u128; (n + 1) as usize];
    // ...
}

// Bon: réutiliser un buffer
struct FibCalculator {
    buffer: Vec<u128>,
}

impl FibCalculator {
    fn calculate(&mut self, n: u64) -> u128 {
        if self.buffer.len() <= n as usize {
            self.buffer.resize((n + 1) as usize, 0);
        }
        // ...
    }
}
```

### Cache-friendly access

```rust
// Bon: accès séquentiel (cache-friendly)
for i in 0..array.len() {
    process(array[i]);
}

// Mauvais: accès aléatoire (cache misses)
for i in random_indices {
    process(array[i]);
}
```

## 🧮 Optimisations arithmétiques

### Wrapping operations

Pour éviter les checks d'overflow en mode debug :

```rust
// Avec checks (plus lent en debug)
let result = a + b;

// Sans checks
let result = a.wrapping_add(b);
```

### Bit manipulation

```rust
// Division par 2
let half = n / 2;
let half_fast = n >> 1;  // Plus rapide

// Modulo 2
let is_odd = n % 2 == 1;
let is_odd_fast = n & 1 == 1;  // Plus rapide
```

### Éviter les opérations coûteuses

```rust
// Coûteux: division
let result = n / 3;

// Optimisation possible pour constantes connues
// Le compilateur fait souvent ça automatiquement
```

## 🔧 Profiling et mesure

### Flamegraphs

```bash
# Installation
cargo install flamegraph

# Génération
cargo flamegraph --bin fib-bench -- calc -n 10000 -m matrix
```

### Criterion Profiling

```bash
cargo bench -- --profile-time 5
```

### Analyse assembleur

```bash
cargo rustc --release -- --emit asm
# ou
cargo asm fib_core::iterative::fib_iterative
```

## 📊 Résultats typiques

| Optimisation | Impact |
|--------------|--------|
| LTO | +10-20% |
| Branchless | +5-15% |
| Inlining | +10-30% |
| Cache align | +5-10% |

## ⚠️ Pièges courants

### Over-optimization

```rust
// Trop optimisé, illisible
fn fib(n:u64)->u128{let(mut a,mut b)=(0u128,1u128);(0..n).for_each(|_|{let t=a+b;a=b;b=t;});a}

// Préférez la lisibilité avec de bonnes performances
fn fib_iterative(n: u64) -> u128 {
    let (mut a, mut b) = (0u128, 1u128);
    for _ in 0..n {
        let temp = a + b;
        a = b;
        b = temp;
    }
    a
}
```

### Micro-benchmarks trompeurs

- Toujours tester avec des données réalistes
- Inclure le warm-up
- Mesurer plusieurs fois

---

*L'optimisation prématurée est la racine de tous les maux.* — Donald Knuth
