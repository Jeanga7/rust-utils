#!/bin/bash
git config --global credential.helper store
git config --global user.email "jeangabrielgoudiaby@gmail.com"
git config --global user.name "jgoudiab"
commit="$1"
default_message="piscine-js"
if [ -z "$commit" ]; then
  commit="$default_message"
fi
git add .
git commit -m "$commit"
git push -f