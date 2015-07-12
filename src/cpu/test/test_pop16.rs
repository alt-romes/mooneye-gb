use quickcheck::quickcheck;

use cpu::registers::{Flags, Reg16};
use cpu::test::run_test;

fn test_pop16(opcode: u8, x: u16, reg: Reg16) -> bool {
  let h = (x >> 8) as u8;
  let l = x as u8;
  let cpu = run_test(
    &[opcode, 0xed, l, h],
    |cpu| {
      cpu.regs.sp = 0x0002;
    }
  );
  cpu.clock_cycles() == 12 &&
    cpu.regs.sp == 0x0004 &&
    cpu.regs.read16(reg) == x
}

#[test]
fn test_c1() {
  fn prop(x: u16) -> bool { test_pop16(0xc1, x, Reg16::BC) }
  quickcheck(prop as fn(u16) -> bool);
}

#[test]
fn test_d1() {
  fn prop(x: u16) -> bool { test_pop16(0xd1, x, Reg16::DE) }
  quickcheck(prop as fn(u16) -> bool);
}

#[test]
fn test_e1() {
  fn prop(x: u16) -> bool { test_pop16(0xe1, x, Reg16::HL) }
  quickcheck(prop as fn(u16) -> bool);
}

#[test]
fn test_f1() {
  fn prop(a: u8, f: u8) -> bool {
    let cpu = run_test(
      &[0xf1, 0xed, f, a],
      |cpu| {
        cpu.regs.sp = 0x0002;
      }
    );
    cpu.clock_cycles() == 12 &&
      cpu.regs.a == a &&
      cpu.regs.f == Flags::from_bits_truncate(f)
  }
  quickcheck(prop as fn(u8, u8) -> bool);
}