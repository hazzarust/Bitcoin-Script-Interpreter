use crate::opcode_enum::Function;
use crate::opcode_enum::*;
use sha1::Sha1;
use sha2::{Sha256, Digest};
use ripemd::{Ripemd160};

#[derive(Debug, Clone, PartialEq)]
pub struct TBytes{
    pub data: Vec<u8>,
    pub parts: Vec<TBytes>,
    pub function: Function,
    pub name: Option<String>,
}


impl TBytes{
    pub fn new(data: Vec<u8>, name: Option<String>) -> Self{
        Self{
            data: data,
            parts: vec![],
            function: Function::Plain,
            name: name,
        }
    }

    pub fn concat(self, other: TBytes, name: Option<String>) -> Self{
        Self{
            data: [self.data.as_slice(), other.data.as_slice()].concat(),
            parts: vec![self, other],
            function: Function::Opcode(Opcode::OpCat),
            name: name,
        }
    }

    pub fn sha256(self, name: Option<String>) -> Self{
        let mut hasher = Sha256::new();
        hasher.update(self.data);
        let y = hasher.finalize();
    

        Self{
            data: y.to_vec(),
            parts: self.parts,
            function: Function::Opcode(Opcode::OpSha256),
            name: name,
        }
    }

    pub fn ripemd160(self, name: Option<String>) -> Self{
        let mut hasher = Ripemd160::new();
        hasher.update(self.data);
        let y = hasher.finalize();

        Self{
            data: y.to_vec(),
            parts: self.parts,
            function: Function::Opcode(Opcode::OpRipeMD160),
            name: name,
        }
    }

    pub fn sha1(self, name: Option<String>) -> Self{
        let mut hasher = Sha1::new();
        hasher.update(self.data);
        let y = hasher.finalize();
            
    
        Self{
            data: y.to_vec(),
            parts: self.parts,
            function: Function::Opcode(Opcode::OpSha1),
            name: name,
        }
    }

    pub fn op_hash160(self, name: Option<String>) -> Self{
        let mut hasher = Sha256::new();
        hasher.update(&self.data);
        let y = hasher.finalize();
        let mut hasher2 = Ripemd160::new();
        hasher2.update(y);
        let z = hasher2.finalize();

        Self{
            data: z.to_vec(),
            parts:vec![self],
            function: Function::Opcode(Opcode::OpHash160),
            name: name,
        }
        
    }

    pub fn op_hash256(self, name: Option<String>) -> Self{
        let mut hasher = Sha256::new();
        hasher.update(&self.data);
        let y = hasher.finalize();
        let mut hasher2 = Sha256::new();
        hasher2.update(y);
        let z = hasher2.finalize();

        Self{
            data: z.to_vec(),
            parts: vec![self],
            function: Function::Opcode(Opcode::OpHash256),
            name: name,
        }
    }

    pub fn op_equal_true(self, ybytes: TBytes, name: Option<String>) -> Self{
        Self{
            data: vec![1],
            parts: vec![self, ybytes],
            function: Function::Opcode(Opcode::OpEqual),
            name: name, 
        }
    }

    pub fn op_equal_false(self, ybytes: TBytes, name: Option<String>) -> Self{
        Self{
            data: vec![],
            parts: vec![self, ybytes],
            function: Function::Opcode(Opcode::OpEqual),
            name: name,
        }
    }

    pub fn op_not_false(self, name: Option<String>) -> Self{
        Self{
            data: vec![1],
            parts: vec![self],
            function: Function::Opcode(Opcode::OpNOT),
            name: name,
        }
    }

    pub fn op_not_true(self, name: Option<String>) -> Self{
        Self{
            data: vec![0],
            parts: vec![self],
            function: Function::Opcode(Opcode::OpNOT),
            name: name,
        }
    }

    pub fn op_or(self, x: Vec<u8>, y: TBytes, name:Option<String>) -> Self{
        Self{
            data: x,
            parts: vec![self, y],
            function: Function::Opcode(Opcode::OpOr),
            name: name,

        }
    }

    pub fn op_add(self, x: Vec<u8>, y: TBytes, name: Option<String>) -> Self{
        Self{
            data: x,
            parts: vec![self, y],
            function: Function::Opcode(Opcode::OpAdd),
            name: name,
        }
    }
    
    pub fn op_bool_and_true(self, y: TBytes, name: Option<String>) -> Self{
        Self{
            data: vec![1],
            parts: vec![self, y],
            function: Function::Opcode(Opcode::OpBoolAnd),
            name: name,
        }
    }
    pub fn op_bool_and_false(self, y: TBytes, name: Option<String>) -> Self{
        Self{
            data: vec![0],
            parts: vec![self, y],
            function: Function::Opcode(Opcode::OpBoolAnd),
            name: name,
        }
    }
    pub fn op_bool_or_true(self, y: TBytes, name: Option<String>) -> Self{
        Self{
            data: vec![1],
            parts: vec![self, y],
            function: Function::Opcode(Opcode::OpBoolOr),
            name: name,
        }
    }  
    pub fn op_bool_or_false(self, y: TBytes, name: Option<String>) -> Self{
        Self{
            data: vec![0],
            parts: vec![self, y],
            function: Function::Opcode(Opcode::OpBoolOr),
            name: name,
        }
    }  

    pub fn op_add_one(self, result: Vec<u8>, name: Option<String>) -> Self{
        Self{
            data: result,
            parts: vec![self],
            function: Function::Opcode(Opcode::OpAddOne),
            name: name,
        }
    }
    pub fn op_sub_one(self, result: Vec<u8>, name: Option<String>) -> Self{
        Self{
            data: result,
            parts: vec![self],
            function: Function::Opcode(Opcode::OpSubOne),
            name: name,
        }
    }

    pub fn op_negate(self, result: Vec<u8>, name: Option<String>) -> Self{
        Self{
            data: result,
            parts: vec![self],
            function: Function::Opcode(Opcode::OpNegate),
            name: name,
        }
    }
    pub fn op_abs_impl(self, result: Vec<u8>, name: Option<String>) -> Self{
        Self{
            data: result,
            parts: vec![self],
            function: Function::Opcode(Opcode::OpAbs),
            name: name,
        }
    }

    pub fn op_div_impl(self, result: Vec<u8>, ybytes: TBytes, name: Option<String>) -> Self{
        Self{
            data: result,
            parts: vec![self, ybytes],
            function: Function::Opcode(Opcode::OpDiv),
            name: name,
        }
    }

    pub fn op_div_two_impl(self, result: Vec<u8>, name: Option<String>) -> Self{
        Self{
            data: result,
            parts: vec![self],
            function: Function::Opcode(Opcode::OpDivTwo),
            name: name,
        }
    }

    pub fn op_mul_two_impl(self, result: Vec<u8>, name: Option<String>) -> Self{
        Self{
            data: result,
            parts: vec![self],
            function: Function::Opcode(Opcode::OpMulTwo),
            name: name,
        }
    }

    pub fn op_num_not_equal_true(self, y: TBytes, name: Option<String>) -> Self{
        Self{
            data: vec![1],
            parts: vec![self, y],
            function: Function::Opcode(Opcode::OpNumNotEqual),
            name: name,
        }
    }
    pub fn op_num_not_equal_false(self, y: TBytes, name: Option<String>) -> Self{
        Self{
            data: vec![],
            parts: vec![self, y],
            function: Function::Opcode(Opcode::OpNumNotEqual),
            name: name,
        }
    }

    pub fn op_within_impl(self, y: TBytes, z: TBytes, name: Option<String>) -> Self{
        Self{
            data: vec![1],
            parts: vec![self, y, z],
            function: Function::Opcode(Opcode::OpWithin),
            name: name,
        }
    }
    pub fn op_within_impl_false(self, y: TBytes, z: TBytes, name: Option<String>) -> Self{
        Self{
            data: vec![0],
            parts: vec![self, y, z],
            function: Function::Opcode(Opcode::OpWithin),
            name: name,
        }
    }

    pub fn op_less_than_true(self, y: TBytes, name: Option<String>) -> Self{
        Self{
            data: vec![1],
            parts: vec![self, y],
            function: Function::Opcode(Opcode::OpLessThan),
            name: name,
        }
    }
    pub fn op_less_than_false(self, y: TBytes, name: Option<String>) -> Self{
        Self{
            data: vec![],
            parts: vec![self, y],
            function: Function::Opcode(Opcode::OpLessThan),
            name: name,
        }
    }
    pub fn op_greater_than_true(self, y: TBytes, name: Option<String>) -> Self{
        Self{
            data: vec![1],
            parts: vec![self, y],
            function: Function::Opcode(Opcode::OpLessThan),
            name: name,
        }
    }
    pub fn op_greater_than_false(self, y: TBytes, name: Option<String>) -> Self{
        Self{
            data: vec![],
            parts: vec![self, y],
            function: Function::Opcode(Opcode::OpGreaterThan),
            name: name,
        }
    }
    pub fn op_less_than_or_equal_true(self, y: TBytes, name: Option<String>) -> Self{
        Self{
            data: vec![1],
            parts: vec![self, y],
            function: Function::Opcode(Opcode::OpLessThanOrEqual),
            name: name,
        }
    }
    pub fn op_less_than_or_equal_false(self, y: TBytes, name: Option<String>) -> Self{
        Self{
            data: vec![],
            parts: vec![self, y],
            function: Function::Opcode(Opcode::OpLessThanOrEqual),
            name: name,
        }
    }
    pub fn op_greater_than_or_equal_true(self, y: TBytes, name: Option<String>) -> Self{
        Self{
            data: vec![1],
            parts: vec![self, y],
            function: Function::Opcode(Opcode::OpGreaterThan),
            name: name,
        }
    }
    pub fn op_greater_than_or_equal_false(self, y: TBytes, name: Option<String>) -> Self{
        Self{
            data: vec![],
            parts: vec![self, y],
            function: Function::Opcode(Opcode::OpGreaterThan),
            name: name,
        }
    }

    pub fn op_min_true(self, y: TBytes, result: Vec<u8>, name: Option<String>) -> Self{
        Self{
            data: result,
            parts: vec![self, y],
            function: Function::Opcode(Opcode::OpMin),
            name: name,
        }
    }
    pub fn op_min_false(self, y: TBytes, result: Vec<u8>, name: Option<String>) -> Self{
        Self{
            data: result,
            parts: vec![self, y],
            function: Function::Opcode(Opcode::OpMin),
            name: name,
        }
    }

    pub fn op_invert_impl(self, result: Vec<u8>, name: Option<String>) -> Self{
        Self{
            data: result,
            parts: vec![self],
            function: Function::Opcode(Opcode::OpInvert),
            name: name, 
        }
    }
    
    pub fn op_and_impl(self, y: TBytes, result: Vec<u8>, name: Option<String>) -> Self{
        Self{
            data: result,
            parts: vec![self, y],
            function: Function::Opcode(Opcode::OpAnd),
            name: name, 
        }
    }

    pub fn op_0_not_equal_false(self, name:Option<String>) -> Self{
        Self{
            data: vec![],
            parts: vec![self],
            function: Function::Opcode(Opcode::Op0NotEqual),
            name: name,
        }
    }
    
    pub fn op_0_not_equal_true(self, name:Option<String>) -> Self{
        Self{
            data: vec![1],
            parts: vec![self],
            function: Function::Opcode(Opcode::Op0NotEqual),
            name: name,
        }
    }
    pub fn op_sub_impl(self, result: Vec<u8>, ybytes: TBytes, name: Option<String>) -> Self{
        Self{
            data: result,
            parts: vec![self, ybytes],
            function: Function::Opcode(Opcode::OpSub),
            name: name,
        }

    }

}

impl TBytes{
    pub fn parts(&self) -> &[TBytes]{
        &self.parts
    }

    pub fn function(&self) -> Function{
        self.function.clone()
    }

    pub fn name(&self) -> Option<&str>{
        self.name.as_deref()
    }
}