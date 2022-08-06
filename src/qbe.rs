use itertools::Itertools;

use std::fmt;

#[allow(dead_code)]
#[derive(Clone, Copy, PartialEq, Debug)]
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
            Type::Variadic => write!(f, "..."),
            _ => panic!(),
        }
    }
}

#[allow(dead_code)]
#[derive(Copy, Clone, PartialEq)]
pub enum Opcode {
    Add,
    And,
    Div,
    Mul,
    Neg,
    Or,
    Rem,
    Sar,
    Shl,
    Shr,
    Sub,
    Udiv,
    Urem,
    Xor,
    Alloc16,
    Alloc4,
    Alloc8,
    Loadd,
    Loadl,
    Loads,
    Loadsb,
    Loadsh,
    Loadsw,
    Loadub,
    Loaduh,
    Loaduw,
    Loadw,
    Storeb,
    Stored,
    Storeh,
    Storel,
    Stores,
    Storew,
    Ceqd,
    Ceql,
    Ceqs,
    Ceqw,
    Cged,
    Cges,
    Cgtd,
    Cgts,
    Cled,
    Cles,
    Cltd,
    Clts,
    Cned,
    Cnel,
    Cnes,
    Cnew,
    Cod,
    Cos,
    Csgel,
    Csgew,
    Csgtl,
    Csgtw,
    Cslel,
    Cslew,
    Csltl,
    Csltw,
    Cugel,
    Cugew,
    Cugtl,
    Cugtw,
    Culel,
    Culew,
    Cultl,
    Cultw,
    Cuod,
    Cuos,
    Dtosi,
    Dtoui,
    Exts,
    Extsb,
    Extsh,
    Extsw,
    Extub,
    Extuh,
    Extuw,
    Sltof,
    Ultof,
    Stosi,
    Stoui,
    Swtof,
    Uwtof,
    Truncd,
    Cast,
    Copy,
    Vastart,
    Vaarg,
    Phi,
    Jmp,
    Jnz,
    Ret,
    Call,
}

#[allow(unstable_name_collisions)]
impl fmt::Display for Opcode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Opcode::Add => write!(f, "add"),
            Opcode::And => write!(f, "and"),
            Opcode::Div => write!(f, "div"),
            Opcode::Mul => write!(f, "mul"),
            Opcode::Neg => write!(f, "neg"),
            Opcode::Or => write!(f, "or"),
            Opcode::Rem => write!(f, "rem"),
            Opcode::Sar => write!(f, "sar"),
            Opcode::Shl => write!(f, "shl"),
            Opcode::Shr => write!(f, "shr"),
            Opcode::Sub => write!(f, "sub"),
            Opcode::Udiv => write!(f, "udiv"),
            Opcode::Urem => write!(f, "urem"),
            Opcode::Xor => write!(f, "xor"),
            Opcode::Alloc16 => write!(f, "alloc16"),
            Opcode::Alloc4 => write!(f, "alloc4"),
            Opcode::Alloc8 => write!(f, "alloc8"),
            Opcode::Loadd => write!(f, "loadd"),
            Opcode::Loadl => write!(f, "loadl"),
            Opcode::Loads => write!(f, "loads"),
            Opcode::Loadsb => write!(f, "loadsb"),
            Opcode::Loadsh => write!(f, "loadsh"),
            Opcode::Loadsw => write!(f, "loadsw"),
            Opcode::Loadub => write!(f, "loadub"),
            Opcode::Loaduh => write!(f, "loaduh"),
            Opcode::Loaduw => write!(f, "loaduw"),
            Opcode::Loadw => write!(f, "loadw"),
            Opcode::Storeb => write!(f, "storeb"),
            Opcode::Stored => write!(f, "stored"),
            Opcode::Storeh => write!(f, "storeh"),
            Opcode::Storel => write!(f, "storel"),
            Opcode::Stores => write!(f, "stores"),
            Opcode::Storew => write!(f, "storew"),
            Opcode::Ceqd => write!(f, "ceqd"),
            Opcode::Ceql => write!(f, "ceql"),
            Opcode::Ceqs => write!(f, "ceqs"),
            Opcode::Ceqw => write!(f, "ceqw"),
            Opcode::Cged => write!(f, "cged"),
            Opcode::Cges => write!(f, "cges"),
            Opcode::Cgtd => write!(f, "cgtd"),
            Opcode::Cgts => write!(f, "cgts"),
            Opcode::Cled => write!(f, "cled"),
            Opcode::Cles => write!(f, "cles"),
            Opcode::Cltd => write!(f, "cltd"),
            Opcode::Clts => write!(f, "clts"),
            Opcode::Cned => write!(f, "cned"),
            Opcode::Cnel => write!(f, "cnel"),
            Opcode::Cnes => write!(f, "cnes"),
            Opcode::Cnew => write!(f, "cnew"),
            Opcode::Cod => write!(f, "cod"),
            Opcode::Cos => write!(f, "cos"),
            Opcode::Csgel => write!(f, "csgel"),
            Opcode::Csgew => write!(f, "csgew"),
            Opcode::Csgtl => write!(f, "csgtl"),
            Opcode::Csgtw => write!(f, "csgtw"),
            Opcode::Cslel => write!(f, "cslel"),
            Opcode::Cslew => write!(f, "cslew"),
            Opcode::Csltl => write!(f, "csltl"),
            Opcode::Csltw => write!(f, "cslttw"),
            Opcode::Cugel => write!(f, "cugel"),
            Opcode::Cugew => write!(f, "cugew"),
            Opcode::Cugtl => write!(f, "cugl"),
            Opcode::Cugtw => write!(f, "cugtw"),
            Opcode::Culel => write!(f, "culel"),
            Opcode::Culew => write!(f, "culew"),
            Opcode::Cultl => write!(f, "cultl"),
            Opcode::Cultw => write!(f, "cultw"),
            Opcode::Cuod => write!(f, "cuod"),
            Opcode::Cuos => write!(f, "cuos"),
            Opcode::Dtosi => write!(f, "dtosi"),
            Opcode::Dtoui => write!(f, "dtoui"),
            Opcode::Exts => write!(f, "exts"),
            Opcode::Extsb => write!(f, "extsb"),
            Opcode::Extsh => write!(f, "extsh"),
            Opcode::Extsw => write!(f, "estsw"),
            Opcode::Extub => write!(f, "extub"),
            Opcode::Extuh => write!(f, "extuh"),
            Opcode::Extuw => write!(f, "extuw"),
            Opcode::Sltof => write!(f, "sltof"),
            Opcode::Ultof => write!(f, "ultof"),
            Opcode::Stosi => write!(f, "stosi"),
            Opcode::Stoui => write!(f, "stoui"),
            Opcode::Swtof => write!(f, "swtof"),
            Opcode::Uwtof => write!(f, "uwtof"),
            Opcode::Truncd => write!(f, "trucd"),
            Opcode::Cast => write!(f, "cast"),
            Opcode::Copy => write!(f, "copy"),
            Opcode::Vastart => write!(f, "vastart"),
            Opcode::Vaarg => write!(f, "vaarg"),
            Opcode::Jmp => write!(f, "jmp"),
            Opcode::Jnz => write!(f, "jnz"),
            Opcode::Ret => write!(f, "ret"),
            Opcode::Phi => write!(f, "phi"),
            Opcode::Call => write!(f, "call"),
        }
    }
}

impl Opcode {}

#[derive(Clone, PartialEq)]
pub struct Instruction {
    opcode: Opcode,
    args: Vec<Value>,
}

impl fmt::Display for Instruction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.opcode {
            Opcode::Phi => {
                let args: String = self
                    .args
                    .windows(2)
                    .map(|values| format!("{} {}", values[0], values[1]))
                    .intersperse(", ".to_string())
                    .collect();
                write!(f, "{} {}", self.opcode, args)
            }
            Opcode::Call => {
                let args: String = self.args[1..]
                    .iter()
                    .map(|arg| match arg {
                        Value::Variadic => format!("{}", arg.get_type().unwrap()),
                        _ => format!(
                            "{} {}",
                            arg.get_type().expect("Expected value with type"),
                            arg
                        ),
                    })
                    .intersperse(", ".to_string())
                    .collect();
                write!(f, "{} {}({})", self.opcode, self.args[0], args)
            }
            _ => {
                let args: String = self
                    .args
                    .iter()
                    .map(|arg| format!("{}", arg))
                    .intersperse(", ".to_string())
                    .collect();
                write!(f, "{} {}", self.opcode, args)
            }
        }
    }
}

impl Instruction {
    pub fn new(opcode: &Opcode, args: Vec<Value>) -> Instruction {
        Instruction {
            opcode: *opcode,
            args,
        }
    }
}

#[allow(dead_code)]
#[derive(Clone, PartialEq)]
pub enum Value {
    Local(String, Type),
    Global(String, Type),
    ConstWord(u32),
    ConstLong(u64),
    ConstSingle(f32),
    ConstDouble(f64),
    ConstByte(u8),
    ConstHalfWord(u16),
    Block(String),
    Variadic,
    None,
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Value::Local(var, _) => write!(f, "%{}", var),
            Value::Global(var, _) => write!(f, "${}", var),
            Value::ConstWord(constant) => write!(f, "{}", constant),
            Value::ConstLong(constant) => write!(f, "{}", constant),
            Value::ConstSingle(constant) => write!(f, "{}", constant),
            Value::ConstDouble(constant) => write!(f, "{}", constant),
            Value::ConstByte(constant) => write!(f, "{}", constant),
            Value::ConstHalfWord(constant) => write!(f, "{}", constant),
            Value::Block(block) => write!(f, "@{}", block),
            _ => panic!(),
        }
    }
}

impl Value {
    fn get_type(&self) -> Result<Type, String> {
        match self {
            Value::Local(_, typ) => Ok(*typ),
            Value::Global(_, typ) => Ok(*typ),
            Value::ConstWord(_) => Ok(Type::Word),
            Value::ConstLong(_) => Ok(Type::Long),
            Value::ConstSingle(_) => Ok(Type::Single),
            Value::ConstDouble(_) => Ok(Type::Double),
            Value::ConstByte(_) => Ok(Type::Byte),
            Value::ConstHalfWord(_) => Ok(Type::HalfWord),
            Value::Block(_) => Err("Block value has no type".to_string()),
            Value::Variadic => Ok(Type::Variadic),
            _ => panic!(),
        }
    }

    pub fn local(name: &str, typ: Type) -> Value {
        Value::Local(name.to_string(), typ)
    }

    pub fn global(name: &str, typ: Type) -> Value {
        Value::Global(name.to_string(), typ)
    }

    pub fn block(name: &str) -> Value {
        Value::Block(name.to_string())
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
            Some(ret) => write!(
                f,
                "{} ={} {}",
                ret,
                ret.get_type().expect("expeced type"),
                self.instruction
            )?,
        };

        Ok(())
    }
}

impl Statement {
    pub fn new(instruction: Instruction, ret: Option<Value>) -> Statement {
        Statement { instruction, ret }
    }

    pub fn set_ret(&mut self, ret: Option<Value>) {
        self.ret = ret;
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

#[cfg(test)]
mod test {
    use super::*;
    use rstest::*;

    #[rstest]
    #[case(Value::Local("a".to_string(), Type::Word),  "%a",     Ok(Type::Word))]
    #[case(Value::Global("a".to_string(), Type::Word), "$a",     Ok(Type::Word))]
    #[case(Value::ConstWord(0), "0", Ok(Type::Word))]
    #[case(Value::ConstLong(0), "0", Ok(Type::Long))]
    #[case(Value::ConstSingle(0.0), "0", Ok(Type::Single))]
    #[case(Value::ConstDouble(0.0), "0", Ok(Type::Double))]
    #[case(Value::ConstByte(0), "0", Ok(Type::Byte))]
    #[case(Value::ConstHalfWord(0), "0", Ok(Type::HalfWord))]
    #[case(Value::Block("block".to_string()),          "@block", Err("Block value has no type".to_string()))]
    fn test_value_init(
        #[case] value: Value,
        #[case] format: &str,
        #[case] typ: Result<Type, String>,
    ) {
        assert_eq!(format!("{}", value), format.to_string());
        assert_eq!(value.get_type(), typ);
    }

    #[rstest]
    #[case(Opcode::Add,  vec![Value::ConstWord(0), Value::ConstWord(1)], "add 0, 1")]
    #[case(Opcode::Neg,  vec![Value::ConstWord(0)], "neg 0")]
    #[case(Opcode::Alloc16,  vec![Value::ConstWord(0)], "alloc16 0")]
    #[case(Opcode::Ret,  vec![], "ret ")]
    #[case(Opcode::Ret,  vec![Value::ConstWord(0)], "ret 0")]
    #[case(Opcode::Phi,  vec![Value::local("a", Type::Word), Value::ConstWord(0)], "phi %a 0")]
    #[case(Opcode::Call,  vec![Value::global("printf", Type::Word), Value::local("fmt", Type::Long), Value::Variadic, Value::ConstWord(0)], "call $printf(l %fmt, ..., w 0)")]
    fn test_instruction_init(
        #[case] opcode: Opcode,
        #[case] args: Vec<Value>,
        #[case] result: &str,
    ) {
        assert_eq!(format!("{}", Instruction::new(&opcode, args)), result);
    }
}
