use std::io;

fn receive_input_string(message: &str) -> String {
    loop {
        // receive # of integers to generate
        println!("{}", message);

        //check input error
        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        let input: String = match input.trim().parse() {
            Ok(input_string) => input_string,
            Err(_) => continue,
        };
        break input;
    }
}

fn pig_latin_translate(input_string: &String) {
    let input_string_char: Vec<char> = input_string.chars().collect();
    let first_char = input_string_char[0];
    let vowels = ["a", "e", "i", "o", "u"];
    let consonants = ["b", "c", "d", "f", "g", "h", "j", "k", "l", "m", "n", 
    "p", "q", "r", "s", "t", "v", "w", "x", "y", "z"];
    
    match first_char {
        // input starts with consonant
        _start_cons if consonants.iter().any(|&i| i == first_char.to_string()) => {
            let mut output_char: Vec<char> = input_string.chars().collect();
            output_char.remove(0);
            let mut output_string: String = output_char.iter().collect();
            output_string.push_str("-");
            output_string.push(first_char);
            output_string = output_string.to_string();
            output_string.push_str("ay");
            println!("Pig latin for {:?} is {:?}.", input_string, output_string);
        },

        // input starts with vowel
        _start_vow if vowels.iter().any(|&i| i == first_char.to_string()) => {
            let mut output_string = String::from(input_string);
            output_string.push_str("-hay");
            println!("Pig latin for {:?} is {:?}.", input_string, output_string);
        },

        // input starts with invalid character (i.e. number)
        _ => println!("Invalid input. Please try with lower case letters.")       
    }


}

fn main() {
    let message = "Please input the strings to translate into pig latin.";
    
    // receive input String from the user
    let input_string = receive_input_string(&message);

    // translate the input String into pig latin
    pig_latin_translate(&input_string);
}
