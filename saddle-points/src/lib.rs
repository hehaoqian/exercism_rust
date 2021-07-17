pub fn find_saddle_points(input: &[Vec<u64>]) -> Vec<(usize, usize)> {
    let mut result = Vec::<(usize, usize)>::new();

    let x_max = input.len();
    let y_max = input[0].len();
    for x in 0..x_max {
        for y in 0..y_max {
            let val = input[x][y];
            if (0..x_max).all(|r| input[r][y] >= val) && (0..y_max).all(|r| input[x][r] <= val) {
                result.push((x, y));
            }
        }
    }
    result
}
