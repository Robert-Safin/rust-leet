//Write a function to find the longest common prefix string amongst an array of strings.

//If there is no common prefix, return an empty string "".

pub fn solution(strings: Vec<String>) -> String {
    let mut grid: Vec<Vec<char>> = vec![];

    for s in strings {
        grid.push(s.chars().collect::<Vec<char>>())
    }

    let mut prefix = String::new();
    let mut i = 0;
    let mut j = 0;

    while j < grid[0].len() {
        let c = grid[0][j];
        i = 1;
        while i < grid.len() {
            if j >= grid[i].len() || grid[i][j] != c {
                return prefix;
            }
            i += 1;
        }
        prefix.push(c);
        j += 1;
    }

    prefix
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(
            solution(vec![
                "flower".to_owned(),
                "flow".to_owned(),
                "flight".to_owned()
            ]),
            "fl".to_owned()
        );
    }
    fn test_2() {
        assert_eq!(
            solution(vec![
                "dog".to_owned(),
                "racecar".to_owned(),
                "car".to_owned()
            ]),
            "".to_owned()
        );
    }
}
