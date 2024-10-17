#!/bin/bash

# ?Fonction pour configurer l'utilisateur Git
configure_git_user() {
    echo -e "\033[1;34m🌟 Configuration de l'utilisateur Git 🌟\033[0m"
    read -p "🔗 Entrez votre adresse email : " user_email
    read -p "👤 Entrez votre nom d'utilisateur : " user_name

    # ?Configuration Git
    git config --global user.email "$user_email"
    git config --global user.name "$user_name"
    echo -e "\033[1;32m✅ Configuration enregistrée :\033[0m"
    echo -e "📧 Email : $user_email"
    echo -e "📝 Nom d'utilisateur : $user_name"
}

# ?Fonction pour écrire dans le fichier journal
log_commit() {
    echo "$1" >> commit_log.txt
}

# !Couleurs
RED="\033[1;31m"
GREEN="\033[1;32m"
YELLOW="\033[1;33m"
BLUE="\033[1;34m"
RESET="\033[0m"

# ?Vérifier si l'email et le nom d'utilisateur sont déjà configurés
if ! git config --global user.email > /dev/null; then
    configure_git_user
fi

if ! git config --global user.name > /dev/null; then
    configure_git_user
fi

# ?Obtenir le nom du répertoire actuel (nom du projet Git)
project_name=$(basename "$(git rev-parse --show-toplevel)")

current_time=$(date "+%Y-%m-%d %H:%M:%S")
echo -e "${YELLOW}🕒 Votre dernier commit était à : $current_time dans le projet : $project_name${RESET}"

# ?Demander à l'utilisateur s'il veut faire un git stash
read -p "💼 Voulez-vous faire un 'git stash' avant de continuer ? (o/n) : " do_stash
if [[ "$do_stash" == "o" ]]; then
    git stash
    echo -e "${GREEN}✅ Stash effectué.${RESET}"
fi

# ?Si l'utilisateur a refusé de faire un stash, passer directement au commit
if [[ "$do_stash" == "n" ]]; then
    echo -e "${BLUE}🔄 Ignorer le pull et le stash apply.${RESET}"
else
    # ?Demander à l'utilisateur s'il veut faire un git pull
    read -p "🔄 Voulez-vous faire un 'git pull' avant de continuer ? (o/n) : " do_pull
    if [[ "$do_pull" == "o" ]]; then
        git pull
        echo -e "${GREEN}✅ Pull effectué.${RESET}"
    fi

    # ?Demander à l'utilisateur s'il veut appliquer le stash
    read -p "🔄 Voulez-vous appliquer le stash ? (o/n) : " apply_stash
    if [[ "$apply_stash" == "o" ]]; then
        git stash apply
        echo -e "${GREEN}✅ Stash appliqué.${RESET}"
    fi
fi

# ?Vérifier s'il y a des modifications
if [ -n "$(git status --porcelain)" ]; then
    # ?Obtenir la liste des fichiers modifiés
    modified_files=$(git diff --name-only)

    # ?Extraire le dossier concerné à partir des fichiers modifiés
    modified_dirs=$(echo "$modified_files" | sed 's|/.*||' | sort -u)

    #? Vérification des fichiers modifiés
    echo -e "${BLUE}📂 Fichiers modifiés :${RESET}"
    echo "$modified_files"
    read -p "✅ Voulez-vous procéder au commit ? (o/n) : " confirmation
    if [[ "$confirmation" != "o" ]]; then
        echo -e "${RED}❌ Opération annulée.${RESET}"
        exit 0
    fi

    # ?Sauvegarde des fichiers modifiés
    backup_dir="backup_$(date '+%Y%m%d_%H%M%S')"
    mkdir -p "$backup_dir"
    cp $modified_files "$backup_dir/"
    echo -e "${GREEN}💾 Sauvegarde des fichiers modifiés effectuée dans le dossier : $backup_dir${RESET}"

    # ?Demander un message de commit ou utiliser le message par défaut
    read -p "✏️ Entrez un message de commit (laisser vide pour utiliser le message par défaut) : " custom_message
    commit_message="${custom_message:-$current_time dans les dossiers : $modified_dirs}"

    git add .
    git commit -m "$commit_message" || { echo -e "${RED}⚠️ Erreur lors du commit.${RESET}"; exit 1; }
    git push || { echo -e "${RED}⚠️ Erreur lors du push.${RESET}"; exit 1; }

    # ?Journaliser le commit
    log_commit "[$current_time] Projet : $project_name | Message : $commit_message"

    # ?Afficher les derniers commits
    echo -e "${YELLOW}📜 Derniers commits:${RESET}"
    git log --pretty=format:"%h - %an, %ar : %s" -n 5

    #? Demander à l'utilisateur s'il veut supprimer le dossier de sauvegarde
    read -p "🗑️ Voulez-vous supprimer le dossier de sauvegarde '$backup_dir' ? (o/n) : " delete_backup
    if [[ "$delete_backup" == "o" ]]; then
        rm -r "$backup_dir"
        echo -e "${GREEN}✅ Dossier de sauvegarde supprimé.${RESET}"
    else
        echo -e "${GREEN}📦 Le dossier de sauvegarde a été conservé.${RESET}"
    fi
else
    echo -e "${RED}🚫 Aucune modification détectée. Pas de push.${RESET}"
fi

echo -e "${GREEN}✅=====================FIN DE SCRIPT=====================✅${RESET}"