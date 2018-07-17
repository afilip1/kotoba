use kotoba::{eval::*, parser::*};

fn eval_eq(source: &str, expected: Type) {
    assert_eq!(
        Environment::new().eval(&Parser::new(source).parse()),
        expected
    );
}

#[test]
fn literals() {
    eval_eq("1", Type::Number(1.0));
    eval_eq("123.123", Type::Number(123.123));
    eval_eq("true", Type::Boolean(true));
    eval_eq("false", Type::Boolean(false));
    eval_eq("\"\"", Type::String("".to_string()));
    eval_eq("\"hello world\"", Type::String("hello world".to_string()));
    eval_eq("nil", Type::Nil);
}

#[test]
fn groupings() {
    eval_eq("(1)", Type::Number(1.0));
    eval_eq("(123.123)", Type::Number(123.123));
    eval_eq("(true)", Type::Boolean(true));
    eval_eq("(false)", Type::Boolean(false));
    eval_eq("(\"\")", Type::String("".to_string()));
    eval_eq("(\"hello world\")", Type::String("hello world".to_string()));
    eval_eq("(nil)", Type::Nil);

    eval_eq("((((1))))", Type::Number(1.0));
}

#[test]
fn unary_minus() {
    eval_eq("-1", Type::Number(-1.0));
    eval_eq("-123.123", Type::Number(-123.123));

    eval_eq("-(1)", Type::Number(-1.0));
    eval_eq("-(123.123)", Type::Number(-123.123));

    eval_eq("--1", Type::Number(1.0));
    eval_eq("-----------123.123", Type::Number(-123.123));
}

#[test]
fn unary_bang() {
    eval_eq("!true", Type::Boolean(false));
    eval_eq("!false", Type::Boolean(true));

    eval_eq("!(true)", Type::Boolean(false));
    eval_eq("!(false)", Type::Boolean(true));

    eval_eq("!!!!!!!!!true", Type::Boolean(false));
    eval_eq("!!!!!!!!false", Type::Boolean(false));
}

#[test]
fn mult_expr() {
    eval_eq("2 * 3", Type::Number(6.0));
    eval_eq("-2.5 * 4", Type::Number(-10.0));
    eval_eq("2 * 3 * 4", Type::Number(24.0));
    eval_eq(
        "2.1 * 3.2 * 4.3 * 5.4 * 6.5",
        Type::Number(2.1 * 3.2 * 4.3 * 5.4 * 6.5),
    );
}

#[test]
fn div_expr() {
    eval_eq("18 / 3", Type::Number(6.0));
    eval_eq("-100 / 2.5", Type::Number(-40.0));
    eval_eq("2 / 3 / 4", Type::Number(2.0 / 3.0 / 4.0));
    eval_eq(
        "2.1 / 3.2 / 4.3 / 5.4 / 6.5",
        Type::Number(2.1 / 3.2 / 4.3 / 5.4 / 6.5),
    );
}

#[test]
fn mixed_mult_and_div_expr() {
    eval_eq("18 / 3 * 4.5", Type::Number(27.0));
    eval_eq("-100 * 4 / 2.5", Type::Number(-160.0));

    eval_eq("18 / (3 * 4.5)", Type::Number(18.0 / (3.0 * 4.5)));
    eval_eq("-100 * (4 / 2.5)", Type::Number(-100.0 * (4.0 / 2.5)));
}

#[test]
fn add_expr() {
    eval_eq("18 + 3", Type::Number(21.0));
    eval_eq("-100 + 2.5", Type::Number(-97.5));
    eval_eq("100 + -2.5", Type::Number(97.5));
    eval_eq("2 + 3 + 4", Type::Number(9.0));
    eval_eq(
        "2.1 + 3.2 + 4.3 + 5.4 + 6.5",
        Type::Number(2.1 + 3.2 + 4.3 + 5.4 + 6.5),
    );
}

#[test]
fn sub_expr() {
    eval_eq("18 - 3", Type::Number(15.0));
    eval_eq("-100 - 2.5", Type::Number(-102.5));
    eval_eq("100 - -2.5", Type::Number(102.5));
    eval_eq("2 - 3 - 4", Type::Number(-5.0));
    eval_eq(
        "2.1 - 3.2 - 4.3 - 5.4 - 6.5",
        Type::Number(2.1 - 3.2 - 4.3 - 5.4 - 6.5),
    );
}

#[test]
fn mixed_add_and_sub_expr() {
    eval_eq("18 + 3 - 4.5", Type::Number(16.5));
    eval_eq("-100 - 4 + 2.5", Type::Number(-101.5));

    eval_eq("18 + (3 - 4.5)", Type::Number(16.5));
    eval_eq("-100 - (4 + 2.5)", Type::Number(-106.5));
}
