use clarity::vm::types::{SequenceSubtype, StringSubtype, TypeSignature};
use clarity::vm::ClarityName;

use super::{SimpleWord, Word};
use crate::wasm_generator::{GeneratorError, WasmGenerator};

fn traverse_comparison(
    name: &str,
    generator: &mut WasmGenerator,
    builder: &mut walrus::InstrSeqBuilder,
    arg_types: &[TypeSignature],
    _return_type: &TypeSignature,
) -> Result<(), GeneratorError> {
    let ty = &arg_types[0];

    let type_suffix = match ty {
        TypeSignature::IntType => "int",
        TypeSignature::UIntType => "uint",
        // same function for buffer and string-ascii
        TypeSignature::SequenceType(SequenceSubtype::StringType(StringSubtype::ASCII(_)))
        | TypeSignature::SequenceType(SequenceSubtype::BufferType(_)) => "buff",
        TypeSignature::SequenceType(SequenceSubtype::StringType(StringSubtype::UTF8(_))) => {
            // For `string-utf8`, comparison is done on a codepoint-by-codepoint basis.
            // Comparing two codepoints is the act of comparing them on a byte-by-byte basis.
            // Since we already have 32-bit unicode scalars, we can just compare them with buff.
            "buff"
        }
        _ => {
            return Err(GeneratorError::TypeError(
                "invalid type for comparison".to_string(),
            ))
        }
    };

    let func = generator
        .module
        .funcs
        .by_name(&format!("stdlib.{name}-{type_suffix}"))
        .ok_or_else(|| {
            GeneratorError::InternalError(format!("function not found: {name}-{type_suffix}"))
        })?;

    builder.call(func);

    Ok(())
}

#[derive(Debug)]
pub struct CmpLess;

impl Word for CmpLess {
    fn name(&self) -> ClarityName {
        "<".into()
    }
}

impl SimpleWord for CmpLess {
    fn visit(
        &self,
        generator: &mut WasmGenerator,
        builder: &mut walrus::InstrSeqBuilder,
        arg_types: &[TypeSignature],
        return_type: &TypeSignature,
    ) -> Result<(), GeneratorError> {
        traverse_comparison("lt", generator, builder, arg_types, return_type)
    }
}

#[derive(Debug)]
pub struct CmpLeq;

impl Word for CmpLeq {
    fn name(&self) -> ClarityName {
        "<=".into()
    }
}

impl SimpleWord for CmpLeq {
    fn visit(
        &self,
        generator: &mut WasmGenerator,
        builder: &mut walrus::InstrSeqBuilder,
        arg_types: &[TypeSignature],
        return_type: &TypeSignature,
    ) -> Result<(), GeneratorError> {
        traverse_comparison("le", generator, builder, arg_types, return_type)
    }
}

#[derive(Debug)]
pub struct CmpGreater;

impl Word for CmpGreater {
    fn name(&self) -> ClarityName {
        ">".into()
    }
}

impl SimpleWord for CmpGreater {
    fn visit(
        &self,
        generator: &mut WasmGenerator,
        builder: &mut walrus::InstrSeqBuilder,
        arg_types: &[TypeSignature],
        return_type: &TypeSignature,
    ) -> Result<(), GeneratorError> {
        traverse_comparison("gt", generator, builder, arg_types, return_type)
    }
}

#[derive(Debug)]
pub struct CmpGeq;

impl Word for CmpGeq {
    fn name(&self) -> ClarityName {
        ">=".into()
    }
}

impl SimpleWord for CmpGeq {
    fn visit(
        &self,
        generator: &mut WasmGenerator,
        builder: &mut walrus::InstrSeqBuilder,
        arg_types: &[TypeSignature],
        return_type: &TypeSignature,
    ) -> Result<(), GeneratorError> {
        traverse_comparison("ge", generator, builder, arg_types, return_type)
    }
}
