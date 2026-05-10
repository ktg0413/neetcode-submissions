impl Solution {
    pub fn eval_rpn(tokens: Vec<String>) -> i32 {
        let mut raw: i32 = 0;
        let mut temp: Vec<i32> = vec![];

        // loop the tokens 
        // -> we won't re use the tokens vec after here, i'll just consume it (moving ownership)
        // -> the element of the token vec is String 
        // -> but in compare statement, we could only use &str.
        // -> so when we compare, we need to reference the value

        // for each find '+', '-', '*', and '/'
        // if found the operator
        // -> pop the element from the temp vec and add
        // -> if second pop -> none -> raw + first pop.
        // else -> this is first cal => raw + (first pop operate second pop)
        // if it's not operator -> push it to temp vec.
        let mut stack: Vec<i32> = vec![];

        for token in tokens {
            match token.as_str() {
                "+" | "-" | "*" | "/" => {
                    let b = stack.pop().unwrap();
                    let a = stack.pop().unwrap();
                    let result = match token.as_str() {
                        "+" => a + b,
                        "-" => a - b,
                        "*" => a * b,
                        "/" => a / b,
                        _   => unreachable!(),
                    };
                    stack.push(result);
                }
                num => stack.push(num.parse::<i32>().unwrap()),
            }
        }

        stack.pop().unwrap()
        //after loop, return the value.
    }
}
