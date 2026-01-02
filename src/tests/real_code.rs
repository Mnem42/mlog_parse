use crate::parser::{
    lexer::Lexer,
    statements::{Statement, WprocStatement},
};

// These are all tests to see if they parse *at all*, not for parsing correctness

#[test]
fn mandelbrot() {
    const SRC: &str = include_str!("../../mlog_files/golem/mandelbrot.mlog");

    let lexer: Lexer<Statement> = Lexer::new(SRC);
    let _ = lexer.map(|x| x.unwrap()).collect::<Vec<_>>();
}

#[test]
fn odd_supply() {
    const SRC: &str = include_str!("../../mlog_files/golem/odd_supply.mlog");

    let lexer: Lexer<Statement> = Lexer::new(SRC);
    let _ = lexer.map(|x| x.unwrap()).collect::<Vec<_>>();
}

#[test]
fn base_builder() {
    const SRC: &str = include_str!("../../mlog_files/golem/base_builder.mlog");

    let lexer: Lexer<Statement> = Lexer::new(SRC);
    let _ = lexer.map(|x| x.unwrap()).collect::<Vec<_>>();
}

#[test]
fn base_builder_wproc() {
    const SRC: &str = include_str!("../../mlog_files/golem/base_builder_wproc.mlog");

    let lexer: Lexer<WprocStatement> = Lexer::new(SRC);
    let _ = lexer.map(|x| x.unwrap()).collect::<Vec<_>>();
}

#[test]
fn power_plant() {
    const SRC: &str = include_str!("../../mlog_files/golem/power_plant.mlog");

    let lexer: Lexer<Statement> = Lexer::new(SRC);
    let _ = lexer.map(|x| x.unwrap()).collect::<Vec<_>>();
}

#[test]
fn unit_transport() {
    const SRC: &str = include_str!("../../mlog_files/golem/unit_transport.mlog");

    let lexer: Lexer<Statement> = Lexer::new(SRC);
    let _ = lexer.map(|x| x.unwrap()).collect::<Vec<_>>();
}
