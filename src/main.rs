extern crate waitgroup;
use std::sync::Arc;
use std::thread;
fn main() {
    let wg = Arc::new(waitgroup::WaitGroup::new());
    wg.add(1);
    let wg2 = wg.clone();
    thread::spawn(move|| {
        thread::sleep_ms(3000);
        wg2.done();
    });
    wg.wait();
    println!("done")
}
