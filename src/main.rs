use std::fs::{self, File};
use std::io::{self, Write, BufRead, BufReader};
use std::path::Path;

fn main() -> io::Result<()> {
    loop {
        let files = fs::read_dir(".")?
            .filter_map(|entry| {
                let entry = entry.ok()?;
                let path = entry.path();
                if path.is_file() && path.extension()? == "menuitem" {
                    path.file_stem()?.to_str().map(String::from)
                } else {
                    None
                }
            })
            .collect::<Vec<String>>();

        for (index, file) in files.iter().enumerate() {
            println!("{}: {}", index + 1, file);
        }
        println!("{}: quit", files.len() + 1);
        println!("{}: new", files.len() + 2);

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).unwrap();
        let choice = choice.trim().parse::<usize>().unwrap_or(0);

        if choice == files.len() + 1 {
            break;
        } else if choice == files.len() + 2 {
            add_new_command()?;
        } else if choice >= 1 && choice <= files.len() {
            let filename = format!("{}.menuitem", files[choice - 1]);
            let file = File::open(filename)?;
            let command = BufReader::new(file).lines().next().unwrap()?;
            println!("Executing: {}", command);
            // Here you should execute the command. In Rust, you'd typically use std::process::Command,
            // but note that executing arbitrary commands can be dangerous.
            // std::process::Command::new("sh").arg("-c").arg(&command).spawn()?.wait()?;
        } else {
            println!("Invalid choice. Please choose a valid number.");
        }
    }

    Ok(())
}

fn add_new_command() -> io::Result<()> {
    println!("Type in the command f.ex: ssh -p 2222 pi@123.123.123.1");
    let mut command = String::new();
    io::stdin().read_line(&mut command)?;
    let command = command.trim();

    println!("Command was: {}", command);
    println!("Nickname for this command:");
    let mut nick = String::new();
    io::stdin().read_line(&mut nick)?;
    let nick = nick.trim();

    let filename = format!("./{}.menuitem", nick);
    let mut file = File::create(Path::new(&filename))?;
    writeln!(file, "{}", command)?;

    println!("Command saved. Returning to menu.");
    Ok(())
}
