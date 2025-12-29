use crate::parser;

#[test]
fn mandelbrot() {
    const SRC: &str = include_str!("../../mlog_files/mandelbrot.mlog");

    let lexer = parser::Lexer::new(SRC);
    let x = lexer.map(|x| x.unwrap()).collect::<Vec<_>>();
    panic!("{:?}", x);
}