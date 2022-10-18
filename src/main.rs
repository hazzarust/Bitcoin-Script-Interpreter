
mod tbytes_stuff;
use tbytes_stuff::TBytes;

mod opcode_enum;
use opcode_enum::*;

mod opcode_functions;
use opcode_functions::*;


fn main(){
    let the_stack = &mut State { 
        main_stack: vec![],
        alt_stack: vec![],
    };
    let x = String::from("function");
    let new_tbytes = TBytes::new(vec![1,2,3], Some(x.clone()));
    let second_tbytes = TBytes::new(vec![4,5,6], None);
    run_script(the_stack, vec![Op::Push(new_tbytes), Op::Push(second_tbytes), Op::Opcode(Opcode::OpCat)]).unwrap();
    println!("{:#?}", the_stack.main_stack);
}



pub fn run_script(stack: &mut State, script: Vec<Op>) -> Result<(), StackError> {
    for y in script{
        match y{
            Op::Push(varname) => op_push(stack, varname)?,
            Op::Opcode(opcode) => run_opcode(stack, opcode)?,
        };
    }
    Ok(())
}


pub fn run_opcode(stack: &mut State, opcode: Opcode) -> Result<(), StackError>{
    match opcode{
        Opcode::OpCat => return op_concat(stack),
        Opcode::OpDup => return op_dup(stack),
        Opcode::OpToAltStack => return op_to_alt_stack(stack),
        Opcode::OpFromAltStack => return op_from_alt_stack(stack),
        Opcode::OpDrop => return op_drop(stack),
        Opcode::OpNip => return op_nip(stack),
        Opcode::OpOver => return op_over(stack),
        Opcode::OpRot => return op_rot(stack),
        Opcode::OpTuck => return op_tuck(stack),
        Opcode::OpSwap => return op_swap(stack),
        Opcode::OpTwoDrop => return op_two_drop(stack),
        Opcode::OpTwoDup => return op_two_dup(stack),
        Opcode::OpThreeDup => return op_three_dup(stack),
        Opcode::OpTwoRot => return op_two_rot(stack),
        Opcode::OpTwoSwap => return op_two_swap(stack),
        Opcode::OpSha256 => return op_sha256(stack),
        Opcode::OpRipeMD160 => return op_ripemd160(stack),
        Opcode::OpSha1 => return op_sha1(stack),
        Opcode::OpHash160 => return op_hash160(stack),
        Opcode::OpHash256 => return op_hash256(stack),
        Opcode::OpEqual => return op_equal(stack),
        Opcode::OpInvert => return op_invert(stack),
        Opcode::OpAnd => return op_and(stack),
        Opcode::OpOr => return op_or(stack),
        Opcode::OpEqualVerify => return op_equal_verify(stack),
        Opcode::Op1add => return op_add_1(stack),
        Opcode::Op1sub => return op_sub_1(stack),
        Opcode::OpMulTwo => return op_mul_two(stack),
        Opcode::OpDivTwo => return op_div_two(stack),
        Opcode::OpNegate => return op_negate(stack),
        Opcode::OpAbs => return op_abs(stack),
        Opcode::OpNOT => return op_not(stack),
        Opcode::OpAdd => return op_add(stack),
        Opcode::OpDiv => return op_div(stack),
        Opcode::OpBoolAnd => return op_bool_and(stack),
        Opcode::OpBoolOr => return op_bool_or(stack),
        Opcode::OpNumNotEqual => return op_num_not_equal(stack),
        Opcode::OpLessThan => return op_less_than(stack),
        Opcode::OpGreaterThan => return op_greater_than(stack),
        Opcode::OpLessThanOrEqual => return op_less_than_or_equal(stack),
        Opcode::OpGreaterThanOrEqual => return op_greater_than_or_equal(stack),
        Opcode::OpMin => return op_min(stack),
        Opcode::OpMax => return op_max(stack),
        Opcode::OpWithin => return op_within(stack),
        Opcode::Op0NotEqual => return op_0_not_equal(stack),
        Opcode::OpSub => return op_sub(stack),
        Opcode::OpPick => return op_pick(stack),
        Opcode::OpRoll => return op_roll(stack),

        _=> todo!(),
    };
    
}



#[derive(Debug, Clone, PartialEq)]
pub struct Hashes{
    tbytes2sha256: Vec<u8>,
    tbytes2hash160: Vec<u8>,
    tbytes2ripemd: Vec<u8>,
    tbytes2hash256: Vec<u8>,
    tbytes2sha1: Vec<u8>,
}

impl Hashes{
    pub fn new_hash() -> Self{
        Self{
            tbytes2sha256: vec![120, 124, 121, 142, 57, 165, 188, 25, 16, 53, 91, 174, 109, 12, 216, 122, 54, 178, 225, 15, 208, 32, 42, 131, 227, 187, 107, 0, 93, 168, 52, 114],
            tbytes2hash160: vec![97, 27, 29, 204, 51, 5, 239, 104, 230, 117, 137, 124, 87, 155, 79, 240, 14, 193, 216, 33],
            tbytes2ripemd: vec![98, 30, 194, 224, 21, 251, 217, 190, 174, 121, 99, 159, 167, 215, 192, 121, 5, 83, 69, 180],
            tbytes2hash256: vec![101, 91, 137, 158, 145, 191, 123, 217, 254, 184, 87, 157, 251, 229, 232, 90, 203, 0, 158, 244, 140, 81, 113, 98, 90, 251, 109, 19, 242, 130, 78, 37],
            tbytes2sha1: vec![232, 9, 197, 209, 206, 164, 123, 69, 227, 71, 1, 210, 63, 96, 138, 154, 88, 3, 77, 201],
        }
    }
}
mod tests{
    #[cfg(test)]
    use super::*;
    
    #[test]
    fn test_sha1(){
        let hash = Hashes::new_hash();
        let the_stack = &mut State { 
            main_stack: vec![],
            alt_stack: vec![],
        };
        let x = String::from("function");
        let new_tbytes = TBytes::new(vec![1,2,3], Some(x.clone()));
        let second_tbytes = TBytes::new(vec![4,5,6], None);
        run_script(the_stack, vec![Op::Push(new_tbytes), Op::Push(second_tbytes), Op::Opcode(Opcode::OpSha1)]).unwrap();
        assert_eq!(the_stack.main_stack[1].data, hash.tbytes2sha1);
    }
    
    #[test]
    fn test_hash256(){
        let hash = Hashes::new_hash();
        let the_stack = &mut State { 
            main_stack: vec![],
            alt_stack: vec![],
        };
        let x = String::from("function");
        let new_tbytes = TBytes::new(vec![1,2,3], Some(x.clone()));
        let second_tbytes = TBytes::new(vec![4,5,6], None);
        run_script(the_stack, vec![Op::Push(new_tbytes), Op::Push(second_tbytes), Op::Opcode(Opcode::OpHash256)]).unwrap();
        assert_eq!(the_stack.main_stack[1].data, hash.tbytes2hash256);
    }
   
    #[test]
    fn test_ripemd160(){
        let hash = Hashes::new_hash();
        let the_stack = &mut State { 
            main_stack: vec![],
            alt_stack: vec![],
        };
        let x = String::from("function");
        let new_tbytes = TBytes::new(vec![1,2,3], Some(x.clone()));
        let second_tbytes = TBytes::new(vec![4,5,6], None);
        run_script(the_stack, vec![Op::Push(new_tbytes), Op::Push(second_tbytes), Op::Opcode(Opcode::OpRipeMD160)]).unwrap();
        assert_eq!(the_stack.main_stack[1].data, hash.tbytes2ripemd);
    }
    #[test]
    fn test_hash160(){
        let hash = Hashes::new_hash();
        let the_stack = &mut State { 
            main_stack: vec![],
            alt_stack: vec![],
        };
        let x = String::from("function");
        let new_tbytes = TBytes::new(vec![1,2,3], Some(x.clone()));
        let second_tbytes = TBytes::new(vec![4,5,6], None);
        run_script(the_stack, vec![Op::Push(new_tbytes), Op::Push(second_tbytes), Op::Opcode(Opcode::OpHash160)]).unwrap();
        assert_eq!(the_stack.main_stack[1].data, hash.tbytes2hash160);
    }

    #[test]
    fn test_sha256(){
        let hash = Hashes::new_hash();
        let the_stack = &mut State { 
            main_stack: vec![],
            alt_stack: vec![],
        };
        let x = String::from("function");
        let new_tbytes = TBytes::new(vec![1,2,3], Some(x.clone()));
        let second_tbytes = TBytes::new(vec![4,5,6], None);
        run_script(the_stack, vec![Op::Push(new_tbytes), Op::Push(second_tbytes), Op::Opcode(Opcode::OpSha256)]).unwrap();
        assert_eq!(the_stack.main_stack[1].data, hash.tbytes2sha256);
    }
    #[test]
    fn test_toaltstack(){
        let the_stack = &mut State { 
            main_stack: vec![],
            alt_stack: vec![],
        };
        let x = String::from("function");
        let new_tbytes = TBytes::new(vec![1,2,3], Some(x.clone()));
        let second_tbytes = TBytes::new(vec![4,5,6], None);
        run_script(the_stack, vec![Op::Push(new_tbytes), Op::Push(second_tbytes), Op::Opcode(Opcode::OpToAltStack)]).unwrap();
        //assert_eq!(the_stack.main_stack[1].data, vec! 
    } 
    
    #[test]
    fn test_fromaltstack(){
        let the_stack = &mut State { 
            main_stack: vec![],
            alt_stack: vec![],
        };
        let x = String::from("function");
        let new_tbytes = TBytes::new(vec![1,2,3], Some(x.clone()));
        let second_tbytes = TBytes::new(vec![4,5,6], None);
        run_script(the_stack, vec![Op::Push(new_tbytes), Op::Push(second_tbytes), Op::Opcode(Opcode::OpToAltStack), Op::Opcode(Opcode::OpToAltStack), Op::Opcode(Opcode::OpFromAltStack)]).unwrap();
        assert_eq!(the_stack.main_stack[0].data, vec![1,2,3]);
        assert_eq!(the_stack.alt_stack[0].data, vec![4,5,6]);

    }

    #[test]
    fn test_concat(){
        let the_stack = &mut State { 
            main_stack: vec![],
            alt_stack: vec![],
        };
        let x = String::from("function");
        let new_tbytes = TBytes::new(vec![1,2,3], Some(x.clone()));
        let second_tbytes = TBytes::new(vec![4,5,6], None);
        run_script(the_stack, vec![Op::Push(new_tbytes), Op::Push(second_tbytes), Op::Opcode(Opcode::OpCat), Op::Opcode(Opcode::OpDup)]).unwrap();

    }
    
    #[test]
    fn test_dup(){
        let the_stack = &mut State { 
            main_stack: vec![],
            alt_stack: vec![],
        };
        let new_tbytes = TBytes::new(vec![1,2,3], None);
        let second_tbytes = TBytes::new(vec![4,5,6], None);
        run_script(the_stack, vec![Op::Push(new_tbytes), Op::Push(second_tbytes), Op::Opcode(Opcode::OpDup)]).unwrap();
        assert_eq!(the_stack.main_stack[2].data, vec![4,5,6]);
        assert_eq!(the_stack.main_stack[1].data, vec![4,5,6]);
    }

     #[test]
    fn test_over(){
        let the_stack = &mut State { 
            main_stack: vec![],
            alt_stack: vec![],
        };
        let new_tbytes = TBytes::new(vec![1,2,3], None);
        let second_tbytes = TBytes::new(vec![4,5,6], None);
        let third_tbytes = TBytes::new(vec![7,8,9], None);
        run_script(the_stack, vec![Op::Push(new_tbytes), Op::Push(second_tbytes), Op::Push(third_tbytes), Op::Opcode(Opcode::OpOver)]).unwrap();
        assert_eq!(the_stack.main_stack[3].data, vec![4,5,6]);
    }

    #[test]
    fn test_rot(){
        let the_stack = &mut State { 
            main_stack: vec![],
            alt_stack: vec![],
        };
        let new_tbytes = TBytes::new(vec![1,2,3], None);
        let second_tbytes = TBytes::new(vec![4,5,6], None);
        let third_tbytes = TBytes::new(vec![7,8,9], None);
        run_script(the_stack, vec![Op::Push(new_tbytes),Op::Push(second_tbytes), Op::Push(third_tbytes), Op::Opcode(Opcode::OpRot)]).unwrap();
        assert_eq!(the_stack.main_stack[2].data, vec![1,2,3]);
        assert_eq!(the_stack.main_stack[1].data, vec![7,8,9]);
        assert_eq!(the_stack.main_stack[0].data, vec![4,5,6]);
    }

    #[test]
    fn test_swap(){
        let the_stack = &mut State { 
            main_stack: vec![],
            alt_stack: vec![],
        };
        let new_tbytes = TBytes::new(vec![1,2,3], None);
        let second_tbytes = TBytes::new(vec![4,5,6], None);
        let third_tbytes = TBytes::new(vec![7,8,9], None);
        run_script(the_stack, vec![Op::Push(new_tbytes),Op::Push(second_tbytes), Op::Push(third_tbytes), Op::Opcode(Opcode::OpSwap)]).unwrap();
        assert_eq!(the_stack.main_stack[0].data, vec![1,2,3]);
        assert_eq!(the_stack.main_stack[1].data, vec![7,8,9]);
        assert_eq!(the_stack.main_stack[2].data, vec![4,5,6]);
    }

    #[test]
    fn test_tuck(){
        let the_stack = &mut State { 
            main_stack: vec![],
            alt_stack: vec![],
        };
        let new_tbytes = TBytes::new(vec![1,2,3], None);
        let second_tbytes = TBytes::new(vec![4,5,6], None);
        let third_tbytes = TBytes::new(vec![7,8,9], None);
        let fourth_tbytes = TBytes::new(vec![10,11,12], None);
        run_script(the_stack, vec![Op::Push(new_tbytes),Op::Push(second_tbytes), Op::Push(third_tbytes), Op::Push(fourth_tbytes), Op::Opcode(Opcode::OpTuck)]).unwrap();
        assert_eq!(the_stack.main_stack[0].data, vec![1,2,3]);
        assert_eq!(the_stack.main_stack[1].data, vec![10,11,12]);
        assert_eq!(the_stack.main_stack[2].data, vec![4,5,6]);
        assert_eq!(the_stack.main_stack[3].data, vec![7,8,9]);
        assert_eq!(the_stack.main_stack[4].data, vec![10,11,12]);
    }

    #[test]
    fn test_two_drop(){
        let the_stack = &mut State { 
            main_stack: vec![],
            alt_stack: vec![],
        };
        let new_tbytes = TBytes::new(vec![1,2,3], None);
        let second_tbytes = TBytes::new(vec![4,5,6], None);
        let third_tbytes = TBytes::new(vec![7,8,9], None);
        run_script(the_stack, vec![Op::Push(new_tbytes),Op::Push(second_tbytes), Op::Push(third_tbytes), Op::Opcode(Opcode::OpTwoDrop)]).unwrap();
        assert_eq!(the_stack.main_stack[0].data, vec![1,2,3]);

    }

    #[test]
    fn test_two_dup(){
        let the_stack = &mut State { 
            main_stack: vec![],
            alt_stack: vec![],
        };
        let new_tbytes = TBytes::new(vec![1,2,3], None);
        let second_tbytes = TBytes::new(vec![4,5,6], None);
        let third_tbytes = TBytes::new(vec![7,8,9], None);
        run_script(the_stack, vec![Op::Push(new_tbytes),Op::Push(second_tbytes), Op::Push(third_tbytes), Op::Opcode(Opcode::OpTwoDup)]).unwrap();
        assert_eq!(the_stack.main_stack[0].data, vec![1,2,3]);
        assert_eq!(the_stack.main_stack[1].data, vec![4,5,6]);
        assert_eq!(the_stack.main_stack[2].data, vec![7,8,9]);
        assert_eq!(the_stack.main_stack[3].data, vec![4,5,6]);
        assert_eq!(the_stack.main_stack[4].data, vec![7,8,9]);
    }

    #[test]
    fn test_three_dup(){
        let the_stack = &mut State { 
            main_stack: vec![],
            alt_stack: vec![],
        };
        let new_tbytes = TBytes::new(vec![1,2,3], None);
        let second_tbytes = TBytes::new(vec![4,5,6], None);
        let third_tbytes = TBytes::new(vec![7,8,9], None);
        run_script(the_stack, vec![Op::Push(new_tbytes),Op::Push(second_tbytes), Op::Push(third_tbytes), Op::Opcode(Opcode::OpThreeDup)]).unwrap();
        assert_eq!(the_stack.main_stack[0].data, vec![1,2,3]);
        assert_eq!(the_stack.main_stack[1].data, vec![4,5,6]);
        assert_eq!(the_stack.main_stack[2].data, vec![7,8,9]);
        assert_eq!(the_stack.main_stack[3].data, vec![1,2,3]);
        assert_eq!(the_stack.main_stack[4].data, vec![4,5,6]);
        assert_eq!(the_stack.main_stack[5].data, vec![7,8,9]);
    }

    #[test]
    fn test_two_rot(){
        let the_stack = &mut State { 
            main_stack: vec![],
            alt_stack: vec![],
        };
        let new_tbytes = TBytes::new(vec![1,2,3], None);
        let second_tbytes = TBytes::new(vec![4,5,6], None);
        let third_tbytes = TBytes::new(vec![7,8,9], None);
        let fourth_tbytes = TBytes::new(vec![10,11,12], None);
        let fifth_tbytes = TBytes::new(vec![13,14,15], None);
        let sixth_tbytes = TBytes::new(vec![16,17,18], None);
        run_script(the_stack, vec![Op::Push(new_tbytes),Op::Push(second_tbytes), Op::Push(third_tbytes), Op::Push(fourth_tbytes), Op::Push(fifth_tbytes), Op::Push(sixth_tbytes), Op::Opcode(Opcode::OpTwoRot)]).unwrap();
        assert_eq!(the_stack.main_stack[0].data, vec![7,8,9]);
        assert_eq!(the_stack.main_stack[1].data, vec![10,11,12]);
        assert_eq!(the_stack.main_stack[2].data, vec![13,14,15]);
        assert_eq!(the_stack.main_stack[3].data, vec![16,17,18]);
        assert_eq!(the_stack.main_stack[4].data, vec![1,2,3]);
        assert_eq!(the_stack.main_stack[5].data, vec![4,5,6]);
    }

    #[test]
    fn test_two_swap(){
        let the_stack = &mut State { 
            main_stack: vec![],
            alt_stack: vec![],
        };
        let new_tbytes = TBytes::new(vec![1,2,3], None);
        let second_tbytes = TBytes::new(vec![4,5,6], None);
        let third_tbytes = TBytes::new(vec![7,8,9], None);
        let fourth_tbytes = TBytes::new(vec![10,11,12], None);
        let fifth_tbytes = TBytes::new(vec![13,14,15], None);
        run_script(the_stack, vec![Op::Push(new_tbytes),Op::Push(second_tbytes), Op::Push(third_tbytes), Op::Push(fourth_tbytes), Op::Push(fifth_tbytes), Op::Opcode(Opcode::OpTwoSwap)]).unwrap();
        assert_eq!(the_stack.main_stack[0].data, vec![1,2,3]);
        assert_eq!(the_stack.main_stack[1].data, vec![10,11,12]);
        assert_eq!(the_stack.main_stack[2].data, vec![13,14,15]);
        assert_eq!(the_stack.main_stack[3].data, vec![4,5,6]);
        assert_eq!(the_stack.main_stack[4].data, vec![7,8,9]);
    }

}
