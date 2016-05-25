use std::env;

const OP_ADD: &'static str = "+";
const OP_SUB: &'static str = "-";
const OP_MUL: &'static str = "*";
const OP_DIV: &'static str = "/";

/*
fn checkNumber(n: String) -> f32 {
   n.parse::<f32>().unwrap();
}*/


fn main(){

	let args: Vec<String> = env::args().collect();
    let result:f32;
    let mut succeed:bool = true;
    
	let p1 = args[1].parse::<f32>().unwrap();
	let p2 = (args[3]).parse::<f32>().unwrap();
	match args[2].as_ref() {
            OP_ADD => result = p1 + p2,
            OP_SUB => result = p1 - p2,
            OP_MUL => result = p1 * p2,
            OP_DIV => result = p1 / p2,
            _ => {
                result = -1.0;
                succeed = false;
                println!("error: Unknown operator.");
            }
        }

    if succeed {
    	println!("succeed reading inputs {} {} {} = {}", p1, args[2], p2, result);
    }
}
