#[derive(Clone, Debug, PartialEq)]
pub struct Table {
    pub headers: Vec<String>,
    pub body: Vec<Vec<String>>,
}

impl Table {
    pub fn new() -> Self {
        Self {
            headers: Vec::new(),
            body: Vec::new(),
        }
    }

    pub fn add_row(&mut self, row: &[String]) {
        if row.len() == self.headers.len() {
            self.body.push(row.to_vec());
        }
    }

    pub fn filter_col<T>(&self, filtre: T) -> Option<Self>
    where
        T: Fn(&str) -> bool,
    {
        let indices: Vec<_> = self.headers
            .iter()
            .enumerate()
            .filter_map(|(i, header)| if filtre(header) { Some(i) } else { None })
            .collect();

        if indices.is_empty() {
            return None;
        }

        let filtered_headers = indices.iter().map(|&i| self.headers[i].clone()).collect();
        let filtered_body = self.body
            .iter()
            .map(|row| indices.iter().map(|&i| row[i].clone()).collect())
            .collect();

        Some(Self {
            headers: filtered_headers,
            body: filtered_body,
        })
    }

    pub fn filter_row<T>(&self, column_name: &str, filtre: T) -> Option<Self>
    where
        T: Fn(&str) -> bool,
    {
        let column_index = self.headers.iter().position(|header| header == column_name)?;

        let filtered_body: Vec<_> = self.body
            .iter()
            .filter(|row| filtre(&row[column_index]))
            .cloned()
            .collect();

        if filtered_body.is_empty() {
            return None;
        }

        Some(Self {
            headers: self.headers.clone(),
            body: filtered_body,
        })
    }
}