use std::fmt;

#[derive(Clone, Debug)]
pub enum Token {
    OpenParen,
    CloseParen,
    Plus,
    Times,
    String(String)
}

enum TokenOrSpace{
    Token(Token),
    Space
}

fn single_char_tokenize(c:char) -> Option<TokenOrSpace>{
    match c {
	'(' => Some(TokenOrSpace::Token(Token::OpenParen)),
	')' => Some(TokenOrSpace::Token(Token::CloseParen)),
	'+' => Some(TokenOrSpace::Token(Token::Plus)),
	'*' => Some(TokenOrSpace::Token(Token::Times)),
	' ' => Some(TokenOrSpace::Space),
	_ => None	
    }
}

impl Token{
    pub fn to_str(&self)-> &str {
	match self {
	    Token::OpenParen => "(",
	    Token::CloseParen => ")",
	    Token::Plus => "+",
	    Token::Times => "*",
	    Token::String(s) => s
	}
    }
}


impl fmt::Display for Token {    
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_str())
    }
}


pub fn tokenize(string: &str) -> Vec<Token> {
    let mut vec: Vec<Token> = Vec::new();

    let mut current_numeric_string :Option<String> = None;
    for c in string.chars() {
	match single_char_tokenize(c) {
	    Some(t) => match current_numeric_string {
		Some(s) => {
		    vec.push(Token::String(s));
		    current_numeric_string = None;
		    match t {
			TokenOrSpace::Token(t) => {vec.push(t);},
			TokenOrSpace::Space => {}
		    }
		},
		None => match t {
		    TokenOrSpace::Token(t) => {vec.push(t);},
		    TokenOrSpace::Space => {}
		}
	    },
	    None => match current_numeric_string {
		Some(ref mut s) => {
		    s.push(c);
		},
		None => {current_numeric_string = Some(String::from(c))}
	    }
	}
    }
    match current_numeric_string {
	Some(s) => {
	    vec.push(Token::String(s));
	},
	_ => {}
    }
    
    return vec;
}
