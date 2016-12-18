use std::io;

fn main() {
    loop {
        println!("You are about to create a text file.\nWhat should it be named?");
        let mut file_name = String::new();
        io::stdin().read_line(&mut file_name).expect("Couldn't read line.");

        let mut final_format = String::new();
        let formats = vec!["txt", "md", "html", "css"];
        println!("Which of the following formats would you like?");
        for format in formats {
            println!("> {}", format);
        }

        let mut acceptableFormat = false;
        while (!acceptableFormat) {
            // io::stdin().read_line(&mut final_format).expect("Typed in wrong data type");
            // match final_format {
            //     "txt" => acceptableFormat = true,
            //     "md" => acceptableFormat = true,
            //     _ => println!("Unknown format")
            // }
            acceptableFormat = true;
        }

    }
}
