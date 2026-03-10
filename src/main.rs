use cobol_lens::parser::parse;

fn main() {
    let input = "\
000100 IDENTIFICATION DIVISION.
000200 PROGRAM-ID. HELLO.
000300 PROCEDURE DIVISION.
000400 DISPLAY \"HELLO WORLD\".
000500 STOP RUN.
";

    let tree = parse(input);
    println!("{:#?}", tree);
}
