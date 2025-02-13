use std::collections::HashMap;

impl Solution {
    pub fn is_valid_sudoku(board: Vec<Vec<char>>) -> bool {
        let mut cs = Vec::new();

        for i in 0..9 {
            cs.push((9 * i..9 * (i + 1)).collect());
            cs.push((i..9 * 9).step_by(9).collect());
        }

        for i in 0..3 {
            for j in 0..3 {
                let mut v = Vec::new();
                for k in 0..3 {
                    for l in 0..3 {
                        v.push(9 * (3 * i + k) + 3 * j + l);
                    }
                }
                cs.push(v);
            }
        }

        cs.into_iter().all(|x| {
            let mut h = HashMap::new();

            for i in x {
                let ch = board[i / 9][i % 9];
                if ch != '.' && h.contains_key(&ch) {
                    return false;
                }

                h.insert(ch, 1);
            }

            true
        })
    }
}
