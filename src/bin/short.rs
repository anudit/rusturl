extern crate redis;

use redis::{Client, Commands, Connection, RedisResult};

fn store(id: &String, link: &String) -> redis::RedisResult<()> {
    let client = redis::Client::open("redis://127.0.0.1/")?;
    let con = client.get_connection()?;
    let _ : () = redis::cmd("SET").arg(id).arg(link).query(&con)?;
    print!("[STORED] {}:{}\n", id, link);
    Ok(())
}

fn remove(id: &String) -> redis::RedisResult<()> {
    let client = redis::Client::open("redis://127.0.0.1/")?;
    let con = client.get_connection()?;
    let _ : () = redis::cmd("REMOVE").arg(id).query(&con)?;
    print!("[REMOVED] {}\n", id);
    Ok(())
}

fn does_exist(id: &String) -> bool {
    let client = Client::open("redis://127.0.0.1/").unwrap();
    let con = client.get_connection().unwrap();
    let check: i32 = redis::cmd("EXISTS").arg(id).query(&con).unwrap();
    if check == 0 {
        return false;
    }
    else {
        return true;
    }
}

fn main() {

    print!("  _     _____ _   _ _  _______   \n");
    print!(" | |   |_   _| \\ | | |/ /  __ \\  \n");
    print!(" | |     | | |  \\| | ' /| |__) | \n");
    print!(" | |     | | | . ` |  < |  _  /  \n");
    print!(" | |___ _| |_| |\\  | . \\| | \\ \\  \n");
    print!(" |_____|_____|_| \\_|_|\\_\\_|  \\_\\ \n");
    print!(" ________________________________ \n");

    let id : String = "google2".to_string();
    let link : String = "https://google.com".to_string();

    if does_exist(&id) == false {
        store(&id, &link);
    }
    else{
        print!("[REMOVING] {}\n", &id);
        remove(&id);
        store(&id, &link);
    }

}
