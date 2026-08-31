/** Grammatical labels: WASM codes / numbers → Sanskrit in Devanagari. */

export const WORD_TYPE = {
  tinanta: "तिङन्त",
  krdanta: "कृदन्त",
  subanta: "सुबन्त",
  sarvanama: "सर्वनाम",
};

export const LINGA = {
  pum: "पुंलिङ्ग",
  stri: "स्त्रीलिङ्ग",
  nap: "नपुंसकलिङ्ग",
  any: "अलिङ्ग",
};

export const VIBHAKTI = {
  prathamA: "प्रथमा",
  dvitIyA: "द्वितीया",
  tfIyA: "तृतीया",
  caturTI: "चतुर्थी",
  paYcamI: "पञ्चमी",
  zazWI: "षष्ठी",
  saptamI: "सप्तमी",
  samboDana: "सम्बोधन",
};

export const VACANA = {
  1: "एकवचन",
  2: "द्विवचन",
  3: "बहुवचन",
};

export const PURUSHA = {
  1: "प्रथमपुरुष",
  2: "मध्यमपुरुष",
  3: "उत्तमपुरुष",
};

export const LAKARA = {
  plat: "लट्",
  alat: "लट्",
  plan: "लङ्",
  alan: "लङ्",
  plang: "लङ्",
  alang: "लङ्",
  plot: "लोट्",
  alot: "लोट्",
  plrt: "लृट्",
  alrt: "लृट्",
  plrut: "लृट्",
  alrut: "लृट्",
  pvidhilin: "विधिलिङ्",
  pvidhiling: "विधिलिङ्",
  avidhilin: "विधिलिङ्",
  plit: "लिट्",
  alit: "लिट्",
  plun: "लुङ्",
  alun: "लुङ्",
  pashirling: "आशीर्लिङ्",
  aling: "आशीर्लिङ्",
  aashirling: "आशीर्लिङ्",
};

export const PADA = {
  P: "परस्मैपद",
  A: "आत्मनेपद",
};

export const PRATYAYA = {
  kta: "क्त",
  ktavatu: "क्तवतु",
  "ktavatu~": "क्तवतुँ",
  Satf: "शतृ",
  "Satf~": "शतृँ",
  SAnac: "शानच्",
  cAnaS: "चानश्",
  tumun: "तुमुन्",
  ktvA: "क्त्वा",
  lyap: "ल्यप्",
  lyuw: "ल्युट्",
  lyu: "ल्यु",
  tavya: "तव्य",
  anIyar: "अनीयर्",
  ktin: "क्तिन्",
  tva: "त्व",
  tal: "तल्",
  tA: "तल्",
  matup: "मतुप्",
  mat: "मतुप्",
  mayaT: "मयट्",
  maya: "मयट्",
  ini: "इन्",
  in: "इन्",
  ka: "क",
  tarap: "तरप्",
  tara: "तरप्",
  tamap: "तमप्",
  tama: "तमप्",
  Ca: "छ",
  Iya: "छ",
  cha: "छ",
  aR: "अण्",
  aN: "अण्",
  Dak: "ढक्",
  yaY: "यञ्",
  tfc: "तृच्",
  yat: "यत्",
  Ryat: "ण्यत्",
  GaY: "घञ्",
  Ramul: "णमुल्",
  Rvul: "ण्वुल्",
  vun: "वुन्",
  ac: "अच्",
  gsnu: "ग्स्नु",
  kvasu: "क्वसु",
  ukaY: "उकञ्",
  kyap: "क्यप्",
  "sya-Satf": "स्य-शतृ",
  "sya-SAnac": "स्य-शानच्",
  "BAvakarma-SAnac": "भावकर्म-शानच्",
  "sya-BAvakarma-SAnac": "स्य-भावकर्म-शानच्",
};

export function wordType(code) {
  return WORD_TYPE[code] || code || "";
}

export function linga(code) {
  return LINGA[code] || code || "";
}

export function vibhakti(code) {
  return VIBHAKTI[code] || code || "";
}

export function vacana(n) {
  return VACANA[Number(n)] || "";
}

export function purusha(n) {
  return PURUSHA[Number(n)] || "";
}

export function lakara(code) {
  return LAKARA[code] || code || "";
}

export function pratyaya(code) {
  return PRATYAYA[code] || code || "";
}

export const DERIVATION = {
  shuddha: "शुद्ध",
  "": "शुद्ध",
  Ric: "णिच्",
  nic: "णिच्",
  san: "सन्",
  yaN: "यङ्",
  yan: "यङ्",
  karma: "कर्मणि",
  yak: "कर्मणि",
};

export function derivation(code) {
  return DERIVATION[code] || code || "";
}

export function padaFromLakara(code) {
  if (!code) return "";
  if (code.startsWith("a")) return PADA.A;
  return PADA.P;
}

export const VIBHAKTI_ORDER = [
  "prathamA", "dvitIyA", "tfIyA", "caturTI",
  "paYcamI", "zazWI", "saptamI", "samboDana",
];
