//   datos[0] = Ciclo byte alto
//   datos[1] = Ciclo byte bajo
//   datos[2] = Reloj
//   datos[3] = direccion & 0xFF;         (Dirección baja)
//   datos[4] = (direccion >> 8) & 0xFF;  (Dirección alta)
//   datos[5] = dato en bus de datos;
//   datos[6] = RD_PIN;
//   datos[7] = WR_PIN;
//   datos[8] = MREQ_PIN;
//   datos[9] = RFSH_PIN;
//   datos[10] = RESET_PIN;
//   datos[11] = HALT_PIN;

//  ... (añadir más datos según sea necesario) ...

mod serial;

mod cpu;
mod tests;

use std::{
    io::{self, Read, Write},
    time::Duration,
};

use serialport::{SerialPort, DataBits, FlowControl, Parity, StopBits};
use crate::serial::Serial;

// Estados del sistema
enum SystemState {
    StateIdle,
    StateRunning,
    StateHalted,
}

const ACK_SUCCESS: u8 = 0xDD;
const ACK_ERROR: u8 = 0x00;

const CMD_ECHO: u8 = 0x01;
const CMD_SEND_BYTE: u8 = 0x02;
const CMD_GET_STATUS: u8 = 0x03;
const CMD_WRITE_Z80_MEMORY: u8 = 0x04;
const CMD_RUN_Z80: u8 = 0x05;
const CMD_READ_Z80_MEMORY: u8 = 0x06;
const CMD_RESET_Z80: u8 = 0x07;
const CMD_RESET_SIN_MODIFICAR_Z80: u8 = 0x08;
const CMD_GET_REGISTERS: u8 = 0x09;
const CMD_START_CAPTURE: u8 = 0x0A;
const CMD_STOP_CAPTURE: u8 = 0x0B;
const CMD_QUITAR_WAIT: u8 = 0x0C;
const CMD_PONER_WAIT: u8 = 0x0D;

const SERIAL_PORT_NAME: &str = "/dev/ttyUSB0";

const BAUD_RATE: u32 = 115_200;
const SERIAL_TIMEOUT: Duration = Duration::from_millis(500);

fn main() -> io::Result<()> {
    let mut serial = Serial::new();
    let builder = serialport::new(SERIAL_PORT_NAME, BAUD_RATE)
        .data_bits(DataBits::Eight)
        .flow_control(FlowControl::None)
        .parity(Parity::None)
        .stop_bits(StopBits::One)
        .timeout(SERIAL_TIMEOUT);

    match builder.open() {
        Ok(mut port) => {
            println!("Puerto serie {} abierto a {} bps con timeout de {} ms.",
                     SERIAL_PORT_NAME, BAUD_RATE, SERIAL_TIMEOUT.as_millis());

            loop {
                println!("\n--- Menú ---");
                println!("1. Probar Echo");
                println!("2. Probar enviar byte");
                println!("3. Comprobar STATUS");
                println!("4. Escribir código Z80 desde fichero");
                println!("5. Leer memoria Z80");
                println!("6. Ejecutar código Z80");
                println!("7. Resetear Z80");
                println!("8. Resetear Z80 sin modificar la memoria, ejecutando en 0x0000");
                println!("9. Capturar datos del Z80");
                println!("a. Quitar WAIT");
                println!("b. Poner WAIT");
                println!("c. Ensamblar Z80");

                println!("z. Salir");
                print!("Selecciona una opción: ");
                io::stdout().flush()?;

                let mut choice = String::new();
                io::stdin().read_line(&mut choice)?;
                let choice = choice.trim();

                match choice {
                    "1" => serial.test_cmd_echo(&mut port)?,
                    "2" => serial.enviar_byte(&mut port)?,
                    "3" => serial.obtener_status(&mut port)?,
                    "4" => serial.escribir_fichero_z80_memoria(&mut port)?,
                    "5" => serial.leer_memoria_z80(&mut port)?,
                    "6" => serial.run_z80(&mut port)?,
                    "7" => serial.reset_z80(&mut port)?,
                    "8" => serial.reset_z80_sin_modificar_memoria(&mut port)?,
                    "9" => serial.capture_z80_data(&mut port)?,
                    "a" => serial.quitar_wait(&mut port)?,
                    "b" => serial.poner_wait(&mut port)?,
                    "c" => serial.ensamblar_z80(&mut port)?,

                    "z" => break,
                    _ => println!("Opción inválida."),
                }
            }

            println!("Programa finalizado.");
            Ok(())
        }
        Err(e) => {
            eprintln!("Error al abrir el puerto serie '{}': {}", SERIAL_PORT_NAME, e);
            Err(io::Error::new(io::ErrorKind::Other, e))
        }
    }
}

