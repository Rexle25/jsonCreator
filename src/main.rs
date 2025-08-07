use std::array;
use std::io;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
struct station {
        name: String,
        pos: Option<(f64, f64)>,
        #[serde(rename = "type")]
        typ: String,
        text: String,
        answer1: String,
        answer2: String,
        next1: String,
        next2: String,
        url: String
    }

    #[derive(Serialize, Deserialize, Debug)]
    struct StationList {
        stations: Vec<station>,
    }

fn main() {
    
    

    let mut stations: Vec<station> = Vec::new();

    loop {
        println!("Was möchtest du tun? Station erstellen (Drücke die 1 dafür) oder JSON ausgeben? (Drücke die 2)");
        let mut entscheidungEingabe = String::new();

        io::stdin()
            .read_line(&mut entscheidungEingabe)
            .expect("Failed to read line");
        
        if (entscheidungEingabe.trim() ==  "1") {
            
            
            let mut name = setUserInput("Name eingeben");

            let mut str_lat = setUserInput("Latitude eingeben");
            let mut str_lon = setUserInput("Longitude eingeben");
            let lat = str_lat.parse::<f64>().ok();
            let lon = str_lon.parse::<f64>().ok();
            let pos = if let (Some(lat), Some(lon)) = (lat, lon) {
                Some((lat, lon))
            } else {
                None
            };

            let mut typ = setUserInput("Typ eingeben");

            let mut text = setUserInput("Text eingeben");

            let mut answer1 = setUserInput("Antwortsmöglichkeit 1 eingeben");

            let mut answer2 = setUserInput("Antwortsmöglichkeit 2 eingeben");

            let mut next1 = setUserInput("Nächste Station 1 eingeben");

            let mut next2 = setUserInput("Nächste Station 2 eingeben");

            let mut url = setUserInput("URL eingeben");


            let station = station {
                name,
                pos,
                typ,
                text,
                answer1,
                answer2,
                next1,
                next2,
                url

            };

            

            stations.push(station)







            




        } else {
            let wrapper = StationList { stations: stations.clone() };
            let json_string = serde_json::to_string_pretty(&wrapper).unwrap();
            println!("JSON wurde erstellt: {}", json_string);
            
        }


    }
}

fn getUserInput() -> String {

    let mut eingabe = String::new();

    io::stdin()
        .read_line(&mut eingabe)
        .expect("Failed to read line");
        
    return eingabe.trim().to_string();



        
}

fn setUserInput(text: &str) -> String {
    let mut eingabe = String::new();
    println!("{}", text);
    eingabe = getUserInput();
    println!("{}", eingabe);

    return eingabe.trim().to_string();
}