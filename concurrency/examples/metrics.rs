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
        task_worker(idx, metrics.clone())?;
    }

    for _ in 0..M {
        request_worker(metrics.clone())?;
    }

    loop {
        thread::sleep(Duration::from_secs(3));
        // println!("{:?}", metrics.snap());
        println!("{}", metrics);
        
    }
}

fn task_worker(idx: usize, metrics: Metric) -> Result<()> {
    thread::spawn(move || {
        loop {
            let mut rng = rand::thread_rng();

            thread::sleep(Duration::from_millis(rng.gen_range(100..5000)));
            metrics.inc(format!("call thread worker: {}", idx))?;
        };
        #[allow(unreachable_code)]
        // 以下返回仅为避免编译错误
        Ok::<_, anyhow::Error>(())
    });

    Ok(())
}

fn request_worker(metrics: Metric) -> Result<()> {
    thread::spawn(move || {
        loop {
            let mut rng = rand::thread_rng();

            thread::sleep(Duration::from_millis(rng.gen_range(50..800)));
            let page_num = rng.gen_range(1..8);
            metrics.inc(format!("req.page.{}", page_num))?;
        };
        
        #[allow(unreachable_code)]
        // 以下返回仅为避免编译错误
        Ok::<_, anyhow::Error>(())
    });
    
    Ok(())
}