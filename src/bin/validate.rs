//! Cross-check live tinanta against the ashtadhyayi.com scrape.
//! The scrape is not the spec — Pāṇini / Siddhānta-Kaumudī is.
//! A mismatch is a prompt to check the sūtra, not an order to copy the site.
//!
//! Prefixes in that DB are only chandrika / upasarga pages, not a full grid.
//!
//! ```text
//! cargo run --release --bin validate --features native-db -- --full --live
//! cargo run --release --bin validate --features native-db -- --id 01.0001 --id 01.1137
//! cargo run --release --bin validate --features native-db -- --gana 01 --lakara plat --all-prefixes
//! ```

use rusqlite::{Connection, OpenFlags};
use skt_morph::engine::lakara::normalize_lakara;
use skt_morph::engine::prefix::UPASARGAS;
use skt_morph::engine::tinanta::{generate_all_with_prefixes, live_generate};
use skt_morph::data::DHATUS;
use std::collections::{HashMap, HashSet};
use std::env;
use std::path::{Path, PathBuf};
use std::process;
use std::time::Instant;

const DEFAULT_FORM_TYPES: &[&str] = &[
    "plat", "alat", "plang", "alang", "plot", "alot", "plrut", "alrut", "pvidhiling",
    "avidhiling", "plit", "alit", "plun", "alun", "pashirling", "aashirling",
];

fn main() {
    let args = match Args::parse() {
        Ok(a) => a,
        Err(e) => {
            eprintln!("{e}");
            print_help();
            process::exit(2);
        }
    };
    if args.help {
        print_help();
        return;
    }
    let db = match resolve_db(&args.db) {
        Some(p) => p,
        None => {
            eprintln!(
                "no skt_morph.db found. Pass --db PATH or set SKT_MORPH_DB.\n\
                 expected ../skt-morph-data/data/skt_morph.db"
            );
            process::exit(1);
        }
    };
    let t0 = Instant::now();
    let report = match run(&db, &args) {
        Ok(r) => r,
        Err(e) => {
            eprintln!("validate failed: {e}");
            process::exit(1);
        }
    };
    print_report(&db, &args, &report, t0.elapsed());
    if report.gold_forms == 0 {
        process::exit(1);
    }
}

struct Args {
    help: bool,
    db: Option<PathBuf>,
    live: bool,
    full: bool,
    gana: Option<String>,
    ids: Vec<String>,
    form_types: Vec<String>,
    all_prefixes: bool,
    prefix: String,
    limit: Option<usize>,
    miss_limit: usize,
}

impl Args {
    fn parse() -> Result<Self, String> {
        let mut help = false;
        let mut db = None;
        let mut live = true;
        let mut full = false;
        let mut gana = None;
        let mut ids = Vec::new();
        let mut form_types = Vec::new();
        let mut all_prefixes = false;
        let mut prefix = String::new();
        let mut limit = None;
        let mut miss_limit = 25usize;
        let mut argv = env::args().skip(1);
        while let Some(a) = argv.next() {
            match a.as_str() {
                "-h" | "--help" => help = true,
                "--db" => db = Some(PathBuf::from(argv.next().ok_or("--db needs a path")?)),
                "--live" => live = true,
                "--pipeline" => live = false,
                "--full" => full = true,
                "--gana" => gana = Some(argv.next().ok_or("--gana needs 01..10")?),
                "--id" => ids.push(argv.next().ok_or("--id needs a dhatu id")?),
                "--lakara" => {
                    let v = argv.next().ok_or("--lakara needs a code")?;
                    for part in v.split(',') {
                        let p = part.trim();
                        if !p.is_empty() {
                            form_types.push(to_gold_form_type(p));
                        }
                    }
                }
                "--all-prefixes" => all_prefixes = true,
                "--prefix" => prefix = argv.next().ok_or("--prefix needs a DB prefix string")?,
                "--limit" => {
                    limit = Some(
                        argv.next()
                            .ok_or("--limit needs N")?
                            .parse()
                            .map_err(|_| "--limit must be an integer")?,
                    )
                }
                "--miss-limit" => {
                    miss_limit = argv
                        .next()
                        .ok_or("--miss-limit needs N")?
                        .parse()
                        .map_err(|_| "--miss-limit must be an integer")?;
                }
                other => return Err(format!("unknown flag: {other}")),
            }
        }
        if form_types.is_empty() {
            form_types = DEFAULT_FORM_TYPES.iter().map(|s| (*s).to_string()).collect();
        }
        if let Some(g) = gana.as_mut() {
            if g.len() == 1 {
                *g = format!("0{g}");
            }
        }
        Ok(Self {
            help,
            db,
            live,
            full,
            gana,
            ids,
            form_types,
            all_prefixes,
            prefix,
            limit,
            miss_limit,
        })
    }
}

fn print_help() {
    eprintln!(
        "\
validate — cross-check live tinanta vs scrape (not the Pāṇini spec)

  --db PATH           SQLite file (or SKT_MORPH_DB)
  --live              compare live_generate only (default)
  --pipeline          overrides + live (no scrape tables)
  --full              every dhātu that has ting rows
  --gana 01           restrict by gaṇa
  --id 01.0001        restrict to this id (repeatable)
  --lakara plat,plan  gold form_type codes (default: lat/laṅ/loṭ/lṛṭ/vidhi/liṭ/luṅ/āśīr × P/A)
  --prefix ''         one DB prefix string (default: unprefixed)
  --all-prefixes      also compare prefixes that exist in the scrape
  --limit N           cap dhātus
  --miss-limit N      how many miss examples to print (default 25)

Prefixes in the DB are only chandrika/upasarga coverage, not a full grid.
Unmapped prefix atoms (e.g. odd scrape strings) are counted as gaps, not misses.
AN is mapped to engine upasarga A."
    );
}

fn to_gold_form_type(s: &str) -> String {
    normalize_lakara(s).1
}

fn resolve_db(cli: &Option<PathBuf>) -> Option<PathBuf> {
    if let Some(p) = cli {
        return p.exists().then(|| p.clone());
    }
    if let Ok(p) = env::var("SKT_MORPH_DB") {
        let pb = PathBuf::from(p);
        if pb.exists() {
            return Some(pb);
        }
    }
    let manifest = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    for p in [
        manifest.join("../skt-morph-data/data/skt_morph.db"),
        PathBuf::from("../skt-morph-data/data/skt_morph.db"),
        PathBuf::from("/home/edhiraj/Documents/projs/skt-morph-data/data/skt_morph.db"),
    ] {
        if p.exists() {
            return Some(p);
        }
    }
    None
}

fn dhatu_name(id: &str) -> &'static str {
    DHATUS
        .iter()
        .find(|r| r.0 == id)
        .map(|r| r.1)
        .unwrap_or("")
}

/// Map scrape prefix (`sam;pra`, `AN`) to engine upasargas. `None` = coverage gap.
fn map_db_prefix(raw: &str) -> Option<Vec<String>> {
    if raw.is_empty() {
        return Some(Vec::new());
    }
    let mut out = Vec::new();
    for chunk in raw.split(';') {
        let chunk = chunk.trim();
        if chunk.is_empty() {
            continue;
        }
        if chunk == "vipra" {
            out.push("vi".into());
            out.push("pra".into());
            continue;
        }
        for atom in chunk.split('-') {
            let mapped = match atom.trim() {
                "" => continue,
                "AN" => "A",
                other => other,
            };
            if !UPASARGAS.contains(&mapped) {
                return None;
            }
            out.push(mapped.to_string());
        }
    }
    Some(out)
}

fn td_flip(s: &str) -> Option<String> {
    let mut chars: Vec<char> = s.chars().collect();
    match chars.last()? {
        't' => {
            *chars.last_mut()? = 'd';
            Some(chars.into_iter().collect())
        }
        'd' => {
            *chars.last_mut()? = 't';
            Some(chars.into_iter().collect())
        }
        _ => None,
    }
}

fn split_gold_value(v: &str) -> Vec<String> {
    let mut out = Vec::new();
    for part in v.split(|c| c == '/' || c == ',' || c == ';') {
        let t = part.trim();
        if !t.is_empty() {
            out.push(t.to_string());
        }
    }
    out
}

fn in_set(set: &HashSet<String>, form: &str) -> bool {
    set.contains(form) || td_flip(form).map(|x| set.contains(&x)).unwrap_or(false)
}

type GoldKey = (String, String, String); // dhatu_id, prefix, form_type

struct Report {
    gold_forms: usize,
    live_forms: usize,
    exact: usize,
    td: usize,
    miss: usize,
    extra: usize,
    paradigms: usize,
    dhatus: usize,
    prefix_pairs: usize,
    prefix_gaps: usize,
    by_lakara: Vec<(String, usize, usize, usize, usize)>, // type, gold, exact+td, miss, extra
    misses: Vec<Miss>,
}

struct Miss {
    dhatu_id: String,
    prefix: String,
    form_type: String,
    gold: Vec<String>,
    live: Vec<String>,
}

fn map_conj_row(r: &rusqlite::Row<'_>) -> rusqlite::Result<(String, String, String, String)> {
    Ok((
        r.get::<_, String>(0)?,
        r.get::<_, String>(1)?,
        r.get::<_, String>(2)?,
        r.get::<_, String>(3)?,
    ))
}

fn load_ting_rows(
    conn: &Connection,
    prefix: Option<&str>,
) -> Result<Vec<(String, String, String, String)>, String> {
    let sql = "SELECT dhatu_id, prefix, form_type, form_value FROM conjugation_forms WHERE category='ting'";
    if let Some(p) = prefix {
        let mut stmt = conn
            .prepare(&format!("{sql} AND prefix = ?1"))
            .map_err(|e| e.to_string())?;
        let mapped = stmt.query_map([p], map_conj_row).map_err(|e| e.to_string())?;
        mapped
            .collect::<Result<Vec<_>, _>>()
            .map_err(|e| e.to_string())
    } else {
        let mut stmt = conn.prepare(sql).map_err(|e| e.to_string())?;
        let mapped = stmt.query_map([], map_conj_row).map_err(|e| e.to_string())?;
        mapped
            .collect::<Result<Vec<_>, _>>()
            .map_err(|e| e.to_string())
    }
}

fn run(db: &Path, args: &Args) -> Result<Report, String> {
    let conn = Connection::open_with_flags(
        db,
        OpenFlags::SQLITE_OPEN_READ_ONLY | OpenFlags::SQLITE_OPEN_NO_MUTEX,
    )
    .map_err(|e| e.to_string())?;

    let wanted_ids: Option<HashSet<String>> = if args.ids.is_empty() {
        None
    } else {
        Some(args.ids.iter().cloned().collect())
    };
    let form_ok: HashSet<String> = args.form_types.iter().cloned().collect();

    let rows = load_ting_rows(&conn, if args.all_prefixes { None } else { Some(&args.prefix) })?;

    let mut gold: HashMap<GoldKey, HashSet<String>> = HashMap::new();
    let mut prefix_gaps: HashSet<(String, String)> = HashSet::new();
    let mut mapped_prefixes: HashMap<String, Vec<String>> = HashMap::new();
    mapped_prefixes.insert(String::new(), Vec::new());

    for (did, prefix, ft, val) in rows {
        if let Some(want) = &wanted_ids {
            if !want.contains(&did) {
                continue;
            }
        }
        if let Some(g) = &args.gana {
            if !did.starts_with(&format!("{g}.")) {
                continue;
            }
        }
        if !form_ok.contains(&ft) {
            continue;
        }
        if !args.all_prefixes && prefix != args.prefix {
            continue;
        }
        if map_db_prefix(&prefix).is_none() {
            prefix_gaps.insert((did.clone(), prefix));
            continue;
        }
        let entry = gold.entry((did, prefix, ft)).or_default();
        for f in split_gold_value(&val) {
            entry.insert(f);
        }
    }

    let mut keys: Vec<GoldKey> = gold.keys().cloned().collect();
    keys.sort();
    if !args.full && args.ids.is_empty() && args.gana.is_none() {
        keys.retain(|(id, _, _)| id == "01.0001" || id == "01.1137");
    }
    let mut seen_dhatu: HashSet<String> = HashSet::new();
    let mut limited: Vec<GoldKey> = Vec::new();
    for k in keys {
        if let Some(lim) = args.limit {
            if !seen_dhatu.contains(&k.0) && seen_dhatu.len() >= lim {
                continue;
            }
        }
        seen_dhatu.insert(k.0.clone());
        limited.push(k);
    }

    let mut exact = 0usize;
    let mut td = 0usize;
    let mut miss = 0usize;
    let mut extra = 0usize;
    let mut gold_forms = 0usize;
    let mut live_forms = 0usize;
    let mut lak: HashMap<String, [usize; 4]> = HashMap::new(); // gold, hit, miss, extra
    let mut misses = Vec::new();
    let mut prefix_pairs: HashSet<(String, String)> = HashSet::new();

    for (did, prefix, ft) in &limited {
        prefix_pairs.insert((did.clone(), prefix.clone()));
        let gold_set = gold.get(&(did.clone(), prefix.clone(), ft.clone())).unwrap();
        let prefs = mapped_prefixes
            .entry(prefix.clone())
            .or_insert_with(|| map_db_prefix(prefix).unwrap_or_default())
            .clone();
        let live_set = generate_live(args.live, did, ft, &prefs);
        gold_forms += gold_set.len();
        live_forms += live_set.len();
        let slot = lak.entry(ft.clone()).or_insert([0, 0, 0, 0]);
        slot[0] += gold_set.len();

        let mut miss_forms = Vec::new();
        for g in gold_set {
            if live_set.contains(g) {
                exact += 1;
                slot[1] += 1;
            } else if td_flip(g).map(|x| live_set.contains(&x)).unwrap_or(false) {
                td += 1;
                slot[1] += 1;
            } else {
                miss += 1;
                slot[2] += 1;
                miss_forms.push(g.clone());
            }
        }
        for l in &live_set {
            if !in_set(gold_set, l) {
                extra += 1;
                slot[3] += 1;
            }
        }
        if !miss_forms.is_empty() && misses.len() < args.miss_limit {
            let mut gvec: Vec<String> = gold_set.iter().cloned().collect();
            gvec.sort();
            let mut lvec: Vec<String> = live_set.iter().cloned().collect();
            lvec.sort();
            miss_forms.sort();
            misses.push(Miss {
                dhatu_id: did.clone(),
                prefix: prefix.clone(),
                form_type: ft.clone(),
                gold: gvec,
                live: lvec,
            });
        }
    }

    let mut by_lakara: Vec<(String, usize, usize, usize, usize)> = lak
        .into_iter()
        .map(|(k, v)| (k, v[0], v[1], v[2], v[3]))
        .collect();
    by_lakara.sort_by(|a, b| a.0.cmp(&b.0));

    Ok(Report {
        gold_forms,
        live_forms,
        exact,
        td,
        miss,
        extra,
        paradigms: limited.len(),
        dhatus: seen_dhatu.len(),
        prefix_pairs: prefix_pairs.len(),
        prefix_gaps: prefix_gaps.len(),
        by_lakara,
        misses,
    })
}

fn generate_live(live_only: bool, dhatu_id: &str, gold_ft: &str, prefixes: &[String]) -> HashSet<String> {
    let (canon, db_lak) = normalize_lakara(gold_ft);
    let mut out = HashSet::new();
    for p in 1..=3u8 {
        for v in 1..=3u8 {
            let forms = if live_only {
                live_generate(dhatu_id, &canon, &db_lak, p, v, prefixes, "")
            } else {
                generate_all_with_prefixes(dhatu_id, gold_ft, p, v, prefixes)
            };
            for f in forms {
                if !f.is_empty() {
                    out.insert(f);
                }
            }
        }
    }
    out
}

fn pct(num: usize, den: usize) -> String {
    if den == 0 {
        "n/a".into()
    } else {
        format!("{:.1}%", 100.0 * num as f64 / den as f64)
    }
}

fn print_report(db: &Path, args: &Args, r: &Report, elapsed: std::time::Duration) {
    let mode = if args.live { "live_generate" } else { "pipeline (gold/override/live)" };
    let prefix_desc = if args.all_prefixes {
        "all scrape prefixes".to_string()
    } else if args.prefix.is_empty() {
        "unprefixed".to_string()
    } else {
        format!("prefix={:?}", args.prefix)
    };
    let hit = r.exact + r.td;
    println!("db          {}", db.display());
    println!("mode        {mode}");
    println!("scope       ting / {prefix_desc}");
    if !args.full && args.ids.is_empty() && args.gana.is_none() {
        println!("sample      01.0001 (BU) + 01.1137 (gamx)  — pass --full or --gana / --id");
    }
    println!(
        "dhatus      {}   paradigms {}   prefix-pairs {}   prefix-gaps {} (unmapped scrape atoms)",
        r.dhatus, r.paradigms, r.prefix_pairs, r.prefix_gaps
    );
    println!(
        "forms       gold {}   live {}   exact {}   t/d {}   miss {}   extra {}",
        r.gold_forms, r.live_forms, r.exact, r.td, r.miss, r.extra
    );
    println!(
        "recall      {} strict  {} with t/d  extra-rate {}",
        pct(r.exact, r.gold_forms),
        pct(hit, r.gold_forms),
        pct(r.extra, r.live_forms)
    );
    println!("elapsed     {:.2}s", elapsed.as_secs_f64());
    println!();
    println!("lakara            gold     hit    miss   extra   recall");
    for (ft, g, h, m, e) in &r.by_lakara {
        println!(
            "  {:<14} {:>6} {:>7} {:>7} {:>7}   {}",
            ft,
            g,
            h,
            m,
            e,
            pct(*h, *g)
        );
    }
    if !r.misses.is_empty() {
        println!();
        println!("miss examples (live ≠ scrape; t/d already counted as hit). Check SK before changing the engine:");
        for m in &r.misses {
            let name = dhatu_name(&m.dhatu_id);
            let pref = if m.prefix.is_empty() { "∅" } else { m.prefix.as_str() };
            println!(
                "  {} {}  {} {}  gold={:?}  live={:?}",
                m.dhatu_id, name, pref, m.form_type, m.gold, m.live
            );
        }
    }
    println!();
    println!(
        "note: spec is Pāṇini as in the Siddhānta-Kaumudī. Scrape mismatches can be site errors,"
    );
    println!("      incomplete prefix pages, or engine bugs — do not copy the DB into the engine.");
}
