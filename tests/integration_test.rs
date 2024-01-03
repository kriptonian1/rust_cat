use rust_cat::logic::logic::{numbered_lines, nonblank_numbered_lines};

#[test]
fn test_numbered_lines() {
    let lines = "Hello\nWorld\n".lines();
    let expected = "\t\x1b[34m1\x1b[0m Hello\t\x1b[34m2\x1b[0m World";
    let resolved_lines = numbered_lines(lines);
    assert_eq!(expected, resolved_lines);
}

#[test]
fn test_nonblank_numbered_lines() {
    let lines = "Hello\n\nWorld\n".lines();
    let expected = "\t\x1b[34m1\x1b[0m Hello\t\t\x1b[34m2\x1b[0m World";
    let resolved_lines = nonblank_numbered_lines(lines);
    assert_eq!(expected, resolved_lines);
}
