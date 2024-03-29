use chrono::NaiveTime;
use core::future::Future;
use core::pin::Pin;
use croissant::{croissant, Croissant};
use std::{thread, time::Duration};
use tokio;

async fn job2(_: ()) {
    println!("Job2");
}

fn job2_wrapper(c: ()) -> Pin<Box<(dyn Future<Output = ()> + Send + Sync)>> {
    Box::pin(job2(c))
}

type C = ();

#[croissant]
async fn job3(c: C) {
    println!("Job3 {:?}", c)
}

#[croissant]
async fn hello(msg: String) {
    println!("{}", msg);
}

fn main() {
    let mut croissant = Croissant::new();
    croissant
        .add_async_job("Hello World".to_string(), Box::new(hello_croissant));
    croissant.run_every(Duration::from_secs(5));
    croissant.run_at(NaiveTime::from_hms(12, 0, 0));
    thread::sleep(Duration::from_secs(100));
}
// #[tokio::main]
// async fn main() {
//     // let mut c = Croissant::new();
//     // c.add_job((), |_| println!("job1"));
//     // c.add_async_job((), Box::new(job2_wrapper));
//     // c.add_async_job((), Box::new(job3_croissant));
//     // // c.run_every(Duration::from_secs(2));
//     // // c.run_every(Duration::from_secs(1));
//     // c.run_at(chrono::NaiveTime::from_hms(20, 49, 40));
//     // thread::sleep(Duration::from_secs(100));
//     let mut croissant = Croissant::new();
//     croissant.add_async_job("Hello World", |msg: &str| async move {
//         println!("{}", msg);
//     });
//     croissant.run_every(Duration::from_secs(5));
//     croissant.run_at(NaiveTime::from_hms(12, 0, 0));
// }
