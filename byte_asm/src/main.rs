use byte_asm::scanner::Scanner;

fn main() {
    let file = std::env::args().nth(1).unwrap_or("test.s".to_owned());
    let data = std::fs::read_to_string(file).expect("failed to read the provided file");

    let mut scanner = Scanner::new(&data);
    while let Ok(token) = scanner.scan_token() {
        println!("{:?}", token);

        if token.eof() {
            break;
        }
    }
}
