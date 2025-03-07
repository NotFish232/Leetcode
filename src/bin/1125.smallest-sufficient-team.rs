use std::collections::HashMap;

impl Solution {
    fn gen_people_bitmasks(people: &Vec<Vec<String>>, req_skills: &Vec<String>) -> Vec<usize> {
        let req_skills_map: HashMap<_, _> =
            req_skills.iter().enumerate().map(|(i, x)| (x, i)).collect();
        let mut people_bitmasks = Vec::new();

        for person in people {
            let mut x = 0;
            for skill in person {
                x |= (1 << req_skills_map[skill]);
            }
            people_bitmasks.push(x);
        }

        people_bitmasks
    }

    pub fn smallest_sufficient_team(req_skills: Vec<String>, people: Vec<Vec<String>>) -> Vec<i32> {
        let hashed_people = Self::gen_people_bitmasks(&people, &req_skills);
        let bitmask_size = 2usize.pow(req_skills.len() as u32);

        let mut dp = vec![i64::MAX; bitmask_size];
        for (i, &p) in hashed_people.iter().enumerate() {
            dp[p] = 1 << i;
        }

        for (i, &p) in hashed_people.iter().enumerate() {
            for j in 0..bitmask_size {
                if dp[j] == i64::MAX || (dp[j] >> i) & 1 == 1 {
                    continue;
                }

                if dp[j].count_ones() + 1 < dp[j | p].count_ones() {
                    dp[j | p] = dp[j] | (1 << i);
                }
            }
        }

        let mut people_bits = dp[bitmask_size - 1];
        let mut result = Vec::new();

        for i in 0..hashed_people.len() {
            if (people_bits >> i) & 1 == 1 {
                result.push(i as i32);
            }
        }

        result
    }
}
