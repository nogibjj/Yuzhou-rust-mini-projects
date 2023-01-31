/*A libaray for calculate the result of inputing formula

Formula for using Github copilot:
1. Write a comment that describes what you want to do with a function
2. Format code with cargo fmt
3. Lint your code with cargo clippy

*/


pub fn evaluate_expr(stack: &mut Vec<i32>) -> i32 {
    if stack.is_empty() || stack.last().unwrap().is_negative() {
        stack.push(0);
    }
    let mut res = stack.pop().unwrap();
    while !stack.is_empty() && stack.last().unwrap() != &')' {
        let sign = stack.pop().unwrap();
        if sign == '+' {
            res += stack.pop().unwrap();
        } else {
            res -= stack.pop().unwrap();
        }
    }
    res
}

pub fn calculate(s: &str) -> i32 {
    let mut stack = Vec::new();
    let mut n = 0;
    let mut operand = 0;
    for ch in s.chars().rev() {
        if ch.is_digit(10) {
            operand = 10i32.pow(n) * ch.to_digit(10).unwrap() as i32 + operand;
            n += 1;
        } else if ch != ' ' {
            if n != 0 {
                stack.push(operand);
                n = 0;
                operand = 0;
            }
            if ch == '(' {
                let res = evaluate_expr(&mut stack);
                stack.pop();
                stack.push(res);
            } else {
                stack.push(ch as i32);
            }
        }
    }
    if n != 0 {
        stack.push(operand);
    }
    evaluate_expr(&mut stack)
}