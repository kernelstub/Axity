use axity::run_source;
use axity::AxityError;

#[test]
fn run_loop_prints() -> Result<(), AxityError> {
    let src = "let x: int = 0; while x < 3 { print(x); x = x + 1; }";
    let out = run_source(src)?;
    assert_eq!(out, "0\n1\n2\n");
    Ok(())
}

#[test]
fn type_error_undefined_var() {
    let src = "print(y);";
    let res = run_source(src);
    assert!(res.is_err());
}

#[test]
fn call_function_in_expr() -> Result<(), AxityError> {
    let src = "fn add(a: int, b: int) -> int { return a + b; } print(add(2,3));";
    let out = run_source(src)?;
    assert_eq!(out, "5\n");
    Ok(())
}

