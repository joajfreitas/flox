use itertools::Itertools;

use std::fmt;

#[allow(dead_code)]
#[derive(Clone, Copy, PartialEq)]
pub enum Type {
    Word,
    Long,
    Single,
    Double,
    Byte,
    HalfWord,
    Variadic,
}

impl fmt::Display for Type {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Type::Word => write!(f, "w"),
            Type::Long => write!(f, "l"),
            _ => panic!(),
        }
    }
}

#[allow(dead_code)]
#[derive(Clone, PartialEq)]
pub enum Instruction {
    Add(Value, Value),
    And(Value, Value),
    Div(Value, Value),
    Mul(Value, Value),
    Neg(Value, Value),
    Or(Value, Value),
    Rem(Value, Value),
    Sar(Value, Value),
    Shl(Value, Value),
    Shr(Value, Value),
    Sub(Value, Value),
    Udiv(Value, Value),
    Urem(Value, Value),
    Xor(Value, Value),
    Alloc16(Value),
    Alloc4(Value),
    Alloc8(Value),
    Loadd(Value),
    Loadl(Value),
    Loads(Value),
    Loadsb(Value),
    Loadsh(Value),
    Loadsw(Value),
    Loadub(Value),
    Loaduh(Value),
    Loaduw(Value),
    Loadw(Value),
    Storeb(Value, Value),
    Stored(Value, Value),
    Storeh(Value, Value),
    Storel(Value, Value),
    Stores(Value, Value),
    Storew(Value, Value),
    Ceqd(Value, Value),
    Ceql(Value, Value),
    Ceqs(Value, Value),
    Ceqw(Value, Value),
    Cged(Value, Value),
    Cges(Value, Value),
    Cgtd(Value, Value),
    Cgts(Value, Value),
    Cled(Value, Value),
    Cles(Value, Value),
    Cltd(Value, Value),
    Clts(Value, Value),
    Cned(Value, Value),
    Cnel(Value, Value),
    Cnes(Value, Value),
    Cnew(Value, Value),
    Cod(Value, Value),
    Cos(Value, Value),
    Csgel(Value, Value),
    Csgew(Value, Value),
    Csgtl(Value, Value),
    Csgtw(Value, Value),
    Cslel(Value, Value),
    Cslew(Value, Value),
    Csltl(Value, Value),
    Csltw(Value, Value),
    Cugel(Value, Value),
    Cugew(Value, Value),
    Cugtl(Value, Value),
    Cugtw(Value, Value),
    Culel(Value, Value),
    Culew(Value, Value),
    Cultl(Value, Value),
    Cultw(Value, Value),
    Cuod(Value, Value),
    Cuos(Value, Value),
    Dtosi(Value),
    Dtoui(Value),
    Exts(Value),
    Extsb(Value),
    Extsh(Value),
    Extsw(Value),
    Extub(Value),
    Extuh(Value),
    Extuw(Value),
    Sltof(Value),
    Ultof(Value),
    Stosi(Value),
    Stoui(Value),
    Swtof(Value),
    Uwtof(Value),
    Truncd(Value),
    Cast(Value),
    Copy(Value),
    Vastart(Value),
    Vaarg(Value),
    Phi(Vec<(String, Value)>),
    Jmp(String),
    Jnz(Value, String, String),
    Ret(Value),
    Call(String, Vec<(Type, Value)>),
}

#[allow(unstable_name_collisions)]
impl fmt::Display for Instruction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Instruction::Add(a, b) => write!(f, "add {}, {}", a, b),
            Instruction::And(a, b) => write!(f, "and {}, {}", a, b),
            Instruction::Div(a, b) => write!(f, "div {}, {}", a, b),
            Instruction::Mul(a, b) => write!(f, "mul {}, {}", a, b),
            Instruction::Neg(a, b) => write!(f, "neg {}, {}", a, b),
            Instruction::Or(a, b) => write!(f, "or {}, {}", a, b),
            Instruction::Rem(a, b) => write!(f, "rem {}, {}", a, b),
            Instruction::Sar(a, b) => write!(f, "sar {}, {}", a, b),
            Instruction::Shl(a, b) => write!(f, "shl {}, {}", a, b),
            Instruction::Shr(a, b) => write!(f, "shr {}, {}", a, b),
            Instruction::Sub(a, b) => write!(f, "sub {}, {}", a, b),
            Instruction::Udiv(a, b) => write!(f, "udiv {}, {}", a, b),
            Instruction::Urem(a, b) => write!(f, "urem {}, {}", a, b),
            Instruction::Xor(a, b) => write!(f, "xor {}, {}", a, b),
            Instruction::Alloc16(x) => write!(f, "alloc16 {}", x),
            Instruction::Alloc4(x) => write!(f, "alloc4 {}", x),
            Instruction::Alloc8(x) => write!(f, "alloc8 {}", x),
            Instruction::Loadd(x) => write!(f, "loadd {}", x),
            Instruction::Loadl(x) => write!(f, "loadl {}", x),
            Instruction::Loads(x) => write!(f, "loads {}", x),
            Instruction::Loadsb(x) => write!(f, "loadsb {}", x),
            Instruction::Loadsh(x) => write!(f, "loadsh {}", x),
            Instruction::Loadsw(x) => write!(f, "loadsw {}", x),
            Instruction::Loadub(x) => write!(f, "loadub {}", x),
            Instruction::Loaduh(x) => write!(f, "loaduh {}", x),
            Instruction::Loaduw(x) => write!(f, "loaduw {}", x),
            Instruction::Loadw(x) => write!(f, "loadw {}", x),
            Instruction::Storeb(a, b) => write!(f, "storeb {} {}", a, b),
            Instruction::Stored(a, b) => write!(f, "stored {} {}", a, b),
            Instruction::Storeh(a, b) => write!(f, "storeh {} {}", a, b),
            Instruction::Storel(a, b) => write!(f, "storel {} {}", a, b),
            Instruction::Stores(a, b) => write!(f, "stores {} {}", a, b),
            Instruction::Storew(a, b) => write!(f, "storew {} {}", a, b),
            Instruction::Ceqd(a, b) => write!(f, "ceqd {} {}", a, b),
            Instruction::Ceql(a, b) => write!(f, "ceql {} {}", a, b),
            Instruction::Ceqs(a, b) => write!(f, "ceqs {} {}", a, b),
            Instruction::Ceqw(a, b) => write!(f, "ceqw {} {}", a, b),
            Instruction::Cged(a, b) => write!(f, "cged {} {}", a, b),
            Instruction::Cges(a, b) => write!(f, "cges {} {}", a, b),
            Instruction::Cgtd(a, b) => write!(f, "cgtd {} {}", a, b),
            Instruction::Cgts(a, b) => write!(f, "cgts {} {}", a, b),
            Instruction::Cled(a, b) => write!(f, "cled {} {}", a, b),
            Instruction::Cles(a, b) => write!(f, "cles {} {}", a, b),
            Instruction::Cltd(a, b) => write!(f, "cltd {} {}", a, b),
            Instruction::Clts(a, b) => write!(f, "clts {} {}", a, b),
            Instruction::Cned(a, b) => write!(f, "cned {} {}", a, b),
            Instruction::Cnel(a, b) => write!(f, "cnel {} {}", a, b),
            Instruction::Cnes(a, b) => write!(f, "cnes {} {}", a, b),
            Instruction::Cnew(a, b) => write!(f, "cnew {} {}", a, b),
            Instruction::Cod(a, b) => write!(f, "cod {} {}", a, b),
            Instruction::Cos(a, b) => write!(f, "cos {} {}", a, b),
            Instruction::Csgel(a, b) => write!(f, "csgel {} {}", a, b),
            Instruction::Csgew(a, b) => write!(f, "csgew {} {}", a, b),
            Instruction::Csgtl(a, b) => write!(f, "csgtl {} {}", a, b),
            Instruction::Csgtw(a, b) => write!(f, "csgtw {} {}", a, b),
            Instruction::Cslel(a, b) => write!(f, "cslel {} {}", a, b),
            Instruction::Cslew(a, b) => write!(f, "cslew {} {}", a, b),
            Instruction::Csltl(a, b) => write!(f, "csltl {} {}", a, b),
            Instruction::Csltw(a, b) => write!(f, "cslttw {} {}", a, b),
            Instruction::Cugel(a, b) => write!(f, "cugel {} {}", a, b),
            Instruction::Cugew(a, b) => write!(f, "cugew {} {}", a, b),
            Instruction::Cugtl(a, b) => write!(f, "cugl {} {}", a, b),
            Instruction::Cugtw(a, b) => write!(f, "cugtw {} {}", a, b),
            Instruction::Culel(a, b) => write!(f, "culel {} {}", a, b),
            Instruction::Culew(a, b) => write!(f, "culew {} {}", a, b),
            Instruction::Cultl(a, b) => write!(f, "cultl {} {}", a, b),
            Instruction::Cultw(a, b) => write!(f, "cultw {} {}", a, b),
            Instruction::Cuod(a, b) => write!(f, "cuod {} {}", a, b),
            Instruction::Cuos(a, b) => write!(f, "cuos {} {}", a, b),
            Instruction::Dtosi(x) => write!(f, "dtosi {}", x),
            Instruction::Dtoui(x) => write!(f, "dtoui {}", x),
            Instruction::Exts(x) => write!(f, "exts {}", x),
            Instruction::Extsb(x) => write!(f, "extsb {}", x),
            Instruction::Extsh(x) => write!(f, "extsh {}", x),
            Instruction::Extsw(x) => write!(f, "estsw {}", x),
            Instruction::Extub(x) => write!(f, "extub {}", x),
            Instruction::Extuh(x) => write!(f, "extuh {}", x),
            Instruction::Extuw(x) => write!(f, "extuw {}", x),
            Instruction::Sltof(x) => write!(f, "sltof {}", x),
            Instruction::Ultof(x) => write!(f, "ultof {}", x),
            Instruction::Stosi(x) => write!(f, "stosi {}", x),
            Instruction::Stoui(x) => write!(f, "stoui {}", x),
            Instruction::Swtof(x) => write!(f, "swtof {}", x),
            Instruction::Uwtof(x) => write!(f, "uwtof {}", x),
            Instruction::Truncd(x) => write!(f, "trucd {}", x),
            Instruction::Cast(x) => write!(f, "cast {}", x),
            Instruction::Copy(x) => write!(f, "copy {}", x),
            Instruction::Vastart(x) => write!(f, "vastart {}", x),
            Instruction::Vaarg(x) => write!(f, "vaarg {}", x),
            Instruction::Jmp(x) => write!(f, "jmp {}", x),
            Instruction::Jnz(v, i1, i2) => write!(f, "jnz {} {} {}", v, i1, i2),
            Instruction::Ret(x) => write!(f, "ret {}", x),

            Instruction::Phi(phis) => {
                let args: String = phis
                    .iter()
                    .map(|(identifier, value)| format!("@{} {}", identifier, value))
                    .intersperse(", ".to_string())
                    .collect();
                write!(f, "phi {}", args)
            }
            Instruction::Call(name, arguments) => {
                write!(f, "call ${}(", name)?;

                let args: String = arguments
                    .iter()
                    .map(|(typ, value)| match *typ {
                        Type::Variadic => format!("..."),
                        _ => format!("{} {}", typ, value),
                    })
                    .intersperse(", ".to_string())
                    .collect();

                write!(f, "{})", args)
            }
        }
    }
}

#[allow(dead_code)]
#[derive(Clone, PartialEq)]
pub enum Value {
    Local(String, Type),
    Global(String, Type),
    Const(u64, Type),
    ConstSingle(f32, Type),
    ConstDouble(f64, Type),
    None,
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Value::Local(var, _) => write!(f, "%{}", var),
            Value::Global(var, _) => write!(f, "${}", var),
            Value::Const(constant, _) => write!(f, "{}", constant),
            Value::ConstSingle(constant, _) => write!(f, "s_{}", constant),
            Value::ConstDouble(constant, _) => write!(f, "d_{}", constant),
            _ => panic!(),
        }
    }
}

impl Value {
    fn get_type(&self) -> Type {
        match self {
            Value::Local(_, typ) => *typ,
            Value::Global(_, typ) => *typ,
            Value::Const(_, typ) => *typ,
            Value::ConstSingle(_, typ) => *typ,
            Value::ConstDouble(_, typ) => *typ,
            _ => panic!(),
        }
    }
}

#[derive(Clone)]
pub struct Statement {
    instruction: Instruction,
    ret: Option<Value>,
}

impl fmt::Display for Statement {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.ret {
            None => write!(f, "{}", self.instruction)?,
            Some(ret) => write!(f, "{} ={} {}", ret, ret.get_type(), self.instruction)?,
        };

        Ok(())
    }
}

impl Statement {
    pub fn new(instruction: Instruction, ret: Option<Value>) -> Statement {
        Statement { instruction, ret }
    }
}

#[derive(Clone)]
pub struct Block {
    statements: Vec<Statement>,
    name: String,
}

impl fmt::Display for Block {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "@{}\n", self.name)?;
        for statement in &self.statements {
            write!(f, "\t{}\n", statement)?;
        }
        Ok(())
    }
}

impl Block {
    pub fn new(name: &str) -> Block {
        Block {
            statements: Vec::new(),
            name: name.to_string(),
        }
    }

    pub fn add_statement(&mut self, statement: Statement) {
        self.statements.push(statement);
    }
}

#[derive(Clone)]
pub struct Function {
    block: Block,
    name: String,
    args: Vec<(Type, Value)>,
    ret: Option<Type>,
    export: bool,
}

#[allow(unstable_name_collisions)]
impl fmt::Display for Function {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let export = if self.export == true { "export" } else { "" };

        let args: String = self
            .args
            .iter()
            .map(|x| format!("{} {}", x.0, x.1))
            .intersperse(", ".to_string())
            .collect();

        match self.ret {
            None => write!(f, "{} function ${}({}) {{\n", export, self.name, args)?,
            Some(ret) => write!(
                f,
                "{} function {} ${}({}) {{\n",
                export, ret, self.name, args
            )?,
        }
        write!(f, "{}\n", self.block)?;
        write!(f, "}}")
    }
}

impl Function {
    pub fn new(name: &str, ret: Option<Type>, args: Vec<(Type, Value)>, export: bool) -> Function {
        Function {
            name: name.to_string(),
            block: Block::new("start"),
            ret: ret,
            args: args,
            export,
        }
    }

    pub fn add_statement(&mut self, statement: Statement) {
        self.block.add_statement(statement);
    }
}

#[derive(Clone)]
pub struct Data {
    identifier: String,
    data: String,
}

impl fmt::Display for Data {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "data ${} = {{ b \"{}\"}}", self.identifier, self.data)
    }
}

impl Data {
    pub fn new(identifier: String, data: String) -> Data {
        Data { identifier, data }
    }
}

#[allow(dead_code)]
#[derive(Clone)]
pub struct Program {
    functions: Vec<Function>,
    types: Vec<String>,
    data: Vec<Data>,
}

impl fmt::Display for Program {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for function in &self.functions {
            write!(f, "{}\n", function)?;
        }

        for data in &self.data {
            write!(f, "{}\n", data)?;
        }

        Ok(())
    }
}

impl Program {
    pub fn new() -> Program {
        Program {
            functions: Vec::new(),
            types: Vec::new(),
            data: Vec::new(),
        }
    }

    pub fn add_function(&mut self, function: &Function) {
        self.functions.push(function.clone());
    }

    pub fn add_data(&mut self, data: &Data) {
        self.data.push(data.clone());
    }
}
