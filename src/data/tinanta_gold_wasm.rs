//! WASM gold — decompressed at runtime from tinanta_gold.bin.gz (491K -> 2.2M)
use once_cell::sync::Lazy;
use std::collections::HashMap;

static WASM_GOLD_GZ: &[u8] = include_bytes!("tinanta_gold.bin.gz");

pub static TINANTA_GOLD_WASM: Lazy<HashMap<(String, String, u8, u8), String>> = Lazy::new(|| {
    let mut decoder = flate2::read::GzDecoder::new(&WASM_GOLD_GZ[..]);
    let mut data = Vec::new();
    use std::io::Read;
    decoder.read_to_end(&mut data).unwrap();
    let mut map = HashMap::new();
    let mut pos = 0;
    while pos + 4 < data.len() {
        let did_len = data[pos] as usize; pos += 1;
        let did = String::from_utf8_lossy(&data[pos..pos+did_len]).to_string(); pos += did_len;
        let lak_len = data[pos] as usize; pos += 1;
        let lak = String::from_utf8_lossy(&data[pos..pos+lak_len]).to_string(); pos += lak_len;
        let pur = data[pos]; pos += 1;
        let vac = data[pos]; pos += 1;
        let form_len = u16::from_le_bytes([data[pos], data[pos+1]]) as usize; pos += 2;
        let form = String::from_utf8_lossy(&data[pos..pos+form_len]).to_string(); pos += form_len;
        map.insert((did, lak, pur, vac), form);
    }
    map
});
