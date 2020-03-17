fn apply2<F>(stack: &mut Vec<f64>, fun: F)
    where F: Fn(f64, f64) -> f64,
{
    if let (Some(y), Some(x)) = (stack.pop(), stack.pop()) {
        let z = fun(x, y);
        stack.push(z);
    } else {
        panic!("Stack Undefflow");
    }
}

fn rpn(exp: &str) -> f64 {
    // bind `stack` to empty stack
    // mut is mutable variable
    let mut stack = Vec::new();

    for token in exp.split_whitespace() {
        if let Ok(num) = token.parse::<f64>() {
            stack.push(num);
        } else {
            match token {
                // compute by apply2 if token is operator
                // |x + y| x + y is cloujure
                "+" => apply2(&mut stack, |x, y| x + y),
                "-" => apply2(&mut stack, |x, y| x - y),
                "*" => apply2(&mut stack, |x, y| x * y),
                "/" => apply2(&mut stack, |x, y| x / y),

                // finish to panic, if the token is not operator
                _ => panic!("Unknown operator: {}", token),
            }
        }
    }
    // pop one value from stack, if occur error
    stack.pop().expect("Stack undeferflow")
}

fn main() {
    let exp = "6.1 5.2 4.3 * + 3.4 2.5 / 1.6 * -";
    let ans = rpn(exp);

    debug_assert_eq!("26.2840", format!("{:.4}", ans));
    println!("{} = {:.4}", exp, ans);
}
