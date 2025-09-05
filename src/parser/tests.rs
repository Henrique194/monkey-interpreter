use super::*;

fn test_parser(input: &str, expected: &str) {
    let mut parser = Parser::new(input);
    let program = parser.parse();
    assert_eq!(program.to_string(), expected);
}

#[test]
fn let_statement() {
    let input = r#"
        let a = 100 + 12
        let b = 10 + 5 * 2 + 32
        let c = (a + b) * b / a
        let d = (a * a) + (b * b);
        let e = ((a + b) * c) / (d + d);
        let f = add((1 + multiply(10, 4) * 10), 2);
        let g = fn() { return 10; }();
        let h = true;
        let i = false;
        let j = if (x < y) { x } else { y };
        let k = !true;
        let l = --10;
    "#;
    let expected = "\
        let a = (100 + 12);\n\
        let b = ((10 + (5 * 2)) + 32);\n\
        let c = (((a + b) * b) / a);\n\
        let d = ((a * a) + (b * b));\n\
        let e = (((a + b) * c) / (d + d));\n\
        let f = add((1 + (multiply(10, 4) * 10)), 2);\n\
        let g = (fn() {\n\
            return 10;\n\
        })();\n\
        let h = true;\n\
        let i = false;\n\
        let j = if ((x < y)) {\n\
            x;\n\
        } else {\n\
            y;\n\
        };\n\
        let k = !(true);\n\
        let l = -(-(10));\n\
    ";
    test_parser(input, expected);
}

#[test]
fn return_statement() {
    let input = r#"
        return 100 + 12
        return 10 + 5 * 2 + 32
        return (a + b) * b / a
        return (a * a) + (b * b);
        return ((a + b) * c) / (d + d);
        return add((1 + (multiply(10, 4) * 10)), 2);
        return fn() { return 10; }();
        return true;
        return false;
        return if (x < y) { x } else { y };
    "#;
    let expected = "\
        return (100 + 12);\n\
        return ((10 + (5 * 2)) + 32);\n\
        return (((a + b) * b) / a);\n\
        return ((a * a) + (b * b));\n\
        return (((a + b) * c) / (d + d));\n\
        return add((1 + (multiply(10, 4) * 10)), 2);\n\
        return (fn() {\n\
            return 10;\n\
        })();\n\
        return true;\n\
        return false;\n\
        return if ((x < y)) {\n\
            x;\n\
        } else {\n\
            y;\n\
        };\n\
    ";
    test_parser(input, expected);
}

#[test]
fn function_literal() {
    let input = r#"
        fn() {
            return 10;
        }
        fn(x, y) {
            return x + y;
        }
        fn(x, y, z) {
            return x * x + y * y + z * z;
        }
        fn() {
            return fn() { return 10; };
        }
    "#;
    let expected = "\
        fn() {\n\
            return 10;\n\
        };\n\
        fn(x, y) {\n\
            return (x + y);\n\
        };\n\
        fn(x, y, z) {\n\
            return (((x * x) + (y * y)) + (z * z));\n\
        };\n\
        fn() {\n\
            return fn() {\n\
                return 10;\n\
            };\n\
        };\n\
    ";
    test_parser(input, expected);
}

#[test]
fn conditional() {
    let input = r#"
        if (false) {
            return true;
        }
        if (5 < 10) {
            x
        } else {
            y
        }
    "#;
    let expected = "\
        if (false) {\n\
            return true;\n\
        };\n\
        if ((5 < 10)) {\n\
            x;\n\
        } else {\n\
            y;\n\
        };\n\
    ";
    test_parser(input, expected);
}
