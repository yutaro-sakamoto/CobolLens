use cobol_lens::parser::parse;

fn main() {
    let input = "\
       IDENTIFICATION DIVISION.
       PROGRAM-ID. HELLO.
       PROCEDURE DIVISION.
       DISPLAY \"HELLO WORLD\".
       STOP RUN.
";

    let tree = parse(input);
    println!("{:#?}", tree);
}
