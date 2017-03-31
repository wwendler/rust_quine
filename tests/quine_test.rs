use std::process::Command;

#[test]
fn quine_property_test() {
    let quine_output = Command::new("./target/debug/simple_quine")
        .output()
        .expect("could not run quine");
    let source_text = Command::new("cat")
        .arg("src/main.rs")
        .output()
        .expect("could not print source code with cat");
    let quine_output = String::from_utf8_lossy(&quine_output.stdout);
    let source_text = String::from_utf8_lossy(&source_text.stdout);
    assert_eq!(quine_output, source_text);
}
