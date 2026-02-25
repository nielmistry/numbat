use std::collections::HashMap;

pub fn fetch_pubchem_property(cid: &str, property: &str) -> Option<String> {
    let url = format!(
        "https://pubchem.ncbi.nlm.nih.gov/rest/pug/compound/cid/{}/property/{}/TXT",
        cid, property
    );
    let res = attohttpc::get(&url).send().ok()?;
    if res.status().is_success() {
        res.text().ok()
    } else {
        None
    }
}

pub fn fetch_pubchem_properties(cid: &str, properties: &[&str]) -> Option<HashMap<String, String>> {
    use serde_json::Value;

    let properties_str = properties.join(",");
    let url = format!(
        "https://pubchem.ncbi.nlm.nih.gov/rest/pug/compound/cid/{}/property/{}/JSON",
        cid, properties_str
    );

    println!("Fetching from URL: {}", url);
    let res = attohttpc::get(&url).send().ok()?;
    if res.status().is_success() {
        let text = res.text().ok()?;
        let json: Value = serde_json::from_str(&text).ok()?;

        // Extract the first property object from PropertyTable.Properties array
        let props = json
            .get("PropertyTable")?
            .get("Properties")?
            .as_array()?
            .first()?;

        // Extract values in the same order as requested properties
        let mut values = Vec::new();
        for prop_name in properties {
            if let Some(val) = props.get(*prop_name) {
                let value_str = match val {
                    Value::String(s) => s.clone(),
                    Value::Number(n) => n.to_string(),
                    _ => continue,
                };
                values.push(value_str);
            } else {
                return None;
            }
        }

        let mut map = HashMap::new();
        for (i, prop_name) in properties.iter().enumerate() {
            map.insert(prop_name.to_string(), values[i].clone());
        }
        map.insert("cid".to_string(), cid.to_string());
        Some(map)
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fetch_pubchem_property_works() {
        let cid = "2244"; // Aspirin
        let property = "MolecularWeight";
        let result = fetch_pubchem_property(cid, property);
        assert!(result.is_some());
        let value = result.unwrap();
        assert_eq!(value.trim(), "180.16");
    }

    #[test]
    fn fetch_pubchem_properties_works() {
        let cid = "2244"; // Aspirin
        let properties = vec!["MolecularWeight", "ExactMass"];
        let result = fetch_pubchem_properties(cid, &properties);
        assert!(result.is_some());
        let values = result.unwrap();
        assert_eq!(values.get("MolecularWeight").unwrap().trim(), "180.16");
        assert_eq!(values.get("ExactMass").unwrap().trim(), "180.04225873");
    }
    #[test]
    fn fetch_pubchem_property_nonexistent() {
        let cid = "2244"; // Aspirin
        let property = "NonExistentProperty";
        let result = fetch_pubchem_property(cid, property);
        assert!(result.is_none());
    }
}
