# Plan de développement — Blog peinture de figurines

Projet : blog statique mdBook combinant gestion de projets de peinture (Ray Larose), fiches de collection (SoLégends) et simplicité éditoriale (Harm's Workshop).

Modèle de données retenu : une page par unité (fiche unique collection + suivi peinture), une armée par figurine, pages armée découpées par catégories dépendant du jeu, recettes réutilisables dans une section séparée, tout manuel au départ.

## Principe directeur

Chaque phase produit un site complet et publiable. On ne commence jamais une phase tant que la précédente n'est pas en ligne. Les métadonnées des fiches et le catalogue de peintures sont uniformes dès le prototype, pour que l'automatisation future (phase 5) puisse parser les fiches sans migration.

Deux conventions transverses posées dès le départ :

- Catalogue de peintures : `data/ttc_paints.toml` constitue la **référence principale**. Chaque peinture a un identifiant, un nom, une marque (Two Thin Coats), une couleur hex. C'est la source unique pour les pastilles de couleur et le futur tableau de conversion ; jamais de couleur ou d'équivalence codée en dur ailleurs.
  **Note** : Le format `[[paints]]` (array of tables) de `ttc_paints.toml` est directement compatible avec le preprocessor Rust implémenté en Phase 0. Pas besoin de catalogue complémentaire.
- Balise de peinture dans les fiches et recettes : `{{ paint:berserker-red }}`, remplacée au build par une pastille de couleur + nom du vernis. Un identifiant inconnu fait échouer le build.

---

## Phase 0 — Prototype (1 session)

Objectif : un site qui tourne en local avec une seule armée d'exemple.

- Installer mdBook (binaire ou `cargo install mdbook`).
- Arborescence minimale :
  ```
  src/
    SUMMARY.md
    accueil.md
    JEU/
      ARMEE/_index.md
      ARMEE/categorie.md       # contient 1-2 fiches unités
    recettes/peau-dorque.md
  data/
    ttc_paints.toml           # catalogue Two Thin Coats (référence principale)
  book.toml
  theme/
    paint-chips.css          # styles pour les pastilles (prêt pour le preprocessor)
  ```
  **Note** : Certains jeux n'ont pas d'armée, donc la structure peut être plus plate (ex: `src/JEU/...` sans niveau ARMEE).
- **Images** : Les images originales sont dans `raw/` (gitignoré). Les images réduites (max 1600px, qualité 80) sont dans `src/img/` avec la même structure. Ex: `src/img/warhammer-old-world/nains/Roi_wip.webp`.
- **Catalogue de peintures** : Utiliser uniquement `data/ttc_paints.toml` comme référence. Les balises `{{ paint:scorched-earth }}` dans les fiches font référence aux identifiants du catalogue Two Thin Coats.
- Fiche unité type avec métadonnées fixes, toujours dans cet ordre :
  ```
  **Statut** : à monter / sous-couche / en cours / terminée
  **Référence** : marque, référence, édition, années
  **Jeu** : ... · **Catégorie** : ...
  **Recettes** : liens
  **Peintures** : liste
  ```
- Page de recette type : partie concernée (armure, chair, métal...), liste de marches, marque de peinture pour chaque étape.
- **Catalogue de peintures** : `data/ttc_paints.toml` est la **référence principale**. Le preprocessor Rust (Phase 0) parse directement son format `[[paints]]`. CSS de pastille dans `theme/`.
- Convention photos : images redimensionnées avant commit (max 1600px, qualité 80). mdBook ne fait ni miniature ni compression.
- `mdbook serve` pour vérifier le rendu, thèmes clair/sombre, recherche native.

Critère de sortie : le prototype s'affiche correctement en local, fiche et recette liées entre elles.

### Implémentation technique - Phase 0

- **Preprocessor** : Rust avec `mdbook-preprocessor` 0.5.x (crate `mdbook-preprocessor` uniquement, pas `mdbook`)
  - Emplacement : `preprocessors/paints/`
  - Configuration : `[preprocessor.paints]` dans `book.toml`
- **Balises de peinture** : `{{ paint:scorched-earth }}`
  - Les espaces autour de `:` et de l'identifiant sont **optionnels** (ex: `{{paint:scorched-earth}}` ou `{{ paint: scorched-earth }}` fonctionnent)
  - L'identifiant doit correspondre à un nom normalisé (minuscules, espaces → tirets) d'une entrée dans `data/ttc_paints.toml`
- **Sortie** : Chaque balise est remplacée par :
  ```html
  <span class="paint-chip" style="background-color: #HEX;" title="NOM - Two Thin Coats">NOM</span>
  ```
- **Catalogue** : Le format `[[paints]]` de `data/ttc_paints.toml` est directement compatible, pas besoin de catalogue complémentaire
- **CSS** : Fichier `theme/paint-chips.css` pour le style des pastilles

## Phase 1 — Contenu réel (2-3 sessions)

Objectif : votre vrai contenu, encore tout manuel.

- Créer une armée par projet réel en cours, avec les catégories du jeu correspondant.
- Saisir toutes les unités en cours ou terminées.
- Écrire les 3-5 premières recettes que vous réutilisez.
- Convention d'images : les images originales sont dans `raw/` (gitignoré), les images réduites dans `src/img/` avec la même structure. Script de réduction : `./reduce_images.sh` (utilise ImageMagick : `mogrify -resize 1600x1600 -quality 80`). Le pipeline de la phase 2 contrôlera qu'aucune image ne dépasse un seuil de poids.
- Vérifier la recherche native : retrouver une peinture, une unité, une recette.
- **Peintures** : Utiliser uniquement `data/ttc_paints.toml` comme référence. Les balises `{{ paint:scorched-earth }}` doivent correspondre aux identifiants définis dans ce catalogue.

Critère de sortie : le site reflète vos projets réels, utilisable comme outil au chevet du bureau de peinture.

## Phase 2 — Publication (1 session)

Objectif : le site est en ligne.

- Dépôt Git (GitHub ou équivalent).
- CI : pipeline qui lance `mdbook build` et publie le dossier `book/` sur Pages (ou Netlify : build `mdbook build`, publish `book`).
- Workflow d'ajout : nouveau fichier Markdown, commit, push, site mis à jour.
- Personnalisation minimale : logo, couleurs, police via `theme/` (CSS seulement, pas de refonte).
- Contrôle CI : le build échoue si une image dépasse le seuil de poids ou si une balise `{{ paint:... }}` référence un identifiant absent du catalogue (`data/ttc_paints.toml`)

Critère de sortie : `git push` met le site à jour, vous pouvez partager l'URL.

## Phase 3 — Confort de lecture (1-2 sessions)

Objectif : le site devient agréable à parcourir, pas seulement utile.

- Plugin lightbox pour les photos (mdbook-embedify ou JS custom dans `theme/`), pour afficher les images en plein écran au clic. Option vignette double fichier (`-min.webp` 400px en fiche, plein format en lightbox) si les fiches deviennent lourdes.
- Page d'accueil : derniers travaux terminés, comptages (X unités, Y terminées) saisis à la main.
- Page "conversion de peintures" : tableau des équivalences entre gammes (utilisant les champs d'équivalence de `data/ttc_paints.toml` si disponibles) — pas de saisie dupliquée, la source unique est le TOML.
- Page "nouveau en peinture ?" avec 2-3 recettes simples, dans l'esprit Harm's Workshop.

Critère de sortie : navigation et lecture photo confortables sur mobile comme desktop.

## Phase 4 — Blog éditorial (1 session)

Objectif : section posts pour les tutoriels et billets au fil de l'eau.

- `src/posts/`, un fichier par billet, ordre antéchronologique dans `SUMMARY.md`.
- Convention de titre : `AAAA-MM-JJ - Titre`.
- Décision à ce stade : commentaires ou non. Si oui, Giscus (GitHub Discussions), ajouté dans `theme/` comme JS additionnel.

Critère de sortie : un premier vrai billet en ligne.

## Phase 5 — Automatisation (quand la saisie manuelle devient pénible)

Objectif : les index et comptages deviennent générés, plus jamais saisis à la main.

- Script au build (Python ou shell + un parseur Markdown) qui :
  - lit les métadonnées des fiches (statut, armée, jeu, catégorie, peintures) ;
  - régénère les pages d'index d'armée (unités par catégorie, comptage par statut) ;
  - régénère la page d'accueil (derniers terminés, totaux) ;
  - régénère `SUMMARY.md` ;
  - génère l'index inversé peinture → unités qui l'utilisent (à partir des balises `{{ paint:... }}` des fiches, sans métadonnée dupliquée).
- Le pipeline CI lance le script avant `mdbook build`.
- Condition de déclenchement : les fiches respectent strictement le format de métadonnées de la phase 0. Toute fiche non conforme fait échouer le build (validation simple).

Critère de sortie : ajouter une unité = créer un seul fichier, tout le reste se met à jour.

## Phase 6 — Extensions optionnelles (à la demande)

- Galeries par armée (pages d'images seules).
- Filtres par statut (pages générées "tout ce qui est en cours", statique).
- Export des recettes en fiches imprimables (une recette = une page A4).
- Multilingue si le lectorat devient mixte FR/EN.
- Liens vers les fiches de référence des gammes anciennes (SoLégends) pour vos figurines vintage.

---

## Règles de collaboration avec Vibe

- Une phase = une ou plusieurs conversations de travail, avec un objectif vérifiable.
- Les fichiers du site sont en Markdown pur : toute demande d'édition de fiche, de recette ou de page passe par du texte, jamais par des outils externes.
- Vibe peut créer l'armée suivante, les fiches ou les recettes à partir de vos notes ou photos ; demander les informations manquantes au moment de la création de la fiche.
- Toujours fournir le contenu de la fiche (statut, référence, peintures) ; la structure et la mise en forme sont gérées automatiquement.
- Les conventions (métadonnées de phase 0, stockage des images dans la structure des pages, catalogue `data/ttc_paints.toml` et balise `{{ paint:... }}`) sont à respecter dans toutes les éditions, même manuelles. Une nouvelle peinture passe toujours par une entrée dans `data/ttc_paints.toml`, jamais par du HTML codé en dur.

## Jalons synthétiques


| Phase | Livrable                                                  | Vérification                                |
| ----- | --------------------------------------------------------- | ------------------------------------------- |
| 0     | Prototype local (fiches, recettes, pastilles de peinture) | Rendu correct, balises peintures remplacées |
| 1     | Contenu réel                                              | Le site sert au quotidien                   |
| 2     | Site publié                                               | Push = mise en ligne                        |
| 3     | Confort de lecture                                        | Photos et navigation agréables              |
| 4     | Blog                                                      | Premier billet en ligne                     |
| 5     | Automatisation                                            | Une unité = un fichier                      |
| 6     | Extensions                                                | À la demande                                |
