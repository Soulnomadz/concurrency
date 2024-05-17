use anyhow::Result;

use concurrency::Metric;

fn main() -> Result<()> {
    let mut metrics = Metric::new();

    for i in 0..100 {
        metrics.inc("rbac.web01");
        metrics.inc("nzp-jxhd-server.app01");
        if i % 2 == 0 {
            metrics.inc("oa-jz-shop-server.app06");
        }
    }

    for _ in 0..27 {
        metrics.inc("weixin-server.app02");
    }

    println!("{:?}", metrics.snap());
    Ok(())
}