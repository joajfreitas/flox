use std::fmt;

#[derive(Debug)]
pub enum OpCode {
    OpReturn,
    OpConstant,
    OpConstantLong,
}

pub enum Value {
    Value(f64),
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Value::Value(value) => write!(f, "{:.1}", value),
        }
    }
}

#[derive(Debug)]
pub enum Element {
    OpCode(OpCode),
    Constant(u8),
}

impl Element {
    fn get_constant(&self) -> Option<u8> {
        match self {
            Element::Constant(i) => Some(*i),
            _ => None,
        }
    }
}

pub struct Chunk<'a> {
    name: &'a str,
    code: Vec<Element>,
    constants: Vec<Value>,
    lines: Vec<(usize, usize)>,
}

impl fmt::Display for Chunk<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "==={}===\n", &self.name)?;
        let mut pc: usize = 0;
        loop {
            pc += self.display_instruction(pc, f)?;
            if pc >= self.code.len() {
                break;
            }
        }

        write!(f, "{:?}", self.lines);

        Ok(())
    }
}

impl Chunk<'_> {
    pub fn new<'a>(name: &'a str) -> Chunk {
        Chunk {
            name: name,
            code: Vec::new(),
            constants: Vec::new(),
            lines: Vec::new(),
        }
    }

    pub fn write(&mut self, element: Element, line: usize) {
        self.code.push(element);
        self.annotate_line(line)
    }

    pub fn write_opcode(&mut self, opcode: OpCode, line: usize) {
        self.code.push(Element::OpCode(opcode));
        self.annotate_line(line);
    }

    pub fn write_constant(&mut self, constant: u8, line: usize) {
        self.code.push(Element::Constant(constant));
        self.annotate_line(line);
    }

    pub fn write_constant_long(&mut self, constant: usize, line: usize) {
        self.code.push(Element::Constant((constant >> 16 & 0xFF) as u8));
        self.annotate_line(line);
        self.code.push(Element::Constant((constant >> 8 & 0xFF) as u8));
        self.annotate_line(line);
        self.code.push(Element::Constant((constant >> 0 & 0xFF) as u8));
        self.annotate_line(line);
    }

    fn annotate_line(&mut self, line: usize) {
        if self.lines.len() == 0 {
            self.lines.push((line, 1));
        }
        else {
            let l = self.lines.len() - 1;
            let r = self.lines[l];
            if r.0 == line {
                self.lines[l] = (r.0, r.1 + 1);
            }
            else {
                self.lines.push((line, 1));
            }
        }

        println!("{:?}", self.lines);
    }

    fn get_line(&self, index: usize) -> &usize {
        let mut acc = 0;
        for (line, count) in &self.lines {
            acc += count;
            if index < acc {
                return line;
            }
        }

        return &0;
    }

    pub fn add_constant(&mut self, value: Value) -> usize {
        self.constants.push(value);
        self.constants.len() - 1
    }

    pub fn get_opcode(&self, index: usize) -> &OpCode {
        match &self.code[index] {
            Element::OpCode(opcode) => opcode,
            _ => panic!(),
        }
    }

    pub fn get_constant(&self, index: usize) -> (usize, &Value) {
        match &self.code[index] {
            Element::Constant(i) => (*i as usize, &self.constants[*i as usize]),
            _ => panic!(),
        }
    }

    pub fn get_constant_long(&self, index: usize) -> Option<&Value> {
        let c1 = self.code[index].get_constant()?;
        let c2 = self.code[index + 1].get_constant()?;
        let c3 = self.code[index + 2].get_constant()?;
        let index: usize = ((c1 as usize) << 16) + ((c2 as usize) << 8) + c3 as usize;
        Some(&self.constants[index as usize])
    }

    pub fn display_instruction(
        &self,
        index: usize,
        f: &mut fmt::Formatter<'_>,
    ) -> Result<usize, fmt::Error> {

        write!(f, "{:0>4} ", index)?;
        if index > 0 && self.get_line(index) == self.get_line(index - 1) {
            write!(
                f,
                "{}| ",
                std::iter::repeat(" ")
                    .take(self.get_line(index).to_string().chars().count())
                    .collect::<String>()
            )?;
        } else {
            write!(f, "{} ", self.get_line(index))?;
        }

        let opcode = self.get_opcode(index);
        let (r, i) = match opcode {
            OpCode::OpReturn => (write!(f, "{:?}\n", opcode), 1),
            OpCode::OpConstant => {
                let (n, c) = self.get_constant(index + 1);
                (write!(f, "{:?} {}:'{}'\n", opcode, n, c), 2)
            }
            OpCode::OpConstantLong => {
                let value = self.get_constant_long(index + 1).ok_or(fmt::Error)?;
                (write!(f, "{:?} '{}'\n", opcode, value), 4)
            }
        };

        match r {
            Ok(_) => Ok(i),
            Err(err) => Err(err),
        }
    }
}
