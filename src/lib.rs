mod quantum_sys;

pub struct QuReg {
    reg: quantum_sys::quantum_reg,
}

impl QuReg {
    pub fn new(width: usize, init: u64) -> QuReg {
        QuReg {
            reg: unsafe { quantum_sys::quantum_new_qureg(init, width as i32) }
        }
    }

    pub fn len(&self) -> usize {
        self.reg.width as usize
    }

    pub fn add_scratch(&mut self, bits: usize) {
        unsafe { quantum_sys::quantum_addscratch(bits as i32, &mut self.reg as *mut quantum_sys::quantum_reg) }
    }
}

impl Drop for QuReg {
    fn drop(&mut self) {
        unsafe { quantum_sys::quantum_delete_qureg(&mut self.reg as *mut quantum_sys::quantum_reg); }
    }
}

#[test]
fn test() {
    let x = QuReg::new(2, 0);
    println!("{}", x.len());
}
