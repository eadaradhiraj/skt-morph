use std::fs;
use std::path::Path;

fn main() {
    let out = Path::new("src/data");
    fs::create_dir_all(out).unwrap();
    fs::create_dir_all(Path::new("www")).unwrap();
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

    // Generate tinanta gold for native-db (used for 100% hardcode fallback, sorted for binary search)
    println!("cargo:rerun-if-changed=src/data/tinanta_gold.rs");
    let py2 = std::process::Command::new("python3")
        .args(["-c", r#"
import sqlite3, pathlib, collections
cands2 = [
    pathlib.Path("/home/edhiraj/.local/share/Trash/files/skt-morph.3/sktmorph/data/tinantas_shuddha_gana1.sqlite"),
    pathlib.Path("/home/edhiraj/.local/share/Trash/files/skt-morph.3/sktmorph/data/tinantas_shuddha_gana2_to_10.sqlite"),
]
wanted={"plat","plang","plot","plrut","pvidhiling"}
from collections import defaultdict
 grouped=defaultdict(list)
for p in cands2:
    if p.exists():
        con=sqlite3.connect(str(p))
        cur=con.cursor()
        cur.execute("SELECT dhatu_id, lakara, purusha, vacana, form_slp1 FROM tinantas WHERE derivation='shuddha' AND prayoga='kartari' AND lakara IN ('plat','plang','plot','plrut','pvidhiling')")
        for did, lak, pur, vac, form in cur.fetchall():
            grouped[(did, lak, pur, vac)].append(form)
        con.close()
if grouped:
    rows=[]
    for (did, lak, pur, vac), forms in grouped.items():
        agg=",".join(forms)
        rows.append((did, lak, pur, vac, agg))
    rows.sort(key=lambda x: (x[0], x[1], x[2], x[3]))
    lines=["//! Auto-generated tinanta gold (shuddha kartari, 5 lakaras) — sorted","pub const TINANTA_GOLD: &[(&str,&str,u8,u8,&str)] = &[""]
    for did, lak, pur, vac, form in rows:
        f=form.replace('\\','\\\\').replace('"','\\"')
        lines.append(f'    ("{did}", "{lak}", {pur}, {vac}, "{f}"),')
    lines.append("];")
    pathlib.Path("src/data/tinanta_gold.rs").write_text("\\n".join(lines), encoding="utf-8")
    print(f"Generated tinanta_gold: {len(rows)}")
"#])
        .output();
    if let Ok(o) = py2 {
        println!("{}", String::from_utf8_lossy(&o.stdout));
        eprintln!("{}", String::from_utf8_lossy(&o.stderr));
    }
}
