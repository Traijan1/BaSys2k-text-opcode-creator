use std::{env, fs::File, io::Write};

const CHARS_PER_INSTRUCTION: usize = 4;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() == 1 {
        println!("You need to enter the sentence you want to translate.");
        return;
    }

    let vec = text_to_hexadecimal(&args[1]);

    println!("All chars in hexadecimal: ");
    for (char, hex) in args[1].chars().zip(vec.iter()) {
        println!("{}: {}", char, hex);
    };

    let instructions = create_instructions(vec);

    let mut file = File::create("output.txt").expect("File couldn't be created");
    file.write_all(instructions.as_bytes()).expect("Can't write to file");
    file.flush().expect("Can't flush file");
}

fn text_to_hexadecimal(text: &String) -> Vec<String> {
    let mut vec : Vec<String> = vec![];
    
    let _ = &text.chars().all(|char| {
        vec.push(format!("{:02x}", char as u8));
        true
    });
    
    vec
}

fn create_instructions(mut data: Vec<String>) -> String {
    let mut result = String::new();
    let mut address: u32 = 0;
    
    let padding = CHARS_PER_INSTRUCTION - data.len() % CHARS_PER_INSTRUCTION;

    for _ in 0..padding {
        data.push(String::from("00"));
    }

    for index in (0..data.len()).step_by(CHARS_PER_INSTRUCTION) {
        result += &format!("0x0000AA00{}{}{}{},\n", data[index], data[index + 1], data[index + 2], data[index + 3]);
        result += &format!("0x0003AA00{:08x}, \n", address);

        address += CHARS_PER_INSTRUCTION as u32;
    }

    result
}