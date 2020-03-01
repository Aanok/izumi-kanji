extern crate csv;


fn get_first_arg() -> Result<std::ffi::OsString, Box<dyn std::error::Error>> {
    match std::env::args_os().nth(1) {
        None => Err(From::from("Please pass the CSV file as argument.")),
        Some(file_path) => Ok(file_path),
    }
}

fn run() -> Result<(), Box<dyn std::error::Error>> {
//     let file_path = get_first_arg()?;
    let file_path = if let Some(file_path) = std::env::args_os().nth(1) {
        Ok(file_path);
    } else {
        Err(From::from("Please pass the CSV file as argument."));
    };
    let mut reader = csv::ReaderBuilder::new()
        .has_headers(false)
        .from_path(file_path)?;
    for result in reader.records() {
        println!("{:?}", result);
    }
    Ok(())
}


fn main() {
    if let Err(err) = run() {
        println!("{}", err);
        std::process::exit(1);
    }
}
