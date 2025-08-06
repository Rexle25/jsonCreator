use std::array;
use std::io;

fn main() {
    struct station {
        name: String,
        pos: Option<Vec<String>>,
        typ: String,
        text: String,
        answer1: String,
        answer2: String,
        next1: String,
        next2: String,
        url: String
    }

    let mut stations: Vec<station> = Vec::new();

    loop {
        println!("Was möchtest du tun? Station erstellen (Drücke die 1 dafür) oder JSON ausgeben? (Drücke die 2)");
        let mut entscheidungEingabe = String::new();

        io::stdin()
            .read_line(&mut entscheidungEingabe)
            .expect("Failed to read line");
        
        if (entscheidungEingabe ==  "1") {



        } else {
            
            
        }


    }
}

fn getUserInput() {

    let mut eingabe = String::new();

    io::stdin()
        .read_line(&mut eingabe)
        .expect("Failed to read line");



        
}