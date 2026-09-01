/** Harvard-Kyoto / Devanagari (JS) ↔ SLP1 (WASM) — browser side, Kaumudī SLP1 canonical.
 *  WASM stays SLP1-only; this file handles Devanagari/HK → SLP1 for analyze/generate inputs. */

const DEV_CONS = {
  क: "k", ख: "K", ग: "g", घ: "G", ङ: "N",
  च: "c", छ: "C", ज: "j", झ: "J", ञ: "Y",
  ट: "w", ठ: "W", ड: "q", ढ: "Q", ण: "R",
  त: "t", थ: "T", द: "d", ध: "D", न: "n",
  प: "p", फ: "P", ब: "b", भ: "B", म: "m",
  य: "y", र: "r", ल: "l", व: "v",
  श: "S", ष: "z", स: "s", ह: "h", ळ: "L",
};

const DEV_VOWEL = {
  अ: "a", आ: "A", इ: "i", ई: "I", उ: "u", ऊ: "U",
  ऋ: "f", ॠ: "F", ऌ: "x", ॡ: "X",
  ए: "e", ऐ: "E", ओ: "o", औ: "O",
};

const DEV_MATRA = {
  "ा": "A", "ि": "i", "ी": "I", "ु": "u", "ू": "U",
  "ृ": "f", "ॄ": "F", "ॢ": "x", "ॣ": "X",
  "े": "e", "ै": "E", "ो": "o", "ौ": "O",
};

const SLP_CONS = {
  k: "क", K: "ख", g: "ग", G: "घ", N: "ङ",
  c: "च", C: "छ", j: "ज", J: "झ", Y: "ञ",
  w: "ट", W: "ठ", q: "ड", Q: "ढ", R: "ण",
  t: "त", T: "थ", d: "द", D: "ध", n: "न",
  p: "प", P: "फ", b: "ब", B: "भ", m: "म",
  y: "य", r: "र", l: "ल", v: "व",
  S: "श", z: "ष", s: "स", h: "ह", L: "ळ",
};

const SLP_VOWEL = {
  a: "अ", A: "आ", i: "इ", I: "ई", u: "उ", U: "ऊ",
  f: "ऋ", F: "ॠ", x: "ऌ", X: "ॡ",
  e: "ए", E: "ऐ", o: "ओ", O: "औ",
};

const SLP_MATRA = {
  A: "ा", i: "ि", I: "ी", u: "ु", U: "ू",
  f: "ृ", F: "ॄ", x: "ॢ", X: "ॣ",
  e: "े", E: "ै", o: "ो", O: "ौ",
};

const DEV_RE = /[\u0900-\u097F]/;
const DHATU_ID_RE = /^\d{2}\.\d+$/;

const HK_MULTI = [
  ["lRR", "X"], ["lrr", "X"], ["lR", "x"], ["lr", "x"],
  ["RR", "F"], ["ai", "E"], ["au", "O"],
  ["kh", "K"], ["gh", "G"], ["ch", "C"], ["jh", "J"],
  ["Th", "W"], ["Dh", "Q"], ["th", "T"], ["dh", "D"],
  ["ph", "P"], ["bh", "B"],
];

const HK_SINGLE = {
  G: "N", J: "Y", T: "w", D: "q", N: "R",
  z: "S", S: "z", R: "f",
  a: "a", A: "A", i: "i", I: "I", u: "u", U: "U",
  e: "e", o: "o",
  k: "k", g: "g", c: "c", j: "j",
  t: "t", d: "d", n: "n",
  p: "p", b: "b", m: "m",
  y: "y", r: "r", l: "l", v: "v",
  s: "s", h: "h",
  M: "M", H: "H",
};

export function hasDevanagari(s) {
  return DEV_RE.test(s);
}

export function isDhatuId(s) {
  return DHATU_ID_RE.test(s.trim());
}

export function hkToSlp1(s) {
  let out = "";
  let i = 0;
  while (i < s.length) {
    let hit = null;
    for (const [hk, slp] of HK_MULTI) {
      if (s.startsWith(hk, i)) {
        hit = slp;
        i += hk.length;
        break;
      }
    }
    if (hit !== null) {
      out += hit;
      continue;
    }
    const ch = s[i];
    out += Object.prototype.hasOwnProperty.call(HK_SINGLE, ch) ? HK_SINGLE[ch] : ch;
    i += 1;
  }
  return out;
}

export function deVToSlp1(s) {
  let out = "";
  const chars = [...s];
  let i = 0;
  while (i < chars.length) {
    const ch = chars[i];
    if (ch === "\u200c" || ch === "\u200d" || ch === "़") {
      i += 1;
      continue;
    }
    if (ch === "ं") {
      out += "M";
      i += 1;
      continue;
    }
    if (ch === "ः") {
      out += "H";
      i += 1;
      continue;
    }
    if (ch === "ँ") {
      out += "~";
      i += 1;
      continue;
    }
    if (ch === "ऽ") {
      out += "'";
      i += 1;
      continue;
    }
    if (DEV_VOWEL[ch]) {
      out += DEV_VOWEL[ch];
      i += 1;
      continue;
    }
    if (DEV_CONS[ch]) {
      const cons = DEV_CONS[ch];
      const next = chars[i + 1];
      if (next === "़") {
        i += 1;
      }
      const n2 = chars[i + 1];
      if (n2 === "्") {
        out += cons;
        i += 2;
        continue;
      }
      if (n2 && DEV_MATRA[n2]) {
        out += cons + DEV_MATRA[n2];
        i += 2;
        continue;
      }
      out += cons + "a";
      i += 1;
      continue;
    }
    out += ch;
    i += 1;
  }
  return out;
}

export function slp1ToDeva(s) {
  if (s == null || s === "") return "";
  let out = "";
  let i = 0;
  while (i < s.length) {
    const c = s[i];
    if (c === "M") {
      out += "ं";
      i += 1;
      continue;
    }
    if (c === "H") {
      out += "ः";
      i += 1;
      continue;
    }
    if (c === "~") {
      out += "ँ";
      i += 1;
      continue;
    }
    if (c === "'") {
      out += "ऽ";
      i += 1;
      continue;
    }
    if (SLP_CONS[c]) {
      out += SLP_CONS[c];
      const next = s[i + 1];
      if (next && SLP_VOWEL[next]) {
        if (next !== "a") out += SLP_MATRA[next];
        i += 2;
      } else {
        out += "्";
        i += 1;
      }
      continue;
    }
    if (SLP_VOWEL[c]) {
      out += SLP_VOWEL[c];
      i += 1;
      continue;
    }
    out += c;
    i += 1;
  }
  return out;
}

/** User-facing string → SLP1. Devanagari wins; else Harvard-Kyoto. Dhātu ids pass through. */
export function toSlp1(input) {
  if (input == null) return "";
  const s = String(input).trim();
  if (!s) return "";
  if (isDhatuId(s)) return s;
  if (hasDevanagari(s)) return deVToSlp1(s);
  return hkToSlp1(s);
}

/** Comma-separated upasargas → SLP1 list string for WASM. */
export function prefixesToSlp1(input) {
  return String(input || "")
    .split(/[,+\s]+/)
    .map((p) => toSlp1(p))
    .filter(Boolean)
    .join(",");
}

export function toDeva(slp) {
  if (slp == null || slp === "") return "";
  const s = String(slp);
  if (isDhatuId(s)) return s;
  if (hasDevanagari(s)) return s;
  return slp1ToDeva(s);
}

export function formsToDeva(forms) {
  if (!forms) return [];
  return [...forms].map((f) => toDeva(String(f)));
}
