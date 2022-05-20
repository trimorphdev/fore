extern crate fore;

use fore::prelude::*;
use std::time::Instant;

#[cfg(test)]
#[test]
fn hello_rsx() {
    let start = Instant::now();
    let rendered = rsx! {
        <Div>
            { std::env::consts::OS }
        </Div>
    }.render();
    let end = start.elapsed().as_nanos();
    println!("{}", rendered);
    println!("{}ms", (end as f64) / 1e+6_f64);
}