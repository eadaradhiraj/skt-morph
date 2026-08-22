use std::fs;
use std::path::Path;

fn main() {
    let out = Path::new("src/data");
    fs::create_dir_all(out).unwrap();
    println!("cargo:rerun-if-changed=sktmorph/data/dhatus.sqlite");
    println!("cargo:rerun-if-changed=build.rs");

    let py = std::process::Command::new("python3")
        .args(["-c", r#"
import sqlite3, json, pathlib
db = pathlib.Path("sktmorph/data/dhatus.sqlite")
if db.exists():
    con = sqlite3.connect(str(db))
    cur = con.cursor()
    cur.execute("SELECT dhatu_id, details_json FROM dhatus")
    rows = cur.fetchall()
    data = [{"dhatu_id": r[0], "details": json.loads(r[1])} for r in rows]
    pathlib.Path("src/data/dhatus.json").write_text(json.dumps(data, ensure_ascii=False, indent=2), encoding="utf-8")
    # generate compact Rust
    lines = ["//! Auto-generated compact dhatus", "pub const DHATUS: &[(&str,&str,u8,&str,&str,&str,&str)] = &["]
    # fields: id, dhatu, gana, pada, tags, antarganas, aupadeshik
    for r in rows:
        did, det = r[0], json.loads(r[1])
        dhatu = det.get("dhatu","")
        gana = int(det.get("gana") or 1)
        pada = det.get("pada") or "P"
        tags = (det.get("tags") or "").replace("\\","\\\\").replace('"','\\"')
        ant = (det.get("antarganas") or "").replace("\\","\\\\").replace('"','\\"')
        aup = (det.get("aupadeshik") or "").replace("\\","\\\\").replace('"','\\"')
        lines.append(f'    ("{did}", "{dhatu}", {gana}, "{pada}", "{tags}", "{ant}", "{aup}"),')
    lines.append("];")
    pathlib.Path("src/data/dhatus_compact.rs").write_text("\n".join(lines), encoding="utf-8")
    print(f"Generated compact: {len(rows)}")
"#])
        .output();
    if let Ok(o) = py {
        println!("{}", String::from_utf8_lossy(&o.stdout));
        eprintln!("{}", String::from_utf8_lossy(&o.stderr));
    }
    // also watch compact
    println!("cargo:rerun-if-changed=src/data/dhatus_compact.rs");
}
