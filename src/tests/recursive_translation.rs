use crate::parser::{lexer::Lexer, statements::WprocStatement};

/// Compare while ignoring extra trailing tokens
fn compare_line(a: &str, b: &str) -> bool {
    a.split_whitespace()
        .zip(b.split_whitespace())
        .all(|(a, b)| a == b)
}

/// Inverted operation from the one in Lexer
fn do_renaming<'a>(tokens: &'a [&str]) -> Vec<&'a str> {
    match tokens {
        ["op", "b-and", rest @ ..] => {
            let mut vec = vec!["op", "and"];
            vec.extend(rest);
            vec
        }
        ["op", "or", rest @ ..] => {
            let mut vec = vec!["op", "lor"];
            vec.extend(rest);
            vec
        }
        ["op", "flip", rest @ ..] => {
            let mut vec = vec!["op", "not"];
            vec.extend(rest);
            vec
        }
        ["nop", ..] => {
            vec!["noop"]
        }
        x => x.into(),
    }
}

#[test]
fn sample_v7a() {
    // The slice is to remove the comments at the top to make it compare correctly
    let src: &str = &include_str!("../../mlog_files/samples/is_v7a.mlog")[107..];

    let parsed = Lexer::<WprocStatement>::new(src);
    let recursive_translated: Vec<_> = parsed.map(|x| x.unwrap().to_string()).collect();

    let val: Vec<_> = recursive_translated.iter().map(|x| x.as_str()).collect();

    let statements = do_renaming(&val);

    assert!(
        statements
            .iter()
            .zip(src.lines())
            .all(|(a, b)| compare_line(a, b))
    )
}

#[test]
fn sample_v8b() {
    // The slice is to remove the comments at the top to make it compare correctly
    let src: &str = &include_str!("../../mlog_files/samples/is_v8b.mlog")[107..];

    let parsed = Lexer::<WprocStatement>::new(src);
    let recursive_translated: Vec<_> = parsed.map(|x| x.unwrap().to_string()).collect();

    let val: Vec<_> = recursive_translated.iter().map(|x| x.as_str()).collect();

    let statements = do_renaming(&val);

    assert!(
        statements
            .iter()
            .zip(src.lines())
            .all(|(a, b)| compare_line(a, b))
    )
}
