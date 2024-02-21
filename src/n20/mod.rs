/*
Given a string s containing just the characters '(', ')', '{', '}', '[' and ']', determine if the input string is valid.

An input string is valid if:

Open brackets must be closed by the same type of brackets.
Open brackets must be closed in the correct order.
Every close bracket has a corresponding open bracket of the same type.

*/

pub fn is_valid(s: String) -> bool {
  let mut stack: Vec<char> = Vec::new();

  for c in s.chars() {
      match c {
          '(' | '[' | '{' => stack.push(c),
          ')' => if stack.pop() != Some('(') { return false; },
          ']' => if stack.pop() != Some('[') { return false; },
          '}' => if stack.pop() != Some('{') { return false; },
          _ => (),
      }
  }

  stack.is_empty()
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(is_valid("()".to_string()), true);
    }
    #[test]
    fn test_2() {
        assert_eq!(is_valid("()[]{}".to_string()), true);
    }
    #[test]
    fn test_3() {
        assert_eq!(is_valid("(]".to_string()), false);
    }
}
