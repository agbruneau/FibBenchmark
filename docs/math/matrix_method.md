# 🔢 La Méthode Matricielle pour Fibonacci

Ce document explique en détail la méthode matricielle pour calculer les nombres de Fibonacci en O(log n).

## 💡 L'idée clé

La suite de Fibonacci peut être représentée comme une transformation linéaire. Si on définit un vecteur d'état :

```
État(n) = [F(n+1), F(n)]ᵀ
```

Alors la transition vers l'état suivant est :

```
État(n+1) = M × État(n)
```

où M est la **matrice de Fibonacci** :

```
M = ┌       ┐
    │ 1  1  │
    │ 1  0  │
    └       ┘
```

## 📐 Démonstration

### Pourquoi ça fonctionne

Calculons M × [F(n+1), F(n)]ᵀ :

```
┌       ┐   ┌      ┐   ┌ 1×F(n+1) + 1×F(n) ┐   ┌ F(n+2) ┐
│ 1  1  │ × │F(n+1)│ = │                   │ = │        │
│ 1  0  │   │ F(n) │   │ 1×F(n+1) + 0×F(n) │   │ F(n+1) │
└       ┘   └      ┘   └                   ┘   └        ┘
```

Par récurrence, on obtient F(n+1) = F(n) + F(n-1), ce qui est exactement la définition de Fibonacci !

### L'identité matricielle

En appliquant la transformation n fois à partir de l'état initial [1, 0]ᵀ :

```
Mⁿ × [1, 0]ᵀ = [F(n+1), F(n)]ᵀ
```

Ou de manière équivalente :

```
Mⁿ = ┌              ┐
     │ F(n+1)  F(n) │
     │ F(n)  F(n-1) │
     └              ┘
```

## ⚡ Exponentiation rapide

Le calcul naïf de Mⁿ nécessite n-1 multiplications (O(n)). Mais on peut faire mieux avec l'**exponentiation par carrés**.

### Principe

L'observation clé est que :

```
M¹⁶ = M⁸ × M⁸
M⁸ = M⁴ × M⁴
M⁴ = M² × M²
M² = M × M
```

Ainsi, M¹⁶ ne nécessite que 4 multiplications au lieu de 15 !

### Pour n quelconque

On décompose n en binaire. Par exemple, n = 13 = 1101₂ :

```
M¹³ = M⁸ × M⁴ × M¹
    = M^(1000₂) × M^(0100₂) × M^(0001₂)
```

Cela nécessite seulement O(log n) multiplications.

## 💻 Implémentation

### Structure de données

```rust
#[derive(Clone, Copy)]
struct Matrix2x2 {
    data: [[u128; 2]; 2]
}

impl Matrix2x2 {
    fn identity() -> Self {
        Self { data: [[1, 0], [0, 1]] }
    }
    
    fn fibonacci_base() -> Self {
        Self { data: [[1, 1], [1, 0]] }
    }
}
```

### Multiplication matricielle

```rust
impl Mul for Matrix2x2 {
    type Output = Self;
    
    fn mul(self, other: Self) -> Self {
        let a = self.data;
        let b = other.data;
        
        Matrix2x2 { data: [
            [
                a[0][0] * b[0][0] + a[0][1] * b[1][0],
                a[0][0] * b[0][1] + a[0][1] * b[1][1],
            ],
            [
                a[1][0] * b[0][0] + a[1][1] * b[1][0],
                a[1][0] * b[0][1] + a[1][1] * b[1][1],
            ],
        ]}
    }
}
```

### Exponentiation rapide

```rust
fn fib_matrix_fast(mut n: u64) -> u128 {
    if n == 0 { return 0; }
    
    let mut result = Matrix2x2::identity();
    let mut base = Matrix2x2::fibonacci_base();
    
    while n > 0 {
        if n % 2 == 1 {
            result = result * base;
        }
        base = base * base;
        n /= 2;
    }
    
    result.data[0][1]  // F(n)
}
```

## 📊 Analyse de complexité

### Nombre d'opérations

- **Multiplications matricielles** : ⌈log₂(n)⌉
- **Multiplications scalaires par matmul** : 8 (pour 2×2)
- **Additions par matmul** : 4

Total : O(log n) opérations

### Comparaison

| n | Itératif (ops) | Matriciel (ops) |
|---|---------------|-----------------|
| 10 | 10 | 4 |
| 100 | 100 | 7 |
| 1000 | 1000 | 10 |
| 10000 | 10000 | 14 |
| 1000000 | 1000000 | 20 |

## 🔧 Variantes

### Avec modulo

Pour éviter les overflows sur de très grands n :

```rust
fn fib_matrix_modulo(n: u64, modulo: u128) -> u128 {
    fn mul_mod(a: [[u128; 2]; 2], b: [[u128; 2]; 2], m: u128) -> [[u128; 2]; 2] {
        [
            [
                (a[0][0] * b[0][0] % m + a[0][1] * b[1][0] % m) % m,
                (a[0][0] * b[0][1] % m + a[0][1] * b[1][1] % m) % m,
            ],
            [
                (a[1][0] * b[0][0] % m + a[1][1] * b[1][0] % m) % m,
                (a[1][0] * b[0][1] % m + a[1][1] * b[1][1] % m) % m,
            ],
        ]
    }
    
    // ... exponentiation avec mul_mod
}
```

### Méthode de doublement

Une alternative utilisant les identités :

```
F(2k) = F(k) × (2×F(k+1) - F(k))
F(2k+1) = F(k)² + F(k+1)²
```

```rust
fn fib_doubling(n: u64) -> u128 {
    fn fib_pair(n: u64) -> (u128, u128) {
        if n == 0 { return (0, 1); }
        
        let (f_k, f_k1) = fib_pair(n / 2);
        let f_2k = f_k * (2 * f_k1 - f_k);
        let f_2k1 = f_k * f_k + f_k1 * f_k1;
        
        if n % 2 == 0 {
            (f_2k, f_2k1)
        } else {
            (f_2k1, f_2k + f_2k1)
        }
    }
    
    fib_pair(n).0
}
```

## 🎯 Quand utiliser cette méthode ?

### ✅ Idéal pour

- Grands n (> 100)
- Calcul unique (pas de requêtes répétées)
- Contraintes de temps strictes
- Calcul modulo (cryptographie)

### ❌ Moins adapté pour

- Très petits n (overhead de la multiplication)
- Calculs séquentiels F(1), F(2), ... F(n)
- Quand la simplicité du code prime

## 📚 Pour aller plus loin

- **Généralisation** : Cette technique s'applique à toute récurrence linéaire
- **Lucas numbers** : L(n) utilise la même matrice
- **Tribonacci** : Extension à une matrice 3×3

---

*Voir aussi : [binet_formula.md](binet_formula.md) pour l'approche en forme close.*
