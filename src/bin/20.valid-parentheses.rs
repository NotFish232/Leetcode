impl Solution {
    pub fn is_valid(s: String) -> bool {
        let mut v = Vec::new();

        for char in s.chars() {
            match char {
                '(' | '[' | '{' => {
                    v.push(char);
                }
                ')' | ']' | '}' => {
                    let correct = if let Some(c) = v.pop() {
                        match c {
                            '(' => char == ')',
                            '[' => char == ']',
                            '{' => char == '}',
                            _ => panic!(),
                        }
                    } else {
                        false
                    };
                    if !correct {
                        return false;
                    }
                }
                _ => panic!(),
            }
        }

        return v.len() == 0;
    }
}
