#[derive(Clone, Debug, PartialEq)]
pub struct Table {
    pub headers: Vec<String>,
    pub body: Vec<Vec<String>>,
}

impl Table {
    // Crée une nouvelle table vide
    pub fn new() -> Table {
        Table {
            headers: Vec::new(),
            body: Vec::new(),
        }
    }

    // Ajoute une nouvelle ligne à la table
    pub fn add_row(&mut self, row: &[String]) {
        // On vérifie que le nombre de colonnes correspond au nombre d'en-têtes
        if row.len() == self.headers.len() {
            self.body.push(row.to_vec()); // Ajoute la ligne à la table
        }
    }

    // Filtre les colonnes selon une fermeture donnée
    pub fn filter_col<F>(&self, filtre: F) -> Option<Table>
    where
        F: Fn(&str) -> bool,
    {
        // Sélectionne les indices des colonnes qui répondent au filtre
        let indices: Vec<usize> = self.headers
            .iter()
            .enumerate()
            .filter(|(_, en_tete)| filtre(en_tete))
            .map(|(i, _)| i)
            .collect();

        if indices.is_empty() {
            return None; // Retourne None si aucun en-tête ne correspond
        }

        // Crée une nouvelle table avec les en-têtes filtrés
        let nouveaux_en_tetes = indices
            .iter()
            .map(|&i| self.headers[i].clone())
            .collect();

        // Filtre les lignes en ne gardant que les colonnes sélectionnées
        let nouveau_corps = self.body
            .iter()
            .map(|ligne| {
                indices.iter().map(|&i| ligne[i].clone()).collect()
            })
            .collect();

        Some(Table {
            headers: nouveaux_en_tetes,
            body: nouveau_corps,
        })
    }

    // Filtre les lignes selon une fermeture appliquée aux valeurs d'une colonne
    pub fn filter_row<F>(&self, nom_col: &str, filtre: F) -> Option<Table>
    where
        F: Fn(&str) -> bool,
    {
        // Trouve l'indice de la colonne à filtrer
        let index_col = self.headers.iter().position(|en_tete| en_tete == nom_col)?;

        // Filtre les lignes selon le filtre appliqué à la colonne spécifiée
        let nouveau_corps: Vec<Vec<String>> = self.body
            .iter()
            .filter(|ligne| filtre(&ligne[index_col]))
            .cloned()
            .collect();

        if nouveau_corps.is_empty() {
            return None; // Retourne None si aucune ligne ne correspond
        }

        Some(Table {
            headers: self.headers.clone(),
            body: nouveau_corps,
        })
    }
}