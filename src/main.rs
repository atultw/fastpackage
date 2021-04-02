use std::fs::{File};
use std::io::{Write};
use std::path::Path;
use std::process::Command;
use std::env;

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {

    println!("Welcome to the package manager!");

    let cli_args: Vec<String> = env::args().collect();

    let url = &cli_args[1];
    // "https://github.com/git-for-windows/git/releases/download/v2.31.1.windows.1/Git-2.31.1-64-bit.exe";
    // "https://launcher.mojang.com/download/MinecraftInstaller.msi";
    let exename = Path::new(url).file_name().unwrap();
    let progtype = Path::new(url).extension().unwrap();

    let bytes = reqwest::get(url)
        .await?
        .bytes()
        .await?;

    // println!("bytes: {:?}", bytes);
    let mut file_to_save = File::create(exename).unwrap();

    let downloaded_file = File::write(&mut file_to_save, &*bytes).unwrap();

    drop(file_to_save);
    drop(downloaded_file);

    println!("Executing: {:?}", exename.to_str().unwrap());

    if progtype=="exe" {
        let runnable =
            Command::new(exename.to_str().unwrap())
            .output()
            .expect("There was an error launching.");

        // print!("{:?}", runnable.status);

        if runnable.status.success() {
            println!("The software was installed successfully.");
        } else {
            println!("There was an error installing.");
        }

    } else if progtype=="msi" {
        let runnable = Command::new("msiexec".to_owned())
            .arg("/i")
            .arg(exename.to_str().unwrap())
            .output()
            .expect("There was an error launching.");

        // println!("{:?}", runnable.status);

        if runnable.status.success() {
            println!("The software was installed successfully.");
        } else {
            println!("There was an error installing.");
        }
    }

    Ok(())
}