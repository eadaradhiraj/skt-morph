//! Bench reverse lookup — measures OnceLock cold vs warm.
//! First `analyze_word` builds tinanta (≈300k) + kṛdanta (≈60×2k) indexes; subsequent calls are hash lookups + upasarga peel.
//! Usage:
//!   cargo run --release --bin time_analyze -- Bavati pragacCati rAmeRa wrampeRa
//! Expect: first ~0.8–1.5s (release), warm ~µs.

use std::env;
use std::time::Instant;

fn main() {
    let words: Vec<String> = env::args().skip(1).collect();
    let words = if words.is_empty() {
        vec![
            "Bavati".into(),
            "gacCati".into(),
            "praBavati".into(),
            "rAmeRa".into(),
        ]
    } else {
        words
    };

    let t0 = Instant::now();
    let first = skt_morph::engine::analyze::analyze_word(&words[0]);
    println!(
        "first  {}  {} analyses  {:?}",
        words[0],
        first.len(),
        t0.elapsed()
    );

    for w in &words {
        let t = Instant::now();
        let hits = skt_morph::engine::analyze::analyze_word(w);
        println!("  {}  {} analyses  {:?}", w, hits.len(), t.elapsed());
    }
}
