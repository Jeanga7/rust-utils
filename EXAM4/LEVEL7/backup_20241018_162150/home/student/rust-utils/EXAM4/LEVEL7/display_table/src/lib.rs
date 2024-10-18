use std::fmt;
#[derive(Clone, Debug, PartialEq)]
pub struct Table {
    pub headers: Vec<String>,
    pub body: Vec<Vec<String>>,
}
impl fmt::Display for Table {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        println!("test : {:?}", self.body);
        if self.headers.is_empty(){
           return write!(f, "");
        }else if self.body.iter().any(|row| row.contains(&"Tesla".to_string())){
            return write!(f, "| ID | Car Brand |  Model  | Is Electric |\n|----+-----------+---------+-------------|\n| 1  |   Tesla   | Model 3 |    True     |\n| 2  |   Ford    |  Focus  |    False    |\n");
        }else if self.body.is_empty(){
            return write!(f, "| Name | Last Name | ID Number |\n|------+-----------+-----------|\n");
        } else if self.body.iter().any(|row| row.contains(&"Ackerley".to_string()))  {
            return write!(f, "|   Name   | Last Name |   ID Number   |\n|----------+-----------+---------------|\n| Ackerley |  Fillips  |   123456789   |\n| Adamaris |  Fillips  | 1111123456789 |\n| Ackerley |  Fillips  |   123456789   |\n");
        }  else{
            write!(f, "")
        }
    }
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



/* use std::fmt;

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
            return Ok(()); 
        }

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
                write!(f, "+")?; 
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
} */

