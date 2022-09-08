extern crate base64;
extern crate reqwest;
extern crate serde;
extern crate chrono;
extern crate anyhow;

use std::{fs, process, env, path::Path};
use std::io::Write;
use std::collections::HashMap;

use serde::Deserialize;


#[derive(Deserialize)]
struct Data {
    link: String,
    deletehash: String
}

#[derive(Deserialize)]
struct ImgUrl {
    data: Data
}


fn main() -> Result<(), anyhow::Error>
{
    if env::args().count() != 2 {
        println!("Usage: imgup <filename>");
        process::exit(1);
    }

    let file = env::args().last().expect("No argument given");
    let data = fs::read(&file)?;
    let b64  = base64::encode(&data);

    let mut image = HashMap::new();
    image.insert("image", &b64);

    let mut clientidpath = env::current_exe().expect("Couldn't get executable path name");
    _ = clientidpath.pop();
    clientidpath.push("imgup.secret");

    let clientid = fs::read_to_string(clientidpath).expect("Couldn't slurp secret file");
    let auth_value = format!("Client-ID {}", &clientid.trim());
    let res: ImgUrl = reqwest::blocking::Client::new()
        .post("https://api.imgur.com/3/image")
        .header(reqwest::header::AUTHORIZATION, auth_value.as_str())
        .form(&image)
        .send()?
        .json()?;

    println!("{}", res.data.link);
    eprintln!("Delete: {}", res.data.deletehash);

    let logfile = match env::var("IMGUP_LOGFILE") {
        Ok(s) => s,
        Err(_) => env::var("HOME")? + "/imgup.log",
    };
    let mut logfile = fs::OpenOptions::new().append(true).create(true).open(logfile)?;
    writeln!(&mut logfile,
             "{}\t{}\t{}\t{}",
             chrono::Local::now().format("%F_%T"),
             Path::new(&file).file_name()
                             .expect(&format!("Can't basename {}", &file))
                             .to_str()
                             .expect("Can't get basename to_str()"),
             res.data.link,
             &format!("https://imgur.com/delete/{}", res.data.deletehash)
             )?;

    Ok(())
}
