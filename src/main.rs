mod timer;
mod simple_future;

use {
    crate::timer::TimerFuture,
    crate::simple_future::new_executor_and_spawner,
    std::time::Duration,
};

fn main() {
    let (executor, spawner) = new_executor_and_spawner();

    spawner.spawn(async {
        println!("waiting 2 seconds...");
        TimerFuture::new(Duration::new(2, 0)).await;
        println!("2 seconds elapsed!");
    });

    spawner.spawn(async {
        println!("waiting 1 second...");
        TimerFuture::new(Duration::new(1, 0)).await;
        println!("1 second elapsed!");
    });

    // Drop すると Receiver の recv が Err を返すので
    // Executor の while ループが終了する。
    drop(spawner);

    executor.run();
}
