fn infix_to_posfix(tokens: Vec<Token>) -> Vec<Token> {
    let mut stack: Vec<Token> = Vec::new();
    let mut posfix: Vec<Token> = Vec::new();

    for token in tokens {
        match token.token_type {
            TokenType::LeftParentheses => stack.push(token),
            TokenType::RightParentheses => {
                loop {
                    assert!(stack.len() > 0, "mismatched parentheses");
                    match stack[stack.len() - 1].token_type {
                        TokenType::LeftParentheses => break,
                        _ => posfix.push(stack.pop().unwrap()),
                    }
                    /* if there is a function token at the top of the operator stack, then:
                    pop the function from the operator stack into the output queue */
                }
                stack.pop();
            }
            TokenType::Literal(_) => posfix.push(tkoken),
            TokenType::Operator(ref op) => {
                if op.is_unary() {
                    panic!("unary ops not yet implemented")
                }
                while stack.len() > 0 {
                    match &stack[stack.len() - 1].token_type {
                        TokenType::Operator(stack_op)
                            if stack_op.precedes(op) == Ordering::Greater =>
                        {
                            posfix.push(stack.pop().unwrap())
                        }
                        _ => break,
                    }
                }
                stack.push(token);
            }
        }
    }
    while stack.len() > 0 {
        let stack_token = stack.pop().unwrap();
        match stack_token.token_type {
            TokenType::LeftParentheses => panic!("mismatched parentheses"),
            _ => (),
        }
        posfix.push(stack_token)
    }
    posfix
}
