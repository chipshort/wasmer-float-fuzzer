use anyhow::bail;
use instructions::float_instructions;
use rand_chacha::rand_core::{RngCore, SeedableRng};
use std::env;
use types::*;
use wasmer::{imports, Cranelift, Instance, Module, Store, Value};

mod instructions;
mod types;

fn main() -> anyhow::Result<()> {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        bail!("Usage: wasmer-float-fuzzer <num_iterations>");
    }

    let num_iterations: u64 = match args[1].parse() {
        Ok(n) => n,
        Err(_) => bail!("Error: invalid number of iterations"),
    };

    // create rng with fixed seed for deterministic random values
    let mut rng = rand_chacha::ChaChaRng::seed_from_u64(123456);

    let instrs = float_instructions();
    // setup module
    let mut store = store();
    let import_object = imports! {};
    let module = Module::new(&store, &create_wat_module(instrs.as_slice()))?;
    let instance = Instance::new(&mut store, &module, &import_object)?;

    println!("Instruction,Param1,Param1(as bits),Param2,Param2(as bits),Result,Result(as bits)");
    for (op, params, _) in instrs {
        run_iterations(&instance, &mut store, op, &params, &mut rng, num_iterations)?;
    }
    // TODO: also run instructions for specific combinations (subnormals, NaNs, infinities, etc.)

    Ok(())
}

fn store() -> Store {
    // TODO: also test Singlepass? Somehow it does not work on my M1 Macbook
    let mut config = Cranelift::default();
    config.canonicalize_nans(true);
    // Store::new(config)
    Store::default()
}

/// Creates a wat module containing one exported function per instruction given
fn create_wat_module(instrs: &[(&str, Vec<Type>, Type)]) -> String {
    let mut module = "(module ".to_string();
    for (op, params, ret) in instrs {
        module.push_str(&create_wat_fn(op, params, *ret));
    }
    module.push(')');

    module
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
        let (result, result_bits) = op
            .call(store, &params)
            .map(|res| format_value(&res[0]))
            .map_err(|_| ())
            .transpose();

        let mut param_strs = params
            .into_iter()
            .map(|v| {
                let (v, b) = format_value(&v);
                format!("{},{}", v, b)
            })
            .collect::<Vec<_>>();
        // assuming at most 2 parameters for table layout (can be changed later)
        if param_strs.len() > 2 {
            panic!("too many parameters");
        }
        param_strs.resize(2, ",".to_string());
        let param_strs = param_strs.join(",");

        println!("{instr},{param_strs},{result:?},{result_bits:?}");
    }
    Ok(())
}

fn format_value(value: &Value) -> (String, String) {
    match value {
        wasmer::Value::I32(v) => (v.to_string(), (*v as u32).to_string()),
        wasmer::Value::I64(v) => (v.to_string(), (*v as u32).to_string()),
        wasmer::Value::F32(v) => (v.to_string(), v.to_bits().to_string()),
        wasmer::Value::F64(v) => (v.to_string(), v.to_bits().to_string()),
        wasmer::Value::ExternRef(_) => unimplemented!("ExternRef not supported"),
        wasmer::Value::FuncRef(_) => unimplemented!("FuncRef not supported"),
        wasmer::Value::V128(_) => unimplemented!("V128 not supported"),
    }
}

trait Transposable {
    type Transposed;
    fn transpose(self) -> Self::Transposed;
}

impl<A, B, E: Clone> Transposable for Result<(A, B), E> {
    type Transposed = (Result<A, E>, Result<B, E>);

    fn transpose(self) -> Self::Transposed {
        match self {
            Ok((a, b)) => (Ok(a), Ok(b)),
            Err(e) => (Err(e.clone()), Err(e)),
        }
    }
}
