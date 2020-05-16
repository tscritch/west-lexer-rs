use std::fs;

#[derive(Debug)]
enum Types {
    KeyFN,
    Identifier,
    
    TVoid,

    SymParenLeft,
    SymParenRight,
    SymBracketLeft,
    SymBracketRight,
}

#[derive(Debug)]
struct Token {
    t: Types,
    // ident_name: str
}

fn main() -> Result<(), Box<dyn std::error::Error + 'static>> {
    let contents = fs::read_to_string("test1.w")?;

    let mut tokens: Vec<Token> = Vec::new();

    let cs: Vec<char> = contents.chars().collect();

    let mut current: String = String::new();
    let mut getting_ident = false;

    println!("\n");
    
    for c in cs {
        println!("{}", c);
        if c == ' ' || c == '\n' {
            continue;
        }

        if getting_ident && ['(',')','{','}','[',']'].contains(&c) {
            tokens.push(Token { t: Types::Identifier });
            getting_ident = false;
            current.clear();
        }

        current.push(c);

        if getting_ident {
            continue
        }

        match current.as_str() {
            "_" => { tokens.push(Token { t: Types::TVoid }); current.clear(); },
            "fn" => { tokens.push(Token { t: Types::KeyFN }); getting_ident = true; current.clear(); },
            "(" => { tokens.push(Token { t: Types::SymParenLeft }); current.clear(); },
            ")" => { tokens.push(Token { t: Types::SymParenRight }); current.clear(); },
            "{" => { tokens.push(Token { t: Types::SymBracketLeft }); current.clear(); },
            "}" => { tokens.push(Token { t: Types::SymBracketRight }); current.clear(); },
            _ => ()
        }
    }

    println!("{:?}", tokens);

    println!("\n");
    Ok(())
}

// fn get_next_token() -> Token {

// }