/*  qureg.rs: Quantum register

    Copyright (C) 2017 Michael Anthony Knyszek

    This file is part of rust-libquantum

    rust-libquantum is free software: you can redistribute it and/or modify
    it under the terms of the GNU General Public License as published by
    the Free Software Foundation, either version 3 of the License, or
    (at your option) any later version.

    rust-libquantum is distributed in the hope that it will be useful,
    but WITHOUT ANY WARRANTY; without even the implied warranty of
    MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
    GNU General Public License for more details.

    You should have received a copy of the GNU General Public License
    along with this program.  If not, see <http://www.gnu.org/licenses/>.
*/

use quantum_sys::{self, quantum_reg};

/// A quantum register.
///
/// This structure is a wrapper around `quantum_reg` from the libquantum
/// library. Represents the most basic quantum data structure for which
/// elementary gate operations are implemented as methods.
pub struct QuReg {
    reg: quantum_reg,
}

impl QuReg {
    /// Allocates a new `QuReg`.
    ///
    /// The quantum register is allocated by libquantum given a specified
    /// width (number of qubits) and an initialization value. Only the first
    /// `width` bits of the `init` value will be used in initialization.
    pub fn new(width: usize, init: u64) -> QuReg {
        QuReg {
            reg: unsafe { quantum_sys::quantum_new_qureg(init, width as i32) }
        }
    }

    /// Returns the current width of the quantum register.
    pub fn width(&self) -> usize {
        self.reg.width as usize
    }

    /// Adds a `bits` scratch qubits to the quantum register.
    ///
    /// Scratch qubits are added to the least-significant bit of the register
    /// and initialized to zero.
    ///
    /// Scratch qubits should not be preallocated to avoid registering them in
    /// the underlying hashtable, however creating new basis states in the
    /// scratch space could overfill the underlying data structure.
    pub fn add_scratch(&mut self, bits: usize) {
        unsafe { quantum_sys::quantum_addscratch(bits as i32, self.reg_ptr()) }
    }

    /// Compute the Kronecker (tensor) product of two registers.
    ///
    /// Consumes the two registers to produce a new register which will contain
    /// the tensor product of the two (loosely maps to concatenation).
    pub fn tensor(mut self, mut other: QuReg) -> QuReg {
        QuReg {
            reg: unsafe { quantum_sys::quantum_kronecker(self.reg_ptr(), other.reg_ptr()) } 
        }
    }
    
    /// Applies a controlled-NOT gate between two qubits in the quantum register.
    pub fn cnot(&mut self, control: usize, target: usize) {
        unsafe { quantum_sys::quantum_cnot(control as i32, target as i32, self.reg_ptr()) }
    }

    /// Applies a Toffoli gate between three qubits in the quantum register.
    pub fn toffoli(&mut self, control1: usize, control2: usize, target: usize) {
        unsafe { quantum_sys::quantum_toffoli(control1 as i32, control2 as i32, target as i32, self.reg_ptr()) }
    }

    /// Applies a Pauli X (NOT) gate to a qubit in the quantum register.
    pub fn sigma_x(&mut self, target: usize) {
        unsafe { quantum_sys::quantum_sigma_x(target as i32, self.reg_ptr()) }
    }

    /// Applies a Pauli Y (phase flip) gate to a qubit in the quantum register.
    pub fn sigma_y(&mut self, target: usize) {
        unsafe { quantum_sys::quantum_sigma_y(target as i32, self.reg_ptr()) }
    }

    /// Applies a Pauli Z gate to a qubit in the quantum register.
    pub fn sigma_z(&mut self, target: usize) {
        unsafe { quantum_sys::quantum_sigma_z(target as i32, self.reg_ptr()) }
    }

    /// Rotates a qubit around the x-axis in the Bloch sphere in the quantum register.
    pub fn rotate_x(&mut self, target: usize, gamma: f32) {
        unsafe { quantum_sys::quantum_r_x(target as i32, gamma, self.reg_ptr()) }
    }

    /// Rotates a qubit around the y-axis in the Bloch sphere in the quantum register.
    pub fn rotate_y(&mut self, target: usize, gamma: f32) {
        unsafe { quantum_sys::quantum_r_y(target as i32, gamma, self.reg_ptr()) }
    }

    /// Rotates a qubit around the z-axis in the Bloch sphere in the quantum register.
    pub fn rotate_z(&mut self, target: usize, gamma: f32) {
        unsafe { quantum_sys::quantum_r_z(target as i32, gamma, self.reg_ptr()) }
    }

    /// Applies a global phase to a qubit in the quantum register.
    pub fn phase(&mut self, target: usize, gamma: f32) {
        unsafe { quantum_sys::quantum_phase_scale(target as i32, gamma, self.reg_ptr()) }
    }

    /// Applies a phase shift to a qubit in the quantum register.
    pub fn phaseby(&mut self, target: usize, gamma: f32) {
        unsafe { quantum_sys::quantum_phase_kick(target as i32, gamma, self.reg_ptr()) }
    }

    /// Applies the Hadamard gate to a qubit in the quantum register.
    pub fn hadamard(&mut self, target: usize) {
        unsafe { quantum_sys::quantum_hadamard(target as i32, self.reg_ptr()) }
    }

    /// Applies the Hadamard transform to qubits in the quantum register.
    ///
    /// More specifically, this method applies a Hadamard gate to the first
    /// `width` qubits in the quantum register.
    pub fn walsh(&mut self, width: usize) {
        unsafe { quantum_sys::quantum_walsh(width as i32, self.reg_ptr()) }
    }

    /// Applies a controlled phase shift to a qubit in the quantum register.
    ///
    /// The applied phase shift is by `pi/2**k` where `k = control - target`
    pub fn cond_phase(&mut self, control: usize, target: usize) {
        unsafe { quantum_sys::quantum_cond_phase(control as i32, target as i32, self.reg_ptr()) }
    }

    /// Applies a controlled arbitrary phase shift to a qubit in the quantum register.
    ///
    /// The applied phase shift is by gamma.
    pub fn cond_phaseby(&mut self, control: usize, target: usize, gamma: f32) {
        unsafe { quantum_sys::quantum_cond_phase_kick(control as i32, target as i32, gamma, self.reg_ptr()) }
    }

    /// Applies the quantum Fourier transform to the quantum register.
    ///
    /// More specifically, this method applies a QFT to the first
    /// `width` qubits in the quantum register.
    pub fn qft(&mut self, width: usize) {
        unsafe { quantum_sys::quantum_qft(width as i32, self.reg_ptr()) }
    }

    /// Applies the inverse of the quantum Fourier transform to the quantum 
    /// register.
    ///
    /// More specifically, this method applies an inverse QFT to the first
    /// `width` qubits in the quantum register.
    pub fn qft_inv(&mut self, width: usize) {
        unsafe { quantum_sys::quantum_qft_inv(width as i32, self.reg_ptr()) }
    }

    /// Measures the entire quantum register and discards it.
    ///
    /// Returns the result as the first `width` bits in an unsigned integer.
    pub fn measure(self) -> usize {
        unsafe { quantum_sys::quantum_measure(self.reg) as usize }
    }

    /// Measures a qubit in the quantum register and discards it.
    ///
    /// Returns the result as a Boolean value.
    pub fn measure_bit(&mut self, pos: usize) -> bool {
        unsafe { quantum_sys::quantum_bmeasure(pos as i32, self.reg_ptr()) != 0 }
    }

    /// Measures a qubit in the quantum register without discarding it.
    pub fn measure_bit_preserve(&mut self, pos: usize) -> usize {
        unsafe { quantum_sys::quantum_bmeasure_bitpreserve(pos as i32, self.reg_ptr()) as usize }
    }

    #[inline]
    unsafe fn reg_ptr(&mut self) -> *mut quantum_reg {
        &mut self.reg as *mut quantum_reg
    }
}

impl Drop for QuReg {
    /// Reclaims memory from the quantum_reg when a QuReg value is dropped.
    fn drop(&mut self) {
        unsafe { quantum_sys::quantum_delete_qureg(self.reg_ptr()); }
    }
}

#[cfg(test)]
mod tests {
    use qureg::QuReg;

    #[test]
    fn width() {
        assert_eq!(QuReg::new(2, 0).width(), 2);
    }
}
