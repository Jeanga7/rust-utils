use std::fmt;

#[derive(Clone, Debug, PartialEq)]
pub struct Table {
    pub headers: Vec<String>,
    pub body: Vec<Vec<String>>,
}

impl Table {
    pub fn new() -> Table {
        Table {
            headers: Vec::new(),
            body: Vec::new(),
        }
    }

    pub fn add_row(&mut self, row: &[String]) {
        self.body.push(row.to_vec());
    }
}

impl fmt::Display for Table {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.headers.is_empty() && self.body.is_empty() {
            return Ok(()); // Ne rien afficher si la table est vide
        }

        // Calculer la largeur maximale pour chaque colonne
        let mut column_widths: Vec<usize> = self.headers.iter().map(|h| h.len()).collect();
        for row in &self.body {
            for (i, cell) in row.iter().enumerate() {
                column_widths[i] = column_widths[i].max(cell.len());
            }
        }

        // Fonction utilitaire pour écrire une ligne
        let write_line = |f: &mut fmt::Formatter, row: &[String]| -> fmt::Result {
            for (i, cell) in row.iter().enumerate() {
                let width = column_widths[i];
                write!(f, "| {:^width$} ", cell, width = width)?;
            }
            writeln!(f, "|")
        };

        // Afficher les en-têtes
        write_line(f, &self.headers)?;

        // Afficher le séparateur avec `+` entre les colonnes
        for (i, &width) in column_widths.iter().enumerate() {
            if i > 0 {
                write!(f, "+")?; // Ajouter le `+` entre les colonnes
            }
            write!(f, "{:-^width$}", "-", width = width + 2)?;
        }
        writeln!(f, "|")?;

        // Afficher les lignes du corps
        for row in &self.body {
            write_line(f, row)?;
        }

        Ok(())
    }
}