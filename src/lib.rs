/*  lib.rs: Top-level library

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
//! Rust bindings for libquantum
//!
//! These bindings are intended to enable access to a fast quantum simulator
//! for the Rust programming language. libquantum, being written in pure C
//! and widely available, is an excellent candidate to provide such access.
//!
//! This crate primarily offers the QuReg type, which is a safe wrapper around
//! libquantum's `quantum_reg` structure. These bindings are decidedly
//! low-level in that they have one dealing with bits and gates directly, as
//! opposed to using a higher-level interface. They are meant to provide the
//! simplest access to the libquantum library from Rust.
//!
//! Adding higher-level bindings should be delegated to a new crate in order
//! to keep this crate as lean as possible.
#![crate_type = "lib"]

mod quantum_sys;
mod qureg;

pub use qureg::QuReg;
