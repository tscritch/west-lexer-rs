use std::fs;

#[derive(Debug)]
enum Types {
    KeyFN,
    KeyStruct,
    KeyReturn,

    Identifier,
    
    TVoid,
    TFloat,
    TInt,

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

enum LexState {
    Default,
    Identifier,
    Function,
    Literal
}

fn main() -> Result<(), Box<dyn std::error::Error + 'static>> {
    let contents = fs::read_to_string("test2.w")?;

    let mut tokens: Vec<Token> = Vec::new();

    let chunks = get_chunks(&contents);

    let mut current: String = String::new();
    let mut state = LexState::Default;

    println!("\n");

    println!("{:?}", chunks);

    for chunk in chunks {
        match chunk.as_str() {
            "_" => { tokens.push(Token { t: Types::TVoid }); },
            
            "fn" => { tokens.push(Token { t: Types::KeyFN }); },
            "struct" => { tokens.push(Token { t: Types::KeyStruct }); },
            "float" => { tokens.push(Token { t: Types::TFloat }); },
            "int" => { tokens.push(Token { t: Types::TInt }); },
            "return" => { tokens.push(Token { t: Types::KeyReturn }); },

            "(" => { tokens.push(Token { t: Types::SymParenLeft }); },
            ")" => { tokens.push(Token { t: Types::SymParenRight }); },
            "{" => { tokens.push(Token { t: Types::SymBracketLeft }); },
            "}" => { tokens.push(Token { t: Types::SymBracketRight }); },
            "update" => { tokens.push(Token { t: Types::Identifier }); },
            _ => println!("Unknown chunk: {}", chunk)
        }
    }
    
    // for i in 0..cs.len() {
    //     let c = cs[i];
    //     println!("{}", cs[i]);

    //     if c == ' ' || c == '\n' {
    //         continue;
    //     }

    //     // if getting_ident && ['(',')','{','}','[',']'].contains(&c) {
    //     //     tokens.push(Token { t: Types::Identifier });
    //     //     getting_ident = false;
    //     //     current.clear();
    //     // }

    //     // current.push(c);

    //     // if getting_ident {
    //     //     continue
    //     // }

    //     match current.as_str() {
    //         "_" => { tokens.push(Token { t: Types::TVoid }); current.clear(); },
    //         "fn" => { tokens.push(Token { t: Types::KeyFN }); getting_ident = true; current.clear(); },
    //         "(" => { tokens.push(Token { t: Types::SymParenLeft }); current.clear(); },
    //         ")" => { tokens.push(Token { t: Types::SymParenRight }); current.clear(); },
    //         "{" => { tokens.push(Token { t: Types::SymBracketLeft }); current.clear(); },
    //         "}" => { tokens.push(Token { t: Types::SymBracketRight }); current.clear(); },
    //         _ => ()
    //     }
    // }

    println!("{:?}", tokens);

    println!("\n");
    Ok(())
}

const WHITESPACE: [char; 3] = [' ', '\n', '\t'];
const BREAKERS: [char; 7] = ['{', '}', '(', ')', '[', ']', '|'];

fn get_chunks(string: &String) -> Vec<String> {
    let mut chunks: Vec<String> = Vec::new();
    let mut collect = String::new();

    for c in string.chars() {
        if WHITESPACE.contains(&c) { // if c is whitespace
            if !collect.is_empty() {
                chunks.push(collect.clone());
                collect.clear();
            }
            continue;
        }

        if BREAKERS.contains(&c) { // if c breaks the string
            if !collect.is_empty() {
                chunks.push(collect.clone()); // push the current chunk and then push the current char
                collect.clear();
            }
            collect.push(c);
            chunks.push(collect.clone());
            collect.clear();
            continue;
        }

        collect.push(c);
    }

    chunks
}

// fn get_next_token() -> Token {

// }