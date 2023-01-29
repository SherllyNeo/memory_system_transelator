mod input_wrapper;
extern crate serde;
use serde_json;
use std::fs;
use std::io::Result;
use input_wrapper::get_input;
use std::time::Duration;
use std::thread::sleep;

//requires json structure
// [
// {'number':'001','image':'father'},
// {'number':'002','image':'mother'},
//
//
//          ]


fn get_image(number: String) -> String {
//worst hash function in the world


let mut new_number = number.clone();
//forces string to be 3 characters long
while new_number.len() < 3 {
    println!("adding zeros to end to make images... \n");
    new_number = [new_number,"0".to_string()].join("");
    println!("{}\n",new_number)
}
let number = new_number;
let file = fs::File::open("memory.json")
    .expect("file should open read only");
let json: Vec<serde_json::Value> = serde_json::from_reader(file)
    .expect("file should be proper JSON");
let filter_json = json.into_iter().filter(|x| x.get("number").expect("expected number") == &number).collect::<Vec<serde_json::Value>>();
let image = filter_json[0].get("image")
    .expect("file should have number key").to_string();
return image;

}



fn main() -> Result<()> {

println!("Please type in a number to be changed into images. Anything not mapped in memory json will cause a panic");


let user_input = get_input();
// split every 3 characters
let n = 3;
let split_by_n = user_input.chars()
    .enumerate()
    .flat_map(|(i, c)| {
        if i != 0 && i % n == 0 {
            Some(' ')
        } else {
            None
        }
        .into_iter()
        .chain(std::iter::once(c))
    })
    .collect::<String>();

let vector_input = split_by_n.split(' ').collect::<Vec<&str>>();
let img_vectors = vector_input.into_iter().map(|x| get_image(x.to_string()) ).collect::<Vec<String>>();
// turn vector into string for printing
let img_string = img_vectors.join(", ");
println!("{}",img_string);

let time = Duration::from_secs(30);

// sleep
sleep(time);
Ok(())
}
