extern crate regex;

use regex::Regex;
use std::env;
use std::fs::File;
use std::io::Read;
use std::process;

#[derive(Debug)]
struct Token {
    tag: &'static str,
    text: String,
}

// Expressões regulares e suas tags correspondentes
const TOKEN_EXPRS: &[(&str, &str)] = &[
    (r"FLOAT", "KW_FLOAT"),
    (r"BOOLEAN", "KW_BOOL"),
    (r"INTEGER", "KW_INT"),
    (r"PARAM", "KW_PARAM"),
    (r"VECTOR", "KW_VECTOR"),
    (r"TYPE", "KW_TYPE"),
    (r"FRAGMENT", "KW_FRAG"),
    (r"ENDFRAGMENT", "KW_ENDFRAG"),
    (r"IF", "KW_IF"),
    (r"ELSE", "KW_ELSE"),
    (r"WHILE", "KW_WHILE"),
    (r"READ", "KW_READ"),
    (r"WRITE", "KW_WRITE"),
    (r"SELECT", "KW_SELECT"),
    (r"BREAK", "KW_BREAK"),
    (r"CASE", "KW_CASE"),
    (r"DEFAULT", "KW_DEFAULT"),
    (r"TRUE", "KW_TRUE"),
    (r"FALSE", "KW_FALSE"),
    (r"==", "OP_EQ"),
    (r"<=", "OP_LE"),
    (r">=", "OP_GE"),
    (r"!=", "OP_NE"),
    (r"&&", "OP_AND"),
    (r"\|\|", "OP_OR"),
    (r"\|", "OP_BITOR"),
    (r"%", "OP_MOD"),
    (r"&", "OP_BITAND"),
    (r"!", "OP_NOT"),
    (r"=", "OP_ASSIGN"),
    (r"<", "OP_LT"),
    (r">", "OP_GT"),
    (r"\+", "OP_PLUS"),
    (r"-", "OP_MINUS"),
    (r"\*", "OP_TIMES"),
    (r"/", "OP_DIVIDE"),
    (r"\(", "L_PAREN"),
    (r"\)", "R_PAREN"),
    (r"\{", "L_BRACE"),
    (r"\}", "R_BRACE"),
    (r"\[", "L_BRACKET"),
    (r"\]", "R_BRACKET"),
    (r";", "SEMICOLON"),
    (r",", "COMMA"),
    (r"\.", "DOT"),
    (r"[a-zA-Z_][a-zA-Z0-9_]*", "ID"),
    (r"[0-9]+", "NUM"),
    (r"'[^']*'", "CHAR_LITERAL"),
    (r#""[^"]*""#, "STRING"),
    (r"/\*([^*]|\*[^/])*\*/", "COMMENT"),
    (r"[ \t\n\r]+", "WHITESPACE"),
];

fn lex(characters: &str) -> Vec<Token> {
    let mut pos = 0;
    let mut tokens = Vec::new();

    while pos < characters.len() {
        let mut matched = false;

        for &(pattern, tag) in TOKEN_EXPRS.iter() {
            let regex = Regex::new(pattern).unwrap();
            if let Some(mat) = regex.find(&characters[pos..]) {
                if mat.start() == 0 {
                    let text = mat.as_str().to_string();
                    if tag != "WHITESPACE" && tag != "COMMENT" {
                        tokens.push(Token { tag, text });
                    }
                    pos += mat.end();
                    matched = true;
                    break;
                }
            }
        }

        if !matched {
            eprintln!("Erro léxico: {}", &characters[pos..pos + 1]);
            process::exit(1);
        }
    }

    tokens.push(Token {
        tag: "EOF",
        text: "EOF".to_string(),
    });

    tokens
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        eprintln!("Usage: {} [file.frag]", args[0]);
        process::exit(1);
    }

    let filename = &args[1];
    let mut file = File::open(filename).expect("Não foi possível abrir o arquivo");
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("Não foi possível ler o arquivo");

    let tokens = lex(&contents);

    for token in tokens {
        println!("{:?}", token);
    }
}
