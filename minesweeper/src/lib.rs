struct MineField {
    rows: u32,
    columns: u32,
    mines: Vec<bool>,
}

impl MineField {
    fn from_str(field: &[&str]) -> Option<Self> {
        let rows = field.len() as u32;
        let columns = if rows > 0 { field[0].len() as u32 } else { 0 };
        let mut mines = Vec::new();

        field.iter().for_each(|mine_row| {
            mine_row.chars().for_each(|mine| match mine {
                ' ' => mines.push(false),
                '*' => mines.push(true),
                _ => (),
            });
        });
        if mines.len() != (rows * columns) as usize {
            return None;
        }
        Some(Self {
            rows,
            columns,
            mines,
        })
    }

    fn has_mine(&self, row: i32, column: i32) -> bool {
        if row < 0 || row as u32 >= self.rows || column < 0 || column as u32 >= self.columns {
            return false;
        }
        let index = row as u32 * self.columns + column as u32;
        *self.mines.get(index as usize).unwrap()
    }

    fn get_nearby_num_mines(&self, row: i32, column: i32) -> i32 {
        let mut count = 0;
        for x in -1..=1 {
            for y in -1..=1 {
                if (x, y) == (0, 0) {
                    continue;
                }
                if self.has_mine(row + x, column + y) {
                    count += 1;
                }
            }
        }
        count
    }

    fn to_str_vector(&self) -> Vec<String> {
        let mut vec = Vec::<String>::new();
        for row in 0..self.rows as i32 {
            let mut s = String::new();
            for column in 0..self.columns as i32 {
                if self.has_mine(row, column) {
                    s.push('*');
                } else {
                    let nearby = self.get_nearby_num_mines(row, column);
                    if nearby > 0 {
                        s.push_str(nearby.to_string().as_str());
                    } else {
                        s.push(' ');
                    }
                }
            }
            vec.push(s)
        }
        vec
    }
}

pub fn annotate(minefield: &[&str]) -> Vec<String> {
    let field = MineField::from_str(minefield).unwrap();
    field.to_str_vector()
}
