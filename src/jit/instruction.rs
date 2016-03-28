use std::collections::HashSet;

pub struct InstructionCollector {
    instructions: Vec<Instruction>,
    unused: HashSet<u8>,
}

pub struct XmmRegister(u8);

pub enum Instruction {
    Movs {
        source: XmmRegister,
        dest: XmmRegister,
    },
    Adds {
        source: XmmRegister,
        dest: XmmRegister,
    },
    Subs {
        source: XmmRegister,
        dest: XmmRegister,
    },
    Muls {
        source: XmmRegister,
        dest: XmmRegister,
    },
    Divs {
        source: XmmRegister,
        dest: XmmRegister,
    },
    Maxs {
        source: XmmRegister,
        dest: XmmRegister,
    },
    Sqrts {
        source: XmmRegister,
        dest: XmmRegister,
    },
    Ucomis {
        source: XmmRegister,
        dest: XmmRegister,
    }
}


impl InstructionCollector {
    pub fn new() -> InstructionCollector {
        let mut hs = HashSet::new();
        for i in 2 .. 16 {
            hs.insert(i);
        }

        InstructionCollector {
            instructions: vec![],
            unused: hs,
        }
    }

    pub fn get_register(&mut self) -> XmmRegister {

    }
}
