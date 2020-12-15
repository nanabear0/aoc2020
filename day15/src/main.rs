use std::collections::HashMap;

fn main() {
    let initial_v = vec![8, 11, 0, 19, 1, 2];
    let mut s = HashMap::new();
    initial_v.iter().enumerate().for_each(|(i, x)| {
        s.insert(*x, i);
    });
    s.remove(&initial_v.last().unwrap());

    let mut last = initial_v.last().unwrap().to_owned();
    for i in initial_v.len()..30_000_000 {
        let next;
        if let Some(x) = s.get(&last) {
            next = i - x - 1;
        } else {
            next = 0;
        }
        s.insert(last, i - 1);
        last = next;
    }
    println!("last: {:?}", last);
}
