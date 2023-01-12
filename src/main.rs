use anyhow::bail;
use float::{Class, Classifier};
use instructions::float_instructions;
use rand_chacha::rand_core::{RngCore, SeedableRng};
use std::env;
use types::*;
use wasmer::{imports, Cranelift, Instance, Module, Singlepass, Store, Value};

mod float;
mod instructions;
mod types;

fn main() -> anyhow::Result<()> {
    let args: Vec<String> = env::args().collect();

    if args.len() != 3 {
        bail!("Usage: wasmer-float-fuzzer <singlepass|cranelift> <num_iterations>");
    }

    let is_singlepass = match args[1].as_str() {
        "singlepass" => true,
        "cranelift" => false,
        _ => bail!(
            "Error: invalid backend specified. Only singlepass and cranelift are valid options"
        ),
    };

    let num_iterations: u64 = match args[2].parse() {
        Ok(n) => n,
        Err(_) => bail!("Error: invalid number of iterations"),
    };

    // create rng with fixed seed for deterministic random values
    let mut rng = rand_chacha::ChaChaRng::seed_from_u64(123456);

    let instrs = float_instructions();

    // setup module
    let mut store = store(is_singlepass);
    let import_object = imports! {};
    let module = Module::new(&store, create_wat_module(instrs.as_slice()))?;
    let instance = Instance::new(&mut store, &module, &import_object)?;

    println!("Instruction,Param1,Param1(as bits),Param1 Class,Param2,Param2(as bits),Param2 Class,Result,Result(as bits),Result Class");
    for (op, params, _) in instrs {
        run_iterations(&instance, &mut store, op, &params, &mut rng, num_iterations)?;
    }

    Ok(())
}

fn store(singlepass: bool) -> Store {
    if singlepass {
        let mut config = Singlepass::default();
        config.canonicalize_nans(true);
        Store::new(config)
    } else {
        let mut config = Cranelift::default();
        config.canonicalize_nans(true);
        Store::new(config)
    }
}

/// Creates a wat module containing one exported function per instruction given
fn create_wat_module(instrs: &[(&str, Vec<Type>, Type)]) -> Vec<u8> {
    let mut module = "(module ".to_string();
    for (op, params, ret) in instrs {
        module.push_str(&create_wat_fn(op, params, *ret));
    }
    module.push(')');

    wasmer::wat2wasm(module.as_bytes()).unwrap().into_owned()
}

/// Returns the WAT for an exported function that just calls the given operation
fn create_wat_fn(operation: &str, inputs: &[Type], output: Type) -> String {
    let mut params = String::new();
    let mut args = String::new();
    for (i, input) in inputs.iter().enumerate() {
        params.push_str(&format!("(param $x{} {})", i, input));
        args.push_str(&format!("(get_local $x{})", i));
    }
    format!(
        r#"
        (func (export "{operation}") {params} (result {output})
        ({operation} {args}))
        "#
    )
}

/// Runs the given instruction with random inputs for the given number of iterations
fn run_iterations(
    instance: &Instance,
    store: &mut Store,
    instr: &str,
    inputs: &[Type],
    rng: &mut impl RngCore,
    num_iterations: u64,
) -> anyhow::Result<()> {
    for _ in 0..num_iterations {
        let op = instance.exports.get_function(instr)?;

        let params: Vec<_> = inputs.iter().map(|input| input.random_value(rng)).collect();
        let (result, result_bits, result_class) = op
            .call(store, &params)
            .map(|res| format_value(&res[0]))
            .map_err(|rte| rte.to_trap())
            .transpose();

        let mut param_strs = params
            .into_iter()
            .map(|v| {
                let (v, b, c) = format_value(&v);
                format!("{},{},{:?}", v, b, c)
            })
            .collect::<Vec<_>>();
        // assuming at most 2 parameters for table layout (can be changed later)
        if param_strs.len() > 2 {
            panic!("too many parameters");
        }
        param_strs.resize(2, ",".to_string());
        let param_strs = param_strs.join(",");

        println!("{instr},{param_strs},{result:?},{result_bits:?},{result_class:?}");
    }
    Ok(())
}

fn format_value(value: &Value) -> (String, String, Class) {
    match value {
        wasmer::Value::I32(v) => (
            v.to_string(),
            (*v as u32).to_string(),
            (*v as u32).classify(),
        ),
        wasmer::Value::I64(v) => (
            v.to_string(),
            (*v as u64).to_string(),
            (*v as u64).classify(),
        ),
        wasmer::Value::F32(v) => (
            v.to_string(),
            v.to_bits().to_string(),
            v.to_bits().classify(),
        ),
        wasmer::Value::F64(v) => (
            v.to_string(),
            v.to_bits().to_string(),
            v.to_bits().classify(),
        ),
        wasmer::Value::ExternRef(_) => unimplemented!("ExternRef not supported"),
        wasmer::Value::FuncRef(_) => unimplemented!("FuncRef not supported"),
        wasmer::Value::V128(_) => unimplemented!("V128 not supported"),
    }
}
