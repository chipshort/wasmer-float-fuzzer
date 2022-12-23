use crate::types::Type;

/// Taken from [`wasmer::wasmparser::Operator`].
pub fn float_instructions() -> [(&'static str, Vec<Type>, Type); 70] {
    [
        ("f32.eq", vec![Type::F32, Type::F32], Type::I32),
        ("f32.ne", vec![Type::F32, Type::F32], Type::I32),
        ("f32.lt", vec![Type::F32, Type::F32], Type::I32),
        ("f32.gt", vec![Type::F32, Type::F32], Type::I32),
        ("f32.le", vec![Type::F32, Type::F32], Type::I32),
        ("f32.ge", vec![Type::F32, Type::F32], Type::I32),
        ("f64.eq", vec![Type::F64, Type::F64], Type::I32),
        ("f64.ne", vec![Type::F64, Type::F64], Type::I32),
        ("f64.lt", vec![Type::F64, Type::F64], Type::I32),
        ("f64.gt", vec![Type::F64, Type::F64], Type::I32),
        ("f64.le", vec![Type::F64, Type::F64], Type::I32),
        ("f64.ge", vec![Type::F64, Type::F64], Type::I32),
        //
        ("f32.abs", vec![Type::F32], Type::F32),
        ("f32.neg", vec![Type::F32], Type::F32),
        ("f32.ceil", vec![Type::F32], Type::F32),
        ("f32.floor", vec![Type::F32], Type::F32),
        ("f32.trunc", vec![Type::F32], Type::F32),
        ("f32.nearest", vec![Type::F32], Type::F32),
        ("f32.sqrt", vec![Type::F32], Type::F32),
        ("f32.add", vec![Type::F32, Type::F32], Type::F32),
        ("f32.sub", vec![Type::F32, Type::F32], Type::F32),
        ("f32.mul", vec![Type::F32, Type::F32], Type::F32),
        ("f32.div", vec![Type::F32, Type::F32], Type::F32),
        ("f32.min", vec![Type::F32, Type::F32], Type::F32),
        ("f32.max", vec![Type::F32, Type::F32], Type::F32),
        ("f32.copysign", vec![Type::F32, Type::F32], Type::F32),
        ("f64.abs", vec![Type::F64], Type::F64),
        ("f64.neg", vec![Type::F64], Type::F64),
        ("f64.ceil", vec![Type::F64], Type::F64),
        ("f64.floor", vec![Type::F64], Type::F64),
        ("f64.trunc", vec![Type::F64], Type::F64),
        ("f64.nearest", vec![Type::F64], Type::F64),
        ("f64.sqrt", vec![Type::F64], Type::F64),
        ("f64.add", vec![Type::F64, Type::F64], Type::F64),
        ("f64.sub", vec![Type::F64, Type::F64], Type::F64),
        ("f64.mul", vec![Type::F64, Type::F64], Type::F64),
        ("f64.div", vec![Type::F64, Type::F64], Type::F64),
        ("f64.min", vec![Type::F64, Type::F64], Type::F64),
        ("f64.max", vec![Type::F64, Type::F64], Type::F64),
        ("f64.copysign", vec![Type::F64, Type::F64], Type::F64),
        //
        ("i32.trunc_f32_s", vec![Type::F32], Type::I32),
        ("i32.trunc_f32_u", vec![Type::F32], Type::I32),
        ("i32.trunc_f64_s", vec![Type::F64], Type::I32),
        ("i32.trunc_f64_u", vec![Type::F64], Type::I32),
        //
        ("i64.trunc_f32_s", vec![Type::F32], Type::I64),
        ("i64.trunc_f32_u", vec![Type::F32], Type::I64),
        ("i64.trunc_f64_s", vec![Type::F64], Type::I64),
        ("i64.trunc_f64_u", vec![Type::F64], Type::I64),
        //
        ("f32.convert_i32_s", vec![Type::I32], Type::F32),
        ("f32.convert_i32_u", vec![Type::I32], Type::F32),
        ("f32.convert_i64_s", vec![Type::I64], Type::F32),
        ("f32.convert_i64_u", vec![Type::I64], Type::F32),
        ("f32.demote_f64", vec![Type::F64], Type::F32),
        ("f64.convert_i32_s", vec![Type::I32], Type::F64),
        ("f64.convert_i32_u", vec![Type::I32], Type::F64),
        ("f64.convert_i64_s", vec![Type::I64], Type::F64),
        ("f64.convert_i64_u", vec![Type::I64], Type::F64),
        ("f64.promote_f32", vec![Type::F32], Type::F64),
        //
        ("i32.reinterpret_f32", vec![Type::F32], Type::I32),
        ("i64.reinterpret_f64", vec![Type::F64], Type::I64),
        ("f32.reinterpret_i32", vec![Type::I32], Type::F32),
        ("f64.reinterpret_i64", vec![Type::I64], Type::F64),
        //
        ("i32.trunc_sat_f32_s", vec![Type::F32], Type::I32),
        ("i32.trunc_sat_f32_u", vec![Type::F32], Type::I32),
        ("i32.trunc_sat_f64_s", vec![Type::F64], Type::I32),
        ("i32.trunc_sat_f64_u", vec![Type::F64], Type::I32),
        ("i64.trunc_sat_f32_s", vec![Type::F32], Type::I64),
        ("i64.trunc_sat_f32_u", vec![Type::F32], Type::I64),
        ("i64.trunc_sat_f64_s", vec![Type::F64], Type::I64),
        ("i64.trunc_sat_f64_u", vec![Type::F64], Type::I64),
        //
        // F32x4Splat,
        // F64x2Splat,
        //
        // F32x4Eq,
        // F32x4Ne,
        // F32x4Lt,
        // F32x4Gt,
        // F32x4Le,
        // F32x4Ge,
        // F64x2Eq,
        // F64x2Ne,
        // F64x2Lt,
        // F64x2Gt,
        // F64x2Le,
        // F64x2Ge,
        //
        // F32x4Ceil,
        // F32x4Floor,
        // F32x4Trunc,
        // F32x4Nearest,
        // F32x4Abs,
        // F32x4Neg,
        // F32x4Sqrt,
        // F32x4Add,
        // F32x4Sub,
        // F32x4Mul,
        // F32x4Div,
        // F32x4Min,
        // F32x4Max,
        // F32x4PMin,
        // F32x4PMax,
        // F64x2Ceil,
        // F64x2Floor,
        // F64x2Trunc,
        // F64x2Nearest,
        // F64x2Abs,
        // F64x2Neg,
        // F64x2Sqrt,
        // F64x2Add,
        // F64x2Sub,
        // F64x2Mul,
        // F64x2Div,
        // F64x2Min,
        // F64x2Max,
        // F64x2PMin,
        // F64x2PMax,
        // I32x4TruncSatF32x4S,
        // I32x4TruncSatF32x4U,
        // F32x4ConvertI32x4S,
        // F32x4ConvertI32x4U,
        // I32x4TruncSatF64x2SZero,
        // I32x4TruncSatF64x2UZero,
        // F64x2ConvertLowI32x4S,
        // F64x2ConvertLowI32x4U,
        // F32x4DemoteF64x2Zero,
        // F64x2PromoteLowF32x4,
        // I32x4RelaxedTruncSatF32x4S,
        // I32x4RelaxedTruncSatF32x4U,
        // I32x4RelaxedTruncSatF64x2SZero,
        // I32x4RelaxedTruncSatF64x2UZero,
        // F32x4Fma,
        // F32x4Fms,
        // F64x2Fma,
        // F64x2Fms,
        //
        // F32x4RelaxedMin,
        // F32x4RelaxedMax,
        // F64x2RelaxedMin,
        // F64x2RelaxedMax,
    ]
}