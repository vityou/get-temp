use std::env;
use std::error;
use std::process;

fn main() -> Result<(), Box<dyn error::Error>> {
    let args: Vec<String> = env::args().collect();
    let temp_unit = get_units(&args).unwrap_or_else(|err| {
        eprintln!("Error parsing arguments: {}", err);
        eprintln!("Usage: {} [-f | -c]", args[0]);
        process::exit(1);
    });
    
    let json: serde_json::Value = ureq::get("http://wttr.in/")
        .query("format", "j1")
        .call()?
        .into_json()?;

    let temp = extract_temp_from_json(&json, &temp_unit)
        .unwrap_or_else(|| {
            eprintln!("Error parsing online weather data");
            process::exit(1);
        });

    println!("{}", temp);
    Ok(())
}

fn extract_temp_from_json<'a>(json: &'a serde_json::Value, temp_unit: &String) -> Option<&'a str> {
    json.get("current_condition")?
        .get(0)?
        .get(temp_unit)?
        .as_str()
}

fn get_units(args: &Vec<String>) -> Result<String, String> {
    if args.len() == 2 {
        if args[1] == "-c" {
            Ok(String::from("temp_C"))
        } else if args[1] != "-f" {
            Err(format!("Invalid argument: {}", args[1]))
        } else {
            Ok(String::from("temp_F"))
        }
    } else if args.len() > 2 {
        Err(String::from("Too many Arguments"))
    } else {
        Ok(String::from("temp_F"))
    }
}
