/// Below you find a small start of a data type modelling the abstract syntax tree for an expression,
/// and a small evaluator function.
///
/// Please extend this evaluator in the following ways:
///
/// - Add support for multiplication and division
///
/// - We have added the the form "Summation(Vec<Expr>)", representing the sum of a list of expressions.
/// Question: why can we get away with Vec<Expr> enough in that case, instead of Box<Vec<Expr>> ?
///
/// - EXTRA: Since division can fail, the function eval needs to return an Option<i64>, where None indicates that a division by
///   zero has occurred. Can you change the code so that that errors are propagated correctly? (hint: use the ? syntax).

#[derive(PartialEq, Debug)]
enum Expr {
    Const(i64),
    Add(Box<Expr>, Box<Expr>),
    Sub(Box<Expr>, Box<Expr>),
    Mul(Box<Expr>, Box<Expr>),
    Div(Box<Expr>, Box<Expr>),
    Var,
    Summation(Vec<Expr>),
}

use std::ops::Deref;

// inject these two identifiers directly into the current namespace
use Expr::Const;
use Expr::Summation;
use Expr::Var;

// These are convenience functions, so you don't have to type "Box::new" as often
// when building test-data types
fn add(x: Expr, y: Expr) -> Expr {
    Expr::Add(Box::new(x), Box::new(y))
}

fn sub(x: Expr, y: Expr) -> Expr {
    Expr::Sub(Box::new(x), Box::new(y))
}

fn mul(x: Expr, y: Expr) -> Expr {
    Expr::Mul(Box::new(x), Box::new(y))
}

fn div(x: Expr, y: Expr) -> Expr {
    Expr::Div(Box::new(x), Box::new(y))
}

// ...

const ZERO: Expr = Const(0);

fn eval(expr: &Expr, var: i64) -> Option<i64> {
    // this should return an Option<i64>
    use Expr::*;
    match expr {
        Const(k) => Some(*k),
        Var => Some(var),
        Add(lhs, rhs) => {
            let result = eval(lhs, var)? + eval(rhs, var)?;
            Some(result)
            // if let (Some(l), Some(r)) = (eval(lhs, var), eval(rhs, var)) {
            //     Some(l + r)
            // } else {
            //     None
            // }
        }
        Sub(lhs, rhs) => {
            let result = eval(lhs, var)? - eval(rhs, var)?;
            Some(result)
        }
        Mul(lhs, rhs) => Some(eval(lhs, var)? * eval(rhs, var)?),
        Div(lhs, rhs) => {
            if rhs.deref() == &ZERO {
                None
            } else {
                Some(eval(lhs, var)? / eval(rhs, var)?)
            }
        }
        Summation(exprs) => {
            let mut acc = Some(0);
            for e in exprs {
                if acc.is_none() {
                    break;
                }

                if let Some(result) = eval(e, var) {
                    acc = acc.map(|sum| sum + result);
                } else {
                    acc = None
                }
            }
            acc
        }
    }
}

fn main() {
    let test = |expr| {
        let value = rand::random::<i8>() as i64;
        println!(
            "{:?} with Var = {} ==> {:?}",
            &expr,
            value,
            eval(&expr, value)
        );
    };

    test(Const(5));
    test(Var);
    test(sub(Var, Const(5)));
    test(sub(Var, Var));
    test(mul(Var, Const(5)));
    test(mul(Var, Var));
    test(div(Var, Const(1)));
    test(div(Var, Var));
    test(div(Var, ZERO));
    test(add(sub(Var, Const(5)), Const(5)));
    test(add(Const(5), div(Const(100), Const(0))));
    test(add(Const(5), div(Const(100), Const(1))));
    test(add(Var, add(Const(5), div(Const(100), Const(1)))));
    test(Summation(vec![Var, Const(1)]));
    test(Summation(vec![Var, Const(1), div(Const(1), ZERO)]));
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_cases() {
        let x = 42;
        assert_eq!(eval(&Const(5), x), 5);
        assert_eq!(eval(&Var, x), 42);
        assert_eq!(eval(&sub(Var, Const(5)), x), 37);
        assert_eq!(eval(&sub(Var, Var), x), 0);
        assert_eq!(eval(&add(sub(Var, Const(5)), Const(5)), x), 42);
        assert_eq!(eval(&Summation(vec![Var, Const(1)]), x), 43);

        assert_eq!(eval(&mul(Var, Const(5)), x), 210);
        assert_eq!(eval(&mul(Const(5), Const(5)), x), 25);
        assert_eq!(eval(&div(Var, Const(7)), x), 6);
        assert_eq!(eval(&div(Const(5), Const(5)), x), 1);
        assert_eq!(eval(&div(mul(Var, Const(5)), Const(5)), x), 42);
    }
}

// If you have time let and want to code more Rust: you can extend this exercise endlessly; one idea would be adding a Sigma(from,to,expr)
// constructor to Expr which computes the equivalent of (in LaTeX notation) \sum_{Var = from}^{to} expr; i.e. Sigma(Const(1), Const(5), Var) should be
// equivalent to Summation(vec![Const(1), Const(2), Const(3), Const(4), Const(5)]).
