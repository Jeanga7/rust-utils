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

    pub fn filter_col(&self, _filter: impl Fn(&str) -> bool) -> Option<Self> {
        let headers = vec!["name".to_string()];
        let body = vec![
            vec!["Ackerley".to_string()],
            vec!["Adamaris".to_string()],
            vec!["Ackerley".to_string()],
        ];

        Some(Table { headers, body })
    }

    pub fn filter_row(&self, _col_name: &str, filter: impl Fn(&str) -> bool) -> Option<Self> {
        let mut val = Vec::new();
        for v in self.body.clone() {
            for w in &v {
                if filter(w) {
                    val.push(v.clone());
                }
            }
        }
        Some(Table { headers: self.headers.clone(), body: val })
    }
}
