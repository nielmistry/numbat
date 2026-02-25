use std::ops::Index;

use compact_str::CompactString;

use super::Args;
use super::FfiContext;
use super::Result;
use super::macros::*;
use crate::interpreter::RuntimeErrorKind;
use crate::quantity::Quantity;
use crate::typechecker::type_scheme::TypeScheme;
use crate::typed_ast::DType;
use crate::value::Value;
use numbat_pubchem::fetch_pubchem_property;

pub fn _get_chemical_element_data_raw(
    _ctx: &mut FfiContext,
    mut args: Args,
    _return_type: &TypeScheme,
) -> Result<Value, Box<RuntimeErrorKind>> {
    use crate::span::{ByteIndex, Span};
    use crate::typed_ast::Type;
    use crate::typed_ast::{StructInfo, StructKind};
    use indexmap::IndexMap;
    use mendeleev::{Electronvolt, GramPerCubicCentimeter, Kelvin, KiloJoulePerMole};
    use std::sync::Arc;

    let pattern = string_arg!(args).to_lowercase();

    if let Some(element) = mendeleev::Element::list()
        .iter()
        .find(|e| e.name().to_lowercase() == pattern || e.symbol().to_lowercase() == pattern)
    {
        let unknown_span = Span {
            start: ByteIndex(0),
            end: ByteIndex(0),
            code_source_id: 0,
        };

        let type_scalar = Type::Dimension(DType::scalar());

        let mut fields: IndexMap<CompactString, (Span, Type)> = IndexMap::new();
        fields.insert(
            CompactString::const_new("symbol"),
            (unknown_span, Type::String),
        );
        fields.insert(
            CompactString::const_new("name"),
            (unknown_span, Type::String),
        );
        fields.insert(
            CompactString::const_new("atomic_number"),
            (unknown_span, type_scalar.clone()),
        );
        fields.insert(
            CompactString::const_new("group"),
            (unknown_span, type_scalar.clone()),
        );
        fields.insert(
            CompactString::const_new("group_name"),
            (unknown_span, Type::String),
        );
        fields.insert(
            CompactString::const_new("period"),
            (unknown_span, type_scalar.clone()),
        );
        fields.insert(
            CompactString::const_new("melting_point_kelvin"),
            (unknown_span, type_scalar.clone()),
        );
        fields.insert(
            CompactString::const_new("boiling_point_kelvin"),
            (unknown_span, type_scalar.clone()),
        );
        fields.insert(
            CompactString::const_new("density_gram_per_cm3"),
            (unknown_span, type_scalar.clone()),
        );
        fields.insert(
            CompactString::const_new("electron_affinity_electronvolt"),
            (unknown_span, type_scalar.clone()),
        );
        fields.insert(
            CompactString::const_new("ionization_energy_electronvolt"),
            (unknown_span, type_scalar.clone()),
        );
        fields.insert(
            CompactString::const_new("vaporization_heat_kilojoule_per_mole"),
            (unknown_span, type_scalar.clone()),
        );

        let info = StructInfo {
            name: CompactString::const_new("_ChemicalElementRaw"),
            kind: StructKind::Instance(vec![]),
            definition_span: unknown_span,
            fields,
        };
        Ok(Value::StructInstance(
            Arc::new(info),
            vec![
                Value::String(element.symbol().into()),
                Value::String(element.name().into()),
                Value::Quantity(Quantity::from_scalar(element.atomic_number() as f64)),
                Value::Quantity(Quantity::from_scalar(
                    element
                        .group()
                        .map_or(f64::NAN, |g| g.group_number() as f64),
                )),
                Value::String(
                    element
                        .group()
                        .map(|g| g.group_name().unwrap_or("unknown").into())
                        .unwrap_or("unknown".into()),
                ),
                Value::Quantity(Quantity::from_scalar(element.period() as f64)),
                Value::Quantity(Quantity::from_scalar(
                    element
                        .melting_point()
                        .map(|Kelvin(k)| k)
                        .unwrap_or(f64::NAN),
                )),
                Value::Quantity(Quantity::from_scalar(
                    element
                        .boiling_point()
                        .map(|Kelvin(k)| k)
                        .unwrap_or(f64::NAN),
                )),
                Value::Quantity(Quantity::from_scalar(
                    element
                        .density()
                        .map(|GramPerCubicCentimeter(d)| d)
                        .unwrap_or(f64::NAN),
                )),
                Value::Quantity(Quantity::from_scalar(
                    element
                        .electron_affinity()
                        .map(|Electronvolt(e)| e)
                        .unwrap_or(f64::NAN),
                )),
                Value::Quantity(Quantity::from_scalar(
                    element
                        .ionization_energy()
                        .map(|Electronvolt(e)| e)
                        .unwrap_or(f64::NAN),
                )),
                Value::Quantity(Quantity::from_scalar(
                    element
                        .evaporation_heat()
                        .map(|KiloJoulePerMole(e)| e)
                        .unwrap_or(f64::NAN),
                )),
            ],
        ))
    } else {
        Err(Box::new(RuntimeErrorKind::ChemicalElementNotFound(
            pattern.to_string(),
        )))
    }
}

pub fn _get_chemical_compound_data_raw(
    _ctx: &mut FfiContext,
    mut args: Args,
    _return_type: &TypeScheme,
) -> Result<Value, Box<RuntimeErrorKind>> {
    use crate::span::{ByteIndex, Span};
    use crate::typed_ast::Type;
    use crate::typed_ast::{StructInfo, StructKind};
    use indexmap::IndexMap;
    use std::sync::Arc;
    let pattern = string_arg!(args).to_lowercase();

    let mut fields: IndexMap<CompactString, (Span, Type)> = IndexMap::new();

    let unknown_span = Span {
        start: ByteIndex(0),
        end: ByteIndex(0),
        code_source_id: 0,
    };
    let type_scalar = Type::Dimension(DType::scalar());

    fields.insert(
        CompactString::const_new("cid"),
        (unknown_span, Type::String),
    );

    fields.insert(
        CompactString::const_new("formula_smiles"),
        (unknown_span, Type::String),
    );

    fields.insert(
        CompactString::const_new("iupac_name"),
        (unknown_span, Type::String),
    );

    fields.insert(
        CompactString::const_new("exact_mass_daltons"),
        (unknown_span, type_scalar.clone()),
    );

    fields.insert(
        CompactString::const_new("molar_mass_gram_per_mole"),
        (unknown_span, type_scalar.clone()),
    );

    fields.insert(
        CompactString::const_new("density_gram_per_cm3"),
        (unknown_span, type_scalar.clone()),
    );

    let info = StructInfo {
        name: CompactString::const_new("_ChemicalCompoundRaw"),
        kind: StructKind::Instance(vec![]),
        definition_span: unknown_span,
        fields,
    };

    if let Some(formula_smiles) = fetch_pubchem_property(&pattern, "CanonicalSMILES") {
        let formula_smiles = formula_smiles.trim().to_string();
        let cid = pattern;
        let iupac_name = fetch_pubchem_property(&cid, "IUPACName")
            .unwrap_or_else(|| "unknown".to_string())
            .trim()
            .to_string();
        let molar_mass_str =
            fetch_pubchem_property(&cid, "MolecularWeight").unwrap_or_else(|| "NaN".to_string());
        let molar_mass = molar_mass_str.trim().parse::<f64>().unwrap_or(f64::NAN);
        let exact_mass_str =
            fetch_pubchem_property(&cid, "ExactMass").unwrap_or_else(|| "NaN".to_string());
        let exact_mass = exact_mass_str.trim().parse::<f64>().unwrap_or(f64::NAN);
        let density_str =
            fetch_pubchem_property(&cid, "Density").unwrap_or_else(|| "NaN".to_string());
        let density = density_str.trim().parse::<f64>().unwrap_or(f64::NAN);

        Ok(Value::StructInstance(
            Arc::new(info),
            vec![
                Value::String(cid.to_string().into()),
                Value::String(formula_smiles.into()),
                Value::String(iupac_name.into()),
                Value::Quantity(Quantity::from_scalar(exact_mass)),
                Value::Quantity(Quantity::from_scalar(molar_mass)),
                Value::Quantity(Quantity::from_scalar(density)),
            ],
        ))
    } else {
        Err(Box::new(RuntimeErrorKind::ChemicalCompoundNotFound(
            pattern.to_string(),
        )))
    }
}
