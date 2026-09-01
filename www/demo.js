import init, {
  analyze,
  generate_verb_with_prefix,
  generate_verb_paradigm_with_prefix,
  generate_verb_derived,
  generate_verb_paradigm_derived,
  generate_noun,
  generate_pronoun,
  generate_krdanta,
  generate_krdanta_with_prefix,
  generate_krdanta_declension,
  krdanta_lingas,
} from "../pkg/skt_morph.js";
import { toSlp1, toDeva, prefixesToSlp1, formsToDeva } from "./translit.js";
import * as L from "./labels.js";

await init();

function asObj(m) {
  if (!m) return {};
  if (m instanceof Map) return Object.fromEntries(m);
  return m;
}

function strfy(v) {
  return JSON.stringify(v, (_, x) => (x instanceof Map ? Object.fromEntries(x) : x), 2);
}

function esc(s) {
  return String(s)
    .replace(/&/g, "&amp;")
    .replace(/</g, "&lt;")
    .replace(/>/g, "&gt;");
}

function cellForms(forms) {
  const list = formsToDeva(forms).filter(Boolean);
  return list.length ? esc(list.join(", ")) : '<span class="empty">—</span>';
}

const tabs = ["analyze", "verb", "noun"];
tabs.forEach((t) => {
  document.getElementById("tab-" + t).onclick = () => {
    tabs.forEach((x) => {
      document.getElementById("sec-" + x).hidden = x !== t;
      document.getElementById("tab-" + x).classList.toggle("active", x === t);
    });
  };
});

/** 1.4.59 — same list as `prefix.rs` / ashtadhyayi upasarga pages. */
const UPASARGAS = [
  ["pra", "प्र"],
  ["parA", "परा"],
  ["apa", "अप"],
  ["sam", "सम्"],
  ["anu", "अनु"],
  ["ava", "अव"],
  ["nis", "निस्"],
  ["nir", "निर्"],
  ["dus", "दुस्"],
  ["dur", "दुर्"],
  ["vi", "वि"],
  ["A", "आ"],
  ["ni", "नि"],
  ["aDi", "अधि"],
  ["api", "अपि"],
  ["ati", "अति"],
  ["su", "सु"],
  ["ud", "उद्"],
  ["aBi", "अभि"],
  ["prati", "प्रति"],
  ["pari", "परि"],
  ["upa", "उप"],
];

const selectedUpa = [];

function renderUpaChips() {
  const box = document.getElementById("upasargas");
  box.innerHTML = UPASARGAS.map(([slp, deva]) => {
    const on = selectedUpa.includes(slp) ? " on" : "";
    return `<button type="button" class="upa${on}" data-slp="${slp}">${deva}</button>`;
  }).join("");
}

document.getElementById("upasargas").onclick = (e) => {
  const b = e.target.closest("button.upa");
  if (!b) return;
  const slp = b.dataset.slp;
  const i = selectedUpa.indexOf(slp);
  if (i >= 0) selectedUpa.splice(i, 1);
  else selectedUpa.push(slp);
  b.classList.toggle("on", i < 0);
};

document.getElementById("btn-upa-clear").onclick = () => {
  selectedUpa.length = 0;
  document.getElementById("verb-prefix-extra").value = "";
  renderUpaChips();
};

function prefixArg() {
  const extra = prefixesToSlp1(document.getElementById("verb-prefix-extra").value);
  const extras = extra ? extra.split(",") : [];
  return [...selectedUpa, ...extras].join(",");
}

renderUpaChips();

function analysisRows(a) {
  const rows = [];
  const type = a.word_type || a.wordType;
  if (type) rows.push(["प्रकार", L.wordType(type)]);
  const dhatu = a.dhatu;
  if (dhatu) {
    const id = a.dhatu_id || a.dhatuId;
    rows.push(["धातु", esc(toDeva(dhatu)) + (id ? ` (${esc(id)})` : "")]);
  }
  if (a.pratyaya) rows.push(["कृत्", esc(L.pratyaya(a.pratyaya))]);
  const pad = a.pratipadika;
  if (pad) rows.push(["प्रातिपदिक", esc(toDeva(pad))]);
  if (a.linga) rows.push(["लिङ्ग", L.linga(a.linga)]);
  if (a.vibhakti) rows.push(["विभक्ति", L.vibhakti(a.vibhakti)]);
  const vac = a.vacana;
  if (vac != null && vac !== "") rows.push(["वचन", L.vacana(vac)]);
  if (a.lakara) {
    rows.push(["लकार", L.lakara(a.lakara)]);
    const pada = L.padaFromLakara(a.lakara);
    if (pada) rows.push(["पद", pada]);
  }
  const pur = a.purusha;
  if (pur != null && pur !== "") rows.push(["पुरुष", L.purusha(pur)]);
  const upa = a.upasarga;
  if (upa) {
    const parts = String(upa).split("+").map((p) => toDeva(p));
    rows.push(["उपसर्ग", esc(parts.join(" + "))]);
  }
  return rows;
}

function renderAnalyses(raw, queryDeva) {
  const el = document.getElementById("out-analyze");
  let list = raw;
  if (raw && typeof raw === "object" && !Array.isArray(raw) && raw[0] !== undefined) {
    list = Array.from(raw);
  }
  if (!Array.isArray(list)) list = list ? [list] : [];
  list = list.map(asObj);

  if (list.length === 0) {
    el.innerHTML =
      `<div class="miss">न किञ्चन विश्लेषणम् — <b>${esc(queryDeva)}</b></div>` +
      `<details><summary>JSON</summary><pre>${esc(strfy(raw))}</pre></details>`;
    return;
  }

  let html = `<div class="count">${list.length} विश्लेषण — <b>${esc(queryDeva)}</b></div>`;
  list.forEach((a, i) => {
    html += `<article class="parse"><h3>${i + 1}. ${esc(L.wordType(a.word_type || a.wordType) || "—")}</h3><dl>`;
    for (const [k, v] of analysisRows(a)) {
      html += `<div><dt>${esc(k)}</dt><dd>${v}</dd></div>`;
    }
    html += `</dl></article>`;
  });
  html += `<details><summary>JSON (SLP1)</summary><pre>${esc(strfy(raw))}</pre></details>`;
  el.innerHTML = html;
}

function asList(raw) {
  let list = raw;
  if (raw && typeof raw === "object" && !Array.isArray(raw) && raw[0] !== undefined) {
    list = Array.from(raw);
  }
  if (!Array.isArray(list)) list = list ? [list] : [];
  return list.map(asObj);
}

function runAnalyze(types, miss) {
  const typed = document.getElementById("q").value.trim();
  if (!typed) return;
  const slp = toSlp1(typed);
  const res = analyze(slp);
  const queryDeva = toDeva(slp) || typed;
  if (!types) {
    renderAnalyses(res, queryDeva);
    return;
  }
  const list = asList(res).filter((a) => types.includes(a.word_type || a.wordType));
  if (list.length === 0) {
    const el = document.getElementById("out-analyze");
    el.innerHTML =
      `<div class="miss">${miss} — <b>${esc(queryDeva)}</b></div>` +
      `<details><summary>JSON</summary><pre>${esc(strfy(res))}</pre></details>`;
    return;
  }
  renderAnalyses(list, queryDeva);
}

document.getElementById("btn-analyze").onclick = () => runAnalyze(null);
document.getElementById("btn-search").onclick = () =>
  runAnalyze(["tinanta"], "न किञ्चन तिङन्तम्");
document.getElementById("btn-subanta").onclick = () =>
  runAnalyze(["subanta", "sarvanama"], "न किञ्चन सुबन्तम्");
// Enter in #q triggers विश्लेषण; debounced to avoid double WASM init race
let analyzeTimer = null;
document.getElementById("q").addEventListener("keydown", (e) => {
  if (e.key === "Enter") {
    e.preventDefault();
    clearTimeout(analyzeTimer);
    analyzeTimer = setTimeout(() => runAnalyze(null), 80);
  }
});
document.getElementById("dhatu").addEventListener("keydown", (e) => {
  if (e.key === "Enter") document.getElementById("btn-verb").click();
});
document.getElementById("nbase").addEventListener("keydown", (e) => {
  if (e.key === "Enter") document.getElementById("btn-noun").click();
});

function renderVerbParadigm(res) {
  const el = document.getElementById("out-verb");
  const rows = Array.isArray(res) ? res : res ? Array.from(res) : [];
  if (!rows.length) {
    el.textContent = strfy(res);
    return;
  }
  const get = (p, v) => (rows.find((e) => e.purusha === p && e.vacana === v) || {}).forms || [];
  let html =
    '<table><tr><th></th><th>एकवचन</th><th>द्विवचन</th><th>बहुवचन</th></tr>';
  const names = ["", "प्रथमपुरुष", "मध्यमपुरुष", "उत्तमपुरुष"];
  for (let p = 1; p <= 3; p++) {
    html += `<tr><td><b>${names[p]}</b></td>`;
    for (let v = 1; v <= 3; v++) html += `<td>${cellForms(get(p, v))}</td>`;
    html += "</tr>";
  }
  html += `</table><details><summary>JSON (SLP1)</summary><pre>${esc(strfy(res))}</pre></details>`;
  el.innerHTML = html;
}

function renderVerbSingle(res) {
  const el = document.getElementById("out-verb");
  const o = asObj(res);
  if (!o.forms) {
    el.textContent = strfy(res);
    return;
  }
  const forms = formsToDeva(o.forms).join(", ");
  const meta = [
    toDeva(o.dhatu),
    L.lakara(o.lakara),
    L.purusha(o.purusha),
    L.vacana(o.vacana),
  ]
    .filter(Boolean)
    .join(" · ");
  el.innerHTML =
    `<div>रूप: <b>${esc(forms)}</b> (${esc(meta)})</div>` +
    `<details><summary>JSON (SLP1)</summary><pre>${esc(strfy(res))}</pre></details>`;
}

function dhatuQuery() {
  return toSlp1(document.getElementById("dhatu").value) || "BU";
}

document.getElementById("btn-verb").onclick = () => {
  const d = dhatuQuery();
  const l = document.getElementById("lakara").value;
  const deriv = document.getElementById("derivation").value;
  const pref = prefixArg();
  const res = deriv
    ? generate_verb_paradigm_derived(d, deriv, l, pref, "")
    : generate_verb_paradigm_with_prefix(d, l, pref, "");
  renderVerbParadigm(res);
};

document.getElementById("btn-verb1").onclick = () => {
  const d = dhatuQuery();
  const l = document.getElementById("lakara").value;
  const deriv = document.getElementById("derivation").value;
  const pref = prefixArg();
  const res = deriv
    ? generate_verb_derived(d, deriv, l, 1, 1, pref, "")
    : generate_verb_with_prefix(d, l, 1, 1, pref, "");
  renderVerbSingle(res);
};

function renderDeclTable(stemDeva, lingaCode, decl) {
  const order = L.VIBHAKTI_ORDER.filter((v) => v in decl || v !== "samboDana");
  let html = `<div class="stem">प्रातिपदिक: <b>${esc(stemDeva)}</b> (${esc(L.linga(lingaCode))})</div>`;
  html += "<table><tr><th>विभक्ति</th><th>एकवचन</th><th>द्विवचन</th><th>बहुवचन</th></tr>";
  for (const v of order) {
    const row = decl[v] || [];
    const cells = [0, 1, 2].map((i) => {
      const cell = row[i];
      if (cell == null || cell === "") return '<td class="empty">—</td>';
      const parts = String(cell).split(/[,/]/).map((p) => toDeva(p.trim()));
      return `<td>${esc(parts.join(", "))}</td>`;
    });
    html += `<tr><td>${esc(L.vibhakti(v))}</td>${cells.join("")}</tr>`;
  }
  html += "</table>";
  return html;
}

function renderDeclension(res) {
  const el = document.getElementById("out-noun");
  const decl = asObj(res?.declension);
  if (!res || !decl || Object.keys(decl).length === 0) {
    el.innerHTML =
      `<div class="miss">न किञ्चन सुबन्तम् — <b>${esc(toDeva(toSlp1(document.getElementById("nbase").value)))}</b></div>` +
      `<div class="hint">उदाहरण: राम / rAma, सीता / sItA, हरि / hari, तद् / tad, द्वि / dvi, त्रि / tri</div>` +
      `<pre>${esc(res ? strfy(res) : "null")}</pre>`;
    return;
  }
  el.innerHTML =
    renderDeclTable(toDeva(res.stem), res.linga, decl) +
    `<details><summary>JSON (SLP1)</summary><pre>${esc(strfy(res))}</pre></details>`;
}

document.getElementById("btn-noun").onclick = () => {
  const b = toSlp1(document.getElementById("nbase").value) || "rAma";
  const l = document.getElementById("linga").value;
  let res = generate_pronoun(b, l);
  const pronounHit = res && asObj(res.table) && Object.keys(asObj(res.table)).length > 0;
  if (!pronounHit) {
    res = generate_noun(b, l);
  }
  if (res && res.table && !res.declension) {
    const tbl = asObj(res.table);
    const el = document.getElementById("out-noun");
    el.innerHTML =
      renderDeclTable(toDeva(res.base), res.linga, tbl) +
      `<details><summary>JSON (SLP1)</summary><pre>${esc(strfy(res))}</pre></details>`;
    return;
  }
  renderDeclension(res);
};

// toArray — for WASM Vec returns (krdanta_lingas etc.) without Analysis mapping.
function toArray(v) {
  if (!v) return [];
  if (Array.isArray(v)) return [...v];
  if (typeof v.length === "number") return Array.from(v);
  return [];
}

function syncKrdLinga() {
  const p = document.getElementById("pratyaya").value;
  const sel = document.getElementById("krd-linga");
  const allowed = toArray(krdanta_lingas(p));
  const labels = { pum: "पुंलिङ्ग", stri: "स्त्रीलिङ्ग", nap: "नपुंसकलिङ्ग" };
  if (allowed.length === 0) {
    sel.hidden = true;
    sel.innerHTML = "";
    return;
  }
  const prev = sel.value;
  sel.hidden = false;
  sel.innerHTML = allowed
    .map((l) => `<option value="${l}">${labels[l] || l}</option>`)
    .join("");
  if (allowed.includes(prev)) sel.value = prev;
}

document.getElementById("pratyaya").onchange = syncKrdLinga;
syncKrdLinga();

document.getElementById("btn-krdanta").onclick = () => {
  const d = dhatuQuery();
  const p = document.getElementById("pratyaya").value;
  const pref = prefixArg();
  const el = document.getElementById("out-krdanta");
  try {
    const res = pref ? generate_krdanta_with_prefix(d, p, pref) : generate_krdanta(d, p);
    if (!res || !res.forms || res.forms.length === 0 || res.forms[0] === "") {
      el.innerHTML =
        `<div class="miss">न किञ्चन कृदन्तम् — <b>${esc(toDeva(d))}</b> + ${esc(L.pratyaya(p))}</div>` +
        `<pre>${esc(strfy(res))}</pre>`;
      return;
    }
    const allowed = toArray(krdanta_lingas(p));
    let html = `<div>रूप: <b>${esc(formsToDeva(res.forms).join(", "))}</b> (${esc(toDeva(d))} + ${esc(L.pratyaya(p))})</div>`;
    if (allowed.length === 0) {
      html += '<div class="hint">अव्यय — न सुबन्तम्</div>';
    } else {
      const linga = document.getElementById("krd-linga").value;
      const declRes = generate_krdanta_declension(d, p, linga, pref);
      const decl = asObj(declRes?.declension);
      if (declRes && decl && Object.keys(decl).length > 0) {
        html += renderDeclTable(toDeva(declRes.stem), declRes.linga, decl);
      } else {
        html += '<div class="hint">सुबन्तं न लब्धम्</div>';
      }
    }
    html += `<details><summary>JSON (SLP1)</summary><pre>${esc(strfy(res))}</pre></details>`;
    el.innerHTML = html;
  } catch (e) {
    el.textContent = "Error: " + e;
  }
};

document.getElementById("out-analyze").innerHTML =
  '<span class="hint">उदाहरण: रामेण / rAmeNa · गच्छति / gacchati · ट्रम्पेण</span>';
