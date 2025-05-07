use std::{fs::File, io::{prelude::*, self}};

/// La estructura Bus aloja el mapa de memoria Z80.
pub struct Bus {
    espacio_direcc: Vec<u8>,
    espacio_rom: Option<ROMEspacio>,
}

/// Direcciones de inicio y final del área de solo lectura (ROM).
struct ROMEspacio {
    pub inicio: u16,
    pub fin: u16,
}

impl Bus {
    /// Crea una nueva instancia de bus. 'size' será su dirección superior.
    pub fn new(size: u16) -> Bus {
        Bus {
            espacio_direcc: vec![0; (size as usize) + 1],
            espacio_rom: None,
        }
    }

    /// Establece un espacio de ROM. Las operaciones de escritura no serán efectivas en este rango de direcciones.
    /// Ejemplo:
    ///    let mut c = CPU::new(0xFFFF);
    ///    c.bus.set_espacio_rom(0xF000, 0xFFFF);
    pub fn set_espacio_rom(&mut self, start: u16, end: u16) {
        self.espacio_rom = Some(ROMEspacio { inicio: start, fin: end });
    }

    /// Reads a slice of bytes from memory
    pub fn read_mem_slice(&self, start: usize, end: usize) -> Vec<u8> {
        if end > self.espacio_direcc.len() {
            panic!("Read operation after the end of address space !")
        }
        self.espacio_direcc[start..=end].to_vec()
    }

    /// Borra una porción de bytes en la memoria
    pub fn clear_mem_slice(&mut self, start: usize, end: usize) {
        if end > self.espacio_direcc.len() {
            panic!("Write operation after the end of address space !")
        }
        for m in start..=end {
            self.espacio_direcc[m] = 0;
        }
    }

    /// Lee un byte de la memoria
    pub fn leer_byte(&self, direccion: u16) -> u8 {
        if direccion as usize >= self.espacio_direcc.len() {
            return 0;
        }
        self.espacio_direcc[usize::from(direccion)]
    }

    /// Escribe un byte en la memoria
    pub fn escribir_byte(&mut self, direccion: u16, data: u8) {
        if direccion as usize >= self.espacio_direcc.len() {
            return;
        }
        // Si se declara espacio rom y se solicita una operación de escritura en el área rom: salimos
        if self.espacio_rom.is_some()
            && direccion >= self.espacio_rom.as_ref().unwrap().inicio
            && direccion <= self.espacio_rom.as_ref().unwrap().fin
        {
            return;
        };
        self.espacio_direcc[usize::from(direccion)] = data;
    }

    /// Lee una palabra almacenada en la memoria en orden de bytes little endian y
    /// devuelve esta palabra en orden de bytes big endian
    pub fn read_word(&self, direccion: u16) -> u16 {
        if direccion as usize >= self.espacio_direcc.len() {
            return 0;
        }
        u16::from(self.espacio_direcc[usize::from(direccion)])
            | (u16::from(self.espacio_direcc[usize::from(direccion + 1)]) << 8)
    }

    /// Lee una palabra (16 bits) almacenada en memoria en orden de bytes Little Endian y
    /// devuelve esta palabra en orden de bytes Big Endian.
    pub fn read_le_word(&self, direccion: u16) -> u16 {
        if direccion as usize >= self.espacio_direcc.len() {
            return 0;
        }
        u16::from(self.espacio_direcc[usize::from(direccion)]) << 8
            | (u16::from(self.espacio_direcc[usize::from(direccion + 1)]))
    }

    /// Lee una doble palabra (32 bits) almacenada en memoria en orden de bytes Little Endian
    /// y devuelve esta doble palabra en orden de bytes Little Endian.
    pub fn read_le_dword(&self, direccion: u16) -> u32 {
        if direccion as usize >= self.espacio_direcc.len() {
            return 0;
        }
        u32::from(self.espacio_direcc[usize::from(direccion)]) << 24
            | u32::from(self.espacio_direcc[usize::from(direccion + 1)]) << 16
            | u32::from(self.espacio_direcc[usize::from(direccion + 2)]) << 8
            | u32::from(self.espacio_direcc[usize::from(direccion + 3)])
    }

    /// Escribe una palabra (16 bits) en memoria en orden de bytes Little Endian.
    pub fn write_word(&mut self, direccion: u16, data: u16) {
        if direccion as usize >= self.espacio_direcc.len() {
            return;
        }
        // Si el espacio de ROM está declarado y se solicita una operación
        // de escritura en el área de ROM: salimos.
        if self.espacio_rom.is_some()
            && direccion >= self.espacio_rom.as_ref().unwrap().inicio
            && direccion <= self.espacio_rom.as_ref().unwrap().fin
        {
            return;
        };
        self.espacio_direcc[usize::from(direccion)] = (data & 0xFF) as u8;
        self.espacio_direcc[usize::from(direccion + 1)] = (data >> 8) as u8;
    }

    /// Carga datos binarios desde el disco a la memoria en la dirección $0000 + offset.
    /// Devuelve el tamaño del archivo cargado.
    pub fn load_bin(&mut self, file: &str, org: u16) -> io::Result<usize> {
        if org as usize >= self.espacio_direcc.len() {
            panic!("Write operation after the end of address space !")
        }
        let mut f = File::open(file)?;
        let mut buf = Vec::new();
        let s = f.read_to_end(&mut buf)?;
        self.espacio_direcc[org as usize..(buf.len() + org as usize)].clone_from_slice(&buf[..]);
        Ok(s)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn r_le_dword() {
        let mut b = Bus::new(0xFFFF);
        b.escribir_byte(0x0000, 0xCC);
        b.escribir_byte(0x0001, 0xDD);
        b.escribir_byte(0x0002, 0xEE);
        b.escribir_byte(0x0003, 0xFF);
        assert_eq!(b.read_le_dword(0x00), 0xCCDDEEFF);
    }

    #[test]
    fn read_invalid() {
        let mut b = Bus::new(0x7FFF);
        b.escribir_byte(0x8000, 0xFF);
        assert_eq!(b.leer_byte(0x8000), 0);
    }

    #[test]
    fn write_romspace() {
        let mut b = Bus::new(0x7FFF);
        b.escribir_byte(0x0000, 0xFF);
        b.set_espacio_rom(0x0000, 0x000F);
        b.escribir_byte(0x0000, 0x00);
        assert_eq!(b.leer_byte(0x0000), 0xFF);
    }

    #[test]
    fn clear_slice() {
        let mut b = Bus::new(0x000F);
        for m in 0..=15 {
            b.escribir_byte(m, 0xFF);
        }
        assert_eq!(b.leer_byte(0x000F), 0xFF);
        b.clear_mem_slice(0x0000, 0x000F);
        assert_eq!(b.leer_byte(0x000F), 0x00);
    }
}
