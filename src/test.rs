mod parser {
    use crate::parser;
    use crate::parser::instructions::{Argument, ConditionOp, Statement};
    use pretty_assertions::assert_eq;

    #[test]
    fn single_input() {
        const SRC: &str = r#"
            set test "12" # A comment
            set testb 3.14159 # PI
            set testc 0xDEADBEEF # dead beef
            set testd -0b01010101 # binary
        "#;

        let lexer = parser::Lexer::new(SRC);

        assert_eq!(
            lexer.map(|x| x.unwrap()).collect::<Vec<_>>(),
            [
                Statement::Set {
                    o: "test".to_string(),
                    i: Argument::String("12")
                },
                Statement::Set {
                    o: "testb".to_string(),
                    #[expect(clippy::approx_constant)]
                    i: Argument::Number(3.14159)
                },
                Statement::Set {
                    o: "testc".to_string(),
                    i: Argument::Number(0xDEADBEEFu32 as f64)
                },
                Statement::Set {
                    o: "testd".to_string(),
                    i: Argument::Number(-85.0)
                }
            ]
        )
    }

    #[test]
    fn operation() {
        const SRC: &str = r#"
            op add a 12 -0x05
            op sub b -0b01 5
            op mul c  0x08 a
            op div d b 0b1001010
        "#;

        let lexer = parser::Lexer::new(SRC);

        assert_eq!(
            lexer.map(|x| x.unwrap()).collect::<Vec<_>>(),
            [
                Statement::OpAdd {
                    c: "a".to_string(),
                    a: Argument::Number(12.0),
                    b: Argument::Number(-5.0)
                },
                Statement::OpSub {
                    c: "b".to_string(),
                    a: Argument::Number(-1.0),
                    b: Argument::Number(5.0)
                },
                Statement::OpMul {
                    c: "c".to_string(),
                    a: Argument::Number(8.0),
                    b: Argument::Variable("a")
                },
                Statement::OpDiv {
                    c: "d".to_string(),
                    a: Argument::Variable("b"),
                    b: Argument::Number(74.0)
                },
            ]
        )
    }

    #[test]
    fn ops_with_jump() {
        const SRC: &str = r#"
            jl1:
                jump 2 greaterThan a 2
                nop
                op add a 12 -0x05
                op sub b -0b01 5
                op mul c  0x08 a
                op div d b 0b1001010
            jump jl1 always
        "#;

        let lexer = parser::Lexer::new(SRC);

        assert_eq!(
            lexer.map(|x| x.unwrap()).collect::<Vec<_>>(),
            [
                Statement::Jump {
                    index: 2,
                    cond: ConditionOp::GreaterThan,
                    lhs: Some(Argument::Variable("a"),),
                    rhs: Some(Argument::Number(2.0),),
                },
                Statement::Noop,
                Statement::OpAdd {
                    c: "a".to_string(),
                    a: Argument::Number(12.0),
                    b: Argument::Number(-5.0)
                },
                Statement::OpSub {
                    c: "b".to_string(),
                    a: Argument::Number(-1.0),
                    b: Argument::Number(5.0)
                },
                Statement::OpMul {
                    c: "c".to_string(),
                    a: Argument::Number(8.0),
                    b: Argument::Variable("a")
                },
                Statement::OpDiv {
                    c: "d".to_string(),
                    a: Argument::Variable("b"),
                    b: Argument::Number(74.0)
                },
                Statement::Jump {
                    index: 1,
                    cond: ConditionOp::Always,
                    lhs: None,
                    rhs: None
                }
            ]
        )
    }

    #[test]
    fn display() {
        let tokens = [Statement::OpAdd {
            c: "a".to_string(),
            a: Argument::Number(12.0),
            b: Argument::Number(-5.0),
        }];

        assert_eq!(tokens.map(|x| x.to_string()), ["op add a -5 12"])
    }
}
