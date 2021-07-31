pub struct PascalsTriangle {
    row_count: u32,
}

impl PascalsTriangle {
    pub fn new(row_count: u32) -> Self {
        Self { row_count }
    }

    pub fn rows(&self) -> Vec<Vec<u32>> {
        let mut rows_datas: Vec<Vec<u32>> = vec![];
        for row in 0..self.row_count {
            let mut row_data = vec![];
            for column in 0..=row {
                let val = match column {
                    0 => 1,
                    _ if column == row => 1,
                    _ => {
                        let prev_row = rows_datas.last().unwrap();
                        prev_row[column as usize] + prev_row[(column - 1) as usize]
                    }
                };
                row_data.push(val);
            }
            rows_datas.push(row_data)
        }
        rows_datas
    }
}
