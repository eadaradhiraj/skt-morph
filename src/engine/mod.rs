//! engine — sūtra-ordered morphology (Kaumudī krama).
//! adadi = gaṇa-2 athematic; lit/lun/ashir = lakāra-specific; join/stems = vikaraṇa+sup; derived = ṇic/san/yaṅ.
//! All I/O in SLP1; sandhi & guṇa/vṛddhi via `phonology`.
//! =============================================================================
pub mod adadi;
pub mod analyze; // reverse lookup: tinanta/kṛdanta/subanta index + upasarga peel
pub mod ashir;   // āśīrlin (6.4.67, 7.4.28 etc.) — kit/liṅ
pub mod derived; // ṇic/san/yaṅ/yaṅluk/karma — derivation stems
pub mod dhatu;   // dhātu lookup + gaṇa/pada/tags
pub mod endings; // tiṅ/sUP vibhakti endings per lakāra
pub mod it;      // 1.3.2–9 it-saṃjñā stripping & iṭ handling
pub mod join;    // aṅga+suffix sandhi (8.2.30 ff.) — central join
pub mod krdanta; // kṛt pratyayas (3.1–3.4)
pub mod lakara;  // lakāra family/normalize
pub mod lang_ya; // laṅ + yaṅ edge cases
pub mod lit;     // liṭ abhyāsa + atideśa
pub mod lun;     // luṅ (aṅ/sic/caṅ/ksa etc.)
pub mod phonology; // guṇa/vṛddhi, ṇatva, sandhi helpers
pub mod prefix;  // upasarga sandhi (1.4.59 + 6.1.87 etc.)
pub mod upa_pada; // 1.3.1 upa-pada pada selection + artha
pub mod redup;   // abhyāsa redup (6.1.8 ff.)
pub mod stems;   // vikaraṇa stems (śap/śnu/śnam etc.)
pub mod tinanta; // tinanta façade (lakāra → stem → join)
