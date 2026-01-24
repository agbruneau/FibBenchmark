# 📐 Fibonacci : Analyse Mathématique & Algorithmes

Ce document présente la théorie mathématique derrière les nombres de Fibonacci et les différents algorithmes pour les calculer.

## 📖 Table des matières

1. [Définition](#1-définition)
2. [Propriétés fondamentales](#2-propriétés-fondamentales)
3. [Complexité algorithmique](#3-complexité-algorithmique)
4. [Méthode matricielle](#4-méthode-matricielle)
5. [Formule de Binet](#5-formule-de-binet)
6. [Propriétés avancées](#6-propriétés-avancées)
7. [Applications](#7-applications)

---

## 1. Définition

La suite de Fibonacci est définie par la relation de récurrence :

```
F(0) = 0
F(1) = 1
F(n) = F(n-1) + F(n-2)  pour n ≥ 2
```

Les premiers termes sont :

```
n:    0  1  2  3  4  5  6   7   8   9  10  11   12   13   14   15
F(n): 0  1  1  2  3  5  8  13  21  34  55  89  144  233  377  610
```

## 2. Propriétés fondamentales

### 2.1 Croissance exponentielle

La suite de Fibonacci croît exponentiellement :

```
F(n) ≈ φⁿ / √5
```

où φ = (1 + √5) / 2 ≈ 1.618... est le **nombre d'or** (golden ratio).

### 2.2 Ratio consécutif

Le ratio de deux termes consécutifs converge vers φ :

```
lim(n→∞) F(n+1) / F(n) = φ
```

| n | F(n+1)/F(n) | Erreur vs φ |
|---|-------------|-------------|
| 5 | 1.6000 | 0.0180 |
| 10 | 1.6176 | 0.0004 |
| 20 | 1.6180339 | 0.0000001 |

### 2.3 Formule de Cassini

```
F(n-1) × F(n+1) - F(n)² = (-1)ⁿ
```

Cette identité montre une propriété remarquable des carrés de Fibonacci.

## 3. Complexité algorithmique

### 3.1 Tableau comparatif

| Algorithme | Temps | Espace | Opérations pour n=50 |
|------------|-------|--------|---------------------|
| Récursif naïf | O(2ⁿ) | O(n) | ~10¹⁵ |
| Mémorisation | O(n) | O(n) | 50 |
| Itératif | O(n) | O(1) | 50 |
| Matriciel | O(log n) | O(1) | 6 |
| Binet | O(1) | O(1) | 1 |

### 3.2 Récursif naïf - Arbre d'appel

L'algorithme récursif naïf génère un arbre d'appels exponentiel :

```
                    F(6)
                   /    \
                F(5)     F(4)
               /    \   /    \
            F(4)  F(3) F(3) F(2)
           /   \
        F(3)  F(2)
        ...   ...
```

Le nombre d'appels pour calculer F(n) est environ F(n+1), ce qui donne une complexité O(φⁿ) ≈ O(1.618ⁿ).

### 3.3 Itératif - Approche optimale simple

```rust
fn fib_iterative(n: u64) -> u128 {
    let (mut a, mut b) = (0, 1);
    for _ in 0..n {
        let temp = a + b;
        a = b;
        b = temp;
    }
    a
}
```

- **n additions** exactement
- Espace constant (2 variables)

## 4. Méthode matricielle

### 4.1 L'identité matricielle

La propriété clé est :

```
┌         ┐ⁿ     ┌              ┐
│  1   1  │   =  │ F(n+1)  F(n) │
│  1   0  │      │ F(n)  F(n-1) │
└         ┘      └              ┘
```

### 4.2 Preuve par induction

**Cas de base** (n=1) :

```
┌       ┐¹   ┌       ┐   ┌            ┐
│ 1  1  │  = │ 1  1  │ = │ F(2)  F(1) │
│ 1  0  │    │ 1  0  │   │ F(1)  F(0) │
└       ┘    └       ┘   └            ┘
```

**Étape inductive** :

Si la propriété est vraie pour n, alors pour n+1 :

```
┌       ┐ⁿ⁺¹   ┌              ┐   ┌       ┐
│ 1  1  │    = │ F(n+1)  F(n) │ × │ 1  1  │
│ 1  0  │      │ F(n)  F(n-1) │   │ 1  0  │
└       ┘      └              ┘   └       ┘

              ┌                          ┐
            = │ F(n+1)+F(n)     F(n+1)   │
              │ F(n)+F(n-1)       F(n)   │
              └                          ┘

              ┌                  ┐
            = │ F(n+2)    F(n+1) │
              │ F(n+1)      F(n) │
              └                  ┘
```

### 4.3 Exponentiation rapide

L'idée est d'utiliser l'exponentiation par carrés successifs :

```
M^13 = M^8 × M^4 × M^1     (13 = 1101 en binaire)
```

Cela réduit le nombre de multiplications de O(n) à O(log n).

```rust
fn matrix_power(mut n: u64) -> Matrix2x2 {
    let mut result = Matrix2x2::identity();
    let mut base = Matrix2x2::fibonacci_base();
    
    while n > 0 {
        if n % 2 == 1 {
            result = result * base;
        }
        base = base * base;
        n /= 2;
    }
    result
}
```

## 5. Formule de Binet

### 5.1 Définition

```
F(n) = (φⁿ - ψⁿ) / √5
```

où :
- φ = (1 + √5) / 2 ≈ 1.6180339887... (nombre d'or)
- ψ = (1 - √5) / 2 ≈ -0.6180339887...

### 5.2 Dérivation

Les racines de l'équation caractéristique x² = x + 1 sont φ et ψ.

La solution générale de la récurrence est :
```
F(n) = A × φⁿ + B × ψⁿ
```

En utilisant F(0) = 0 et F(1) = 1, on trouve A = 1/√5 et B = -1/√5.

### 5.3 Simplification pour grands n

Puisque |ψ| < 1, ψⁿ → 0 quand n → ∞.

Pour n ≥ 1 :
```
F(n) = round(φⁿ / √5)
```

### 5.4 Limites de précision

| n | Exact F(n) | Binet f64 | Erreur |
|---|------------|-----------|--------|
| 70 | 190392490709135 | 190392490709135 | 0 |
| 75 | 2111485077978050 | 2111485077978050 | 0 |
| 80 | 23416728348467685 | 23416728348467744 | 59 |

La précision IEEE 754 double (f64) limite la formule à n ≤ 78 environ.

## 6. Propriétés avancées

### 6.1 Identité GCD

```
gcd(F(m), F(n)) = F(gcd(m, n))
```

Exemple : gcd(F(12), F(8)) = gcd(144, 21) = 3 = F(4) = F(gcd(12, 8))

### 6.2 Algorithme Fast Doubling

L'algorithme Fast Doubling utilise les identités de doublement pour calculer F(n) en O(log n).

#### Identités de doublement

```
F(2n) = F(n) × (2×F(n+1) - F(n))
F(2n+1) = F(n)² + F(n+1)²
```

#### Preuve

Ces identités peuvent être dérivées des propriétés matricielles :

Pour n pair (n = 2k) :
```
F(2k) = F(k) × (F(k+1) + F(k-1))
      = F(k) × (F(k+1) + (F(k+1) - F(k)))
      = F(k) × (2×F(k+1) - F(k))
```

Pour n impair (n = 2k+1) :
```
F(2k+1) = F(k+1)² + F(k)²
```

#### Algorithme récursif

L'algorithme Fast Doubling calcule récursivement F(n) et F(n+1) en utilisant les identités de doublement :

```rust
fn fib_pair(n: u64) -> (u128, u128) {
    if n == 0 {
        return (0, 1);  // (F(0), F(1))
    }
    
    let (f_k, f_k1) = fib_pair(n / 2);
    let f_2k = f_k * (2 * f_k1 - f_k);
    let f_2k1 = f_k * f_k + f_k1 * f_k1;
    
    if n % 2 == 0 {
        (f_2k, f_2k1)      // (F(2k), F(2k+1))
    } else {
        (f_2k1, f_2k + f_2k1)  // (F(2k+1), F(2k+2))
    }
}

fn fib_doubling(n: u64) -> u128 {
    if n == 0 {
        return 0;
    }
    fib_pair(n).0
}
```

#### Complexité

- **Temps** : O(log n) - la profondeur de récursion est O(log n)
- **Espace** : O(log n) - due à la pile d'appels récursifs

#### Comparaison avec la méthode matricielle

Les deux méthodes ont une complexité temporelle O(log n), mais :

- **Matrix** : O(1) espace, multiplication de matrices 2×2
- **Fast Doubling** : O(log n) espace (récursion), opérations arithmétiques directes

En pratique, les performances sont très similaires, avec Matrix légèrement plus rapide pour de très grands n.

### 6.3 Divisibilité

- F(3) = 2 divise F(3k) pour tout k
- F(4) = 3 divise F(4k) pour tout k
- Plus généralement : F(m) divise F(mn)

### 6.4 Période de Pisano

F(n) mod m est périodique. La période est appelée **période de Pisano** π(m).

| m | π(m) |
|---|------|
| 2 | 3 |
| 3 | 8 |
| 5 | 20 |
| 10 | 60 |
| 1000000007 | 2000000016 |

## 7. Applications

### 7.1 En informatique

- **Tas de Fibonacci** - structures de données avec amortissement
- **Recherche Fibonacci** - algorithme de recherche similaire à dichotomique
- **Compression** - codage de Fibonacci pour entiers

### 7.2 En mathématiques

- **Théorie des nombres** - tests de primalité
- **Combinatoire** - comptage de chemins et pavages
- **Algèbre linéaire** - théorie spectrale

### 7.3 Dans la nature

- Phyllotaxie (arrangement des feuilles)
- Spirales de coquillages
- Proportions artistiques (rectangle d'or)

---

## Références

1. Knuth, D.E. (1997). *The Art of Computer Programming, Vol. 1*
2. Graham, R.L., Knuth, D.E., Patashnik, O. (1994). *Concrete Mathematics*
3. Vorobiev, N.N. (2002). *Fibonacci Numbers*

---

<p align="center">
  <em>« La suite de Fibonacci est l'une des créations les plus élégantes des mathématiques. »</em>
</p>
