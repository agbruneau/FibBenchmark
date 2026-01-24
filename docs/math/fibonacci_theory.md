# 📚 Théorie des Nombres de Fibonacci

Un aperçu complet de la théorie mathématique derrière les nombres de Fibonacci.

## 🌱 Origines historiques

### Leonardo de Pise (Fibonacci)

La suite a été introduite en Occident par **Leonardo de Pise** (1170-1250), surnommé Fibonacci ("fils de Bonaccio"), dans son livre **Liber Abaci** (1202).

### Le problème original

> "Un homme met un couple de lapins dans un lieu clos. Combien y aura-t-il de couples de lapins après un an, sachant que chaque couple produit un nouveau couple chaque mois, et que les couples deviennent fertiles après un mois ?"

```
Mois 1: 1 couple (immature)
Mois 2: 1 couple (mature)
Mois 3: 2 couples (1 mature + 1 nouveau)
Mois 4: 3 couples
Mois 5: 5 couples
...
```

## 📐 Définition formelle

### Définition par récurrence

```
F₀ = 0
F₁ = 1
Fₙ = Fₙ₋₁ + Fₙ₋₂  pour n ≥ 2
```

### Fonction génératrice

```
G(x) = Σ Fₙxⁿ = x / (1 - x - x²)
```

### Série exponentielle

```
Σ Fₙxⁿ/n! = (e^(φx) - e^(ψx)) / √5
```

## 🔗 Identités fondamentales

### Identités de Cassini et Vajda

```
Fₙ₋₁ × Fₙ₊₁ - Fₙ² = (-1)ⁿ           (Cassini)
Fₙ₊ᵢ × Fₙ₊ⱼ - Fₙ × Fₙ₊ᵢ₊ⱼ = (-1)ⁿFᵢFⱼ  (Vajda)
```

### Identités de somme

```
Σᵢ₌₀ⁿ Fᵢ = Fₙ₊₂ - 1                 (somme)
Σᵢ₌₀ⁿ Fᵢ² = Fₙ × Fₙ₊₁              (somme des carrés)
Σᵢ₌₀ⁿ F₂ᵢ₋₁ = F₂ₙ                   (somme des impairs)
Σᵢ₌₁ⁿ F₂ᵢ = F₂ₙ₊₁ - 1               (somme des pairs)
```

### Identités de doublement

```
F₂ₙ = Fₙ × (2Fₙ₊₁ - Fₙ)
F₂ₙ₊₁ = Fₙ² + Fₙ₊₁²
```

## 🔢 Divisibilité

### Propriété fondamentale

```
Fₘ | Fₘₙ  pour tout m, n ≥ 1
```

Autrement dit, Fₘ divise Fₘₙ.

### GCD des Fibonacci

```
gcd(Fₘ, Fₙ) = F_{gcd(m,n)}
```

**Exemple** : gcd(F₁₂, F₈) = gcd(144, 21) = 3 = F₄

### Fibonacci premiers

Les **Fibonacci premiers** sont les Fₙ qui sont premiers :

```
n:  3,  4,  5,  7, 11, 13, 17, 23, 29, 43, 47, 83, ...
Fₙ: 2,  3,  5, 13, 89, 233, 1597, 28657, ...
```

**Conjecture** : Il existe une infinité de Fibonacci premiers.

## 📊 Période de Pisano

### Définition

La suite Fₙ mod m est **périodique**. La période minimale est appelée **période de Pisano** π(m).

### Exemples

| m | π(m) | Cycle |
|---|------|-------|
| 2 | 3 | 0,1,1 |
| 3 | 8 | 0,1,1,2,0,2,2,1 |
| 5 | 20 | 0,1,1,2,3,0,3,3,1,4,0,4,4,3,2,0,2,2,4,1 |
| 10 | 60 | ... |

### Propriétés

1. π(p) divise p² - 1 pour p premier
2. π(p^k) = p^(k-1) × π(p)
3. π(mn) = lcm(π(m), π(n)) si gcd(m,n) = 1

## 🔄 Représentation de Zeckendorf

### Théorème de Zeckendorf

Tout entier positif peut être représenté de manière unique comme somme de nombres de Fibonacci non consécutifs.

### Exemples

```
100 = 89 + 8 + 3        = F₁₁ + F₆ + F₄
50 = 34 + 13 + 3        = F₉ + F₇ + F₄
17 = 13 + 3 + 1         = F₇ + F₄ + F₂
```

### Application : Codage de Fibonacci

Représentation binaire utilisant Zeckendorf, terminée par "11" comme délimiteur.

## 🌀 Spirale de Fibonacci

### Construction

1. Dessiner des carrés de côté Fₙ
2. Les disposer en spirale
3. Tracer des quarts de cercle dans chaque carré

### Approximation de la spirale d'or

La spirale de Fibonacci approxime la **spirale logarithmique** :

```
r = a × e^(bθ)
```

avec b = ln(φ) / (π/2) ≈ 0.306

## 📈 Croissance asymptotique

### Comportement pour grand n

```
Fₙ ~ φⁿ / √5
```

Plus précisément :

```
Fₙ = round(φⁿ / √5)  pour n ≥ 0
```

### Nombre de chiffres

```
digits(Fₙ) ≈ n × log₁₀(φ) + log₁₀(1/√5)
           ≈ 0.209n - 0.349
```

Donc F₁₀₀₀ a environ 209 chiffres.

## 🔗 Généralisations

### Nombres de Lucas

```
L₀ = 2, L₁ = 1
Lₙ = Lₙ₋₁ + Lₙ₋₂
```

Relation : Lₙ = Fₙ₋₁ + Fₙ₊₁

### k-Fibonacci

```
F^(k)_n = F^(k)_{n-1} + F^(k)_{n-2} + ... + F^(k)_{n-k}
```

Pour k=3, c'est la suite de **Tribonacci**.

### Fibonacci négatifs

On peut étendre aux n < 0 :

```
F₋ₙ = (-1)^(n+1) × Fₙ
```

Exemples : F₋₁ = 1, F₋₂ = -1, F₋₃ = 2, F₋₄ = -3

## 🎯 Applications en informatique

### Tas de Fibonacci

Structure de données avec amortissement optimal pour :
- `insert` : O(1)
- `decrease-key` : O(1) amorti
- `extract-min` : O(log n) amorti

### Recherche de Fibonacci

Alternative à la recherche dichotomique utilisant les ratios Fibonacci.

### Systèmes de numération

- Codage de Fibonacci pour compression
- Fibonacci dans les codes de correction d'erreur

## 📚 Références

1. Vorobiev, N.N. *Fibonacci Numbers* (2002)
2. Knuth, D.E. *The Art of Computer Programming, Vol. 1*
3. Graham, Knuth, Patashnik. *Concrete Mathematics*
4. Vajda, S. *Fibonacci and Lucas Numbers*

---

*"Les nombres de Fibonacci sont parmi les objets les plus fascinants des mathématiques."* — Mario Livio
