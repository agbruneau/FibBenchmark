# Dashboard Fibonacci Benchmark

Dashboard web interactif pour visualiser les résultats des benchmarks Fibonacci.

## Fonctionnalites

### Design

- **Dark Gradient Theme** : Design moderne avec gradients violet/emeraude
- **Mode Sombre/Clair** : Toggle avec persistance localStorage
- **Detection Systeme** : Respect de `prefers-color-scheme`
- **Responsive** : Adapte pour mobile, tablette et desktop
- **Animations** : Effets de scroll reveal et count-up

### Graphiques Interactifs

Tous les graphiques supportent :

- **Zoom** : Molette de souris ou pinch sur mobile
- **Pan** : Glisser pour naviguer
- **Reset** : Bouton pour reinitialiser la vue
- **Tooltips** : Informations detaillees au survol
- **Legende Interactive** : Cliquer pour masquer/afficher les series

### Sections

1. **Hero** : Presentation avec badge de date et statistiques
2. **Metriques** : Bento grid avec 4 KPI animes
3. **Algorithmes** : Tableau comparatif avec badges de complexite
4. **Complexite** : Graphique Iteratif vs Matrix
5. **Binet** : Analyse de precision avec echelle logarithmique
6. **Nombre d'Or** : Convergence vers phi

## Installation

Aucune installation requise. Le dashboard est entierement statique.

### Visualisation Locale

```bash
# Option 1 : Ouvrir directement (peut avoir des restrictions CORS)
start index.html  # Windows
open index.html   # macOS

# Option 2 : Serveur local (recommande)
python -m http.server 8080
# Ouvrir http://localhost:8080
```

### Avec Node.js

```bash
npx serve .
```

## Structure

```
dashboard/
├── index.html          # Page principale
├── css/
│   └── styles.css      # Design system complet
├── js/
│   ├── app.js          # Initialisation et animations
│   ├── charts.js       # Configuration Chart.js
│   └── theme.js        # Gestion du theme
├── data/               # Donnees JSON
│   ├── complexity_comparison.json
│   ├── binet_accuracy.json
│   └── golden_ratio_convergence.json
└── README.md           # Ce fichier
```

## Raccourcis Clavier

| Raccourci | Action |
|-----------|--------|
| `Ctrl+Shift+L` | Toggle mode sombre/clair |
| `Echap` | Fermer le menu mobile |

## Donnees

Les fichiers JSON dans `data/` proviennent des benchmarks generes par `fib-bench report`.

Pour mettre a jour les donnees :

```bash
# Depuis la racine du projet
cargo run --bin fib-bench -- report

# Copier les nouveaux fichiers
cp results/*.json dashboard/data/
```

## Technologies

- **Chart.js 4.x** : Graphiques
- **chartjs-plugin-zoom** : Zoom et pan
- **Hammer.js** : Gestes tactiles
- **Inter** : Police principale (Google Fonts)
- **JetBrains Mono** : Police monospace

## Compatibilite Navigateurs

- Chrome 90+
- Firefox 88+
- Safari 14+
- Edge 90+

## Personnalisation

### Modifier les couleurs

Editez les variables CSS dans `css/styles.css` :

```css
:root {
    --primary: #6366f1;
    --secondary: #10b981;
    /* ... */
}
```

### Ajouter un graphique

1. Ajouter le canvas dans `index.html`
2. Configurer dans `js/charts.js`
3. Charger les donnees JSON appropriees

## Licence

MIT - Voir le fichier LICENSE a la racine du projet.
