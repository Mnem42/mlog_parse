use crate::parser::{Lexer, statements::WprocStatement};

#[test]
fn sample_v7a() {
    const SRC: &str = include_str!("../../mlog_files/samples/is_v7a.mlog");

    let parsed = Lexer::<WprocStatement>::new(SRC);

    let statements = parsed
        .map(|x| x.unwrap().to_string())
        .collect::<Vec<String>>()
        .join("\n");
    
    assert_eq!(SRC, statements)
}