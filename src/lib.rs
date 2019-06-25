use logos::Logos;

#[derive(Logos, Debug, PartialEq)]
enum Token {
    #[end]
    End,

    #[error]
    Error,

    #[token = "print"]
    Print,

    #[regex = "\\d+"]
    Number,

    #[token = ";"]
    EndOfExp,
}

pub fn run(code: &str) {
    let mut lexer = Token::lexer(code);
    while lexer.token != Token::End {
        println!("{:?}: {}", lexer.token, lexer.slice());
        lexer.advance();
    }
    println!("Lexing done");
}
