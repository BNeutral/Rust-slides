use std::thread;

fn main() {
    let data_from_main_thread = "data from main thread";
    let handle = thread::spawn(move || {
        for i in 1..10 {
            println!("Spawned thread print N� {}. Data: {}.", i, data_from_main_thread);
        }
    });

    for i in 1..5 {
        println!("Print N� {} from main thread.", i);
    }

    handle.join();
}