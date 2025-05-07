use std::fs::File;
use std::{io, thread};
use std::io::{Read, Write};
use std::io::ErrorKind::TimedOut;
use std::path::Path;
use std::process::{Command, Stdio};
use std::time::{Duration, Instant};
use serialport::SerialPort;
use termion::async_stdin;

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

pub struct Serial {}

impl Serial {
    pub fn new() -> Self { Serial {} }

    /// Envía un archivo binario a la memoria del Z80.
    ///
    /// Esta función lee un archivo binario y lo envía a la memoria del Z80 (memoria de Arduino)
    /// a través del puerto serie. Permite especificar la dirección de inicio
    /// donde se escribirá el programa.
    ///
    /// # Parámetros
    /// - `port`: Referencia mutable al puerto serie
    /// - `bin_file_path`: Ruta al archivo binario a enviar
    ///
    /// # Retorno
    /// - `io::Result<()>`:
    ///   - `Ok(())` si la operación fue exitosa
    ///   - `Err(io::Error)` si ocurrió un error durante la operación
    ///
    /// # Notas
    /// - La dirección de inicio se introduce en formato hexadecimal
    /// - Se envía primero un comando con la dirección y longitud
    /// - Luego se envían los datos del programa
    /// - Se espera un breve tiempo después de enviar los datos
    fn enviar_bin(&mut self, port: &mut Box<dyn SerialPort>, bin_file_path: &str) -> io::Result<()> {
        // Abrir el archivo binario
        let path = Path::new(bin_file_path);
        let mut file = match File::open(path) {
            Ok(f) => f,
            Err(e) => {
                // Si hay error al abrir el archivo, mostrar mensaje y retornar
                eprintln!("Error al abrir el fichero binario '{}': {}", bin_file_path, e);
                return Ok(());
            }
        };

        // Leer el contenido del archivo
        let mut program_data = Vec::new();
        match file.read_to_end(&mut program_data) {
            Ok(size) => {
                // Mostrar el tamaño del archivo leído
                println!("Leídos {} bytes del fichero binario '{}'.", size, bin_file_path);

                // Solicitar la dirección de inicio
                print!("Introduce la dirección de inicio para la escritura (hex): ");
                io::stdout().flush()?;
                let mut start_addr_str = String::new();
                io::stdin().read_line(&mut start_addr_str)?;
                let start_address = u16::from_str_radix(start_addr_str.trim(), 16)
                    .unwrap_or(0x0000);
                let length_to_write: u16 = program_data.len() as u16;

                // Convertir la dirección a número hexadecimal
                let start_address_bytes = start_address.to_be_bytes();
                let length_to_write_bytes = length_to_write.to_be_bytes();

                // Preparar los datos del comando
                let mut data_to_send: Vec<u8> = Vec::new();
                data_to_send.extend_from_slice(&start_address_bytes);
                data_to_send.extend_from_slice(&length_to_write_bytes);

                // Enviar el comando de escritura de memoria
                self.enviar_comando(port, crate::CMD_WRITE_Z80_MEMORY, &data_to_send)?;

                // Enviar los datos del programa
                println!("Enviando datos del programa ({} bytes)...", length_to_write);
                port.write_all(&program_data)?;
                port.flush()?;

                // Mostrar mensaje de confirmación
                println!("Datos del programa enviados.");

                // Esperar un breve tiempo para asegurar que los datos se procesen
                thread::sleep(Duration::from_millis(100));
            }
            Err(e) => eprintln!("Error al leer el fichero binario: {}", e),
        }
        Ok(())
    }

    /// Ensambla código Z80 y opcionalmente lo carga en la memoria.
    ///
    /// Esta función permite ensamblar un archivo de código fuente Z80 y,
    /// opcionalmente, cargar el binario resultante en la memoria del emulador.
    /// Utiliza el ensamblador z80asm para generar el archivo binario.
    ///
    /// # Parámetros
    /// - `port`: Referencia mutable al puerto serie
    ///
    /// # Retorno
    /// - `io::Result<()>`:
    ///   - `Ok(())` si la operación fue exitosa
    ///   - `Err(io::Error)` si ocurrió un error durante la operación
    ///
    /// # Notas
    /// - El archivo fuente debe estar en el directorio "PROGRAMAS"
    /// - Se genera un archivo .bin con el mismo nombre base
    /// - Se puede elegir cargar automáticamente el binario en la memoria
    pub fn ensamblar_z80(&mut self, port: &mut Box<dyn SerialPort>) -> io::Result<()> {
        // Mostrar mensaje inicial
        println!("Ensamblar código Z80...");

        // Solicitar el nombre del archivo fuente
        print!("Introduce el nombre del fichero a ensamblar (sin ruta, en /PROGRAMAS/asm/): ");
        io::stdout().flush()?;
        let mut asm_file_name = String::new();
        io::stdin().read_line(&mut asm_file_name)?;

        // Limpiar el nombre del archivo y construir las rutas
        let asm_file_name = asm_file_name.trim();
        let asm_file_path = format!("PROGRAMAS/asm/{}", asm_file_name);

        // Generar el nombre del archivo binario eliminando la extensión .asm si existe
        let bin_file_path = format!("PROGRAMAS/bin/{}.bin", asm_file_name.trim_end_matches(".asm")); // Genera nombre .bin

        // Mostrar mensaje de inicio del ensamblado
        println!("Ensamblando '{}'...", asm_file_path);

        // Ejecutar el ensamblador z80asm
        let output = Command::new("z80asm")
            .arg(&asm_file_path)     // Archivo fuente
            .arg("-o")               // Opción para especificar el archivo de salida
            .arg(&bin_file_path)     // Archivo binario de salida
            .stdout(Stdio::piped())  // Capturar la salida estándar
            .stderr(Stdio::piped())  // Capturar la salida de error
            .spawn()?;                             // Iniciar el proceso

        // Esperar a que termine el ensamblador y obtener el resultado
        let output_result = output.wait_with_output()?;

        // Verificar si el ensamblado fue exitoso
        if output_result.status.success() {
            println!("Ensamblado exitoso. Fichero binario guardado en '{}'.", bin_file_path);

            // Preguntar si se desea cargar el binario en la memoria
            println!("¿Escribir automáticamente el fichero ensamblado a la memoria Z80? (s/n)");
            io::stdout().flush()?;
            let mut respuesta = String::new();
            io::stdin().read_line(&mut respuesta)?;

            // Si la respuesta es 's', cargar el binario en la memoria
            if respuesta.trim().to_lowercase() == "s" {
                self.enviar_bin(port, &bin_file_path)?;
            }
        } else {
            // Si hubo error, mostrar el mensaje de error del ensamblador
            eprintln!("Error al ensamblar '{}':", asm_file_path);
            eprintln!("{}", String::from_utf8_lossy(&output_result.stderr));
        }

        Ok(())
    }

    /// Envía un comando y sus datos asociados al puerto serie.
    ///
    /// Esta función envía un comando de un byte seguido de hasta 4 bytes de datos
    /// al puerto serie. El comando y los datos se envían en un solo buffer.
    ///
    /// # Parámetros
    /// - `port`: Referencia mutable al puerto serie
    /// - `comando`: Byte que representa el comando a enviar
    /// - `datos`: Slice de bytes con los datos a enviar (máximo 4 bytes)
    ///
    /// # Retorno
    /// - `io::Result<()>`:
    ///   - `Ok(())` si la operación fue exitosa
    ///   - `Err(io::Error)` si ocurrió un error durante la escritura
    ///
    /// # Notas
    /// - Si hay más de 4 bytes en datos, solo se envían los primeros 4
    /// - Los bytes restantes se rellenan con 0
    /// - Se muestra un mensaje con el comando y los datos enviados en hexadecimal
    pub fn enviar_comando(&mut self, port: &mut Box<dyn SerialPort>, comando: u8, datos: &[u8]) -> io::Result<()> {
        let mut buffer = Vec::new();
        buffer.push(comando);
        let mut data_buffer = [0u8; 4];
        for (i, &byte) in datos.iter().enumerate().take(4) {
            data_buffer[i] = byte;
        }
        //dbg_hex!(&data_buffer);
        buffer.extend_from_slice(&data_buffer);
        port.write_all(&buffer)?;
        port.flush()?;
        let data_hex: Vec<String> = data_buffer.iter().map(|b| format!("{:02X}", b)).collect();
        println!("Enviado comando: 0x{:02X} con datos: [{}]", comando, data_hex.join(", "));
        Ok(())
    }

    /// Lee bytes del puerto serie con un tiempo máximo de espera.
    ///
    /// Esta función intenta leer una cantidad específica de bytes del puerto serie
    /// dentro de un tiempo máximo determinado. Si no se pueden leer todos los bytes
    /// en el tiempo especificado, retorna un error de timeout.
    ///
    /// # Parámetros
    /// - `port`: Referencia mutable al puerto serie
    /// - `buffer`: Buffer donde se almacenarán los bytes leídos
    /// - `timeout`: Duración máxima de espera para la lectura
    ///
    /// # Retorno
    /// - `io::Result<()>`:
    ///   - `Ok(())` si se leyeron todos los bytes correctamente
    ///   - `Err(io::Error)` si ocurrió un error durante la lectura
    ///
    /// # Errores
    /// - `UnexpectedEof`: Si la conexión se cierra antes de leer todos los bytes
    /// - `TimedOut`: Si no se pueden leer todos los bytes en el tiempo especificado
    /// - Otros errores de I/O que puedan ocurrir durante la lectura
    pub fn leer_bytes_con_timeout(&mut self, port: &mut Box<dyn SerialPort>, buffer: &mut [u8], timeout: Duration) -> io::Result<()> {
        // Registrar el momento de inicio para controlar el timeout
        let start = Instant::now();
        let buffer_len = buffer.len();
        let mut bytes_read = 0;

        // Intentar leer bytes hasta completar el buffer o alcanzar el timeout
        while bytes_read < buffer_len && start.elapsed() < timeout {
            match port.read(&mut buffer[bytes_read..]) {
                Ok(0) => { // Si se leen 0 bytes
                    return if bytes_read == buffer_len {
                        Ok(()) // Si ya teníamos todos los bytes, retornar éxito
                    } else {
                        // Si faltaban bytes, retornar error de conexión cerrada
                        Err(io::Error::new(io::ErrorKind::UnexpectedEof, "Conexión serie cerrada prematuramente"))
                    };
                }
                // Si se leyeron bytes correctamente
                Ok(n) => {
                    bytes_read += n;
                }
                // Si el puerto está bloqueado temporalmente
                Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => {
                    // Esperar un poco y volver a intentar
                    thread::sleep(Duration::from_millis(5));
                    continue;
                }
                // Para cualquier otro error, retornarlo
                Err(e) => return Err(e),
            }
        }

        // Verificar si se leyeron todos los bytes
        if bytes_read == buffer_len {
            Ok(())
        } else {
            // Si no se leyeron todos los bytes, retornar error de timeout
            Err(io::Error::new(TimedOut, "Timeout al leer la respuesta completa por serie"))
        }
    }

    pub fn quitar_wait(&mut self, port: &mut Box<dyn SerialPort>) -> io::Result<()> {
        println!("Quitando WAIT...");
        self.enviar_comando(port, crate::CMD_QUITAR_WAIT, &[0x00, 0x00, 0x00, 0x00])?;
        thread::sleep(Duration::from_millis(50));
        println!("Comando Quitar Wait enviado.");
        Ok(())
    }
    pub fn poner_wait(&mut self, port: &mut Box<dyn SerialPort>) -> io::Result<()> {
        println!("Poniendo WAIT...");
        self.enviar_comando(port, crate::CMD_PONER_WAIT, &[0x00, 0x00, 0x00, 0x00])?;
        thread::sleep(Duration::from_millis(50));
        println!("Comando Poner Wait enviado.");
        Ok(())
    }
    pub fn capture_z80_data(&mut self, port: &mut Box<dyn SerialPort>) -> io::Result<()> {
        println!("Iniciando captura de datos del Z80. Presiona cualquier tecla para detener.");

        self.enviar_comando(port, crate::CMD_START_CAPTURE, &[0x00, 0x00, 0x00, 0x00])?;

        let mut log_file = File::create("z80_log.txt")?;
        let mut stdin = async_stdin().bytes();
        let mut buffer = [0u8; 11]; // Ahora esperamos 11 bytes

        loop {
            match port.read_exact(&mut buffer) { // Usamos read_exact para asegurar la lectura de n bytes
                Ok(_) => {
                    let ciclo = (buffer[0] as u16) << 8 | buffer[1] as u16;
                    let reloj = buffer[2];
                    let direccion = (buffer[3] as u16) << 8 | buffer[4] as u16;
                    let datos = buffer[5];
                    let rd_pin = buffer[6];
                    let wr_pin = buffer[7];
                    let mreq_pin = buffer[8];
                    let rfsh_pin = buffer[9];
                    let reset_pin = buffer[10];
                    //let halt_pin = buffer[11];

                    let log_entry = format!(
                        "CICLO: {:04X} - RELOJ: {:02X} - DIRECCION: {:04X} - DATOS: {:02X} - RD_PIN: {:02X} -\
                     WR_PIN: {:02X} - MREQ_PIN:{:02X} - RFSH_PIN:{:02X} - RESET_PIN:{:02X} \n",
                        ciclo, reloj, direccion, datos, rd_pin, wr_pin, mreq_pin, rfsh_pin, reset_pin
                    );
                    log_file.write_all(log_entry.as_bytes())?;
                    log_file.flush()?;
                }
                Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => {
                    thread::sleep(Duration::from_millis(10));
                    continue;
                }
                Err(e) => {
                    eprintln!("Error al leer datos del Z80: {}", e);
                    break;
                }
            }

            if let Some(Ok(_)) = stdin.next() {
                println!("Captura de datos detenida.");
                self.enviar_comando(port, crate::CMD_STOP_CAPTURE, &[0x00, 0x00, 0x00, 0x00])?;
                break;
            }
        }

        Ok(())
    }

    pub fn run_z80(&mut self, port: &mut Box<dyn SerialPort>) -> io::Result<()> {
        println!("Ejecutar código Z80...");
        print!("Introduce la dirección de inicio (hex): ");
        io::stdout().flush()?;
        let mut start_addr_str = String::new();
        io::stdin().read_line(&mut start_addr_str)?;
        let start_address = u16::from_str_radix(start_addr_str.trim(), 16).unwrap_or(0x0000);

        let comando = crate::CMD_RUN_Z80;
        let start_address_bytes = start_address.to_be_bytes();
        let data_to_send = &[
            start_address_bytes[0],
            start_address_bytes[1],
            0x00,
            0x00,
        ];

        self.enviar_comando(port, comando, data_to_send)?;
        thread::sleep(Duration::from_millis(50));
        println!("Comando RUN enviado. Z80 en ejecución (se espera).");
        Ok(())
    }

    pub fn reset_z80(&mut self, port: &mut Box<dyn SerialPort>) -> io::Result<()> {
        println!("Reseteando el Z80...");
        self.enviar_comando(port, crate::CMD_RESET_Z80, &[0x00, 0x00, 0x00, 0x00])?;
        thread::sleep(Duration::from_millis(50));
        println!("Comando RESET enviado. Z80 reseteado.");
        Ok(())
    }

    pub fn reset_z80_sin_modificar_memoria(&mut self, port: &mut Box<dyn SerialPort>) -> io::Result<()> {
        println!("Reseteando el Z80...");
        self.enviar_comando(port, crate::CMD_RESET_SIN_MODIFICAR_Z80, &[0x00, 0x00, 0x00, 0x00])?;
        thread::sleep(Duration::from_millis(50));
        println!("Comando RESET SIN MODIFICAR MEMORIA enviado. Z80 reseteado.");
        Ok(())
    }

    pub fn test_cmd_echo(&mut self, port: &mut Box<dyn SerialPort>) -> io::Result<()> {
        println!("Probando CMD_ECHO...");
        let echo_data: u8 = 0xAA;
        self.enviar_comando(port, crate::CMD_ECHO, &[0x00, 0x00, 0x00, echo_data])?;
        let mut echo_response = [0u8; 1];
        if let Err(e) = self.leer_bytes_con_timeout(port, &mut echo_response, Duration::from_secs(2)) {
            eprintln!("Error al leer la respuesta ECHO: {}", e);
        } else {
            println!("Respuesta ECHO (byte sin procesar): 0x{:02X}", echo_response[0]);
            if echo_response[0] == echo_data {
                println!("Prueba CMD_ECHO exitosa!");
            } else {
                println!("Error en CMD_ECHO: Se esperaba 0x{:02X}, se recibió 0x{:02X}", echo_data, echo_response[0]);
            }
        }
        Ok(())
    }

    pub fn enviar_byte(&mut self, port: &mut Box<dyn SerialPort>) -> io::Result<()> {
        println!("Probando CMD_SEND_BYTE...");
        let send_data: u8 = 0xBB;
        self.enviar_comando(port, crate::CMD_SEND_BYTE, &[0x00, 0x00, 0x00, send_data])?;
        thread::sleep(Duration::from_millis(50));
        println!("Comando SEND_BYTE enviado.");
        Ok(())
    }

    pub fn obtener_status(&mut self, port: &mut Box<dyn SerialPort>) -> io::Result<()> {
        println!("Probando CMD_GET_STATUS...");
        self.enviar_comando(port, crate::CMD_GET_STATUS, &[0x00, 0x00, 0x00, 0x00])?;
        let mut status_response = [0u8; 1];
        if let Err(e) = self.leer_bytes_con_timeout(port, &mut status_response, Duration::from_secs(2)) {
            eprintln!("Error al leer el status: {}", e);
        } else {
            println!("Status: 0x{:02X}", status_response[0]);
            match status_response[0] {
                0 => println!("Status: Idle"),
                1 => println!("Status: Running"),
                2 => println!("Status: Wait"),
                3 => println!("Status: Halted"),
                _ => println!("Status: Error"),
            }
        }
        Ok(())
    }

    /// Lee un bloque de memoria del Z80 a través del puerto serie.
    ///
    /// Esta función permite leer un bloque de memoria del Z80 especificando
    /// la dirección de inicio y la longitud a leer. Los datos se muestran
    /// en formato hexadecimal, 16 bytes por línea.
    ///
    /// # Parámetros
    /// - `port`: Referencia mutable al puerto serie
    ///
    /// # Retorno
    /// - `io::Result<()>`: Ok(()) si la operación fue exitosa, o un error si falló
    ///
    /// # Notas
    /// - La dirección de inicio se introduce en formato hexadecimal
    /// - La longitud se introduce en formato decimal
    /// - Se usa un timeout de 10 segundos para la lectura
    /// - Los datos se muestran en formato hexadecimal, 16 bytes por línea
    pub fn leer_memoria_z80(&mut self, port: &mut Box<dyn SerialPort>) -> io::Result<()> {
        // Mostrar mensaje inicial
        println!("LEYENDO MEMORIA...");

        // Solicitar y leer la dirección de inicio
        print!("Introduce la dirección de inicio (hex): ");
        io::stdout().flush()?;
        let mut direc_inicio_str = String::new();

        // Lee la línea introducida por el usuario desde la entrada estándar.
        io::stdin().read_line(&mut direc_inicio_str)?;

        // Convierte el &str de la dirección de inicio (después de eliminar espacios en blanco al principio y al final)
        // a un entero de 16 bits (u16) en base 16 (radix = 16) usando from_str_radix().
        // Si la conversión falla, usa 0x0000 como valor predeterminado.
        let direc_inicio = u16::from_str_radix(direc_inicio_str.trim(), 16)
            .unwrap_or(0x0000);

        // Solicitar y leer la longitud a leer
        print!("Introduce la longitud a leer (dec): ");
        io::stdout().flush()?;
        let mut longitud_str = String::new();
        io::stdin().read_line(&mut longitud_str)?;
        // Convertir la entrada a número decimal, usar 16 si hay error
        let longitud_a_leer = longitud_str.trim().parse::<u16>().unwrap_or(16);

        // Convertir la entrada a número hexadecimal, usar 0x0000 si hay error
        let direc_inicio_bytes = direc_inicio.to_be_bytes();
        let longitud_a_leer_bytes = longitud_a_leer.to_be_bytes();

        // Preparar los datos a enviar al Z80
        let data_to_send = &[
            direc_inicio_bytes[0],    // Byte alto de la dirección
            direc_inicio_bytes[1],    // Byte bajo de la dirección
            longitud_a_leer_bytes[0], // Byte alto de la longitud
            longitud_a_leer_bytes[1], // Byte bajo de la longitud
        ];

        // Enviar el comando de lectura de memoria al Z80
        self.enviar_comando(port, crate::CMD_READ_Z80_MEMORY, data_to_send)?;

        // Crear un buffer para almacenar los datos leídos
        let mut buffer_memoria = vec![0u8; longitud_a_leer as usize];

        // Intentar leer los datos con un timeout de 10 segundos
        if let Err(e) = self.leer_bytes_con_timeout(port, &mut buffer_memoria, Duration::from_secs(10)) {
            // Si hay error, mostrar mensaje
            eprintln!("Error al leer la memoria del Z80: {}", e);
        } else {
            // Si la lectura fue exitosa, mostrar los datos
            println!("Memoria Z80 leída desde 0x{:04X} ({} bytes):", direc_inicio, longitud_a_leer);

            // Mostrar los bytes en formato hexadecimal, 16 por línea
            for (i, byte) in buffer_memoria.iter().enumerate() {
                print!("{:02X} ", byte);
                if (i + 1) % 16 == 0 {
                    println!();
                }
            }
            println!();
        }
        Ok(())
    }

    /// Escribe código Z80 en la memoria del Arduino desde un archivo.
    /// binario. El archivo debe estar en el directorio "PROGRAMAS" y se especifica sin la ruta.
    ///
    /// # Parámetros
    /// - `port`: Referencia mutable al puerto serie
    ///
    /// # Retorno
    /// - `io::Result<()>`:
    ///   - `Ok(())` si la operación fue exitosa
    ///   - `Err(io::Error)` si ocurrió un error durante la operación
    ///
    /// # Notas
    /// - El archivo debe estar en el directorio "PROGRAMAS"
    /// - Se solicita el nombre del archivo sin la ruta
    /// - La función delega la escritura real a la función `enviar_bin`
    pub fn escribir_fichero_z80_memoria(&mut self, port: &mut Box<dyn SerialPort>) -> io::Result<()> {
        // Mostrar mensaje inicial
        println!("Escribir código Z80 desde fichero...");

        // Solicitar el nombre del archivo
        print!("Introduce el nombre del fichero (sin ruta): ");
        io::stdout().flush()?;
        let mut file_name = String::new();
        io::stdin().read_line(&mut file_name)?;

        // Limpiar el nombre del archivo de espacios y saltos de línea
        let file_name = file_name.trim();

        // Construir la ruta completa del archivo
        let file_path = format!("PROGRAMAS/bin/{}", file_name);

        // Enviar el contenido del archivo al Z80
        self.enviar_bin(port, &file_path)
    }
}