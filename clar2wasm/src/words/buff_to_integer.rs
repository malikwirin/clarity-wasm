use clarity::vm::types::TypeSignature;

use crate::wasm_generator::{GeneratorError, WasmGenerator};
use crate::words::{SimpleWord, Word};

fn traverse_buffer_to_integer(
    name: &str,
    generator: &mut WasmGenerator,
    builder: &mut walrus::InstrSeqBuilder,
) -> Result<(), GeneratorError> {
    let func = generator
        .module
        .funcs
        .by_name(name)
        .ok_or_else(|| GeneratorError::InternalError(format!("function not found: {name}")))?;
    builder.call(func);
    Ok(())
}

#[derive(Debug)]
pub struct BuffToUintBe;

impl Word for BuffToUintBe {
    fn name(&self) -> clarity::vm::ClarityName {
        "buff-to-uint-be".into()
    }
}

impl SimpleWord for BuffToUintBe {
    fn visit(
        &self,
        generator: &mut crate::wasm_generator::WasmGenerator,
        builder: &mut walrus::InstrSeqBuilder,
        _arg_types: &[TypeSignature],
        _return_type: &TypeSignature,
    ) -> Result<(), crate::wasm_generator::GeneratorError> {
        traverse_buffer_to_integer("stdlib.buff-to-uint-be", generator, builder)
    }
}

#[derive(Debug)]
pub struct BuffToIntBe;

impl Word for BuffToIntBe {
    fn name(&self) -> clarity::vm::ClarityName {
        "buff-to-int-be".into()
    }
}

impl SimpleWord for BuffToIntBe {
    fn visit(
        &self,
        generator: &mut crate::wasm_generator::WasmGenerator,
        builder: &mut walrus::InstrSeqBuilder,
        _arg_types: &[TypeSignature],
        _return_type: &TypeSignature,
    ) -> Result<(), crate::wasm_generator::GeneratorError> {
        // This is the same function as "buff-to-uint-be", with the result interpreted
        // as i128 instead of u128.
        traverse_buffer_to_integer("stdlib.buff-to-uint-be", generator, builder)
    }
}

#[derive(Debug)]
pub struct BuffToUintLe;

impl Word for BuffToUintLe {
    fn name(&self) -> clarity::vm::ClarityName {
        "buff-to-uint-le".into()
    }
}

impl SimpleWord for BuffToUintLe {
    fn visit(
        &self,
        generator: &mut crate::wasm_generator::WasmGenerator,
        builder: &mut walrus::InstrSeqBuilder,
        _arg_types: &[TypeSignature],
        _return_type: &TypeSignature,
    ) -> Result<(), crate::wasm_generator::GeneratorError> {
        traverse_buffer_to_integer("stdlib.buff-to-uint-le", generator, builder)
    }
}

#[derive(Debug)]
pub struct BuffToIntLe;

impl Word for BuffToIntLe {
    fn name(&self) -> clarity::vm::ClarityName {
        "buff-to-int-le".into()
    }
}

impl SimpleWord for BuffToIntLe {
    fn visit(
        &self,
        generator: &mut crate::wasm_generator::WasmGenerator,
        builder: &mut walrus::InstrSeqBuilder,
        _arg_types: &[TypeSignature],
        _return_type: &TypeSignature,
    ) -> Result<(), crate::wasm_generator::GeneratorError> {
        // This is the same function as "buff-to-uint-le", with the result interpreted
        // as i128 instead of u128.
        traverse_buffer_to_integer("stdlib.buff-to-uint-le", generator, builder)
    }
}
