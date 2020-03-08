extern crate csv;
extern crate rand;

use rand::seq::SliceRandom;

#[derive(Debug, serde::Deserialize)]
struct Entry {
    id: String,
    latin: String,
    kanji: String,
    kana: String
}


fn print_help() {
    eprintln!("izumi-kanji [ all | latest | n ] <file>");
}


fn parse_csv(what : &String, csv_path : &String)
    -> Result<Vec<Entry>, Box<dyn std::error::Error>> {
    let mut entries : Vec<Entry> = Vec::new();

    let mut reader = csv::ReaderBuilder::new()
        .has_headers(false)
        .from_path(csv_path)?;

    match &what[..] {
        "all" => {
            for result in reader.deserialize() {
                let entry : Entry = result?;
                entries.push(entry);
            }
        },
        "latest" => {
            let header: Entry = reader.headers()?.deserialize(None)?;
            for result in reader.deserialize() {
                let entry : Entry = result?;
                if entry.id == header.id {
                    entries.push(entry);
                }
            }
        },
        _ => {
            for result in reader.deserialize() {
                let entry : Entry = result?;
                if &entry.id == what {
                    entries.push(entry);
                }
            }
        }
    }

    Ok(entries)
}


fn ask(entries : &mut Vec<Entry>) -> std::io::Result<()> {
    let mut input = String::new();
    entries.shuffle(&mut rand::thread_rng());
    println!("==========");
    for entry in entries {
        println!("Query: {}", entry.latin);
        std::io::stdin().read_line(&mut input)?;
        println!("Solution: {} ({})\n---", entry.kanji, entry.kana);
    }
    Ok(())
}


fn run(what : &String, csv_path : &String)
    -> Result<(), Box<dyn std::error::Error>> {
    let mut entries = parse_csv(what, csv_path)?;
    loop { ask(&mut entries)?; }
}


fn main() {
    let args: Vec<String> = std::env::args().collect();

    match args.len() {
        1 => {
            print_help();
            std::process::exit(1);
        },
        2 => {
            let cmd = &"all".to_string();
            let csv_path = &args[1];
            if let Err(err) = run(cmd, csv_path) {
                eprintln!("{}", err);
                std::process::exit(1);
            }
        },
        3 => {
            let cmd = &args[1];
            let csv_path = &args[2];
            if let Err(err) = run(cmd, csv_path) {
                eprintln!("{}", err);
                std::process::exit(1);
            }
        },
        _ => {
            print_help();
            std::process::exit(1);
        }
    }
}
