use std::fs::File;

use flame;

fn make_vec(size: usize) -> Vec<u32> {
    // start_guard needs to drop to calculate duration.
    let _fg = ::flame::start_guard("make_vec");

    let mut res = flame::span_of("vec init", || vec![0_u32; size]);
    for x in 0..size {
        res[x] = ((x + 10)/3) as u32;
    }
    let mut waste_time = 0;
    for i in 0..size*10 {
        waste_time += i
    }
    res
}

fn more_computing(i: usize) {
    let _fg = ::flame::start_guard("more_computation");

    for x in 0..(i * 100) {
        let mut v = make_vec(x);
        let x = Vec::from(&v[..]);
        for i in 0..v.len() {
            let flip = (v.len() - 1) - i as usize;
            v[i] = x[flip];
        }
    }
}

fn some_computation() {
    let _fg = ::flame::start_guard("some_computation");

    for i in 0..15 {
        more_computing(i);
    }
}


fn main() {
    let _fg = ::flame::start_guard("main");

    some_computation();
    // in order to create the flamegraph you must call one of the
    // flame::dump_* functions.
    flame::dump_html(File::create("flamegraph.html").unwrap()).unwrap();
}