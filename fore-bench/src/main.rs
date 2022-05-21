use std::time::Instant;

use fore::prelude::*;

fn main() {
    let start = Instant::now();
    let rendered = rsx! {
        <Html>
            <Head>
                <Title>Benchmark Website</Title>
            </Head>

            <Body>
                Hello, world!
            </Body>
        </Html>
    }.render();
    let end = start.elapsed().as_nanos();
    println!("{}", rendered);
    println!("{}ms", (end as f64) / 1e+6_f64);
}
