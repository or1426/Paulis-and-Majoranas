mod tokenize;
mod mathtree;
mod majorana;
use rustyline::error::ReadlineError;
use rustyline::{DefaultEditor, Result};


fn main() -> Result<()> {
    //let str = "c0 c1 + c1 c0";//"(1 + 2)*3 + (c6 + 7*1) (c8+c1)";  //"((1) (2*1*3 + 2*4) 3*c0 + (6+3i) c7 c1) c7";
    /*
    println!("{}", str);
    let v = tokenize::tokenize(str);
    let tree = mathtree::make_tree(v);

    println!("{}", &tree);

    let prodtree = mathtree::make_products(&tree);
    println!("{}", &prodtree);

    let mathtree = mathtree::make_mathexpr(&prodtree);
    println!("{}", &mathtree);

    println!("{}", mathtree.eval());
     */
    let mut rl = DefaultEditor::new()?;
    #[cfg(feature = "with-file-history")]
    if rl.load_history("history.txt").is_err() {
        println!("No previous history.");
    }
    loop {
        let readline = rl.readline(">>> ");
        match readline {
            Ok(line) => {
                match rl.add_history_entry(line.as_str()){
		    Err(err) => {
			println!("Error: {:?}", err);
			break
		    },
		    _ => {}
		}
                println!("{}", mathtree::eval(line.as_str()));
            },
            Err(ReadlineError::Interrupted) => {
                println!("CTRL-C");
                break
            },
            Err(ReadlineError::Eof) => {
                println!("CTRL-D");
                break
            },
            Err(err) => {
                println!("Error: {:?}", err);
                break
            }
        }
    }
    #[cfg(feature = "with-file-history")]
    rl.save_history("history.txt");
    Ok(())

	 
}
