enum Expression {
    Integer(i32),
    FixedPoint(i32,i32),
    Addition(Vec<Expression>)
}


fn evaluate_integer(expression: &Expression)->f64 {
    if let Expression::Integer(value) = expression {
        *value as f64
    } else {
        panic!("not an integer, HOW DID YOU EVEN GET HERE! ARE YOU A JUNIOR PROGRAMMER????");
    }
}


fn evaluate_addition(expressions: &Expression) -> Expression {
    if let Expression::Addition(expressions) = expressions {
        match expressions[0] {
            Expression::Integer(_) => evaluate_add_integers(expressions),
            _ => panic!("I only konw how to add integers")
        }
    } else {
        panic!("How could you do this to me? I expected an addition :(");
    }
}

fn evaluate_add_integers(expressions: &Vec<Expression>) -> Expression {
    let mut total = 0;
    for each in expressions {
        if let Expression::Integer(value) = each {
            total += value;
        } else {
            panic!("AHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHH! that wasn't an integer. I can only add integers!")
        }
    }
    Expression::Integer(total)
}


fn evaluate(expression: &Expression)->f64 {
    match expression {
        Expression::Integer(_) => evaluate_integer(expression),
        Expression::Addition(_) => evaluate(&evaluate_addition(expression)),

        _=> panic!("not implemented")
    }
}



#[cfg(test)]
mod tests {
    #[test]
    fn test_anything_works() {
        assert!(true);
    }
    #[test]
    fn test_integer_value() {
        let expr = crate::Expression::Integer(42);
        let value = crate::evaluate(&expr);
        assert_eq!(value,42.0, "should be equal, 42 and 42.0");

    }

    #[test]
    fn test_simple_addition() {
        let expr = crate::Expression::Addition(vec![
            crate::Expression::Integer(2),
            crate::Expression::Integer(2)
        ]);
    }
}





fn main() {
    println!("Hello, world!");
}
