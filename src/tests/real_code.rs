use crate::parser;

// These are all tests to see if they parse *at all*, not for parsing correctness

#[test]
fn mandelbrot() {
    const SRC: &str = include_str!("../../mlog_files/mandelbrot.mlog");

    let lexer = parser::Lexer::new(SRC);
    let _ = lexer.map(|x| x.unwrap()).collect::<Vec<_>>();
}

#[test]
fn odd_supply() {
    const SRC: &str = include_str!("../../mlog_files/odd_supply.mlog");

    let lexer = parser::Lexer::new(SRC);
    let _ = lexer.map(|x| x.unwrap()).collect::<Vec<_>>();
}

#[test]
fn base_builder() {
    const SRC: &str = include_str!("../../mlog_files/base_builder.mlog");

    let lexer = parser::Lexer::new(SRC);
    let _ = lexer.map(|x| x.unwrap()).collect::<Vec<_>>();
}

#[test]
fn power_plant() {
    const SRC: &str = include_str!("../../mlog_files/power_plant.mlog");

    let lexer = parser::Lexer::new(SRC);
    let _ = lexer.map(|x| x.unwrap()).collect::<Vec<_>>();
}

#[test]
fn unit_transport() {
    const SRC: &str = include_str!("../../mlog_files/unit_transport.mlog");

    let lexer = parser::Lexer::new(SRC);
    let _ = lexer.map(|x| x.unwrap()).collect::<Vec<_>>();
}
