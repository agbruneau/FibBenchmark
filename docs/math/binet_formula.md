# 🌟 La Formule de Binet

La formule de Binet est une expression en forme close pour calculer le n-ième nombre de Fibonacci sans récurrence.

## 📐 La formule

```
F(n) = (φⁿ - ψⁿ) / √5
```

où :
- **φ** = (1 + √5) / 2 ≈ 1.6180339887... (le **nombre d'or**)
- **ψ** = (1 - √5) / 2 ≈ -0.6180339887...

## 💡 Dérivation

### L'équation caractéristique

La récurrence F(n) = F(n-1) + F(n-2) suggère de chercher une solution de la forme F(n) = xⁿ.

En substituant :

```
xⁿ = xⁿ⁻¹ + xⁿ⁻²
x² = x + 1  (après division par xⁿ⁻²)
```

Cette équation a deux racines :

```
x = (1 ± √5) / 2
```

Ce sont φ et ψ.

### La solution générale

La solution générale est une combinaison linéaire :

```
F(n) = A × φⁿ + B × ψⁿ
```

### Conditions initiales

En utilisant F(0) = 0 et F(1) = 1 :

```
F(0) = A + B = 0           →  B = -A
F(1) = A×φ + B×ψ = 1       →  A×(φ - ψ) = 1
                           →  A = 1/(φ - ψ) = 1/√5
```

Donc A = 1/√5 et B = -1/√5, ce qui donne la formule de Binet.

## 🔢 Propriétés remarquables

### Propriétés de φ et ψ

```
φ × ψ = -1
φ + ψ = 1
φ - ψ = √5
φ² = φ + 1
```

### Simplification pour grands n

Puisque |ψ| < 1, ψⁿ → 0 rapidement. Pour n ≥ 1 :

```
F(n) ≈ φⁿ / √5
```

Plus précisément, F(n) est l'entier le plus proche de φⁿ/√5.

## 💻 Implémentation

### Version de base (f64)

```rust
pub fn fib_binet_f64(n: u64) -> f64 {
    if n == 0 { return 0.0; }
    
    let sqrt5 = 5.0_f64.sqrt();
    let phi = (1.0 + sqrt5) / 2.0;
    let psi = (1.0 - sqrt5) / 2.0;
    
    (phi.powi(n as i32) - psi.powi(n as i32)) / sqrt5
}
```

### Version arrondie

```rust
pub fn fib_binet_rounded(n: u64) -> u128 {
    fib_binet_f64(n).round() as u128
}
```

### Version simplifiée (approximation)

```rust
pub fn fib_binet_approx(n: u64) -> f64 {
    let sqrt5 = 5.0_f64.sqrt();
    let phi = (1.0 + sqrt5) / 2.0;
    (phi.powi(n as i32) / sqrt5).round()
}
```

## ⚠️ Limitations de précision

### Le problème IEEE 754

Les nombres flottants double précision (f64) ont environ 15-17 chiffres significatifs. Au-delà, les erreurs d'arrondi s'accumulent.

### Table de précision

| n | Exact | Binet f64 | Erreur |
|---|-------|-----------|--------|
| 70 | 190392490709135 | 190392490709135 | 0 |
| 75 | 2111485077978050 | 2111485077978050 | 0 |
| 78 | 8944394323791464 | 8944394323791464 | 0 |
| 79 | 14472334024676221 | 14472334024676220 | 1 |
| 80 | 23416728348467685 | 23416728348467744 | 59 |

### Pourquoi n ≤ 78 ?

À n = 78, F(78) ≈ 8.9 × 10¹⁵, ce qui est proche de la limite de précision f64. Au-delà, les erreurs deviennent significatives.

### Solutions pour grands n

1. **Utiliser la méthode matricielle** (recommandé)
2. **Bibliothèques BigDecimal** avec précision arbitraire
3. **Arithmétique symbolique**

## 📊 Analyse d'erreur

### Erreur relative

```rust
pub fn binet_error_analysis(n: u64) -> (f64, f64) {
    let approx = fib_binet_f64(n);
    let exact = fib_iterative(n) as f64;
    
    let abs_error = (approx - exact).abs();
    let rel_error = abs_error / exact;
    
    (abs_error, rel_error)
}
```

### Croissance de l'erreur

| n | Erreur relative |
|---|-----------------|
| 50 | ~10⁻¹⁵ |
| 60 | ~10⁻¹⁴ |
| 70 | ~10⁻¹² |
| 78 | ~10⁻¹⁰ |
| 80 | ~10⁻⁸ |

## 🌀 Le nombre d'or

### Définition géométrique

Le nombre d'or φ est le ratio a/b tel que :

```
a/b = (a+b)/a = φ
```

C'est la proportion "la plus harmonieuse" selon les anciens Grecs.

### Fraction continue

```
φ = 1 + 1/(1 + 1/(1 + 1/(1 + ...)))
```

La fraction continue la plus simple !

### Convergence de Fibonacci

Le ratio F(n+1)/F(n) converge vers φ :

| n | F(n+1)/F(n) |
|---|-------------|
| 1 | 1.0000 |
| 2 | 2.0000 |
| 5 | 1.6000 |
| 10 | 1.6176 |
| 20 | 1.6180339 |

## 🎯 Quand utiliser Binet ?

### ✅ Idéal pour

- Calculs rapides avec n ≤ 78
- Approximations où la précision exacte n'est pas critique
- Démonstrations mathématiques
- Vérification de résultats

### ❌ Éviter pour

- n > 78 (perte de précision)
- Applications nécessitant des résultats exacts
- Crypto/sécurité
- Calculs financiers

## 📚 Applications du nombre d'or

- **Architecture** : Parthénon, pyramides
- **Art** : Proportions de Léonard de Vinci
- **Nature** : Spirales de tournesols, coquillages
- **Finance** : Retracements de Fibonacci
- **Informatique** : Tas de Fibonacci, recherche

---

*Voir aussi : [matrix_method.md](matrix_method.md) pour la méthode exacte en O(log n).*
