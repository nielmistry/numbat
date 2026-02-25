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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fetch_pubchem_property_works() {
        let cid = 2244; // Aspirin
        let property = "MolecularWeight";
        let result = fetch_pubchem_property(cid, property);
        assert!(result.is_some());
        let value = result.unwrap();
        assert_eq!(value.trim(), "180.16");
    }

    #[test]
    fn fetch_pubchem_property_nonexistent() {
        let cid = 2244; // Aspirin
        let property = "NonExistentProperty";
        let result = fetch_pubchem_property(cid, property);
        assert!(result.is_none());
    }
}
