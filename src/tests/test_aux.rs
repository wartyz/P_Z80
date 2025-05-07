use crate::cpu::cpu::CPU;

pub enum Valores {
    T,
    F,
    I,
}
pub fn ejecutar(cpu: &mut CPU, m0: u8, m1: u8, m2: u8, m3: u8) {
    cpu.halt = false;

    cpu.bus.escribir_byte(0, m0);
    cpu.bus.escribir_byte(1, m1);
    cpu.bus.escribir_byte(2, m2);
    cpu.bus.escribir_byte(3, m3);

    // Ejecuto la instruccion
    cpu.execute();
}

// 0 => false, 1 => true, 2 => indiferente
// s(Sign)  z(Zero)  h(Halfcarry)  pv(Parityoverflow)  n(AddSubstract)  c(Carry)
pub fn prueba_flags(
    cpu: &CPU,
    sign: u8,
    zero: u8,
    half_carry: u8,
    parity_overflow: u8,
    add_substract: u8,
    carry: u8,
) {
    let flags = [
        (sign, cpu.reg.flags.s, "Sign"),
        (zero, cpu.reg.flags.z, "Zero"),
        (half_carry, cpu.reg.flags.h, "HalfCarry"),
        (parity_overflow, cpu.reg.flags.p, "ParityOverflow"),
        (add_substract, cpu.reg.flags.n, "AddSubtract"),
        (carry, cpu.reg.flags.c, "Carry"),
    ];

    for (val, flag, name) in flags {
        match val {
            0 => assert!(!flag, "Flag {} es false", name),
            1 => assert!(flag, "Flag {} es true", name),
            _ => (),
        }
    }
}