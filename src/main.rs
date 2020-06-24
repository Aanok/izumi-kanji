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


const HELP_TEXT : &'static str = "izumi-kanji [ all | latest | n ] <file>";


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

    if entries.len() == 0 {
        return Err(From::from("No entries found matching query."));
    }
    Ok(entries)
}


fn ask(entries : &mut Vec<Entry>) -> std::io::Result<()> {
    let mut input = String::new();
    entries.shuffle(&mut rand::thread_rng());
    for entry in entries {
        println!("Query: {}", entry.latin);
        std::io::stdin().read_line(&mut input)?;
        println!("Solution: {} ({})\n--------------------", entry.kanji, entry.kana);
    }
    Ok(())
}


fn run(what : &String, csv_path : &String)
    -> Result<(), Box<dyn std::error::Error>> {
    let mut entries = parse_csv(what, csv_path)?;
    loop {
        ask(&mut entries)?;
        println!("\x1b[F====================\n");
    }
}


fn main() {
    let args: Vec<String> = std::env::args().collect();

    if let Err(err) =
        match args.len() {
            1 => {
                Err(From::from(HELP_TEXT))
            },
            2 => {
                let cmd = &"latest".to_string();
                let csv_path = &args[1];
                run(cmd, csv_path)
            },
            3 => {
                let cmd = &args[1];
                let csv_path = &args[2];
                run(cmd, csv_path)
            },
            _ => {
                Err(From::from(HELP_TEXT))
            }
        } {
            eprintln!("{}", err);
            std::process::exit(1);
        }

}
