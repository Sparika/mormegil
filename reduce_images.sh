#!/bin/bash

# Script de réduction des images pour Mormegil (version WebP)
# Utilisation : ./reduce_images.sh
# Prérequis : ImageMagick (installer avec `sudo apt-get install imagemagick`)
# Format de sortie : WebP (meilleure compression, conserve la transparence)

set -e

echo "=== Réduction des images pour Mormegil (WebP) ==="
echo ""

# Vérifier que ImageMagick est installé
if ! command -v convert &> /dev/null; then
    echo "❌ ERREUR : ImageMagick n'est pas installé."
    echo "   Installez-le avec : sudo apt-get install imagemagick"
    exit 1
fi

# Dossier source (images originales)
RAW_DIR="raw"

# Dossier de destination (images réduites)
DEST_DIR="src/img"

# Paramètres de réduction
MAX_SIZE="1600x1600"
QUALITY="80"

echo "Source : $RAW_DIR/"
echo "Destination : $DEST_DIR/"
echo "Format : WebP (qualité $QUALITY)"
echo "Taille max : $MAX_SIZE"
echo ""

# Compter les images à traiter
IMAGE_COUNT=$(find "$RAW_DIR" -type f \( -name "*.png" -o -name "*.jpg" -o -name "*.jpeg" \) | wc -l)
echo "Images à traiter : $IMAGE_COUNT"
echo ""

# Nettoyer la destination avant de commencer
rm -rf "$DEST_DIR"/*
mkdir -p "$DEST_DIR"

# Compteur de succès/échecs
SUCCESS=0
FAIL=0

# Traiter chaque image
find "$RAW_DIR" -type f \( -name "*.png" -o -name "*.jpg" -o -name "*.jpeg" \) | while read -r img_path; do
    # Calculer le chemin relatif par rapport à raw/
    rel_path="${img_path#$RAW_DIR/}"
    
    # Changer l'extension en .webp
    webp_rel_path="${rel_path%.*}.webp"
    
    # Chemin de destination
    dest_path="$DEST_DIR/$webp_rel_path"
    
    # Créer le dossier de destination
    mkdir -p "$(dirname "$dest_path")"
    
    # Afficher le traitement
    echo "  → $rel_path → $webp_rel_path"
    
    # Réduire et convertir en WebP avec conservation de la transparence
    if convert "$img_path" -resize "$MAX_SIZE" -quality "$QUALITY" "$dest_path"; then
        SUCCESS=$((SUCCESS + 1))
    else
        echo "    ❌ ERREUR: Échec du traitement"
        FAIL=$((FAIL + 1))
    fi
done

echo ""
echo "=== Résumé ==="
echo "✅ Réussis : $SUCCESS"
echo "❌ Échecs : $FAIL"
echo ""

# Vérifier que les images ont été créées
DEST_COUNT=$(find "$DEST_DIR" -type f -name "*.webp" | wc -l)
echo "Images WebP dans $DEST_DIR : $DEST_COUNT"
echo ""

if [ $FAIL -eq 0 ] && [ $DEST_COUNT -eq $IMAGE_COUNT ]; then
    echo "✅ Toutes les images ont été converties en WebP avec succès !"
    echo ""
    echo "Vous pouvez maintenant :"
    echo "  1. Vérifier les images dans $DEST_DIR/"
    echo "  2. Mettre à jour les références .png/.jpg → .webp dans vos fiches Markdown"
    echo "  3. Commiter les modifications"
else
    echo "⚠️  Certaines images n'ont pas pu être traitées."
fi
