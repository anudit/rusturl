use std::collections::HashMap;

fn add(cache: &mut HashMap<String, String>, id : &String, link : &String) {
    println!("{} : {}", id, link);
    cache.insert(id.to_string(), link.to_string());
}

fn del(cache: &mut HashMap<String, String>, id : &String) {
    println!("{}", id);
    cache.remove(&id.to_string());
}

fn exists(cache: &mut HashMap<String, String>, id : &String) -> bool{
    return cache.contains_key(id)
}

fn main(){

    let mut cache = HashMap::new();

    let id : String = "google".to_string();
    let link : String = "https://google.com".to_string();

    add(&mut cache, &id, &link);

    println!("{}", exists(&mut cache, &id) );
    println!("{}",  cache.len() );

    del(&mut cache, &id);

    println!("{}", exists(&mut cache, &id) );
    println!("{}",  cache.len() );

}
