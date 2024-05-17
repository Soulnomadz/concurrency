use anyhow::Result;
use rand::Rng;

use std::thread;
use std::time::Duration;

use concurrency::Metric;

const N: usize = 2;
const M: usize = 4;

fn main() -> Result<()> {
    let metrics = Metric::new();
    println!("{:?}", metrics.snap());

    for idx in 0..N {
        task_worker(idx, metrics.clone());
    }

    for _ in 0..M {
        request_worker(metrics.clone());
    }

    loop {
        thread::sleep(Duration::from_secs(3));
        println!("{:?}", metrics.snap());
    }

    // for i in 0..100 {
    //     metrics.inc("rbac.web01")?;
    //     metrics.inc("nzp-jxhd-server.app01")?;
    //     if i % 2 == 0 {
    //         metrics.inc("oa-jz-shop-server.app06")?;
    //     }
    // }

    // for _ in 0..27 {
    //     metrics.inc("weixin-server.app02")?;
    // }

    // println!("{:?}", metrics.snap());
    // Ok(())
}

fn task_worker(idx: usize, metrics: Metric) {
    thread::spawn(move || loop {
        let mut rng = rand::thread_rng();

        thread::sleep(Duration::from_millis(rng.gen_range(100..5000)));
        metrics.inc(format!("call thread worker: {}", idx)).unwrap();
    });

    // Ok(())
}

fn request_worker(metrics: Metric) {
    thread::spawn(move || loop {
        let mut rng = rand::thread_rng();

        thread::sleep(Duration::from_millis(rng.gen_range(50..800)));
        let page_num = rng.gen_range(1..8);
        metrics.inc(format!("req.page.{}", page_num)).unwrap();
    });
    
    // Ok(())
}