mod utils;
use std::thread;



fn main() {
    println!("Miku Miku BEAAAAAAAMMM!!!");
    let reproduce = thread::spawn(||{
        utils::reproduce_music();
    });    
    let handle = utils::vampire_cpu();

    reproduce.join().unwrap();
    handle.join().unwrap();
    loop{}
}
