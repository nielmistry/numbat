use std::collections::HashMap;
use std::sync::{Mutex, OnceLock};

static PUBCHEM_CACHE: OnceLock<Mutex<HashMap<String, HashMap<String, String>>>> = OnceLock::new();

fn cache() -> &'static Mutex<HashMap<String, HashMap<String, String>>> {
    PUBCHEM_CACHE.get_or_init(|| Mutex::new(HashMap::new()))
}

pub fn cache_get(name: &str) -> Option<HashMap<String, String>> {
    cache().lock().unwrap().get(name).cloned()
}

/// Populate the cache from a raw PubChem JSON response string.
/// The JSON is expected to be in the standard PubChem PUG REST format:
/// `{"PropertyTable":{"Properties":[{"CID":...,"ConnectivitySMILES":"...",...}]}}`
pub fn cache_set_from_json(name: &str, json_str: &str) -> bool {
    let parsed = parse_pubchem_json(json_str);
    if let Some(map) = parsed {
        cache()
            .lock()
            .unwrap()
            .insert(name.to_lowercase(), map);
        true
    } else {
        false
    }
}

fn parse_pubchem_json(json_str: &str) -> Option<HashMap<String, String>> {
    use serde_json::Value;
    let json: Value = serde_json::from_str(json_str).ok()?;
    let props = json
        .get("PropertyTable")?
        .get("Properties")?
        .as_array()?
        .first()?;

    let property_names = [
        "ConnectivitySMILES",
        "IUPACName",
        "ExactMass",
        "MolecularWeight",
    ];

    let mut map = HashMap::new();
    for prop_name in &property_names {
        if let Some(val) = props.get(*prop_name) {
            let value_str = match val {
                Value::String(s) => s.clone(),
                Value::Number(n) => n.to_string(),
                _ => continue,
            };
            map.insert(prop_name.to_string(), value_str);
        }
    }
    if let Some(cid) = props.get("CID") {
        map.insert("cid".to_string(), cid.to_string());
    }

    Some(map)
}

#[cfg(feature = "fetch-pubchem")]
pub fn fetch_and_cache(
    name: &str,
    properties: &[&str],
) -> Option<HashMap<String, String>> {
    let result = numbat_pubchem::fetch_pubchem_properties(name, properties)?;
    cache()
        .lock()
        .unwrap()
        .insert(name.to_lowercase(), result.clone());
    Some(result)
}
