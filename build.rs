use std::fs;
use std::path::Path;

fn main() {
    let out = Path::new("src/data");
    fs::create_dir_all(out).unwrap();
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=src/data/dhatus_compact.rs");
    for p in ["sktmorph/data/dhatus.sqlite", "../skt-morph-data/data/skt_morph.db", "/home/edhiraj/Documents/projs/skt-morph-data/data/skt_morph.db"] {
        println!("cargo:rerun-if-changed={}", p);
    }

    let py = std::process::Command::new("python3")
        .args(["-c", r#"
import sqlite3, json, pathlib, sys
candidates = [pathlib.Path("sktmorph/data/dhatus.sqlite"), pathlib.Path("../skt-morph-data/data/skt_morph.db"), pathlib.Path("/home/edhiraj/Documents/projs/skt-morph-data/data/skt_morph.db")]
db = next((p for p in candidates if p.exists()), None)
if db is None:
    print("No DB found, keeping existing src/data/dhatus_compact.rs", file=sys.stderr)
    sys.exit(0)

if str(db).endswith(".db"):
    con = sqlite3.connect(str(db))
    cur = con.cursor()
    cur.execute("SELECT dhatu_id, name, value FROM dhatu_info")
    info = {}
    for did, name, val in cur.fetchall():
        info.setdefault(did, {})[name] = val
    cur.execute("SELECT id, gana FROM dhatus")
    rows = []
    for did, gana in cur.fetchall():
        det = info.get(did, {})
        details = {
            "dhatu": det.get("OpadeSikasvarUpam","").replace("~",""),
            "gana": int(str(gana).strip("0") or 1),
            "pada": "P" if det.get("padam","").startswith("parasm") else ("A" if det.get("padam","").startswith("Atman") else "P"),
            "tags": det.get("anubanDaviSezaH",""),
            "antarganas": det.get("antargaRaH",""),
            "aupadeshik": det.get("OpadeSikasvarUpam",""),
        }
        rows.append((did, details))
else:
    con = sqlite3.connect(str(db))
    cur = con.cursor()
    cur.execute("SELECT dhatu_id, details_json FROM dhatus")
    rows = [(r[0], json.loads(r[1])) for r in cur.fetchall()]

# write compact Rust (always)
lines = ["//! Auto-generated compact dhatus", "pub const DHATUS: &[(&str,&str,u8,&str,&str,&str,&str)] = &["]
for did, det in rows:
    dhatu = det.get("dhatu","")
    gana = int(det.get("gana") or 1)
    pada = det.get("pada") or "P"
    tags = (det.get("tags") or "").replace("\\","\\\\").replace('"','\\"')
    ant = (det.get("antarganas") or "").replace("\\","\\\\").replace('"','\\"')
    aup = (det.get("aupadeshik") or "").replace("\\","\\\\").replace('"','\\"')
    lines.append(f'    ("{did}", "{dhatu}", {gana}, "{pada}", "{tags}", "{ant}", "{aup}"),')
lines.append("];")
pathlib.Path("src/data/dhatus_compact.rs").write_text("\n".join(lines), encoding="utf-8")
print(f"Generated compact: {len(rows)} from {db}")
"#])
        .output();
    if let Ok(o) = py {
        println!("{}", String::from_utf8_lossy(&o.stdout));
        eprintln!("{}", String::from_utf8_lossy(&o.stderr));
    }
}
