use crossbeam::thread;
use std::collections::HashMap;

pub fn frequency(input: &[&str], worker_count: usize) -> HashMap<char, usize> {
    let mut result = HashMap::new();
    thread::scope(|s| {
        // Clippy warns me, but I really need to consume the iterators here for performance
        #[allow(clippy::needless_collect)]
        let threads: Vec<_> = input
            .chunks((input.len() + worker_count - 1) / worker_count)
            .map(|chunk| {
                s.spawn(move |_| {
                    let mut map = HashMap::new();
                    chunk.iter().for_each(|s| {
                        s.chars().filter(|c| c.is_alphabetic()).for_each(|c| {
                            *map.entry(c.to_ascii_lowercase()).or_insert(0) += 1;
                        })
                    });
                    map
                })
            })
            .collect();

        for t in threads.into_iter() {
            let r = t.join().unwrap();
            r.iter().for_each(|(&k, &v)| {
                *result.entry(k).or_insert(0) += v;
            });
        }
    })
    .unwrap();
    result
}
