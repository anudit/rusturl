extern crate redis;
use std::error::Error;

fn store(con: &mut redis::Connection, id: &String, link: &String) -> redis::RedisResult<()> {

    let _ : () = redis::cmd("SET").arg(id).arg(link).query(con)?;
    print!("[STORED] {}:{}\n", id, link);
    Ok(())
}

fn remove(con: &mut redis::Connection, id: &String) -> redis::RedisResult<()> {
    let _ : () = redis::cmd("DEL").arg(id).query(con)?;
    println!("[REMOVED] {}", id);
    Ok(())
}

fn does_exist(con: &mut redis::Connection, id: &String) -> bool {
    let check: i32 = redis::cmd("EXISTS").arg(id).query(con).unwrap();
    if check == 0 {
        return false;
    }
    else {
        return true;
    }
}

fn get_link(con: &mut redis::Connection, id: &String)  {
    let link: String = redis::cmd("GET").arg(id).query(con).unwrap();
    print!("[GET] {}\n", link);
}

fn print_header() {

    print!("  _     _____ _   _ _  _______   \n");
    print!(" | |   |_   _| \\ | | |/ /  __ \\  \n");
    print!(" | |     | | |  \\| | ' /| |__) | \n");
    print!(" | |     | | | . ` |  < |  _  /  \n");
    print!(" | |___ _| |_| |\\  | . \\| | \\ \\  \n");
    print!(" |_____|_____|_| \\_|_|\\_\\_|  \\_\\ \n");
    print!(" ________________________________ \n");

}

pub fn test_run() -> redis::RedisResult<()>  {

    let client = redis::Client::open("redis://127.0.0.1/")?;
    let mut con = client.get_connection()?;

    let id : String = "google".to_string();
    let link : String = "https://google.com".to_string();

    if does_exist(&mut con, &id) == false { // new entry
        match store(&mut con, &id, &link) {
            Err(err) => {
                println!("Could not execute does_exist:");
                println!("  {}: {}", err.category(), err.description());
            }
            Ok(()) => {
                get_link(&mut con, &id);
            }
        }
    }
    else{ // old entry
        print!("[REMOVING] {}\n", &id);
        match remove(&mut con, &id) {
            Err(err) => {
                println!("Could not execute remove:");
                println!("  {}: {}", err.category(), err.description());
            }
            Ok(()) => {
                match store(&mut con, &id, &link) {
                    Err(err) => {
                        println!("Could not execute store:");
                        println!("  {}: {}", err.category(), err.description());
                    }
                    Ok(()) => {
                        get_link(&mut con, &id);
                    }
                }
            }
        }
    }

    Ok(())
}

pub fn main() {

    print_header();

    match test_run() {
        Err(err) => {
            println!("Could not execute main:");
            println!("  {}: {}", err.category(), err.description());
        }
        Ok(()) => {}
    }
}
