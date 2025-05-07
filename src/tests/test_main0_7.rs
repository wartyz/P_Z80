use crate::cpu::cpu::CPU;
use crate::tests::test_aux::{ejecutar, prueba_flags};

#[test]
fn nop() {
    // -------------+----+---+------+---------+------------+
    //  Mnemonico   |Clck|Siz|SZHPNC| OP-Code |Descripcion |
    // -------------+----+---+------+---------+------------+
    //  NOP         | 4  | 1 |------| 00      |No Operacion|

    let mut c = CPU::new(0xFFFF);
    c.bus.escribir_byte(0x0000, 0x00);

    c.execute();

    // Comprueba que el PC ha cambiado
    assert_eq!(c.reg.pc, 0x01);
}

#[test]
fn ld_bc_nn() {
    // -------------+----+---+------+----------+
    //  Mnemonico   |Clck|Siz|SZHPNC| OP-Code  |
    // -------------+----+---+------+----------+
    //  LD BC,NN    | 10 | 3 |------| 01 XX XX |

    let mut c = CPU::new(0xFFFF);
    c.bus.escribir_byte(0x0000, 0x01);
    c.bus.escribir_byte(0x0001, 0xFE);
    c.bus.escribir_byte(0x0002, 0xA8);
    c.bus.escribir_byte(0x0003, 0x00);

    c.execute();

    assert_eq!(c.reg.get_bc(), 0xA8FE);
}

#[test]
fn ld_0bc0_a() {
    // -------------+----+---+------+---------+
    //  Mnemonico   |Clck|Siz|SZHPNC| OP-Code |
    // -------------+----+---+------+---------+
    //  LD (BC),A   | 7  | 1 |------| 02      |

    let mut c = CPU::new(0xFFFF);

    c.reg.a = 0xF3;

    c.reg.set_bc(0xEF10);

    ejecutar(&mut c, 0x02, 0, 0, 0);

    assert_eq!(c.bus.leer_byte(0xEF10), 0xF3);
}

#[test]
fn inc_bc() {
    // ------------+----+---+------+---------+-------------------+-------------+
    // Mnemonico   |Clck|Siz|SZHPNC| OP-Code |    Descripcion    | Notas       |
    // ------------+----+---+------+---------+-------------------+-------------+
    // INC BC      | 6  | 1 |------| 03      |Increment (16-bit) | BC = BC + 1 |

    let mut c = CPU::new(0xFFFF);

    c.reg.set_bc(0xEF10);

    ejecutar(&mut c, 0x03, 0, 0, 0);

    assert_eq!(c.reg.get_bc(), 0xEF11);
}

#[test]
fn inc_b() {
    // Flags:  - no afectado    A afectado    0 reset    1 set    ? desconocido    P Parity    V overflow
    // -------------+----+---+------+------------+
    // Mnemonico    |Clck|Siz|SZHPNC|OP-Code     |
    // -------------+----+---+------+------------+
    // INC B        |4   |1  |AAAV02|04          |

    let mut c = CPU::new(0xFFFF);

    c.reg.b = 0xFF;

    ejecutar(&mut c, 0x04, 0, 0, 0);

    assert_eq!(c.reg.b, 0x00);

    prueba_flags(&c, 0, 1, 1, 0, 0, 2);
}

#[test]
fn dec_b() {
    // Flags:  - no afectado    A afectado    0 reset    1 set    ? desconocido    P Parity    V overflow
    // -------------+----+---+------+------------+
    // Mnemonico    |Clck|Siz|SZHPNC| OP-Code    |
    // -------------+----+---+------+------------+
    // DEC B        | 4  | 1 |AAAV1-| 05         |

    let mut c = CPU::new(0xFFFF);

    c.reg.b = 0x00;

    ejecutar(&mut c, 0x05, 0, 0, 0);

    assert_eq!(c.reg.b, 0xFF);

    prueba_flags(&c, 1, 0, 1, 0, 1, 2);
}

#[test]
fn ld_b_n() {
    // Flags:  - no afectado    A afectado    0 reset    1 set    ? desconocido    P Parity    V overflow
    // -------------+----+---+------+------------+
    // Mnemonico    |Clck|Siz|SZHPNC| OP-Code    |
    // -------------+----+---+------+------------+
    // LD B,N       | 7  | 2 |------| 06 XX      |

    let mut c = CPU::new(0xFFFF);

    ejecutar(&mut c, 0x06, 0x5A, 0, 0);

    assert_eq!(c.reg.b, 0x5A);
}

#[test]
fn rlca() {
    // Flags:  - no afectado    A afectado    0 reset    1 set    ? desconocido    P Parity    V overflow
    // -------------+----+---+------+---------+----------------------+-------+
    // Mnemonico    |Clck|Siz|SZHPNC| OP-Code | Descripcion          | Notas |
    // -------------+----+---+------+---------+----------------------+-------+
    // RLCA         | 4  | 1 |--0-0A| 07      | Rotate Left Cir. Acc.| A=A<- |

    let mut c = CPU::new(0xFFFF);

    c.reg.a = 0b1000_0000;

    ejecutar(&mut c, 0x07, 0, 0, 0);

    assert_eq!(c.reg.a, 0x01);

    prueba_flags(&c, 2, 2, 0, 2, 0, 1);
}

#[test]
fn ex_af_afp() {
    // Flags:  - no afectado    A afectado    0 reset    1 set    ? desconocido    P Parity    V overflow
    // -------------+----+---+------+---------+-------------+------------+
    // Mnemonico    |Clck|Siz|SZHPNC| OP-Code | Descripcion | Notas      |
    // -------------+----+---+------+---------+-------------+------------+
    // EX AF,AF'    | 4  | 1 |------| 08      |             | AF <-> AF' |

    let mut c = CPU::new(0xFFFF);

    // AF = 0x3412
    c.reg.a = 0x34;
    c.reg.flags.set_from_byte(0x12);

    // AF' = 0xEFAB
    c.alt.a = 0xEF;
    c.alt.flags.set_from_byte(0xAB);

    ejecutar(&mut c, 0x08, 0, 0, 0);

    assert_eq!(c.reg.a, 0xEF);
    assert_eq!(c.reg.flags.to_byte(), 0xAB);

    assert_eq!(c.alt.a, 0x34);
    assert_eq!(c.alt.flags.to_byte(), 0x12);
}

#[test]
fn add_hl_bc() {
    // Flags:  - no afectado    A afectado    0 reset    1 set    ? desconocido    P Parity    V overflow
    // -------------+----+---+------+---------+------------- +----------------+
    // Mnemonico    |Clck|Siz|SZHPNC| OP-Code | Descripcion  | Notas          |
    // -------------+----+---+------+---------+-------------------------------+
    // ADD HL,BC    | 11 | 1 |--A-0A| 09      | Add (16-bit) | HL + BC -> HL  |

    let mut c = CPU::new(0xFFFF);

    // HL = 0x3412
    c.reg.set_hl(0x3412);

    // BC = 0xB5A4
    c.reg.set_bc(0xB5A4);

    ejecutar(&mut c, 0x09, 0, 0, 0);

    assert_eq!(c.reg.get_hl(), 0xE9B6);

    prueba_flags(&c, 2, 2, 0, 2, 0, 0);
}

#[test]
fn ld_a_0bc0() {
    // Flags:  - no afectado    A afectado    0 reset    1 set    ? desconocido    P Parity    V overflow
    // -------------+----+---+------+---------+
    // Mnemonico    |Clck|Siz|SZHPNC| OP-Code |
    // -------------+----+---+------+---------+
    // LD A,(BC)    | 7  | 1 |------| 0A      |

    let mut c = CPU::new(0xFFFF);

    // Pongo en la direccion de memoria 0x10EF el valor 0xA5
    c.bus.escribir_byte(0xEF10, 0xA5);

    // BC = 0x10EF
    c.reg.set_bc(0xEF10);

    ejecutar(&mut c, 0x0A, 0, 0, 0);

    // compruebo que en A esta el dato =xA5
    assert_eq!(c.reg.a, 0xA5);
}

#[test]
fn dec_bc() {
    // -------------+----+---+------+---------+----------------------+----------------+
    // Mnemonico    |Clck|Siz|SZHPNC| OP-Code | Descripcion          | Notas          |
    // -------------+----+---+------+---------+----------------------+----------------+
    // DEC BC       | 6  | 1 |------| 0B      | Decrement (16-bit)   | BC = BC - 1    |

    let mut c = CPU::new(0xFFFF);

    // BC = 0xEF10
    c.reg.set_bc(0xEF10);

    ejecutar(&mut c, 0x0B, 0, 0, 0);

    assert_eq!(c.reg.b, 0xEF);
    assert_eq!(c.reg.c, 0x0F);
}

#[test]
fn inc_c() {
    // -------------+----+---+------+---------+
    // Mnemonico    |Clck|Siz|SZHPNC| OP-Code |
    // -------------+----+---+------+---------+
    // INC C        | 4  | 1 |AAAV0-| 0C      |

    let mut c = CPU::new(0xFFFF);

    // C = 0xFF
    c.reg.c = 0xFF;

    ejecutar(&mut c, 0x0C, 0, 0, 0);

    // compruebo que ahora c = 0x00
    assert_eq!(c.reg.c, 0x00);

    prueba_flags(&c, 0, 1, 1, 0, 0, 2);
}

#[test]
fn dec_c() {
    // -------------+----+---+------+---------+
    // Mnemonico    |Clck|Siz|SZHPNC| OP-Code |
    // -------------+----+---+------+---------+
    // DEC C        | 4  | 1 |AAAV1-| 0D      |
    let mut c = CPU::new(0xFFFF);

    // c = 0x00
    c.reg.c = 0x00;

    ejecutar(&mut c, 0x0D, 0, 0, 0);

    // compruebo que ahora c = 0xFF
    assert_eq!(c.reg.c, 0xFF);

    prueba_flags(&c, 1, 0, 1, 0, 1, 2);
}

#[test]
fn ld_c_n() {
    // 0x0E No afecta flags
    // -------------+----+---+------+---------+
    // Mnemonico    |Clck|Siz|SZHPNC| OP-Code |
    // -------------+----+---+------+---------+
    // LD C,N       | 7  | 2 |------| 0E XX   |

    let mut c = CPU::new(0xFFFF);

    ejecutar(&mut c, 0x0E, 0x5A, 0, 0);

    // compruebo que ahora c = 0x5A
    assert_eq!(c.reg.c, 0x5A);
}

#[test]
fn rrca() {
    // -------------+----+---+------+---------+-----------------------+----------------+
    // Mnemonico    |Clck|Siz|SZHPNC| OP-Code | Descripcion           | Notas          |
    // -------------+----+---+------+---------+-----------------------+----------------+
    // RRCA         | 4  | 1 |--0-0A| 0F      | Rotate Right Cir.Acc. | A=->A          |

    let mut c = CPU::new(0xFFFF);

    // A = 0b0000_0001
    c.reg.a = 0b0000_0001;

    ejecutar(&mut c, 0x0F, 0, 0, 0);

    // compruebo que ahora a = 0x00
    assert_eq!(c.reg.a, 0b1000_0000);

    prueba_flags(&c, 2, 2, 0, 2, 0, 1);
}
//
// #[test]
// fn djnz_d() {
// 0x10 Instrucción DJNZ d  seguida del desplazamiento d (0xFE para -2) No afecta flags1

// -------------+----+---+------+---------+----------------------+--------------------+
// Mnemonico    |Clck|Siz|SZHPNC| OP-Code | Descripcion          | Notas              |
// -------------+----+---+------+---------+----------------------+--------------------+
// DJNZ $+2     |13/8| 1 |------| 10      | Dec., Jump Non-Zero  | B = B-1 till B = 0 |

//     let mut z80 = Z80::default();
//     let mut c = CPU::new(0xFFFF);
//
//     // Inicializo B a 0x02 para que el bucle se ejecute dos veces
//     z80.reg.b = 0x02;
//     c.reg.b = 0x02;
//
//     ejecutar(&mut z80, 0x10, 0xFE, 0, 0, &mut c);
//
//     assert_eq!(z80.reg.b, 0x01);
//     assert_eq!(z80.reg.pc, 0x00); // Debe haber saltado hacia atrás
//
//     z80.es_halted = false;
//
//     // Ejecuto la segunda vez: B se decrementa a 0, por lo que **no** debe saltar
//     z80.step(&mut c);
//
//     assert_eq!(z80.reg.b, 0x00);
//     assert_eq!(z80.reg.pc, 0x0002); // Avanza a la siguiente instrucción
//
// }
//
// #[test]
// fn ld_de_nn() {
//     // 0x11 C se define N reset, H reset
//     let mut z80 = Z80::default();
//     let mut c = CPU::new(0xFFFF);
//
//     ejecutar(&mut z80, 0x11, 0xFE, 0xA8, 0, &mut c);
//
//     // compruebo que ahora de = 0xA8FE
//     //assert_eq!(z80.reg.d, 0xA8);
//     //assert_eq!(z80.reg.e, 0xFE);
//     assert_eq!(get_de_test_big(&mut z80, &mut c), 0xA8FE);
// }
//
// #[test]
// fn ld_0de0_a() {
//     // 0x12
//     let mut z80 = Z80::default();
//     let mut c = CPU::new(0xFFFF);
//
//     // pongo A = 0xF3
//     z80.reg.a = 0xF3;
//     c.reg.a = 0xF3;
//
//     // pongo en DE = 0xEF10
//     // z80.registros.d = 0xEF;
//     // z80.registros.e = 0x10;
//     // c.reg.d = 0xEF;
//     // c.reg.e = 0x10;
//     set_de_test_big(&mut z80, &mut c, 0xEF10);
//
//     ejecutar(&mut z80, 0x12, 0, 0, 0, &mut c);
//
//     // compruebo que en la direccion 0x10EF esta el valor 0xF3
//     assert_eq!(z80.mem.mem[0xEF10], 0xF3);
// }
//
// #[test]
// fn inc_de() {
//     // 0x13
//     let mut z80 = Z80::default();
//     let mut c = CPU::new(0xFFFF);
//
//     // pongo DE = 0xEF10
//     // z80.registros.d = 0xEF;
//     // z80.registros.e = 0x10;
//     // c.reg.d = 0xEF;
//     // c.reg.e = 0x10;
//     set_de_test_big(&mut z80, &mut c, 0xEF10);
//
//     ejecutar(&mut z80, 0x13, 0, 0, 0, &mut c);
//
//     // compruebo que ahora de=0xEF11
//     //assert_eq!(z80.reg.d, 0xEF);
//     //assert_eq!(z80.reg.e, 0x11);
//     assert_eq!(get_de_test_big(&mut z80, &mut c), 0xEF11);
// }
//
// #[test]
// fn inc_d() {
//     // 0x14 N reset, P/V detecta overflow, H,Z,S se define
//     let mut z80 = Z80::default();
//     let mut c = CPU::new(0xFFFF);
//
//     // pongo D = 0xFF
//     z80.reg.d = 0xFF;
//     c.reg.d = 0xFF;
//
//     ejecutar(&mut z80, 0x14, 0, 0, 0, &mut c);
//
//     // compruebo que ahora d = 0x00
//     assert_eq!(z80.reg.d, 0x00);
//
//     // 0 => false, 1 => true, 2 => indiferente
//     // s(Sign)  z(Zero)  h(Halfcarry)  pv(Parityoverflow)  n(AddSubstract)  c(Carry)
//     prueba_flags(&z80, 0, 1, 1, 0, 0, 2);
// }
//
// #[test]
// fn dec_d() {
//     // 0x15 N set, P/V detecta overflow, H,Z,S se define
//     let mut z80 = Z80::default();
//     let mut c = CPU::new(0xFFFF);
//
//     // pongo D = 0x00
//     z80.reg.d = 0x00;
//     c.reg.d = 0x00;
//
//     ejecutar(&mut z80, 0x15, 0, 0, 0, &mut c);
//
//     // compruebo que ahora d = 0xFF
//     assert_eq!(z80.reg.d, 0xFF);
//
//     // 0 => false, 1 => true, 2 => indiferente
//     // s(Sign)  z(Zero)  h(Halfcarry)  pv(Parityoverflow)  n(AddSubstract)  c(Carry)
//     prueba_flags(&z80, 1, 0, 1, 0, 1, 2);
// }
//
// #[test]
// fn ld_d_n() {
//     // 0x16
//     let mut z80 = Z80::default();
//     let mut c = CPU::new(0xFFFF);
//
//     // pongo segundo argumento 0x5A (se debe cargar en D)
//     ejecutar(&mut z80, 0x16, 0x5A, 0, 0, &mut c);
//
//     // compruebo que ahora d = 0x5A
//     assert_eq!(z80.reg.d, 0x5A);
// }
//
// #[test]
// fn rla() {
//     // 0x17 C se define N reset, H reset
//     // A rota a la izquierda un bit, bit7->carry  carry anterior->bit0
//     let mut z80 = Z80::default();
//     let mut c = CPU::new(0xFFFF);
//
//     // pongo el carry inicial ON
//     z80.reg.set_flag(&StatusFlag::Carry, true);
//     c.reg.flags.c = true;
//
//     // pongo A = 0b1000_0000
//     z80.reg.a = 0b0000_0000;
//     c.reg.a = 0b0000_0000;
//
//     ejecutar(&mut z80, 0x17, 0, 0, 0, &mut c);
//
//     // compruebo que ahora a = 0x01
//     assert_eq!(z80.reg.a, 0b0000_0001);
//
//     // 0 => false, 1 => true, 2 => indiferente
//     // s(Sign)  z(Zero)  h(Halfcarry)  pv(Parityoverflow)  n(AddSubstract)  c(Carry)
//     prueba_flags(&z80, 2, 2, 0, 2, 0, 0);
// }
//
// #[test]
// fn jr_d() {
//     // 0x18 No afecta flgs
//     let mut z80 = Z80::default();
//     let mut c = CPU::new(0xFFFF);
//
//     // 0x80 = -128;
//     //ejecutar(&mut z80, 0x18, 0x80, 0, 0, &mut c);
//     z80.reg.pc = 0xE080;
//     ejecutar_en_direccion(&mut z80, 0xE080, 0x18, 0x80, 0, 0, &mut c);
//
//     // assert_eq!(z80.registros.pc, 0xFF82);
//     assert_eq!(z80.reg.pc, 0xE002);
// }
//
// #[test]
// fn add_hl_de() {
//     // 0x19  DE -> HL     Carry se define, N=0, PV no afectado,  H se define, Z, S no afectados,
//     let mut z80 = Z80::default();
//     let mut c = CPU::new(0xFFFF);
//
//     // pongo HL = 0x3412
//     // z80.registros.h = 0x34;
//     // z80.registros.l = 0x12;
//     // c.reg.set_hl(0x3412);
//     //set_hl_test(&mut z80, &mut c, 0x3412);
//     set_hl_test_big(&mut z80, &mut c, 0x0104);
//
//     // pongo DE = 0xB5A4
//     // z80.registros.d = 0xB5;
//     // z80.registros.e = 0xA4;
//     // c.reg.set_de(0xB5A4);
//     //set_de_test(&mut z80, &mut c, 0xB5A4);
//     set_de_test_big(&mut z80, &mut c, 0xFFFF);
//
//     ejecutar(&mut z80, 0x19, 0, 0, 0, &mut c);
//
//     // compruebo que ahora hl = 0xE9B6
//     //assert_eq!(z80.registros.h, 0xE9);
//     //assert_eq!(z80.registros.l, 0xB6);
//
//     //assert_eq!(z80.reg.h, 0x01);
//     //assert_eq!(z80.reg.l, 0x03);
//     assert_eq!(get_hl_test_big(&mut z80, &mut c), 0x0103);
//     // 0 => false, 1 => true, 2 => indiferente
//     // s(Sign)  z(Zero)  h(Halfcarry)  pv(Parityoverflow)  n(AddSubstract)  c(Carry)
//     prueba_flags(&z80, 2, 2, 1, 2, 0, 1);
// }
//
// #[test]
// fn ld_a_0de0() {
//     // 0x1A  No afecta flags
//     let mut z80 = Z80::default();
//     let mut c = CPU::new(0xFFFF);
//
//     // Pongo en la direccion de memoria 0x10EF el valor 0xA5
//     z80.mem.mem[0xEF10] = 0xA5;
//     c.bus.write_byte(0xEF10, 0xA5);
//
//     // pongo DE = 0x10EF
//     // z80.registros.d = 0xEF;
//     // z80.registros.e = 0x10;
//     // c.reg.set_de(0xEF10);
//     set_de_test_big(&mut z80, &mut c, 0xEF10);
//
//     ejecutar(&mut z80, 0x1A, 0, 0, 0, &mut c);
//
//     // compruebo que en A esta el dato =xA5
//     assert_eq!(z80.reg.a, 0xA5);
// }
//
// #[test]
// fn dec_de() {
//     // 0x1B  No afecta flags
//     let mut z80 = Z80::default();
//     let mut c = CPU::new(0xFFFF);
//
//     // pongo DE = 0xEF10
//     // z80.registros.d = 0xEF;
//     // z80.registros.e = 0x10;
//     // c.reg.set_de(0xEF10);
//     set_de_test_big(&mut z80, &mut c, 0xEF10);
//
//     ejecutar(&mut z80, 0x1B, 0, 0, 0, &mut c);
//
//     // compruebo que ahora de=0xEF0F
//     //assert_eq!(z80.reg.d, 0xEF);
//     //assert_eq!(z80.reg.e, 0x0F);
//     assert_eq!(get_de_test_big(&mut z80, &mut c), 0xEF0F);
// }
// #[test]
// fn inc_e() {
//     // 0x1C N reset, P/V detecta overflow, H,Z,S se define
//     let mut z80 = Z80::default();
//     let mut c = CPU::new(0xFFFF);
//
//     // pongo E = 0xFF
//     z80.reg.e = 0xFF;
//     c.reg.e = 0xFF;
//
//     ejecutar(&mut z80, 0x1C, 0, 0, 0, &mut c);
//     // compruebo que ahora c = 0x00
//     assert_eq!(z80.reg.e, 0x00);
//
//     // 0 => false, 1 => true, 2 => indiferente
//     // s(Sign)  z(Zero)  h(Halfcarry)  pv(Parityoverflow)  n(AddSubstract)  c(Carry)
//     prueba_flags(&z80, 0, 1, 1, 0, 0, 2);
// }
//
// #[test]
// fn dec_e() {
//     // 0x1D N set, P/V detecta overflow, H,Z,S se define
//     let mut z80 = Z80::default();
//     let mut c = CPU::new(0xFFFF);
//
//     // pongo c = 0x00
//     z80.reg.e = 0x00;
//     c.reg.e = 0x00;
//
//     ejecutar(&mut z80, 0x1D, 0, 0, 0, &mut c);
//
//     // compruebo que ahora e = 0xFF
//     assert_eq!(z80.reg.e, 0xFF);
//
//     // 0 => false, 1 => true, 2 => indiferente
//     // s(Sign)  z(Zero)  h(Halfcarry)  pv(Parityoverflow)  n(AddSubstract)  c(Carry)
//     prueba_flags(&z80, 1, 0, 1, 0, 1, 2);
// }
//
// #[test]
// fn ld_e_n() {
//     // 0x1E
//     let mut z80 = Z80::default();
//     let mut c = CPU::new(0xFFFF);
//
//     // pongo segundo argumento 0x5A (se debe cargar en B)
//     ejecutar(&mut z80, 0x1E, 0x5A, 0, 0, &mut c);
//
//     // compruebo que ahora e = 0x5A
//     assert_eq!(z80.reg.e, 0x5A);
// }
//
// #[test]
// fn rra() {
//     // 0x1F C se define N reset, H reset
//     let mut z80 = Z80::default();
//     let mut c = CPU::new(0xFFFF);
//
//     // pongo A = 0b0000_0000
//     z80.reg.a = 0b0000_0000;
//     c.reg.a = 0b0000_0000;
//
//     // Pongo true el contenido inicial del flag carry
//     z80.reg.set_flag(&StatusFlag::Carry, true);
//     c.reg.flags.c = true;
//
//     ejecutar(&mut z80, 0x1F, 0x03, 0, 0, &mut c);
//
//     // compruebo que ahora a = 0x80
//     assert_eq!(z80.reg.a, 0b1000_0000);
//
//     // 0 => false, 1 => true, 2 => indiferente
//     // s(Sign)  z(Zero)  h(Halfcarry)  pv(Parityoverflow)  n(AddSubstract)  c(Carry)
//     prueba_flags(&z80, 2, 2, 0, 2, 0, 0);
// }
//
// #[test]
// fn jr_nz_n() {
//     // 0x20
//     let mut z80 = Z80::default();
//     let mut c = CPU::new(0xFFFF);
//     let pc_inicial = 0;
//
//     // Inicializo el flag Z en true para que no salte
//     z80.reg.set_flag(&StatusFlag::Zero, true);
//     c.reg.flags.z = true;
//
//     // pongo el desplazamiento a 2 (0x03) (salto de 2 bytes si NZ es true)
//     ejecutar(&mut z80, 0x20, 0x03, 0, 0, &mut c);
//
//     // compruebo que el program counter no ha cambiado (Z es true)
//     assert_eq!(z80.reg.pc, pc_inicial + 2);
//
//     // pongo el flag Z en false para que salte
//     z80.reg.set_flag(&StatusFlag::Zero, false);
//     c.reg.flags.z = false;
//
//     // reseteo el program counter
//     z80.reg.pc = pc_inicial;
//     c.reg.pc = pc_inicial;
//
//     z80.es_halted = false;
//     // vuelvo a ejecutar la instruccion
//     z80.step(&mut c);
//
//     // compruebo que el program counter ha saltado 2 bytes
//     assert_eq!(z80.reg.pc, pc_inicial + 5);
//
//     // salto negativo --------------------------------------
//
//     // Inicializo el flag Z en false para que salte
//     z80.reg.set_flag(&StatusFlag::Zero, false);
//
//     // Ejecuto desde direccion 0x11E0 y pongo el desplazamiento a -6 (0xFA)
//     z80.reg.pc = 0x11E0;
//     ejecutar_en_direccion(&mut z80, 0x11E0, 0x20, 0xFA, 0, 0, &mut c);
//
//     // Cálculo del PC esperado con desplazamiento negativo
//     assert_eq!(z80.reg.pc, 0x11DC);
// }
//
// #[test]
// fn ld_hl_nn() {
//     // 0x21 C se define N reset, H reset
//     let mut z80 = Z80::default();
//     let mut c = CPU::new(0xFFFF);
//
//     ejecutar(&mut z80, 0x21, 0xFE, 0xA8, 0, &mut c);
//
//     // compruebo que ahora de = 0xA8FE
//     //assert_eq!(z80.registros.get_reg16_lit_endian(&HL), 0xA8FE);
//     //assert_eq!(z80.reg.h, 0xA8);
//     //assert_eq!(z80.reg.l, 0xFE);
//     assert_eq!(get_hl_test_big(&mut z80, &mut c), 0xA8FE);
// }
//
// #[test]
// fn ld_0nn0_hl() {
//     // 0x22
//     let mut z80 = Z80::default();
//     let mut c = CPU::new(0xFFFF);
//
//     set_hl_test_big(&mut z80, &mut c, 0xCDAB);
//
//     ejecutar(&mut z80, 0x22, 0x34, 0x12, 0, &mut c);
//
//     // compruebo que en la dirección 0x1234 se ha guardado 0xAB
//     assert_eq!(z80.mem.mem[0x1234], 0xAB);
//     assert_eq!(z80.mem.mem[0x1235], 0xCD);
//
//     assert_eq!(c.bus.read_byte(0x1234), 0xAB);
//     assert_eq!(c.bus.read_byte(0x1235), 0xCD);
// }
//
// #[test]
// fn inc_hl() {
//     // 0x23  no afecta flags
//     let mut z80 = Z80::default();
//     let mut c = CPU::new(0xFFFF);
//
//     // z80.registros.h = 0xF0;
//     // z80.registros.l = 0x10;
//     // c.reg.set_hl(0xF010);
//     set_hl_test_big(&mut z80, &mut c, 0xF010);
//
//     ejecutar(&mut z80, 0x23, 0, 0, 0, &mut c);
//
//     //assert_eq!(z80.reg.h, 0xF0);
//     //assert_eq!(z80.reg.l, 0x11);
//     assert_eq!(get_hl_test_big(&mut z80, &mut c), 0xF011);
// }
// #[test]
// fn inc_h() {
//     // 0x24 N reset, P/V detecta overflow, H,Z,S se define
//     let mut z80 = Z80::default();
//     let mut c = CPU::new(0xFFFF);
//
//     // pongo H = 0xFF
//     z80.reg.h = 0xFF;
//     c.reg.h = 0xFF;
//
//     ejecutar(&mut z80, 0x24, 0, 0, 0, &mut c);
//
//     // compruebo que ahora h = 0x00
//     assert_eq!(z80.reg.h, 0x00);
//
//     // 0 => false, 1 => true, 2 => indiferente
//     // s(Sign)  z(Zero)  h(Halfcarry)  pv(Parityoverflow)  n(AddSubstract)  c(Carry)
//     prueba_flags(&z80, 0, 1, 1, 0, 0, 2);
// }
// #[test]
// fn dec_h() {
//     // 0x25 N set, P/V detecta overflow, H,Z,S se define
//     let mut z80 = Z80::default();
//     let mut c = CPU::new(0xFFFF);
//     // pongo H = 0x00
//     z80.reg.h = 0x00;
//     c.reg.h = 0x00;
//
//     ejecutar(&mut z80, 0x25, 0, 0, 0, &mut c);
//
//     // compruebo que ahora h = 0xFF
//     assert_eq!(z80.reg.h, 0xFF);
//
//     // 0 => false, 1 => true, 2 => indiferente
//     // s(Sign)  z(Zero)  h(Halfcarry)  pv(Parityoverflow)  n(AddSubstract)  c(Carry)
//     prueba_flags(&z80, 1, 0, 1, 0, 1, 2);
// }
// #[test]
// fn ld_h_n() {
//     // 0x26
//     let mut z80 = Z80::default();
//     let mut c = CPU::new(0xFFFF);
//     ejecutar(&mut z80, 0x26, 0x5A, 0, 0, &mut c);
//     // compruebo que ahora h = 0xFF
//     assert_eq!(z80.reg.h, 0x5A);
// }
//
// #[test]
// fn daa() {
//     // 0x27 C se define N reset, H reset
//     let mut z80 = Z80::default();
//     let mut c = CPU::new(0xFFFF);
//
//     z80.reg.a = 0x3C; // Valor inicial en A que requiere ajuste BCD
//     c.reg.a = 0x3C;
//
//     // Configuro flags coherentes con la operación previa (ej. suma sin carry)
//     z80.reg.set_flag(&StatusFlag::AddSubtract, false); // Última op fue suma
//     z80.reg.set_flag(&StatusFlag::HalfCarry, false);
//     z80.reg.set_flag(&StatusFlag::Carry, false);
//     c.reg.flags.n = false;
//     c.reg.flags.h = false;
//     c.reg.flags.c = false;
//
//     ejecutar(&mut z80, 0x27, 0, 0, 0, &mut c);
//
//     // Después de DAA, el valor esperado en A es 0x00, y el flag de Carry debería estar activo
//     assert_eq!(z80.reg.a, 0x42);
//
//     // 0 => false, 1 => true, 2 => indiferente
//     // s(Sign)  z(Zero)  h(Halfcarry)  pv(Parityoverflow)  n(AddSubstract)  c(Carry)
//     prueba_flags(&z80, 2, 0, 1, 2, 0, 0);
// }
// #[test]
// fn jr_z_n() {
//     // 0x28 d-$-2      no afecta flags
//     let mut z80 = Z80::default();
//     let mut c = CPU::new(0xFFFF);
//     let pc_inicial = 0x1234;
//     z80.reg.pc = pc_inicial;
//
//     // Inicializo el flag Z en false para que no salte
//     z80.reg.set_flag(&StatusFlag::Zero, false);
//
//     // pongo el desplazamiento a 2 (0x03) (salto de 2 bytes si Z es true)
//     ejecutar_en_direccion(&mut z80, 0x1234, 0x28, 0x03, 0, 0, &mut c);
//
//     // compruebo que el program counter no ha cambiado (Z es true)
//     assert_eq!(z80.reg.pc, pc_inicial + 2);
//
//     // 2 ------------------------ pongo el flag Z en true para que salte
//     z80.reg.set_flag(&StatusFlag::Zero, true);
//     c.reg.flags.z = true;
//
//     // reseteo el program counter
//     z80.reg.pc = pc_inicial;
//     c.reg.pc = pc_inicial;
//
//     z80.es_halted = false;
//     // vuelvo a ejecutar la instruccion
//     z80.step(&mut c);
//
//     // compruebo que el program counter ha saltado 2 bytes
//     assert_eq!(z80.reg.pc, pc_inicial + 5);
//
//     // 3 ------salto negativo ------- Inicializo el flag Z en true para que salte
//     z80.reg.set_flag(&StatusFlag::Zero, true);
//     c.reg.flags.z = true;
//
//     z80.reg.pc = pc_inicial;
//     c.reg.pc = pc_inicial;
//
//     // pongo el desplazamiento a -5 (0xFB)
//     ejecutar_en_direccion(&mut z80, 0x1234, 0x28, 0xFB, 0, 0, &mut c);
//
//     // Cálculo del PC esperado con desplazamiento negativo
//     assert_eq!(z80.reg.pc, 0x1231);
// }
// #[test]
// fn add_hl_hl() {
//     // 0x29  HL -> HL     Carry se define, N=0, PV no afectado,  H se define, Z, S no afectados,
//     let mut z80 = Z80::default();
//     let mut c = CPU::new(0xFFFF);
//
//     // pongo HL = 0x1234
//     // z80.registros.h = 0x34;
//     // z80.registros.l = 0x12;
//     // c.reg.set_hl(0x3412);
//     set_hl_test_big(&mut z80, &mut c, 0x3412);
//
//     ejecutar(&mut z80, 0x29, 0, 0, 0, &mut c);
//
//     // compruebo que ahora hl = 0x6824
//     //assert_eq!(z80.reg.h, 0x68);
//     //assert_eq!(z80.reg.l, 0x24);
//     assert_eq!(get_hl_test_big(&mut z80, &mut c), 0x6824);
//
//     // 0 => false, 1 => true, 2 => indiferente
//     // s(Sign)  z(Zero)  h(Halfcarry)  pv(Parityoverflow)  n(AddSubstract)  c(Carry)
//     prueba_flags(&z80, 2, 2, 0, 2, 0, 0);
// }
// #[test]
// fn ld_hl_0nn0() {
//     // 0x2A nn       no afecta flags
//     let mut z80 = Z80::default();
//     let mut c = CPU::new(0xFFFF);
//
//     // pongo en memoria datos
//     z80.mem.mem[0x1234] = 0xD4;
//     z80.mem.mem[0x1235] = 0xE1;
//     c.bus.write_byte(0x1234, 0xD4);
//     c.bus.write_byte(0x1235, 0xE1);
//
//     ejecutar(&mut z80, 0x2A, 0x34, 0x12, 0, &mut c);
//
//     // compruebo que ahora hl = 0xE1D4
//     //assert_eq!(z80.reg.h, 0xE1);
//     //assert_eq!(z80.reg.l, 0xD4);
//     assert_eq!(get_hl_test_big(&mut z80, &mut c), 0xE1D4);
// }
//
// #[test]
// fn dec_hl() {
//     // 0x2B
//     let mut z80 = Z80::default();
//     let mut c = CPU::new(0xFFFF);
//
//     // pongo HL = 0xEF10
//     // z80.registros.h = 0xEF;
//     // z80.registros.l = 0x10;
//     // c.reg.set_hl(0xEF10);
//     set_hl_test_big(&mut z80, &mut c, 0xEF10);
//
//     ejecutar(&mut z80, 0x2B, 0, 0, 0, &mut c);
//
//     // compruebo que ahora hl=0xEF0F
//     //assert_eq!(z80.reg.h, 0xEF);
//     //assert_eq!(z80.reg.l, 0x0F);
//     assert_eq!(get_hl_test_big(&mut z80, &mut c), 0xEF0F);
// }
// #[test]
// fn inc_l() {
//     // 0x2C N reset, P/V detecta overflow, H,Z,S se define
//     let mut z80 = Z80::default();
//     let mut c = CPU::new(0xFFFF);
//
//     // pongo L = 0xFF
//     z80.reg.l = 0xFF;
//     c.reg.l = 0xFF;
//
//     ejecutar(&mut z80, 0x2C, 0, 0, 0, &mut c);
//
//     // compruebo que ahora l = 0x00
//     assert_eq!(z80.reg.l, 0x00);
//
//     // 0 => false, 1 => true, 2 => indiferente
//     // s(Sign)  z(Zero)  h(Halfcarry)  pv(Parityoverflow)  n(AddSubstract)  c(Carry)
//     prueba_flags(&z80, 0, 1, 1, 0, 0, 2);
// }
// #[test]
// fn dec_l() {
//     // 0x2D N set, P/V detecta overflow, H,Z,S se define
//     let mut z80 = Z80::default();
//     let mut c = CPU::new(0xFFFF);
//
//     // pongo l = 0x00
//     z80.reg.l = 0x00;
//     c.reg.l = 0x00;
//
//     ejecutar(&mut z80, 0x2D, 0, 0, 0, &mut c);
//
//     // compruebo que ahora e = 0xFF
//     assert_eq!(z80.reg.l, 0xFF);
//
//     // 0 => false, 1 => true, 2 => indiferente
//     // s(Sign)  z(Zero)  h(Halfcarry)  pv(Parityoverflow)  n(AddSubstract)  c(Carry)
//     prueba_flags(&z80, 1, 0, 1, 0, 1, 2);
// }
//
// #[test]
// fn ld_l_n() {
//     // 0x2E
//     let mut z80 = Z80::default();
//     let mut c = CPU::new(0xFFFF);
//
//     // pongo segundo argumento 0x5A (se debe cargar en L)
//     ejecutar(&mut z80, 0x2E, 0x5A, 0, 0, &mut c);
//
//     // compruebo que ahora l = 0x5A
//     assert_eq!(z80.reg.l, 0x5A);
// }
//
// #[test]
// fn cpl() {
//     // 0x2F N se define, H se define
//     let mut z80 = Z80::default();
//     let mut c = CPU::new(0xFFFF);
//
//     z80.reg.a = 0b1000_0000;
//     c.reg.a = 0b1000_0000;
//
//     // Ejecuto la instruccion
//     ejecutar(&mut z80, 0x2F, 0, 0, 0, &mut c);
//
//     // compruebo que ahora a = 0x00
//     assert_eq!(z80.reg.a, 0b0111_1111);
//
//     // compruebo flags
//     assert_eq!(z80.reg.get_flag(&StatusFlag::AddSubtract), true);
//     assert_eq!(z80.reg.get_flag(&StatusFlag::HalfCarry), true);
//
//     // 0 => false, 1 => true, 2 => indiferente
//     // s(Sign)  z(Zero)  h(Halfcarry)  pv(Parityoverflow)  n(AddSubstract)  c(Carry)
//     prueba_flags(&z80, 2, 2, 1, 2, 1, 2);
// }
//
// #[test]
// fn jr_nc_d() {
//     // 0x30   No afecta flags
//     let mut z80 = Z80::default();
//     let mut c = CPU::new(0xFFFF);
//     let pc_inicial = 0x1234;
//
//     // Inicializo el flag C en true para que no salte
//     z80.reg.set_flag(&StatusFlag::Carry, true);
//     c.reg.flags.c = true;
//
//     // pongo el desplazamiento a 2 (0x03) (salto de 2 bytes si NC es true)
//     z80.reg.pc = pc_inicial;
//     ejecutar_en_direccion(&mut z80, 0x1234, 0x30, 0x03, 0, 0, &mut c);
//
//     // compruebo que el program counter no ha cambiado (C es true)
//     assert_eq!(z80.reg.pc, pc_inicial + 2);
//
//     // pongo el flag C en false para que salte
//     z80.reg.set_flag(&StatusFlag::Carry, false);
//     c.reg.flags.c = false;
//
//     // reseteo el program counter
//     z80.reg.pc = pc_inicial;
//
//     z80.es_halted = false;
//     // vuelvo a ejecutar la instruccion
//     z80.step(&mut c);
//
//     // compruebo que el program counter ha saltado 2 bytes
//     assert_eq!(z80.reg.get_pc(), pc_inicial + 5);
//
//     // salto negativo --------------------------------------
//
//     // Inicializo el flag C en false para que salte
//     z80.reg.set_flag(&StatusFlag::Carry, false);
//     c.reg.flags.c = false;
//
//     z80.reg.pc = pc_inicial;
//     c.reg.pc = pc_inicial;
//
//     // pongo el desplazamiento a -5 (0xFB)
//     ejecutar(&mut z80, 0x30, 0xFB, 0, 0, &mut c);
//
//     // Cálculo del PC esperado con desplazamiento negativo
//     assert_eq!(z80.reg.pc, 0x1239);
// }
//
// #[test]
// fn ld_sp_nn() {
//     // 0x31 C se define N reset, H reset
//     let mut z80 = Z80::default();
//     let mut c = CPU::new(0xFFFF);
//
//     ejecutar(&mut z80, 0x31, 0xFE, 0xA8, 0, &mut c);
//
//     // compruebo que ahora sp = 0xA8FE
//     assert_eq!(z80.reg.sp, 0xA8FE);
// }
//
// #[test]
// fn ld_0nn0_a() {
//     // 0x32
//     let mut z80 = Z80::default();
//     let mut c = CPU::new(0xFFFF);
//
//     // pongo A = 0xCD
//     z80.reg.a = 0xCD;
//     c.reg.a = 0xCD;
//
//     ejecutar(&mut z80, 0x32, 0x34, 0x12, 0, &mut c);
//
//     // compruebo que en la dirección 0x1234 se ha guardado 0xCD
//     assert_eq!(z80.mem.mem[0x1234], 0xCD);
//     assert_eq!(c.bus.read_byte(0x1234), 0xCD);
// }
//
// #[test]
// fn inc_sp() {
//     // 0x33  No afecta flags
//     let mut z80 = Z80::default();
//     let mut c = CPU::new(0xFFFF);
//
//     // pongo SP = 0x10EF
//     //z80.registros.sp = 0x10EF;
//     //c.reg.sp = 0x10EF;
//     set_sp_test_big(&mut z80, &mut c, 0x10EF);
//
//     ejecutar(&mut z80, 0x33, 0, 0, 0, &mut c);
//
//     // compruebo que ahora SP=0x10F0
//     assert_eq!(z80.reg.sp, 0x10F0);
// }
// #[test]
// fn inc_0hl0() {
//     // 0x34 N reset, P/V detecta overflow, H,Z,S se define
//     let mut z80 = Z80::default();
//     let mut c = CPU::new(0xFFFF);
//
//     // pongo en la direccion 0x10EF el valor 0x07
//     z80.mem.mem[0xEF10] = 0x07;
//     c.bus.write_byte(0xEF10, 0x07);
//
//     // pongo HL = 0xEF10
//     // z80.registros.h = 0xEF;
//     // z80.registros.l = 0x10;
//     // c.reg.set_hl(0xEF10);
//     set_hl_test_big(&mut z80, &mut c, 0xEF10);
//
//     ejecutar(&mut z80, 0x34, 0, 0, 0, &mut c);
//
//     // compruebo que ahora que el dato en la direccion 0x10EF es 0x08
//     assert_eq!(z80.mem.mem[0xEF10], 0x08);
//     assert_eq!(c.bus.read_byte(0xEF10), 0x08);
//
//     // 0 => false, 1 => true, 2 => indiferente
//     // s(Sign)  z(Zero)  h(Halfcarry)  pv(Parityoverflow)  n(AddSubstract)  c(Carry)
//     prueba_flags(&z80, 0, 0, 0, 0, 0, 2);
// }
// #[test]
// fn dec_0hl0() {
//     // 0x35 N set, P/V detecta overflow, H,Z,S se define
//     let mut z80 = Z80::default();
//     let mut c = CPU::new(0xFFFF);
//
//     // pongo en la direccion 0x10EF el valor 0x07
//     z80.mem.mem[0xEF10] = 0x07;
//     c.bus.write_byte(0xEF10, 0x07);
//
//     // pongo HL = 0x10EF
//     // z80.registros.h = 0xEF;
//     // z80.registros.l = 0x10;
//     // c.reg.set_hl(0xEF10);
//     set_hl_test_big(&mut z80, &mut c, 0xEF10);
//
//     ejecutar(&mut z80, 0x35, 0, 0, 0, &mut c);
//
//     // compruebo que ahora que el dato en la direccion 0x10EF es 0x06
//     assert_eq!(z80.mem.mem[0xEF10], 0x06);
//     assert_eq!(c.bus.read_byte(0xEF10), 0x06);
//
//     // 0 => false, 1 => true, 2 => indiferente
//     // s(Sign)  z(Zero)  h(Halfcarry)  pv(Parityoverflow)  n(AddSubstract)  c(Carry)
//     prueba_flags(&z80, 0, 0, 0, 0, 1, 2);
//
//     // Prueba con (HL) = 0x00
//     // pongo en la direccion 0x10EF el valor 0x07
//     z80.mem.mem[0xEF10] = 0x00;
//     c.bus.write_byte(0xEF10, 0x00);
//
//     // pongo HL = 0xEF10
//     set_hl_test_big(&mut z80, &mut c, 0xEF10);
//     z80.reg.pc = 0;
//     c.reg.pc = 0;
//
//     ejecutar(&mut z80, 0x35, 0, 0, 0, &mut c);
//
//     assert_eq!(z80.mem.mem[0xEF10], 0xFF);
//     assert_eq!(c.bus.read_byte(0xEF10), 0xFF);
// }
// #[test]
// fn ld_0hl0_n() {
//     // 0x36
//     let mut z80 = Z80::default();
//     let mut c = CPU::new(0xFFFF);
//
//     // pongo HL = 0xEF10
//     // z80.registros.h = 0xEF;
//     // z80.registros.l = 0x10;
//     // c.reg.set_hl(0xEF10);
//     set_hl_test_big(&mut z80, &mut c, 0xEF10);
//
//     ejecutar(&mut z80, 0x36, 0x5A, 0, 0, &mut c);
//     // compruebo que ahora que la direccion que marca hl = 05A
//     assert_eq!(z80.mem.mem[0xEF10], 0x5A);
//     assert_eq!(c.bus.read_byte(0xEF10), 0x5A);
// }
//
// #[test]
// fn scf() {
//     // 0x37 C se define N reset, H reset
//     let mut z80 = Z80::default();
//     let mut c = CPU::new(0xFFFF);
//
//     z80.reg.a = 0x9A; // Valor inicial en A que requiere ajuste BCD
//     c.reg.a = 0x9A;
//
//     ejecutar(&mut z80, 0x37, 0, 0, 0, &mut c);
//
//     // 0 => false, 1 => true, 2 => indiferente
//     // s(Sign)  z(Zero)  h(Halfcarry)  pv(Parityoverflow)  n(AddSubstract)  c(Carry)
//     prueba_flags(&z80, 2, 2, 0, 2, 0, 1);
// }
// #[test]
// fn jr_c_d() {
//     // 0x38
//     let mut z80 = Z80::default();
//     let mut c = CPU::new(0xFFFF);
//     let pc_inicial = 0x1234;
//     z80.reg.pc = pc_inicial;
//     c.reg.pc = pc_inicial;
//
//     // Inicializo el flag C en false para que no salte
//     z80.reg.set_flag(&StatusFlag::Carry, false);
//     c.reg.flags.c = false;
//
//     // pongo el desplazamiento a 2 (0x03) (salto de 2 bytes si C es true)
//     ejecutar_en_direccion(&mut z80, 0x1234, 0x38, 0x03, 0, 0, &mut c);
//
//     // compruebo que el program counter no ha cambiado (C es true)
//     assert_eq!(z80.reg.get_pc(), pc_inicial + 2);
//
//     // pongo el flag C en true para que salte
//     z80.reg.set_flag(&StatusFlag::Carry, true);
//     c.reg.flags.c = true;
//
//     // reseteo el program counter
//     z80.reg.pc = pc_inicial;
//     c.reg.pc = pc_inicial;
//
//     z80.es_halted = false;
//     // vuelvo a ejecutar la instruccion
//     z80.step(&mut c);
//
//     // compruebo que el program counter ha saltado 2 bytes
//     assert_eq!(z80.reg.get_pc(), pc_inicial + 5);
//
//     // salto negativo --------------------------------------
//
//     // Inicializo el flag C en true para que salte
//     z80.reg.set_flag(&StatusFlag::Carry, true);
//     c.reg.flags.c = true;
//     z80.reg.pc = pc_inicial;
//     c.reg.pc = pc_inicial;
//
//     // pongo el desplazamiento a -3 (0xFD)
//     ejecutar(&mut z80, 0x38, 0xFD, 0, 0, &mut c);
//
//     // Cálculo del PC esperado con desplazamiento negativo
//     //let expected_pc = (pc_inicial + 2).wrapping_add_signed(z80.memoria.memoria[1] as i8 as i16);
//
//     assert_eq!(z80.reg.pc, 0x1239);
// }
//
// #[test]
// fn add_hl_sp() {
//     // 0x39 SP -> HL     Carry se define, N=0, PV no afectado,  H se define, Z, S no afectados,
//     let mut z80 = Z80::default();
//     let mut c = CPU::new(0xFFFF);
//
//     // pongo HL = 0xB21A
//     // z80.registros.h = 0xB2;
//     // z80.registros.l = 0x1A;
//     // c.reg.set_hl(0xB21A);
//     set_hl_test_big(&mut z80, &mut c, 0xB21A);
//
//     // z80.registros.sp = 0x4503;
//     // c.reg.sp = 0x4503;
//     set_sp_test_big(&mut z80, &mut c, 0x4503);
//
//     ejecutar(&mut z80, 0x39, 0, 0, 0, &mut c);
//
//     // compruebo que ahora hl = 0xF71D
//     //assert_eq!(z80.reg.h, 0xF7);
//     //assert_eq!(z80.reg.l, 0x1D);
//     assert_eq!(get_hl_test_big(&mut z80, &mut c), 0xF71D);
//     // 0 => false, 1 => true, 2 => indiferente
//     // s(Sign)  z(Zero)  h(Halfcarry)  pv(Parityoverflow)  n(AddSubstract)  c(Carry)
//     prueba_flags(&z80, 2, 2, 0, 2, 0, 0);
// }
//
// #[test]
// fn ld_a_0nn0() {
//     // 0x3A
//     let mut z80 = Z80::default();
//     let mut c = CPU::new(0xFFFF);
//
//     // Pongo en la direccion de memoria 0x10EF el valor 0xA5
//     z80.mem.mem[0x10EF] = 0xA5;
//     c.bus.write_byte(0x10EF, 0xA5);
//
//     ejecutar(&mut z80, 0x3A, 0xEF, 0x10, 0, &mut c);
//
//     // compruebo que en A esta el dato 0xA5
//     assert_eq!(z80.reg.a, 0xA5);
// }
//
// #[test]
// fn dec_sp() {
//     // 0x3B
//     let mut z80 = Z80::default();
//     let mut c = CPU::new(0xFFFF);
//
//     // pongo SP = 0xEF10
//     //z80.registros.set_reg16_lit_endian(&SP, 0xEF10);
//     // z80.registros.sp = 0xEF10;
//     // c.reg.sp = 0xEF10;
//     set_sp_test_big(&mut z80, &mut c, 0xEF10);
//
//     ejecutar(&mut z80, 0x3B, 0, 0, 0, &mut c);
//
//     // compruebo que ahora sp=0xEF0F
//     assert_eq!(z80.reg.sp, 0xEF0F);
// }
// #[test]
// fn inc_a() {
//     // 0x3C N reset, P/V detecta overflow, H,Z,S se define
//     let mut z80 = Z80::default();
//     let mut c = CPU::new(0xFFFF);
//
//     // pongo A = 0xFF
//     z80.reg.a = 0xFF;
//     c.reg.a = 0xFF;
//
//     ejecutar(&mut z80, 0x3C, 0, 0, 0, &mut c);
//
//     // compruebo que ahora a = 0x00
//     assert_eq!(z80.reg.a, 0x00);
//
//     // 0 => false, 1 => true, 2 => indiferente
//     // s(Sign)  z(Zero)  h(Halfcarry)  pv(Parityoverflow)  n(AddSubstract)  c(Carry)
//     prueba_flags(&z80, 0, 1, 1, 0, 0, 2);
// }
// #[test]
// fn dec_a() {
//     // 0x3D N set, P/V detecta overflow, H,Z,S se define
//     let mut z80 = Z80::default();
//     let mut c = CPU::new(0xFFFF);
//
//     // pongo a = 0x00
//     z80.reg.a = 0x00;
//     c.reg.a = 0x00;
//
//     ejecutar(&mut z80, 0x3D, 0, 0, 0, &mut c);
//
//     // compruebo que ahora a = 0xFF
//     assert_eq!(z80.reg.a, 0xFF);
//
//     // 0 => false, 1 => true, 2 => indiferente
//     // s(Sign)  z(Zero)  h(Halfcarry)  pv(Parityoverflow)  n(AddSubstract)  c(Carry)
//     prueba_flags(&z80, 1, 0, 1, 0, 1, 2);
// }
//
// #[test]
// fn ld_a_n() {
//     // 0x3E
//     let mut z80 = Z80::default();
//     let mut c = CPU::new(0xFFFF);
//
//     // pongo segundo argumento 0x5A (se debe cargar en A)
//     ejecutar(&mut z80, 0x3E, 0x5A, 0, 0, &mut c);
//
//     // compruebo que ahora a = 0x5A
//     assert_eq!(z80.reg.a, 0x5A);
// }
//
// #[test]
// fn ccf() {
//     // 0x3F Invierte el flag carry, N reset, H se define con el carry previo
//     let mut z80 = Z80::default();
//     let mut c = CPU::new(0xFFFF);
//
//     // Pongo carry a 1
//     z80.reg.set_flag(&StatusFlag::Carry, true);
//     c.reg.flags.c = true;
//
//     // Ejecuto la instruccion
//     ejecutar(&mut z80, 0x3F, 0, 0, 0, &mut c);
//
//     // 0 => false, 1 => true, 2 => indiferente
//     // s(Sign)  z(Zero)  h(Halfcarry)  pv(Parityoverflow)  n(AddSubstract)  c(Carry)
//     prueba_flags(&z80, 2, 2, 1, 2, 0, 0);
// }
// #[test]
// fn ld_b_b() {
//     // 0x40  B->B  y no afecta alos flags ??????
//     let mut z80 = Z80::default();
//     let mut c = CPU::new(0xFFFF);
//
//     // pongo B = 0xCD
//     z80.reg.b = 0xCD;
//     c.reg.b = 0xCD;
//
//     ejecutar(&mut z80, 0x40, 0, 0, 0, &mut c);
//
//     // compruebo que B es 0xCD
//     assert_eq!(z80.reg.b, 0xCD);
// }
// #[test]
// fn ld_b_c() {
//     // 0x41  C->B  y no afecta alos flags
//     let mut z80 = Z80::default();
//     let mut c = CPU::new(0xFFFF);
//
//     // pongo C = 0xCD
//     z80.reg.c = 0xCD;
//     c.reg.c = 0xCD;
//
//     ejecutar(&mut z80, 0x41, 0, 0, 0, &mut c);
//
//     // compruebo que B es 0xCD
//     assert_eq!(z80.reg.b, 0xCD);
// }
// #[test]
// fn ld_b_d() {
//     // 0x42  D->B  y no afecta alos flags
//     let mut z80 = Z80::default();
//     let mut c = CPU::new(0xFFFF);
//
//     // pongo D = 0xCD
//     z80.reg.d = 0xCD;
//     c.reg.d = 0xCD;
//
//     ejecutar(&mut z80, 0x42, 0, 0, 0, &mut c);
//
//     // compruebo que B es 0xCD
//     assert_eq!(z80.reg.b, 0xCD);
// }
// #[test]
// fn ld_b_e() {
//     // 0x43  E->B  y no afecta alos flags
//     let mut z80 = Z80::default();
//     let mut c = CPU::new(0xFFFF);
//
//     // pongo E = 0xCD
//     z80.reg.e = 0xCD;
//     c.reg.e = 0xCD;
//
//     ejecutar(&mut z80, 0x43, 0, 0, 0, &mut c);
//
//     // compruebo que B es 0xCD
//     assert_eq!(z80.reg.b, 0xCD);
// }
// #[test]
// fn ld_b_h() {
//     // 0x44  H->B  y no afecta a los flags
//     let mut z80 = Z80::default();
//     let mut c = CPU::new(0xFFFF);
//
//     // pongo H = 0xCD
//     z80.reg.h = 0xCD;
//     c.reg.h = 0xCD;
//
//     ejecutar(&mut z80, 0x44, 0, 0, 0, &mut c);
//
//     // compruebo que B es 0xCD
//     assert_eq!(z80.reg.b, 0xCD);
// }
// #[test]
// fn ld_b_l() {
//     // 0x45  L->B  y no afecta a los flags
//     let mut z80 = Z80::default();
//     let mut c = CPU::new(0xFFFF);
//
//     // pongo L = 0xCD
//     z80.reg.l = 0xCD;
//     c.reg.l = 0xCD;
//
//     ejecutar(&mut z80, 0x45, 0, 0, 0, &mut c);
//
//     // compruebo que B es 0xCD
//     assert_eq!(z80.reg.b, 0xCD);
// }
// #[test]
// fn ld_b_0hl0() {
//     // 0x46  (HL)->B  y no afecta a los flags
//     let mut z80 = Z80::default();
//     let mut c = CPU::new(0xFFFF);
//
//     // pongo en la direccion 0x10EF el valor 0xCD
//     z80.mem.mem[0xEF10] = 0xCD;
//     c.bus.write_byte(0xEF10, 0xCD);
//
//     // pongo HL = 0xEF10
//     // z80.registros.h = 0xEF;
//     // z80.registros.l = 0x10;
//     // c.reg.set_hl(0xEF10);
//     set_hl_test_big(&mut z80, &mut c, 0xEF10);
//
//     ejecutar(&mut z80, 0x46, 0, 0, 0, &mut c);
//
//     // compruebo que B es 0xCD
//     assert_eq!(z80.reg.b, 0xCD);
// }
//
// #[test]
// fn ld_b_a() {
//     // 0x47  A->B  y no afecta a los flags
//     let mut z80 = Z80::default();
//     let mut c = CPU::new(0xFFFF);
//
//     // pongo A = 0xCD
//     z80.reg.a = 0xCD;
//     c.reg.a = 0xCD;
//
//     ejecutar(&mut z80, 0x47, 0, 0, 0, &mut c);
//
//     // compruebo que B es 0xCD
//     assert_eq!(z80.reg.b, 0xCD);
// }
// #[test]
// fn ld_c_b() {
//     // 0x48  B->C  y no afecta a los flags
//     let mut z80 = Z80::default();
//     let mut c = CPU::new(0xFFFF);
//
//     // pongo B = 0xCD
//     z80.reg.b = 0xCD;
//     c.reg.b = 0xCD;
//
//     ejecutar(&mut z80, 0x48, 0, 0, 0, &mut c);
//
//     // compruebo que C es 0xCD
//     assert_eq!(z80.reg.c, 0xCD);
// }
// #[test]
// fn ld_c_c() {
//     // 0x49  C->C  y no afecta a los flags ?????
//     let mut z80 = Z80::default();
//     let mut c = CPU::new(0xFFFF);
//
//     // pongo C = 0xCD
//     z80.reg.c = 0xCD;
//     c.reg.c = 0xCD;
//
//     ejecutar(&mut z80, 0x49, 0, 0, 0, &mut c);
//
//     // compruebo que C es 0xCD
//     assert_eq!(z80.reg.c, 0xCD);
// }
// #[test]
// fn ld_c_d() {
//     // 0x4A  D->C  y no afecta a los flags
//     let mut z80 = Z80::default();
//     let mut c = CPU::new(0xFFFF);
//
//     // pongo D = 0xCD
//     z80.reg.d = 0xCD;
//     c.reg.d = 0xCD;
//
//     ejecutar(&mut z80, 0x4A, 0, 0, 0, &mut c);
//
//     // compruebo que C es 0xCD
//     assert_eq!(z80.reg.c, 0xCD);
// }
// #[test]
// fn ld_c_e() {
//     // 0x4B  E->C  y no afecta a los flags
//     let mut z80 = Z80::default();
//     let mut c = CPU::new(0xFFFF);
//
//     // pongo E = 0xCD
//     z80.reg.e = 0xCD;
//     c.reg.e = 0xCD;
//
//     ejecutar(&mut z80, 0x4B, 0, 0, 0, &mut c);
//
//     // compruebo que C es 0xCD
//     assert_eq!(z80.reg.c, 0xCD);
// }
// #[test]
// fn ld_c_h() {
//     // 0x4C  H->C  y no afecta a los flags
//     let mut z80 = Z80::default();
//     let mut c = CPU::new(0xFFFF);
//
//     // pongo H = 0xCD
//     z80.reg.h = 0xCD;
//     c.reg.h = 0xCD;
//
//     ejecutar(&mut z80, 0x4C, 0, 0, 0, &mut c);
//
//     // compruebo que C es 0xCD
//     assert_eq!(z80.reg.c, 0xCD);
// }
//
// #[test]
// fn ld_c_l() {
//     // 0x4D  L->C  y no afecta a los flags
//     let mut z80 = Z80::default();
//     let mut c = CPU::new(0xFFFF);
//
//     // pongo L = 0xCD
//     z80.reg.l = 0xCD;
//     c.reg.l = 0xCD;
//
//     ejecutar(&mut z80, 0x4D, 0, 0, 0, &mut c);
//
//     // compruebo que C es 0xCD
//     assert_eq!(z80.reg.c, 0xCD);
// }
//
// #[test]
// fn ld_c_0hl0() {
//     // 0x4E  (HL)->C  y no afecta a los flags
//     let mut z80 = Z80::default();
//     let mut c = CPU::new(0xFFFF);
//
//     // pongo en la direccion 0xEF10 el valor 0xCD
//     z80.mem.mem[0xEF10] = 0xCD;
//     c.bus.write_byte(0xEF10, 0xCD);
//
//     // pongo HL = 0x10EF
//     // z80.registros.h = 0xEF;
//     // z80.registros.l = 0x10;
//     // c.reg.set_hl(0xEF10);
//     set_hl_test_big(&mut z80, &mut c, 0xEF10);
//
//     ejecutar(&mut z80, 0x4E, 0, 0, 0, &mut c);
//
//     // compruebo que C es 0xCD
//     assert_eq!(z80.reg.c, 0xCD);
// }
// #[test]
// fn ld_c_a() {
//     // 0x4F  A->C  y no afecta a los flags
//     let mut z80 = Z80::default();
//     let mut c = CPU::new(0xFFFF);
//
//     // pongo A = 0xCD
//     z80.reg.a = 0xCD;
//     c.reg.a = 0xCD;
//
//     ejecutar(&mut z80, 0x4F, 0, 0, 0, &mut c);
//
//     // compruebo que C es 0xCD
//     assert_eq!(z80.reg.c, 0xCD);
// }
//
// #[test]
// fn ld_d_b() {
//     // 0x50  B->D y no afecta alos flags ??????
//     let mut z80 = Z80::default();
//     let mut c = CPU::new(0xFFFF);
//
//     // pongo B = 0xCD
//     z80.reg.b = 0xCD;
//     c.reg.b = 0xCD;
//
//     ejecutar(&mut z80, 0x50, 0, 0, 0, &mut c);
//
//     // compruebo que D es 0xCD
//     assert_eq!(z80.reg.b, 0xCD);
// }
// #[test]
// fn ld_d_c() {
//     // 0x51  C->D  y no afecta alos flags
//     let mut z80 = Z80::default();
//     let mut c = CPU::new(0xFFFF);
//
//     // pongo C = 0xCD
//     z80.reg.c = 0xCD;
//     c.reg.c = 0xCD;
//
//     ejecutar(&mut z80, 0x51, 0, 0, 0, &mut c);
//
//     // compruebo que D es 0xCD
//     assert_eq!(z80.reg.d, 0xCD);
// }
// #[test]
// fn ld_d_d() {
//     // 0x52  D->d  y no afecta alos flags ??????
//     let mut z80 = Z80::default();
//     let mut c = CPU::new(0xFFFF);
//
//     // pongo D = 0xCD
//     z80.reg.d = 0xCD;
//     c.reg.d = 0xCD;
//
//     ejecutar(&mut z80, 0x52, 0, 0, 0, &mut c);
//
//     // compruebo que D es 0xCD
//     assert_eq!(z80.reg.d, 0xCD);
// }
// #[test]
// fn ld_d_e() {
//     // 0x53  E->D  y no afecta alos flags
//     let mut z80 = Z80::default();
//     let mut c = CPU::new(0xFFFF);
//
//     // pongo E = 0xCD
//     z80.reg.e = 0xCD;
//     c.reg.e = 0xCD;
//
//     ejecutar(&mut z80, 0x53, 0, 0, 0, &mut c);
//
//     // compruebo que D es 0xCD
//     assert_eq!(z80.reg.d, 0xCD);
// }
// #[test]
// fn ld_d_h() {
//     // 0x54  H->D  y no afecta a los flags
//     let mut z80 = Z80::default();
//     let mut c = CPU::new(0xFFFF);
//
//     // pongo H = 0xCD
//     z80.reg.h = 0xCD;
//     c.reg.h = 0xCD;
//
//     ejecutar(&mut z80, 0x54, 0, 0, 0, &mut c);
//
//     // compruebo que D es 0xCD
//     assert_eq!(z80.reg.d, 0xCD);
// }
// #[test]
// fn ld_d_l() {
//     // 0x55  L->D  y no afecta a los flags
//     let mut z80 = Z80::default();
//     let mut c = CPU::new(0xFFFF);
//
//     // pongo L = 0xCD
//     z80.reg.l = 0xCD;
//     c.reg.l = 0xCD;
//
//     ejecutar(&mut z80, 0x55, 0, 0, 0, &mut c);
//
//     // compruebo que D es 0xCD
//     assert_eq!(z80.reg.d, 0xCD);
// }
// #[test]
// fn ld_d_0hl0() {
//     // 0x56  (HL)->D  y no afecta a los flags
//     let mut z80 = Z80::default();
//     let mut c = CPU::new(0xFFFF);
//
//     // pongo en la direccion 0x10EF el valor 0xCD
//     z80.mem.mem[0xEF10] = 0xCD;
//     c.bus.write_byte(0xEF10, 0xCD);
//
//     // pongo HL = 0x10EF
//     // z80.registros.h = 0xEF;
//     // z80.registros.l = 0x10;
//     // c.reg.set_hl(0xEF10);
//     set_hl_test_big(&mut z80, &mut c, 0xEF10);
//
//     ejecutar(&mut z80, 0x56, 0, 0, 0, &mut c);
//
//     // compruebo que D es 0xCD
//     assert_eq!(z80.reg.d, 0xCD);
// }
//
// #[test]
// fn ld_d_a() {
//     // 0x57  A->D  y no afecta a los flags
//     let mut z80 = Z80::default();
//     let mut c = CPU::new(0xFFFF);
//
//     // pongo A = 0xCD
//     z80.reg.a = 0xCD;
//     c.reg.a = 0xCD;
//
//     ejecutar(&mut z80, 0x57, 0, 0, 0, &mut c);
//
//     // compruebo que D es 0xCD
//     assert_eq!(z80.reg.d, 0xCD);
// }
// #[test]
// fn ld_e_b() {
//     // 0x58  B->E  y no afecta a los flags
//     let mut z80 = Z80::default();
//     let mut c = CPU::new(0xFFFF);
//
//     // pongo B = 0xCD
//     z80.reg.b = 0xCD;
//     c.reg.b = 0xCD;
//
//     ejecutar(&mut z80, 0x58, 0, 0, 0, &mut c);
//
//     // compruebo que E es 0xCD
//     assert_eq!(z80.reg.e, 0xCD);
// }
// #[test]
// fn ld_e_c() {
//     // 0x59  C->E  y no afecta a los flags
//     let mut z80 = Z80::default();
//     let mut c = CPU::new(0xFFFF);
//
//     // pongo C = 0xCD
//     z80.reg.c = 0xCD;
//     c.reg.c = 0xCD;
//
//     ejecutar(&mut z80, 0x59, 0, 0, 0, &mut c);
//
//     // compruebo que E es 0xCD
//     assert_eq!(z80.reg.e, 0xCD);
// }
// #[test]
// fn ld_e_d() {
//     // 0x5A  D->E  y no afecta a los flags
//     let mut z80 = Z80::default();
//     let mut c = CPU::new(0xFFFF);
//
//     // pongo D = 0xCD
//     z80.reg.d = 0xCD;
//     c.reg.d = 0xCD;
//
//     ejecutar(&mut z80, 0x5A, 0, 0, 0, &mut c);
//
//     // compruebo que D es 0xCD
//     assert_eq!(z80.reg.d, 0xCD);
// }
// #[test]
// fn ld_e_e() {
//     // 0x5B  E->E  y no afecta a los flags  ?????
//     let mut z80 = Z80::default();
//     let mut c = CPU::new(0xFFFF);
//
//     // pongo E = 0xCD
//     z80.reg.e = 0xCD;
//     c.reg.e = 0xCD;
//
//     ejecutar(&mut z80, 0x5B, 0, 0, 0, &mut c);
//
//     // compruebo que E es 0xCD
//     assert_eq!(z80.reg.e, 0xCD);
// }
// #[test]
// fn ld_e_h() {
//     // 0x5C  H->E  y no afecta a los flags
//     let mut z80 = Z80::default();
//     let mut c = CPU::new(0xFFFF);
//
//     // pongo H = 0xCD
//     z80.reg.h = 0xCD;
//     c.reg.h = 0xCD;
//
//     ejecutar(&mut z80, 0x5C, 0, 0, 0, &mut c);
//
//     // compruebo que E es 0xCD
//     assert_eq!(z80.reg.e, 0xCD);
// }
//
// #[test]
// fn ld_e_l() {
//     // 0x5D  L->E  y no afecta a los flags
//     let mut z80 = Z80::default();
//     let mut c = CPU::new(0xFFFF);
//
//     // pongo L = 0xCD
//     z80.reg.l = 0xCD;
//     c.reg.l = 0xCD;
//
//     ejecutar(&mut z80, 0x5D, 0, 0, 0, &mut c);
//
//     // compruebo que E es 0xCD
//     assert_eq!(z80.reg.e, 0xCD);
// }
//
// #[test]
// fn ld_e_0hl0() {
//     // 0x5E  (HL)->E  y no afecta a los flags
//     let mut z80 = Z80::default();
//     let mut c = CPU::new(0xFFFF);
//
//     // pongo en la direccion 0xEF10 el valor 0xCD
//     z80.mem.mem[0xEF10] = 0xCD;
//     c.bus.write_byte(0xEF10, 0xCD);
//
//     // pongo HL = 0x10EF
//     // z80.registros.h = 0xEF;
//     // z80.registros.l = 0x10;
//     // c.reg.set_hl(0xEF10);
//     set_hl_test_big(&mut z80, &mut c, 0xEF10);
//
//     ejecutar(&mut z80, 0x5E, 0, 0, 0, &mut c);
//
//     // compruebo que E es 0xCD
//     assert_eq!(z80.reg.e, 0xCD);
// }
// #[test]
// fn ld_e_a() {
//     // 0x5F  A->E  y no afecta a los flags
//     let mut z80 = Z80::default();
//     let mut c = CPU::new(0xFFFF);
//
//     // pongo A = 0xCD
//     z80.reg.a = 0xCD;
//     c.reg.a = 0xCD;
//
//     ejecutar(&mut z80, 0x5F, 0, 0, 0, &mut c);
//
//     // compruebo que E es 0xCD
//     assert_eq!(z80.reg.e, 0xCD);
// }
//
// #[test]
// fn ld_h_b() {
//     // 0x60  B->H y no afecta alos flags
//     let mut z80 = Z80::default();
//     let mut c = CPU::new(0xFFFF);
//
//     // pongo B = 0xCD
//     z80.reg.b = 0xCD;
//     c.reg.b = 0xCD;
//
//     ejecutar(&mut z80, 0x60, 0, 0, 0, &mut c);
//
//     // compruebo que H es 0xCD
//     assert_eq!(z80.reg.h, 0xCD);
// }
// #[test]
// fn ld_h_c() {
//     // 0x61  C->H  y no afecta alos flags
//     let mut z80 = Z80::default();
//     let mut c = CPU::new(0xFFFF);
//
//     // pongo C = 0xCD
//     z80.reg.c = 0xCD;
//     c.reg.c = 0xCD;
//
//     ejecutar(&mut z80, 0x61, 0, 0, 0, &mut c);
//
//     // compruebo que H es 0xCD
//     assert_eq!(z80.reg.h, 0xCD);
// }
// #[test]
// fn ld_h_d() {
//     // 0x62  D->H  y no afecta a los flags ??????
//     let mut z80 = Z80::default();
//     let mut c = CPU::new(0xFFFF);
//
//     // pongo D = 0xCD
//     z80.reg.d = 0xCD;
//     c.reg.d = 0xCD;
//
//     ejecutar(&mut z80, 0x62, 0, 0, 0, &mut c);
//
//     // compruebo que H es 0xCD
//     assert_eq!(z80.reg.h, 0xCD);
// }
// #[test]
// fn ld_h_e() {
//     // 0x63  E->D  y no afecta a los flags
//     let mut z80 = Z80::default();
//     let mut c = CPU::new(0xFFFF);
//
//     // pongo E = 0xCD
//     z80.reg.e = 0xCD;
//     c.reg.e = 0xCD;
//
//     ejecutar(&mut z80, 0x63, 0, 0, 0, &mut c);
//
//     // compruebo que H es 0xCD
//     assert_eq!(z80.reg.h, 0xCD);
// }
// #[test]
// fn ld_h_h() {
//     // 0x64  H->H  y no afecta a los flags ??
//     let mut z80 = Z80::default();
//     let mut c = CPU::new(0xFFFF);
//
//     // pongo H = 0xCD
//     z80.reg.h = 0xCD;
//     c.reg.h = 0xCD;
//
//     ejecutar(&mut z80, 0x64, 0, 0, 0, &mut c);
//
//     // compruebo que H es 0xCD
//     assert_eq!(z80.reg.h, 0xCD);
// }
// #[test]
// fn ld_h_l() {
//     // 0x65  L->H  y no afecta a los flags
//     let mut z80 = Z80::default();
//     let mut c = CPU::new(0xFFFF);
//
//     // pongo L = 0xCD
//     z80.reg.l = 0xCD;
//     c.reg.l = 0xCD;
//
//     ejecutar(&mut z80, 0x65, 0, 0, 0, &mut c);
//
//     // compruebo que H es 0xCD
//     assert_eq!(z80.reg.h, 0xCD);
// }
// #[test]
// fn ld_h_0hl0() {
//     // 0x66  (HL)->D  y no afecta a los flags
//     let mut z80 = Z80::default();
//     let mut c = CPU::new(0xFFFF);
//
//     // pongo en la direccion 0xEF10 el valor 0xCD
//     z80.mem.mem[0xEF10] = 0xCD;
//     c.bus.write_byte(0xEF10, 0xCD);
//
//     // pongo HL = 0xEF10
//     // z80.registros.h = 0xEF;
//     // z80.registros.l = 0x10;
//     // c.reg.set_hl(0xEF10);
//     set_hl_test_big(&mut z80, &mut c, 0xEF10);
//
//     ejecutar(&mut z80, 0x66, 0, 0, 0, &mut c);
//
//     // compruebo que H es 0xCD
//     assert_eq!(z80.reg.h, 0xCD);
// }
//
// #[test]
// fn ld_h_a() {
//     // 0x67  A->D  y no afecta a los flags
//     let mut z80 = Z80::default();
//     let mut c = CPU::new(0xFFFF);
//
//     // pongo A = 0xCD
//     z80.reg.a = 0xCD;
//     c.reg.a = 0xCD;
//
//     ejecutar(&mut z80, 0x67, 0, 0, 0, &mut c);
//
//     // compruebo que H es 0xCD
//     assert_eq!(z80.reg.h, 0xCD);
// }
// #[test]
// fn ld_l_b() {
//     // 0x68  B->L  y no afecta a los flags
//     let mut z80 = Z80::default();
//     let mut c = CPU::new(0xFFFF);
//
//     // pongo B = 0xCD
//     z80.reg.b = 0xCD;
//     c.reg.b = 0xCD;
//     ejecutar(&mut z80, 0x68, 0, 0, 0, &mut c);
//
//     // compruebo que L es 0xCD
//     assert_eq!(z80.reg.l, 0xCD);
// }
// #[test]
// fn ld_l_c() {
//     // 0x69  C->L  y no afecta a los flags
//     let mut z80 = Z80::default();
//     let mut c = CPU::new(0xFFFF);
//
//     // pongo C = 0xCD
//     z80.reg.c = 0xCD;
//     c.reg.c = 0xCD;
//
//     ejecutar(&mut z80, 0x69, 0, 0, 0, &mut c);
//
//     // compruebo que L es 0xCD
//     assert_eq!(z80.reg.l, 0xCD);
// }
// #[test]
// fn ld_l_d() {
//     // 0x6A  D->L  y no afecta a los flags
//     let mut z80 = Z80::default();
//     let mut c = CPU::new(0xFFFF);
//
//     // pongo D = 0xCD
//     z80.reg.d = 0xCD;
//     c.reg.d = 0xCD;
//
//     ejecutar(&mut z80, 0x6A, 0, 0, 0, &mut c);
//
//     // compruebo que L es 0xCD
//     assert_eq!(z80.reg.l, 0xCD);
// }
// #[test]
// fn ld_l_e() {
//     // 0x6B  E->L  y no afecta a los flags
//     let mut z80 = Z80::default();
//     let mut c = CPU::new(0xFFFF);
//
//     // pongo E = 0xCD
//     z80.reg.e = 0xCD;
//     c.reg.e = 0xCD;
//
//     ejecutar(&mut z80, 0x6B, 0, 0, 0, &mut c);
//
//     // compruebo que L es 0xCD
//     assert_eq!(z80.reg.l, 0xCD);
// }
// #[test]
// fn ld_l_h() {
//     // 0x6C  H->L  y no afecta a los flags
//     let mut z80 = Z80::default();
//     let mut c = CPU::new(0xFFFF);
//
//     // pongo H = 0xCD
//     z80.reg.h = 0xCD;
//     c.reg.h = 0xCD;
//
//     ejecutar(&mut z80, 0x6C, 0, 0, 0, &mut c);
//
//     // compruebo que L es 0xCD
//     assert_eq!(z80.reg.l, 0xCD);
// }
//
// #[test]
// fn ld_l_l() {
//     // 0x6D  L->L y no afecta a los flags ??????
//     let mut z80 = Z80::default();
//     let mut c = CPU::new(0xFFFF);
//
//     // pongo L = 0xCD
//     z80.reg.l = 0xCD;
//     c.reg.l = 0xCD;
//
//     ejecutar(&mut z80, 0x6D, 0, 0, 0, &mut c);
//
//     // compruebo que L es 0xCD
//     assert_eq!(z80.reg.l, 0xCD);
// }
//
// #[test]
// fn ld_l_0hl0() {
//     // 0x6E  (HL)->L  y no afecta a los flags
//     let mut z80 = Z80::default();
//     let mut c = CPU::new(0xFFFF);
//
//     // pongo en la direccion 0xEF10 el valor 0xCD
//     z80.mem.mem[0xEF10] = 0xCD;
//     c.bus.write_byte(0xEF10, 0xCD);
//
//     // pongo HL = 0xEF10
//     // z80.registros.h = 0xEF;
//     // z80.registros.l = 0x10;
//     // c.reg.set_hl(0xEF10);
//     set_hl_test_big(&mut z80, &mut c, 0xEF10);
//
//     ejecutar(&mut z80, 0x6E, 0, 0, 0, &mut c);
//
//     // compruebo que L es 0xCD
//     assert_eq!(z80.reg.l, 0xCD);
// }
// #[test]
// fn ld_l_a() {
//     // 0x6F  A->L  y no afecta a los flags
//     let mut z80 = Z80::default();
//     let mut c = CPU::new(0xFFFF);
//
//     // pongo A = 0xCD
//     z80.reg.a = 0xCD;
//     c.reg.a = 0xCD;
//
//     ejecutar(&mut z80, 0x6F, 0, 0, 0, &mut c);
//
//     // compruebo que L es 0xCD
//     assert_eq!(z80.reg.l, 0xCD);
// }
// #[test]
// fn ld_0hl0_b() {
//     // 0x70  B->(HL) y no afecta a los flags
//     let mut z80 = Z80::default();
//     let mut c = CPU::new(0xFFFF);
//
//     // pongo HL = 0x10EF
//     // z80.registros.h = 0xEF;
//     // z80.registros.l = 0x10;
//     // c.reg.set_hl(0xEF10);
//     set_hl_test_big(&mut z80, &mut c, 0xEF10);
//
//     // pongo B = 0xCD
//     z80.reg.b = 0xCD;
//     c.reg.b = 0xCD;
//
//     ejecutar(&mut z80, 0x70, 0, 0, 0, &mut c);
//
//     // compruebo que que en la direccion 0x10EF es 0xCD
//     assert_eq!(z80.mem.mem[0xEF10], 0xCD);
// }
// #[test]
// fn ld_0hl0_c() {
//     // 0x71  C->(HL) y no afecta a los flags
//     let mut z80 = Z80::default();
//     let mut c = CPU::new(0xFFFF);
//
//     // pongo HL = 0x10EF
//     // z80.registros.h = 0xEF;
//     // z80.registros.l = 0x10;
//     // c.reg.set_hl(0xEF10);
//     set_hl_test_big(&mut z80, &mut c, 0xEF10);
//
//     // pongo C = 0xCD
//     z80.reg.c = 0xCD;
//     c.reg.c = 0xCD;
//
//     ejecutar(&mut z80, 0x71, 0, 0, 0, &mut c);
//
//     // compruebo que que en la direccion 0x10EF es 0xCD
//     assert_eq!(z80.mem.mem[0xEF10], 0xCD);
// }
// #[test]
// fn ld_0hl0_d() {
//     // 0x72  D->(HL) y no afecta a los flags
//     let mut z80 = Z80::default();
//     let mut c = CPU::new(0xFFFF);
//
//     // pongo HL = 0xEF10
//     // z80.registros.h = 0xEF;
//     // z80.registros.l = 0x10;
//     // c.reg.set_hl(0xEF10);
//     set_hl_test_big(&mut z80, &mut c, 0xEF10);
//
//     // pongo D = 0xCD
//     z80.reg.d = 0xCD;
//     c.reg.d = 0xCD;
//
//     ejecutar(&mut z80, 0x72, 0, 0, 0, &mut c);
//
//     // compruebo que que en la direccion 0x10EF es 0xCD
//     assert_eq!(z80.mem.mem[0xEF10], 0xCD);
// }
// #[test]
// fn ld_0hl0_e() {
//     // 0x73  E->(HL) y no afecta a los flags
//     let mut z80 = Z80::default();
//     let mut c = CPU::new(0xFFFF);
//
//     // pongo HL = 0xEF10
//     // z80.registros.h = 0xEF;
//     // z80.registros.l = 0x10;
//     // c.reg.set_hl(0xEF10);
//     set_hl_test_big(&mut z80, &mut c, 0xEF10);
//
//     // pongo E = 0xCD
//     z80.reg.e = 0xCD;
//     c.reg.e = 0xCD;
//
//     ejecutar(&mut z80, 0x73, 0, 0, 0, &mut c);
//
//     // compruebo que que en la direccion 0x10EF es 0xCD
//     assert_eq!(z80.mem.mem[0xEF10], 0xCD);
// }
// #[test]
// fn ld_0hl0_h() {
//     // 0x74  H->(HL) y no afecta a los flags
//     let mut z80 = Z80::default();
//     let mut c = CPU::new(0xFFFF);
//
//     // pongo HL = 0xEF10
//     // z80.registros.h = 0xEF;
//     // z80.registros.l = 0x10;
//     // c.reg.set_hl(0xEF10);
//     set_hl_test_big(&mut z80, &mut c, 0xEF10);
//
//     ejecutar(&mut z80, 0x74, 0, 0, 0, &mut c);
//
//     // compruebo que que en la direccion 0x10EF es 0xCD
//     assert_eq!(z80.mem.mem[0xEF10], 0xEF);
// }
// #[test]
// fn ld_0hl0_l() {
//     // 0x75  L->(HL) y no afecta a los flags
//     let mut z80 = Z80::default();
//     let mut c = CPU::new(0xFFFF);
//
//     // pongo HL = 0xEF10
//     // z80.registros.h = 0xEF;
//     // z80.registros.l = 0x10;
//     // c.reg.set_hl(0xEF10);
//     set_hl_test_big(&mut z80, &mut c, 0xEF10);
//
//     ejecutar(&mut z80, 0x75, 0, 0, 0, &mut c);
//
//     // compruebo que que en la direccion 0x10EF es 0x10
//     assert_eq!(z80.mem.mem[0xEF10], 0x10);
//     assert_eq!(c.bus.read_byte(0xEF10), 0x10);
// }
// #[test]
// fn halt() {
//     // 0x76 Suspende el funcionamiento de la CPU hasta que se reciba una interrupción o
//     // un reinicio posterior. Mientras está en el estado HALT, el procesador ejecuta operaciones NOP
//     // para mantener la lógica de actualización de la memoria.
//     let z80 = Z80::default();
//
//     assert_eq!(z80.es_halted, true);
// }
//
// #[test]
// fn ld_0hl0_a() {
//     // 0x77  A->(HL) y no afecta a los flags
//     let mut z80 = Z80::default();
//     let mut c = CPU::new(0xFFFF);
//
//     z80.reg.a = 0x1F;
//     c.reg.a = 0x1F;
//
//     // pongo HL = 0xEF10
//     // z80.registros.h = 0xEF;
//     // z80.registros.l = 0x10;
//     // c.reg.set_hl(0xEF10);
//     set_hl_test_big(&mut z80, &mut c, 0xEF10);
//
//     ejecutar(&mut z80, 0x77, 0, 0, 0, &mut c);
//
//     // compruebo que que en la direccion 0x10EF es 0x1F
//     assert_eq!(z80.mem.mem[0xEF10], 0x1F);
// }
// #[test]
// fn ld_a_b() {
//     // 0x78  B->A  y no afecta a los flags
//     let mut z80 = Z80::default();
//     let mut c = CPU::new(0xFFFF);
//
//     // pongo B = 0xCD
//     z80.reg.b = 0xCD;
//     c.reg.b = 0xCD;
//
//     ejecutar(&mut z80, 0x78, 0, 0, 0, &mut c);
//
//     // compruebo que A es 0xCD
//     assert_eq!(z80.reg.a, 0xCD);
// }
// #[test]
// fn ld_a_c() {
//     // 0x79  C->A  y no afecta a los flags
//     let mut z80 = Z80::default();
//     let mut c = CPU::new(0xFFFF);
//
//     // pongo C = 0xCD
//     z80.reg.c = 0xCD;
//     c.reg.c = 0xCD;
//
//     ejecutar(&mut z80, 0x79, 0, 0, 0, &mut c);
//
//     // compruebo que A es 0xCD
//     assert_eq!(z80.reg.a, 0xCD);
// }
// #[test]
// fn ld_a_d() {
//     // 0x7A  D->A  y no afecta a los flags
//     let mut z80 = Z80::default();
//     let mut c = CPU::new(0xFFFF);
//
//     // pongo D = 0xCD
//     z80.reg.d = 0xCD;
//     c.reg.d = 0xCD;
//
//     ejecutar(&mut z80, 0x7A, 0, 0, 0, &mut c);
//
//     // compruebo que A es 0xCD
//     assert_eq!(z80.reg.a, 0xCD);
// }
// #[test]
// fn ld_a_e() {
//     // 0x7B  E->A  y no afecta a los flags
//     let mut z80 = Z80::default();
//     let mut c = CPU::new(0xFFFF);
//
//     // pongo E = 0xCD
//     z80.reg.e = 0xCD;
//     c.reg.e = 0xCD;
//
//     ejecutar(&mut z80, 0x7B, 0, 0, 0, &mut c);
//
//     // compruebo que A es 0xCD
//     assert_eq!(z80.reg.a, 0xCD);
// }
// #[test]
// fn ld_a_h() {
//     // 0x7C  H->A  y no afecta a los flags
//     let mut z80 = Z80::default();
//     let mut c = CPU::new(0xFFFF);
//
//     // pongo H = 0xCD
//     z80.reg.h = 0xCD;
//     c.reg.h = 0xCD;
//
//     ejecutar(&mut z80, 0x7C, 0, 0, 0, &mut c);
//
//     // compruebo que A es 0xCD
//     assert_eq!(z80.reg.a, 0xCD);
// }
//
// #[test]
// fn ld_a_l() {
//     // 0x7D  L->A y no afecta a los flags
//     let mut z80 = Z80::default();
//     let mut c = CPU::new(0xFFFF);
//
//     // pongo L = 0xCD
//     z80.reg.l = 0xCD;
//     c.reg.l = 0xCD;
//
//     ejecutar(&mut z80, 0x7D, 0, 0, 0, &mut c);
//
//     // compruebo que A es 0xCD
//     assert_eq!(z80.reg.a, 0xCD);
// }
//
// #[test]
// fn ld_a_0hl0() {
//     // 0x7E  (HL)->A  y no afecta a los flags
//     let mut z80 = Z80::default();
//     let mut c = CPU::new(0xFFFF);
//
//     // pongo en la direccion 0xEF10 el valor 0xCD
//     z80.mem.mem[0xEF10] = 0xCD;
//     c.bus.write_byte(0xEF10, 0xCD);
//
//     // pongo HL = 0xEF10
//     // z80.registros.h = 0xEF;
//     // z80.registros.l = 0x10;
//     // c.reg.set_hl(0xEF10);
//     set_hl_test_big(&mut z80, &mut c, 0xEF10);
//
//     ejecutar(&mut z80, 0x7E, 0, 0, 0, &mut c);
//
//     // compruebo que A es 0xCD
//     assert_eq!(z80.reg.a, 0xCD);
// }
// #[test]
// fn ld_a_a() {
//     // 0x7F  A->A y no afecta a los flags ???????
//     let mut z80 = Z80::default();
//     let mut c = CPU::new(0xFFFF);
//
//     // pongo A = 0xCD
//     z80.reg.a = 0xCD;
//     c.reg.a = 0xCD;
//
//     ejecutar(&mut z80, 0x7F, 0, 0, 0, &mut c);
//
//     // compruebo que A es 0xCD
//     assert_eq!(z80.reg.a, 0xCD);
// }
//
