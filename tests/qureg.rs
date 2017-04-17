extern crate libquantum;

use libquantum::QuReg;

#[test]
fn width() {
    assert_eq!(QuReg::new(2, 0).width(), 2);
}

#[test]
fn tensor() {
    let q1 = QuReg::new(2, 0b01);
    let q2 = QuReg::new(2, 0b10);
    assert_eq!(q1.tensor(q2).measure(), 0b0110);
}

#[test]
fn cnot() {
    let mut q = QuReg::new(2, 0b11);
    q.cnot(0, 1);
    assert_eq!(q.measure(), 1);
}

#[test]
fn toffoli() {
    let mut q = QuReg::new(3, 0b111);
    q.toffoli(2, 1, 0);
    assert_eq!(q.measure(), 0b110);
}

#[test]
fn and_gate() {
    let mut q = QuReg::new(2, 0b11);
    q.add_scratch(1);
    q.toffoli(2, 1, 0);
    assert_eq!(q.measure_bit(0), true);
}

#[test]
fn sigma_x() {
    let mut q = QuReg::new(1, 0);
    q.sigma_x(0);
    assert_eq!(q.measure(), 1);
}

#[test]
fn sigma_y() {
    let mut q = QuReg::new(1, 1);
    q.hadamard(0);
    q.sigma_y(0);
    q.hadamard(0);
    assert_eq!(q.measure(), 0);
}

#[test]
fn sigma_z() {
    let mut q = QuReg::new(1, 1);
    q.hadamard(0);
    q.sigma_z(0);
    q.hadamard(0);
    assert_eq!(q.measure(), 0);
}

#[test]
fn hadamard() {
    let mut q = QuReg::new(1, 0);
    q.hadamard(0);
    q.hadamard(0);
    assert_eq!(q.measure(), 0);
}

#[test]
fn walsh() {
    let mut q = QuReg::new(3, 0b110);
    q.walsh(3);
    q.walsh(3);
    assert_eq!(q.measure(), 0b110);
}

#[test]
fn measure_bit() {
    let mut q = QuReg::new(3, 0b110);
    assert_eq!(q.measure_bit_preserve(1), true);
    assert_eq!(q.width(), 3);
    assert_eq!(q.measure_bit(0), false);
    assert_eq!(q.width(), 2);
}

#[test]
fn measure_partial() {
    let mut q1 = QuReg::new(3, 0b110);
    assert_eq!(q1.measure_width(2), 0b10);
    assert_eq!(q1.width(), 1);
    let q2 = QuReg::new(2, 0b10);
    let mut q3 = q1.tensor(q2);
    assert_eq!(q3.measure_partial(0..2), 0b10);
    assert_eq!(q3.width(), 3);
    assert_eq!(q3.measure_partial(..), 0b110);
}
