use std::{thread, time};
use std::io::{Write, stdout};
use chrono::prelude::DateTime;
use chrono::{Local,Utc,TimeZone};
use std::time::{SystemTime,UNIX_EPOCH};
use crossterm::{QueueableCommand, cursor};

fn unix_to_binary(n: i64) -> String{
    let mut i = 1 << 31;
    let mut space = 0;
    let mut res:String = "".to_string();
    while i > 0 {
        space += 1;
        if n&i != 0{
            res.push('1');
        }
        else {
            res.push('0');
        }
        if space%8 == 0 && res.len() < 32{
            res.push(' ')
        }
        i = i / 2
    }
    res
}

fn get_unix_time() -> i64{
    SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs() as i64
}
fn main() {
    loop {
        let utc = DateTime::<Utc>::from(Utc.timestamp(get_unix_time(), 0)).format("%d-%m-%Y %H:%M:%S");
        let local = DateTime::<Local>::from(Local.timestamp(get_unix_time(),0)).format("%d-%m-%Y %H:%M:%S");
        stdout().queue(cursor::SavePosition).unwrap();
        stdout().write("Decimal: ".as_bytes()).unwrap();
        stdout().write(get_unix_time().to_string().as_bytes()).unwrap();
        stdout().write("\nBinario: ".as_bytes()).unwrap();
        stdout().write(unix_to_binary(get_unix_time()).as_bytes()).unwrap();
        stdout().write("\nUTC: ".as_bytes()).unwrap();
        stdout().write(utc.to_string().as_bytes()).unwrap();
        stdout().write("\nLocal: ".as_bytes()).unwrap();
        stdout().write(local.to_string().as_bytes()).unwrap();
        stdout().queue(cursor::RestorePosition).unwrap();
        stdout().flush().unwrap();
        thread::sleep(time::Duration::from_secs(1));
    }
}