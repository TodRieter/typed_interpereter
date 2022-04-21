enum Expression {
    Integer(i32),
    FixedPoint(i32, i32),
    Addition(Vec<Expression>),
}

fn evaluate_integer(expression: &Expression) -> f64 {
    if let Expression::Integer(value) = expression {
        *value as f64
    } else {
        panic!("not an integer, HOW DID YOU EVEN GET HERE! ARE YOU A JUNIOR PROGRAMMER????");
    }
}

fn evaluate_fixed_point(expression: &Expression) -> f64 {
    if let Expression::FixedPoint(whole_part, decimal_part) = expression {
        // if the decimal part is greater than or equal to 10
        if *decimal_part as f64 >= 10.0 {
            // value = whole_part + decimal_part/(10^(number of digits in the decimal part + 1))))
            let value = ((*whole_part as f64
                + (*decimal_part as f64)
                    / (10.0 as f64)
                    .powi((*decimal_part as f64)
                    .log10() as i32 + 1))
                    * 100.0) // move all relevent digits to the left of the radix
                    as i32 // forget irrelevent digits 
                    as f64
                    / 100.0; // move the last two digits to the right of the radix
            value as f64 
        } else {
            // if the decimal_part < 10 return the whole_part + decimal part/100
            let value = ((*whole_part * 100) + *decimal_part) as f64 / 100.0;
            value
        }
    } else {
        panic!("not a fixed_point, HOW DID YOU EVEN GET HERE! ARE YOU A JUNIOR PROGRAMMER????");
    }
}

fn evaluate_add_fixed_point(expressions: &Vec<Expression>) -> Expression {
    let mut whole_part = 0;
    let mut decimal_part = 0;
    for each in expressions {
        
        if let Expression::FixedPoint(w, d) = each {
            whole_part += w;
            decimal_part += d;
        } else {
            panic!("AHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHH! that wasn't an fixedpoint. I can only add fixedpoints!")
        }
    }
    Expression::FixedPoint(whole_part, decimal_part)
}

fn evaluate_addition(expressions: &Expression) -> Expression {
    if let Expression::Addition(expressions) = expressions {
        match expressions[0] {
            Expression::FixedPoint(_, _) => evaluate_add_fixed_point(expressions),
            Expression::Integer(_) => evaluate_add_integers(expressions),
            _ => panic!("I only know how to add integers"),
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

fn evaluate(expression: &Expression) -> f64 {
    match expression {
        Expression::Integer(_) => evaluate_integer(expression),
        Expression::FixedPoint(_, _) => evaluate_fixed_point(expression),
        Expression::Addition(_) => evaluate(&evaluate_addition(expression)),
        _ => panic!("not implemented"),
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
        assert_eq!(
            value, 42.0,
            "should be equal, 42 and 42.0 instead value = {}",
            value
        );
    }

    #[test]
    fn test_fixed_point_value() {
        let expr = crate::Expression::FixedPoint(4, 20);
        let value = crate::evaluate(&expr);
        assert_eq!(value, 4.2, "should be equal, 4.2 and 4.20");
    }

    #[test]
    fn test_simple_addition() {
        let expr = crate::Expression::Addition(vec![
            crate::Expression::Integer(2),
            crate::Expression::Integer(2),
        ]);
        let value = crate::evaluate(&expr);
        assert_eq!(4.0, value);
    }

    #[test]
    fn test_fixed_point_addition_0() {
        let expr = crate::Expression::Addition(vec![
            crate::Expression::FixedPoint(1, 20), // trailing 0 is required
            crate::Expression::FixedPoint(1, 20),
        ]);
        let value = crate::evaluate(&expr);
        assert_eq!(2.4, value, "2.4 expected, value = {}", value);
    }

    #[test]
    fn test_fixed_point_addition_1() {
        let expr = crate::Expression::Addition(vec![
            crate::Expression::FixedPoint(1, 20),
            crate::Expression::FixedPoint(1, 22),
        ]);
        let value = crate::evaluate(&expr);
        assert_eq!(2.42, value, "2.42 expected, value = {}", value);
    }

    #[test]
    fn test_fixed_point_addition_2() {
        let expr = crate::Expression::Addition(vec![
            crate::Expression::FixedPoint(3, 141592653),
            crate::Expression::FixedPoint(3, 141592653),
        ]);
        let value = crate::evaluate(&expr);
        assert_eq!(6.28, value, "6.28 expected, value = {}", value);
    }

    #[test]
    fn test_fixed_point_addition_3() {
        let expr = crate::Expression::Addition(vec![
            crate::Expression::FixedPoint(0, 0),
            crate::Expression::FixedPoint(0, 0),
        ]);
        let value = crate::evaluate(&expr);
        assert_eq!(0.0, value, "0.0 expected, value = {}", value);
    }

    #[test]
    fn test_fixed_point_addition_4() {
        let expr = crate::Expression::Addition(vec![
            crate::Expression::FixedPoint(1, 02), //the leading 0 is not required
            crate::Expression::FixedPoint(1, 2),  // proof of the statement above
        ]);
        let value = crate::evaluate(&expr);
        assert_eq!(2.04, value, "2.04 expected, value = {}", value);
    }

    #[test]
    fn test_fixed_point_addition_5() {
        let expr = crate::Expression::Addition(vec![
            crate::Expression::FixedPoint(1, 20),
            crate::Expression::FixedPoint(1, 02), 
        ]);
        let value = crate::evaluate(&expr);
        assert_eq!(2.22, value, "2.22 expected, value = {}", value);
    }

    #[test]
    fn test_fixed_point_addition_6() {
        let expr = crate::Expression::Addition(vec![
            crate::Expression::FixedPoint(0, 20),
            crate::Expression::FixedPoint(0, 02), 
        ]);
        let value = crate::evaluate(&expr);
        assert_eq!(0.22, value, "0.22 expected, value = {}", value);
    }

    #[test]
    fn test_fixed_point_addition_7() {
        let expr = crate::Expression::Addition(vec![
            crate::Expression::FixedPoint(2, 0),
            crate::Expression::FixedPoint(2, 0), 
        ]);
        let value = crate::evaluate(&expr);
        assert_eq!(4.0, value, "4 expected, value = {}", value);
    }
}
fn main() {
    println!("Hello, world!");
}
