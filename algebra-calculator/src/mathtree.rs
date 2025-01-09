pub use crate::tokenize::{tokenize, Token};

use std::fmt;

use crate::majorana::MajoranaExpr;

#[derive(Debug)]
pub enum TokenTree {
    Children(Vec<TokenTree>),
    Leaf(Token)
}

fn _make_tree(vec: &Vec<Token>, idx: usize) -> (usize, TokenTree) {
    let mut i: usize = idx;

    let mut current_vec :Vec<TokenTree> = Vec::new();
    
    while i < vec.len() {	
	match &vec[i] {
	    Token::CloseParen => {return (i+1, TokenTree::Children(current_vec))},
	    _ => {}
	}

	let (new_i, val) = match &vec[i] {
	    Token::OpenParen => _make_tree(vec, i+1),
	    other => (i+1, TokenTree::Leaf(other.clone()))
	};
	i = new_i;
	current_vec.push(val);

		      
    }

    return (i, TokenTree::Children(current_vec));
}

impl fmt::Display for TokenTree {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
	match self {
	    TokenTree::Leaf(token) => {write!(f, "{}", token)},
	    TokenTree::Children(v) => {
		match write!(f, "(") {
		    Err(error) => {return Err(error)},
		    _ => {}
		}
		for val in &v[0..v.len() - 1] {
		    match write!(f, "{}, ", &val) {
			Err(error) => {return Err(error)},
			_ => {}
		    }		    
		}
		write!(f, "{})", &v[v.len() - 1])
	    }
	}
    }
}

pub fn make_tree(vec: Vec<Token>) -> TokenTree {    
    let (_, val) = _make_tree(&vec, 0);
    return val;
}


#[derive(Debug)]
pub enum TokenProductTree {
    Children(Vec<TokenProductTree>),
    Product(Vec<TokenProductTree>),
    Leaf(Token)
}

pub fn make_products(token_tree: &TokenTree) -> TokenProductTree {
    match &token_tree{
	TokenTree::Leaf(token) => TokenProductTree::Leaf(token.clone()),
	TokenTree::Children(vec) => {
	    let mut new_vec : Vec<TokenProductTree> = Vec::new();
	    let mut idx = 0;
	    while idx < vec.len() {
		let new_val = match &vec[idx] {
		    TokenTree::Leaf(Token::Times) => {
			let mut product_vec: Vec<TokenProductTree> = Vec::new();
			match new_vec.pop() {
			    Some(token) => {
				idx += 1;				
				product_vec.push(token);
				product_vec.push(make_products(&vec[idx]));			
			    },
			    None => {},
			};
			TokenProductTree::Product(product_vec)
		    },
		    TokenTree::Leaf(Token::Plus) => TokenProductTree::Leaf(Token::Plus),
		    other => { //if we have two non-operator tokens in a row its an implicit product
			let new_other : TokenProductTree = make_products(other);
			match new_vec.pop() {
			    Some(TokenProductTree::Leaf(Token::Plus)) => {
				new_vec.push(TokenProductTree::Leaf(Token::Plus));
				new_other
			    },
			    None => new_other,
			    Some(thing) => {
				TokenProductTree::Product(vec![thing, new_other])
			    }
			}
		    }
		};
		new_vec.push(new_val);
		idx += 1;
	    }
	    TokenProductTree::Children(new_vec)
	}
    }
}

impl fmt::Display for TokenProductTree {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
	match self {
	    TokenProductTree::Leaf(token) => {write!(f, "{}", token)},
	    TokenProductTree::Children(v) => {
		match write!(f, "(") {
		    Err(error) => {return Err(error)},
		    _ => {}
		}
		for val in &v[0..v.len() - 1] {
		    match write!(f, "{}, ", &val) {
			Err(error) => {return Err(error)},
			_ => {}
		    }		    
		}
		write!(f, "{})", &v[v.len() - 1])
	    },
	    TokenProductTree::Product(v) => {
		match write!(f, "[") {
		    Err(error) => {return Err(error)},
		    _ => {}
		}
		for val in &v[0..v.len() - 1] {
		    match write!(f, "{} * ", &val) {
			Err(error) => {return Err(error)},
			_ => {}
		    }		    
		}
		write!(f, "{}]", &v[v.len() - 1])
	    }
	}
    }
}

pub enum MathExpr {
    //Number(Complex<f64>),
    MajoranaExpr(MajoranaExpr),
    Product(Vec<MathExpr>),
    Sum(Vec<MathExpr>)
}

impl fmt::Display for MathExpr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
	match self {
	    //MathExpr::Number(val) => {write!(f, "{}", val)},
	    MathExpr::MajoranaExpr(val) => {write!(f, "{}", val)},
	    MathExpr::Product(v) => {
		match write!(f, "[") {
		    Err(error) => {return Err(error)},
		    _ => {}
		}
		for val in &v[0..v.len() - 1] {
		    match write!(f, "{} * ", &val) {
			Err(error) => {return Err(error)},
			_ => {}
		    }		    
		}
		write!(f, "{}]", &v[v.len() - 1])
	    }
	    MathExpr::Sum(v) => {
		match write!(f, "(") {
		    Err(error) => {return Err(error)},
		    _ => {}
		}
		for val in &v[0..v.len() - 1] {
		    match write!(f, "{} + ", &val) {
			Err(error) => {return Err(error)},
			_ => {}
		    }		    
		}
		write!(f, "{})", &v[v.len() - 1])
	    }
	}
    }
}

pub fn make_mathexpr(tp_tree: &TokenProductTree) -> MathExpr {
    match &tp_tree {
	TokenProductTree::Product(vec) => MathExpr::Product(vec.iter().map(make_mathexpr).collect()),
	TokenProductTree::Leaf(Token::String(str)) => MathExpr::MajoranaExpr(MajoranaExpr::from_str(str)),
	TokenProductTree::Children(vec) => {
	    let mut new_vec : Vec<MathExpr> = Vec::new();
	    for val in vec {
		match val {
		    TokenProductTree::Leaf(Token::Plus) => {},
		    other => {new_vec.push(make_mathexpr(other))}
		}
	    }
	    MathExpr::Sum(new_vec)
	},
	_ => {panic!("we should never get here in make_mathexpr")}
    }
}

impl MathExpr{
    pub fn eval(self) -> MajoranaExpr {
	match self {
	    MathExpr::MajoranaExpr(val) => val,
	    MathExpr::Sum(vec) => vec.into_iter().map(|x| x.eval()).sum(),
	    MathExpr::Product(vec) => vec.into_iter().map(|x| x.eval()).product()
	}
    }
}

pub fn eval(s:&str) -> MajoranaExpr {
    make_mathexpr(&make_products(&make_tree(tokenize(s)))).eval()
}

