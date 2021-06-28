#[allow(unused_imports)]
use concurrency::{tmutex, tspawn};

fn main() {
    // 消息传递
    tspawn();

    // 共享状态并发  锁
    tmutex();
}
