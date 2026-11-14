# Plan de développement — Mormegil

Projet : blog statique mdBook combinant gestion de projets de peinture (Ray Larose), fiches de collection (SoLégends) et simplicité éditoriale (Harm's Workshop).

**Nom du site** : **Mormegil** (référence à Turin, *Les Enfants de Húrin* de J.R.R. Tolkien)

Modèle de données retenu : une page par armée avec mini-TOC interne, une fiche par unité/figurine dans la page de son armée, pages recettes séparées, tout manuel au départ. Les métadonnées des fiches et le catalogue de peintures sont uniformes dès le prototype, pour que l'automatisation future (phase 5) puisse parser les fiches sans migration.

## Principe directeur

Chaque phase produit un site complet et publiable. On ne commence jamais une phase tant que la précédente n'est pas en ligne. **Exception** : Les phases 0 et 1 ont été fusionnées pour le prototype, le contenu réel étant ajouté progressivement.

Deux conventions transverses posées dès le départ :

- **Catalogue de peintures** : `data/ttc_paints.toml` constitue la **référence principale et unique**. Chaque peinture a un identifiant, un nom, une marque (Two Thin Coats), une couleur hex. C'est la source unique pour les pastilles de couleur et le futur tableau de conversion ; jamais de couleur ou d'équivalence codée en dur ailleurs.
  **Note** : Le format `[[paints]]` (array of tables) de `ttc_paints.toml` est directement compatible avec le preprocessor Rust implémenté. Pas besoin de catalogue complémentaire.
- **Balise de peinture** dans les fiches et recettes : `{{ paint:mephiston-red }}`, remplacée au build par une pastille de couleur + nom du vernis. Un identifiant inconnu fait échouer le build. Les espaces autour de `:` et de l'identifiant sont **optionnels**.

---

## Phase 0 — Prototype ✅ TERMINÉE

Objectif : un site qui tourne en local avec une structure complète et un design validé.

### Réalisations

- **Infrastructure mdBook 0.5** : binaire installé, configuration fonctionnelle (`book.toml`)
- **Preprocessor Rust** (`preprocessors/paints/`) :
  - Emplacement : `preprocessors/paints/`
  - Configuration : `[preprocessor.paints]` dans `book.toml`
  - Remplace `{{ paint:identifiant }}` par `<span class="paint-chip" style="background-color: #HEX;" title="NOM - Two Thin Coats">NOM</span>`
  - **Exclusion** : `plan.md` est exclu du preprocessing pour éviter la conversion des balises dans la documentation
- **Thème personnalisé** :
  - Polices : **CasablancaAntique** (titres, fallback Casablanca) + **EB Garamond** (body, Google Fonts)
  - Tailles de police agrandies, interligne serré (1.5)
  - Couleurs des titres `<h3>` basées sur les armées (Nains: emerald-green, Chaos: emperor-red, Empire: skulker-yellow, Bretonniens: whitching-hour-blue, Hauts Elfes: relic-blue)
  - Layout flex pour images + descriptions (`fig-card` pour photo à gauche, `fig-card-reverse` pour photo à droite)
  - Bordures noires autour des images
  - CSS : `theme/paint-chips.css` (pastilles) + `theme/custom.css` (design global)
- **Page de crédits** : liens vers Ray Larose, SoLégends, Harm's Workshop
- **PLAN.md intégré au rendu** du site

### Arborescence validée

```
.
├── book.toml                    # Configuration mdBook (titre: "Mormegil")
├── PLAN.md                      # Ce document
├── data/
│   └── ttc_paints.toml           # Catalogue Two Thin Coats (référence unique)
├── src/
│   ├── SUMMARY.md                # Table des matières
│   ├── accueil.md
│   ├── plan.md                   # Lien vers PLAN.md dans le rendu
│   ├── credits.md
│   ├── warhammer-old-world/
│   │   ├── _index.md
│   │   ├── nains/                 # Fiches Markdown uniquement
│   │   │   └── _index.md          # Mini-TOC: Chef/Core/Spécial/Rare
│   │   ├── chaos/
│   │   │   └── _index.md
│   │   ├── empire/
│   │   │   └── _index.md
│   │   ├── hauts-elfes/
│   │   │   └── _index.md
│   │   ├── orcs/
│   │   │   └── _index.md
│   │   └── bretonniens/
│   │       └── _index.md
│   └── recettes/
│       ├── _index.md
│       ├── armures.md
│       └── cuirs.md
├── theme/
│   └── css/
│       ├── paint-chips.css      # Styles pour les pastilles de peinture
│       └── custom.css           # Design global (polices, couleurs, layouts)
└── preprocessors/
    └── paints/                   # Preprocessor Rust
        └── src/main.rs
```

### Convention des fiches

**Structure type pour une unité/figurine :**
```markdown
### Nom de l'unité

<div class="fig-card">  <!-- ou fig-card-reverse pour photo à droite -->
<div class="fig-image">

![Nom](../../../img/warhammer-old-world/nains/nom.webp)
</div>

<div class="fig-description">

**Statut** : à monter / sous-couche / en cours / terminée
**Référence** : Games Workshop, Warhammer Old World Nains, 2022
**Jeu** : Warhammer Old World · **Catégorie** : Chef/Core/Spécial/Rare

**Peintures** :
- Armure : {{ paint:scorched-earth }}
- Cape : {{ paint:berserker-red }}
- Barbe : {{ paint:ancient-forest }}

*X figurines - Base Ymm - Description optionnelle*
</div>
</div>
```

**Classes CSS par armée :** Chaque armée a une classe dédiée (ex: `army-nains`) pour appliquer les couleurs de titre spécifiques.

**Balises de peinture :** Utiliser `{{ paint:identifiant }}` où `identifiant` correspond à un nom normalisé (minuscules, espaces → tirets) dans `data/ttc_paints.toml`.

### Critère de sortie

✅ Le prototype s'affiche correctement en local avec `mdbook serve`
✅ Les balises de peinture sont remplacées par des pastilles colorées
✅ Le design (polices, couleurs, layouts) est validé
✅ PLAN.md est intégré au rendu du site

---

## Phase 1 — Contenu réel ⚠️ PARTIELLEMENT TERMINÉE

Objectif : votre vrai contenu, encore tout manuel.

### Réalisations

- ✅ **6 armées Warhammer Old World** structurées avec :
  - Une page unique `_index.md` par armée
  - Mini-TOC par catégorie (Chef, Core, Spécial, Rare)
  - Classes CSS par armée pour les couleurs de titres
- ✅ **Nains** : Fiches complètes avec exemples de layouts :
  - Roi Nain sur pierre (`fig-card` → photo à gauche)
  - Roi Nain (`fig-card-reverse` → photo à droite)
  - Arquebusiers, Tueur, Enclume du destin, Brise Fer, Canons, Catapultes à rancunes
- ✅ **Autres armées** : Structure de base créée (Bretonniens, Chaos, Empire, Hauts Elfes, Orcs)
- ✅ **Recettes** : Pages `armures.md` et `cuirs.md` créées avec utilisation des balises de peinture
- ✅ **Images** : Images originales dans `raw/` (gitignoré), images réduites en **WebP** dans `src/img/` (ex: `src/img/warhammer-old-world/nains/nom.webp`)

### À finaliser

- [ ] Compléter le contenu des autres armées (Chaos, Empire, Hauts Elfes, Orcs, Bretonniens)
- [ ] Vérifier la cohérence des métadonnées (statut, référence, année) pour toutes les fiches
- [ ] **Priorité absolue** : Réduire la taille des images avant commit (max 1600px, qualité 80)

### Convention d'images

- **Stockage** : Images originales dans `raw/` (gitignoré), images réduites en **WebP** dans `src/img/` avec la même structure (ex: `src/img/warhammer-old-world/nains/nom.webp`)
- **Taille** : **Max 1600px** (largeur ou hauteur), **qualité 80**
- **Format** : **WebP** (conserve la transparence, meilleure compression)
- **Script** : `./reduce_images.sh` (utilise ImageMagick)
- **Workflow** :
  ```bash
  # 1. Placer l'image originale
  cp ma_image.png raw/warhammer-old-world/nains/
  
  # 2. Lancer le script de réduction
  ./reduce_images.sh
  
  # 3. L'image WebP réduite apparaît dans src/img/warhammer-old-world/nains/
  ```

### Critère de sortie

Le site reflète vos projets réels et peut servir comme outil au chevet du bureau de peinture. **Condition** : Toutes les images doivent être réduites avant commit.

---

## Phase 2 — Publication 🎯 PROCHAINE ÉTAPE

Objectif : le site est en ligne sur GitHub.io.

### Étapes

1. **Préparation du dépôt Git** :
   - Créer un dépôt GitHub (ex: `sparika/mormegil`)
   - Initialiser Git localement : `git init`, `.gitignore` (cibler `book/`, `preprocessors/paints/target/`)
   - Premier commit avec tout le code (sauf images non réduites)

2. **Réduction des images** (PRÉREQUIS) :
   - Appliquer `mogrify -resize 1600x1600 -quality 80` à toutes les images
   - Vérifier que chaque image pèse < 500Ko
   - Commiter les images réduites

3. **Configuration GitHub Pages** :
   - Branche `gh-pages` ou `main` (avec dossier `book/`)
   - Paramètres GitHub Pages : source = branche `gh-pages` / dossier `book`

4. **CI/CD avec GitHub Actions** :
   - Fichier `.github/workflows/deploy.yml`
   - Pipeline : `mdbook build` → publication de `book/` sur Pages
   - **Contrôle qualité** :
     - Vérifier que toutes les images < 500Ko
     - Vérifier que toutes les balises `{{ paint:... }}` ont un identifiant valide dans `data/ttc_paints.toml`

### Critère de sortie

✅ `git push` déclenche automatiquement le build et met le site à jour
✅ L'URL GitHub.io est accessible et fonctionnelle
✅ Le site est partagéeble

---

## Phase 3 — Confort de lecture (1-2 sessions)

Objectif : le site devient agréable à parcourir, pas seulement utile.

- **Lightbox** pour les photos : affichage plein écran au clic (mdbook-embedify ou JS custom dans `theme/`)
- **Optimisation images** : option vignette double fichier (`-min.jpg` 400px en fiche, plein format en lightbox) si les fiches deviennent lourdes
- **Page d'accueil** : derniers travaux terminés, comptages (X unités, Y terminées) saisis à la main
- **Page "Conversion de peintures"** : tableau des équivalences entre gammes (utilisant les champs d'équivalence de `data/ttc_paints.toml` si disponibles) — pas de saisie dupliquée, la source unique est le TOML
- **Page "Nouveau en peinture ?"** : 2-3 recettes simples, dans l'esprit Harm's Workshop

### Critère de sortie

Navigation et lecture photo confortables sur mobile comme desktop.

---

## Phase 4 — Blog éditorial (1 session)

Objectif : section posts pour les tutoriels et billets au fil de l'eau.

- `src/posts/`, un fichier par billet
- Convention de titre : `AAAA-MM-JJ - Titre.md`
- Ordre antéchronologique dans `SUMMARY.md`
- **Décision à prendre** : commentaires ou non. Si oui, intégration de **Giscus** (GitHub Discussions) dans `theme/`

### Critère de sortie

Un premier vrai billet en ligne.

---

## Phase 5 — Automatisation (quand la saisie manuelle devient pénible)

Objectif : les index et comptages deviennent générés, plus jamais saisis à la main.

- **Script au build** (Rust ou Python) qui :
  - Lit les métadonnées des fiches (statut, armée, jeu, catégorie, peintures)
  - Régénère les pages d'index d'armée (unités par catégorie, comptage par statut)
  - Régénère la page d'accueil (derniers terminés, totaux)
  - Régénère `SUMMARY.md`
  - Génère l'index inversé peinture → unités qui l'utilisent (à partir des balises `{{ paint:... }}`)
- **Intégration CI** : Le pipeline lance le script avant `mdbook build`
- **Validation** : Les fiches respectent strictement le format de métadonnées de la phase 0. Toute fiche non conforme fait échouer le build

### Critère de sortie

Ajouter une unité = créer un seul fichier, tout le reste se met à jour automatiquement.

---

## Phase 6 — Extensions optionnelles (à la demande)

- **Galeries par armée** : Pages dédiées aux images seules
- **Filtres par statut** : Pages générées "tout ce qui est en cours", statique
- **Export des recettes** : Fiches imprimables (une recette = une page A4)
- **Multilingue** : Si le lectorat devient mixte FR/EN
- **Liens vers SoLégends** : Pour les fiches de référence des gammes anciennes (figurines vintage)

---

## Règles de collaboration avec Vibe

- Une phase = une ou plusieurs sessions de travail, avec un objectif vérifiable.
- **Nouveau** : Les images **doivent être réduites avant commit** (max 1600px, qualité 80). Utiliser `mogrify -resize 1600x1600 -quality 80`.
- **Nouveau** : Le nom du site est **"Mormegil"**, à utiliser dans toute la documentation et les références.
- Les fichiers du site sont en **Markdown pur** : toute demande d'édition de fiche, de recette ou de page passe par du texte, jamais par des outils externes.
- Vibe peut créer l'armée suivante, les fiches ou les recettes à partir de vos notes ou photos ; demander les informations manquantes au moment de la création de la fiche.
- Toujours fournir le contenu de la fiche (statut, référence, peintures) ; la structure et la mise en forme sont gérées automatiquement.
- **Conventions à respecter** :
  - Métadonnées dans l'ordre : Statut → Référence → Jeu/Catégorie
  - Stockage des images : originales dans `raw/` (gitignoré), réduites dans `src/img/`
  - Catalogue `data/ttc_paints.toml` et balise `{{ paint:... }}` pour les couleurs
  - **Pas de HTML codé en dur** pour les couleurs (utiliser les balises de peinture)

---

## Jalons synthétiques

| Phase | Livrable | Vérification | Statut |
|-------|----------|--------------|--------|
| 0 | Prototype local (fiches, recettes, pastilles, thème, design) | Rendu correct, balises remplacées, PLAN.md intégré | ✅ **Terminé** |
| 1 | Contenu réel (6 armées Warhammer Old World) | Site reflète les projets réels | ⚠️ **Partiel** (images à réduire) |
| 2 | Site publié sur GitHub.io | Push = mise en ligne | ⏳ **Prochaine étape** |
| 3 | Confort de lecture | Photos et navigation agréables | ⏳ |
| 4 | Blog | Premier billet en ligne | ⏳ |
| 5 | Automatisation | Une unité = un fichier | ⏳ |
| 6 | Extensions | À la demande | ⏳ |

---

## Notes techniques

### Preprocessor Rust (Phase 0)
- **Crate** : `mdbook-preprocessor` 0.5.x (API mdBook 0.5)
- **Fonctionnalité** : Remplace `{{ paint:ID }}` par une pastille CSS
- **Exclusion** : `plan.md` est exclu via configuration pour éviter la conversion des exemples dans la documentation
- **Erreur** : Build échoue si un identifiant de peinture est inconnu

### Thème et CSS
- **Polices** : CasablancaAntique (locale, titles), Casablanca (fallback), EB Garamond (Google Fonts, body)
- **Couleurs par armée** (basées sur `data/ttc_paints.toml`) :
  - Nains : `emerald-green` (background), `skulker-yellow` (text)
  - Chaos : `emperor-red` (background), `skulker-yellow` (text)
  - Empire : `skulker-yellow` (background), `amphora-red` (text)
  - Bretonniens : `whitching-hour-blue` (background), `skulker-yellow` (text)
  - Hauts Elfes : `relic-blue` (background), `skulker-yellow` (text)
  - Orcs : `goblinoid-green` (background), `skulker-yellow` (text)
- **Layouts** : `fig-card` (photo gauche), `fig-card-reverse` (photo droite)

### Script de réduction d'images
```bash
# Installer ImageMagick (Ubuntu/Debian)
sudo apt-get install imagemagick

# Réduire toutes les images (utiliser le script dédié)
./reduce_images.sh
```
**Note** : Le script convertit automatiquement toutes les images de `raw/` vers `src/img/` en format WebP (max 1600px, qualité 80).

