# Changelog

Engine and demo history. Iter docs are now in code comments; this file tracks **features + TODO**.

## Features (v1, live)

**तिङन्त (stems/adadi/join/phonology/lakara):**
- 10 gaṇas live (`stems`, `adadi`), no override table — lat/laṅ/lot/vidhi-liṅ/lṛṭ/liṭ/luṅ/āśir-liṅ, parasmai+ātmanepada
- vikaraṇa, it-strip (1.3.2–9), guṇa/vṛddhi (7.3.84/86), sandhi join, atmanepada adadi (dugh/lih 8.2.31/32)
- upasarga 1.4.59 + free field (`prefix.rs`), sandhi + ṇatva 8.4.1, artha 1.3 (karma/kram etc.)

**Derived (derived.rs, 3.1.26/3.1.7/3.1.22/2.4.74/3.1.67):**
- ṇic, san, yaṅ (ātmane boBUyate 3.1.22), yaṅluk luk 2.4.74 (parasmai boBU/vAvac/cekrI/jaṄgam/pApac/jaṄgan), karmaṇi yak — not indexed in analyze

**कृदन्त (krdanta.rs, 3.1–3.4):**
- 60+ pratyayas (was 19): kta/ktavatu/śatṛ/śānac/tumun/ktvā/lyap/tavya/tṛc/lyu/kvasu etc.
- kta class sandhi before 7.2.35 iṭ: 8.2.30/36/37/40/42/45, 6.4.37/42, 6.1.15/16/45 (mukta/labdha/bhinna/gata/gīta/uta/uṣṭa/rakta) + named ādeśa jagdha/pakva/soḍha/bhagna/dyūta/naddha/jīna (6.1.16 etc.)
- declension via subanta where linga-bearing

**सुबन्त/सर्वनाम (declension):**
- ending-class subanta (8.2.30/39/66): a/Ā/i/I/u/U/ṛ + halanta j/d/t/p/h/B/r/s/S/as/is/us/an/in + o/O (go/nau), foreign Trump→Trumpeṇa
- covers ~300+ stems: vaṇij/dik/kakubh, pathin/panthāḥ, rājan/rājñī, śvan/yuvan, ahan, puṃs, ap, div, etc.
- sarvanāma 1.1.27: tad/etad/idam/ad as/tyad/dvi/tri/catur/ubha/pañcan etc., numbers 20–100, ṅīp (4.1.5/6)

**Analyze (analyze.rs):**
- One SLP1 form → every parse (tinanta/kṛdanta/subanta/sarvanāma), upasarga-peeled, OnceLock maps, empty guards, search_prefix

**Demo/WASM:**
- SLP1 canonical, Devanagari I/O (`translit.js`/`labels.js`), yaṅ/yaṅluk dropdown 2.4.74, a11y/Enter, debounce, dhatu HashMap cache 15s→7s, 187 tests, pkg 1.2M (268k gz)

## TODO / Later (from README Scope)

- **taddhita full 4.1/5.x:** now tva/tal/matup/mayaT/in/tarap/tamap/cha/ka/aṇ/ḍhak/yañ + iñ (4.1.95) + tasil/tral/dāc (5.3.7/10/15) + vat/Sas (5.1.115/5.4.42)  + hA  + tAti  + dvitaya  + kftvas  + kaR  + Ga  + TaK  + Pa  + Da in `taddhita.rs` — remaining 4.1/5.1-5.4 pratyayas + vṛddhi/īp (188 tests)
- **fuller yaṅ:** yaṅ + yaṅluk done, extend to rarer abhyāsa + atmanepada nuances
- **kta ādeśa:** remaining class sandhi edge + named forms (full 7.2.10/6.4.42 list)
- **fuller subanta:** extend halanta beyond j/d/t/p/h/B/r/s/S/as/is/us (add c/ch/ñ etc. + nap special)
- **derived in analyze:** index ṇic/san/yaṅ/yaṅluk/karma for reverse lookup (currently generate-only)
- **gold cross-check:** optional skt_morph.db validate (`--full --live`), keep live ≠ scrape principle

## History (condensed)

- 2026-09-01: de-noise 70k boilerplate, KRTS 19→60+, ending-class, yaṅluk, dhatu cache, a11y — 187 tests, live generate (no overrides)
- Earlier: kta sandhi classes (8.2.30 ff., 6.4.37 ff.), halanta loc/instr fixes, sarvanāma idam/ad as/numbers, śatṛ/ātmanepada luṅ/lit etc. — see git log pre-2026-09-01
- Notes: comments are now concise sūtra-gated rustdoc; per-fn docs carry sūtra + SLP1 I/O. No logic drift since 132.
