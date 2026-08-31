# Changelog

Engine and demo history. The project outline lives in [README.MD](README.MD).

## 2026-08-31

- **लिट् आ-anta:** 7.1.34 आत औ णलः, 6.4.64 आ-lopa (ददौ/ददतुः, दधौ, तस्थौ, पपौ, जगौ, जहौ; ātmane ददे/दधे). 6.1.45 ग्लै → जग्लौ; 6.1.64 ष्ठा → स्था.
- **लिट् यजादि ए-anta:** वे 2.4.41/6.1.39 (उवाय, ऊयतुः/ऊवतुः, ववौ); व्ये 6.1.46 विव्याय; ह्वे 6.1.33 जुहाव/जुहुवतुः.
- **लिट् इण् / अस्:** 7.4.69 इयाय, ईयतुः, ईये; 7.4.70 अत आदेः आस, आद, आट.
- **लिट् नुट् / आम् / ब्रू / जक्षादि:** आनञ्ज; एधाञ्चक्रे; उवाच; जागार, दरिद्रौ; तत्याज.
- **लुङ्:** सिच् लुक् अभूत्/अदात्; गम् अगमत्; कृ अकार्षीत्; नी अनैषीत्.
- **आशीर्लिङ्** split from विधिलिङ्: भूयात्, क्रियात्, गम्यात्.
- **क्त / क्त्वा / ल्यप्:** गत, कृत, उक्त, दत्त; प्र+भू → प्रभूय.
- **लुङ् fuller:** चङ् (अशिश्रियत्), क्स (अदिक्षत्), हन् अवधीत्, आत्मने अकृत / अगमन्त / अभविष्ट; सेट्/अनिट् सिच्.
- **आशीर्लिङ् आत्मने:** कृषीष्ट, भविषीष्ट.
- **लिट्:** तुष्टाव (ष्टुत्व), चकास, थल् वेट् (पपक्थ); प्रजगाम round-trip.
- **लृट् इट्:** गमिष्यति / करिष्यति / पक्ष्यति / स्थास्यति.
- **गण 2–9 लट्:** अत्ति, जुहोति, सुनोति, रुणद्धि, तनोति, क्रीणाति, अस्ति, एति.
- **1.3:** स्था+सम्, उद्+चर्, आ+दा; उपसर्ग परसवर्ण सङ्करोति, षत्व निषीदति, आ+ऋ आर्च्छति.
- **कृत्:** कर्तव्य, करणीय, करण, कर्तृ; शतृ जुह्वत्.
- **सुबन्त:** राजा / पितरम् / नाम.


## 2026-08-28

- **णत्व / लिट्:** parasmai as before; **ātmanepada** 3.4.81 एश्/इरेच् on the weak aṅga (ईजे, चक्रे, निन्ये, जगृहे, बभूवे).

- **Kaumudī first:** generate never returns scrape tables. `validate` vs `skt_morph.db` is a probe only.
- **live_generate:** gold tables dropped from the generate path. `tinanta_overrides.rs` is a shrinking patch list. गम् लृट् is गमिष्यति (`gamizya`). *akzi विधिलिङ् is a stem, not a special join.
- **Lookup / analyze:** dhātu by id / name / it (`gam` → `gamx`). Analyze indexes unprefixed forms; upasargas peeled at query time.

## 2026-08-27

- JS demo: Devanagari or Harvard-Kyoto in; display and grammatical names in देवनागरी. WASM stays SLP1 (`www/translit.js`, `www/labels.js`).
