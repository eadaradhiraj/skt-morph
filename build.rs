use std::fs;
use std::path::Path;

fn main() {
    let out = Path::new("src/data");
    fs::create_dir_all(out).unwrap();
    fs::create_dir_all(Path::new("www")).unwrap();
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=src/data/dhatus_compact.rs");
    println!("cargo:rerun-if-changed=src/engine/hardcode_all.rs");
    println!("cargo:rerun-if-changed=src/engine/hardcode_g01.rs");
    println!("cargo:rerun-if-changed=www/hardcode.json");
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
        # gana in DB is TEXT like "01","10" – use int() directly, not strip("0") which mangles "10"→"1"
        try:
            g = int(str(gana).strip())
        except:
            g = 1
        padam = det.get("padam","") or ""
        if padam.startswith("uBaya"):
            pada = "U"
        elif padam.startswith("parasm"):
            pada = "P"
        elif padam.startswith("Atman"):
            pada = "A"
        else:
            pada = "P"
        details = {
            "dhatu": det.get("OpadeSikasvarUpam","").replace("~",""),
            "gana": g,
            "pada": pada,
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

    // --- hardcode.json for 100% no-bloat (lite 970K + 207K gz) ---
    // Regenerate www/hardcode.json from embedded hardcode_*.rs so lite+fetch stays 100% in sync.
    // If hardcode_*.rs are present, derive JSON; else keep existing www/hardcode.json.
    let py2 = std::process::Command::new("python3")
        .args(["-c", r#"
import pathlib, re, json, gzip, sys
import pathlib as P
srcs = [P.Path("src/engine/hardcode_all.rs"), P.Path("src/engine/hardcode_g01.rs")]
if not all(p.exists() for p in srcs):
    print("hardcode rs missing, keep www/hardcode.json", file=sys.stderr)
    sys.exit(0)
pat = re.compile(r'\(\s*"([^"]+)"\s*,\s*"([^"]+)"\s*,\s*(\d+)\s*,\s*(\d+)\s*,\s*"([^"]+)"\s*\)')
entries=[]
for src in srcs:
    txt=src.read_text(encoding="utf-8")
    for m in pat.findall(txt):
        entries.append({"id":m[0],"lak":m[1],"p":int(m[2]),"v":int(m[3]),"forms":m[4].split("|")})
# dedup
seen=set()
uniq=[]
for e in entries:
    k=(e["id"],e["lak"],e["p"],e["v"])
    if k not in seen:
        seen.add(k)
        uniq.append(e)
uniq.sort(key=lambda x:(x["id"],x["lak"],x["p"],x["v"]))
import json as J
out=P.Path("www/hardcode.json")
existing=None
if out.exists():
    try: existing=J.loads(out.read_text(encoding="utf-8"))
    except: existing=None
if existing!=uniq:
    out.write_text(J.dumps(uniq, ensure_ascii=False, separators=(',',':')), encoding="utf-8")
    P.Path("www/hardcode.json.gz").write_bytes(gzip.compress(out.read_bytes(), compresslevel=9))
    print(f"Generated hardcode.json: {len(uniq)} entries, {out.stat().st_size} bytes, gz {P.Path('www/hardcode.json.gz').stat().st_size}")
else:
    print(f"hardcode.json up to date: {len(uniq)} entries")
"#])
        .output();
    if let Ok(o) = py2 {
        println!("{}", String::from_utf8_lossy(&o.stdout));
        eprintln!("{}", String::from_utf8_lossy(&o.stderr));
    }
}
