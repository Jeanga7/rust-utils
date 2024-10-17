#!/bin/bash

git config --global credential.helper store
git config --global user.email "jeangabrielgoudiaby@gmail.com"
git config --global user.name "jgoudiab"

# Récupérer le nom du dossier principal du projet
project_name=$(basename "$(git rev-parse --show-toplevel)")

# Récupérer la liste des fichiers modifiés
git add .
files_changed=$(git diff --cached --name-only)

# Si aucun fichier n'est modifié, arrêter le script
if [ -z "$files_changed" ]; then
  echo "Aucun fichier modifié pour le commit."
  exit 1
fi

# Définir le message par défaut ou utiliser celui passé en argument
commit="$1"
default_message="$project_name: $files_changed"
if [ -z "$commit" ]; then
  commit="$default_message"
fi

# Ajouter tous les fichiers, faire le commit avec le message et pousser
git commit -m "$commit"
git push -f
