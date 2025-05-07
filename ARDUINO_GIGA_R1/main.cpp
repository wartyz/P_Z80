#include <Arduino.h>
#include <SPI.h>
#include <Wire.h>
//#include <U8g2lib.h>
#include "config.h"


//SPIClass hspi(HSPI); // Usar SPI3 (pines alternativos si es necesario)

// Envía dirección y datos al MAX7219
void desplaza(byte direccion, byte datos)
{
  digitalWrite(MAX7219_CHIP_SELECT, LOW);
  shiftOut(MAX7219_DATA_IN, MAX7219_CLOCK, MSBFIRST, direccion);
  shiftOut(MAX7219_DATA_IN, MAX7219_CLOCK, MSBFIRST, datos);
  digitalWrite(MAX7219_CHIP_SELECT, HIGH);
}
void TrigguerReset()
{
  direccion = 0;
  ciclo = 0; // Reiniciar el contador de ciclos
}

void inicializaSieteSeg()
{
  pinMode(MAX7219_DATA_IN, OUTPUT);
  pinMode(MAX7219_CHIP_SELECT, OUTPUT);
  pinMode(MAX7219_CLOCK, OUTPUT);
  digitalWrite(MAX7219_CLOCK, HIGH);
  delay(200);

  // Setup of MAX7219 chip
  desplaza(0x0f, 0x00); // display test register - test mode off
  desplaza(0x0c, 0x01); // shutdown register - normal operation
  desplaza(0x0b, 0x07); // scan limit register - display digits 0 thru 7
  desplaza(0x0a, 0x0f); // intensity register - max brightness
}

void apagaSieteSeg()
{
  desplaza(0x08, 0x00);
  desplaza(0x07, 0x00);
  desplaza(0x06, 0x00);
  desplaza(0x05, 0x00);
  desplaza(0x04, 0x00);
  desplaza(0x03, 0x00);
  desplaza(0x02, 0x00);
  desplaza(0x01, 0x00);
}

// Muestra la dirección (4 dígitos izq), dos dígitos apagados y los datos (2 dígitos dcha)
void presentaSieteSeg(uint16_t direccionZ80, uint8_t datosZ80)
{
  // Dirección: en dígitos 8,7,6,5
  desplaza(0x08, tabla7seg[(direccionZ80 >> 12) & 0x0F]);
  desplaza(0x07, tabla7seg[(direccionZ80 >> 8) & 0x0F]);
  desplaza(0x06, tabla7seg[(direccionZ80 >> 4) & 0x0F]);
  desplaza(0x05, tabla7seg[direccionZ80 & 0x0F]);

  // Dígitos 4 y 3 apagados
  desplaza(0x04, 0x00);
  desplaza(0x03, 0x00);

  // Datos: en dígitos 2,1
  desplaza(0x02, tabla7seg[(datosZ80 >> 4) & 0x0F]);
  desplaza(0x01, tabla7seg[datosZ80 & 0x0F]);
}

void enviar_datos_ciclo()
{
  uint8_t datos[11] = {0}; // Ajustar el tamaño según la cantidad de datos a enviar

  // Ojo recibo direccion en little endian
  datos[0] = (ciclo >> 8) & 0xFF; // Ciclo byte alto
  datos[1] = ciclo & 0xFF;        // Ciclo byte bajo
  // datos[2] = clk_state;                // Reloj
  datos[2] = digitalRead(CLOCK_PIN);
  datos[3] = int_direccion & 0xFF;        // Dirección baja (Little Endian primero)
  datos[4] = (int_direccion >> 8) & 0xFF; // Dirección alta (Little Endian segundo)
  datos[5] = int_databus;                 // Datos
  // datos[6] = digitalRead(RD_PIN);
  // datos[7] = digitalRead(WR_PIN);
  // datos[8] = digitalRead(MREQ_PIN);
  // datos[9] = digitalRead(RFSH_PIN);
  // datos[10] = digitalRead(RESET_PIN);
  datos[6] = int_rd;
  datos[7] = int_wr;
  datos[8] = int_mreq;
  datos[9] = int_rfsh;
  datos[10] = digitalRead(RESET_PIN);

  // ... (añadir más datos según sea necesario) ...

  ciclo++;
  delay(15);
  if ((currentState == STATE_RUNNING) && (digitalRead(RFSH_PIN) == HIGH))
  {
    Serial4.write(datos, sizeof(datos));
  }
}

void ClearRAM()
{
  memset(z80_memoria, 0, sizeof(z80_memoria));
}

// Esperando uso futuro
void ClearIO_Input()
{
  memset(io_input, 0, sizeof(io_input));
}
// Esperando uso futuro
void ClearIO_Output()
{
  memset(io_output, 0, sizeof(io_output));
}

// Esperando uso futuro
void WriteIO_INPUT(unsigned int addr, unsigned int data)
{
  io_input[addr] = data;
}

// Esperando uso futuro
void WriteIO_OUTPUT(unsigned int addr, unsigned int data)
{
  io_output[addr] = data;
}

// Devuelve el valor del bus de direcciones
uint16_t leerBusDireccion()
{
  uint16_t dir = 0;

  noInterrupts(); // Deshabilitar interrupciones
  for (int i = 0; i < 16; i++)
  {
    dir |= (digitalRead(AddrBus16[i]) << i);
  }

  interrupts(); // Habilitar interrupciones
  return dir;
}

// escribe en variable dato el valor del dato que dice el bus de direcciones
void leerDatoEnDireccion()
{
  dato = z80_memoria[leerBusDireccion()];
  interrupts(); // Habilitar interrupciones
}

// escribe en la variable dato el valor del bus de datos
void leerBusDatos()
{
  for (int i = 0; i < 8; i++)
  {
    dato |= (digitalRead(DataBus8[i]) << i);
  }
}

// Escribe dato en el bus de datos
void escribeBusDatos(uint8_t dat)
{
  for (int i = 0; i < 8; i++)
  {
    digitalWrite(DataBus8[i], (dat >> i) & 1);
  }
  Serial.println("escribeBusDatos");
  char dato_str[20];

  snprintf(dato_str, sizeof(dato_str), "Dato = 0x%02X", dat);
  Serial.println(dato_str);
}

// Obtiene todos los datos necesarios con la subida del reloj
void TrigguerClock()
{
  statePba = !statePba;
}

void manejar_comando()
{
  // Recibimos los parámetros en Little Endian
  if (Serial4.available() >= 5)
  { // Esperar 5 bytes (comando + 4 de datos)
    uint8_t comando = Serial4.read();
    uint8_t data_bytes[4];
    for (int i = 0; i < 4; i++)
    {
      data_bytes[i] = Serial4.read();
    }

    char buff0[150];
    snprintf(buff0, sizeof(buff0), "data_bytes[0] = 0x%02X  data_bytes[1] = 0x%02X data_bytes[2] = 0x%02X  data_bytes[3] = 0x%02X",
             data_bytes[0], data_bytes[1], data_bytes[2], data_bytes[3]);
    Serial.println(buff0);

    switch (comando)
    {
    case CMD_ECHO:
    {
      Serial4.write(data_bytes[3]); // El último byte es el dato a hacer eco
      // Serial.write(data_bytes[3]);
      Serial.println("ECHOOOOOOOOOOOOOOOOOOOOOOOOO");
      break;
    }
    case CMD_SEND_BYTE:
    {
      // Recibido byte: data_bytes[3]
      break;
    }
    case CMD_GET_STATUS:
    {
      Serial4.write(currentState);
      break;
    }
    case CMD_READ_Z80_MEMORY:
    {
      uint16_t start_address = (data_bytes[0] << 8) | data_bytes[1];
      uint16_t length_to_read = (data_bytes[2] << 8) | data_bytes[3];
      // Serial.printf("CMD_READ_Z80_MEMORY: Dirección Inicio = 0x%04X, Longitud = %u\n", start_address, length_to_read);

      if (start_address + length_to_read <= 65536)
      {
        for (uint16_t i = 0; i < length_to_read; i++)
        {
          uint8_t byte_leido = z80_memoria[start_address + i];
          Serial4.write(byte_leido);
        }
      }
      break;
    }
    case CMD_WRITE_Z80_MEMORY:
    {
      // AHH ALL DHH DLL  AHH direccion alta, ALL Direccion baja,  DHH num bytes alto,  DLL num bytes bajo
      uint16_t start_address = (data_bytes[0] << 8) | data_bytes[1];
      uint16_t length_to_write = (data_bytes[2] << 8) | data_bytes[3];

      if (start_address + length_to_write <= 65536 && length_to_write > 0)
      {
        // Espera activa hasta que lleguen todos los bytes
        while (Serial4.available() < length_to_write)
        {
          // Hacemos un pequeño delay para evitar bloquear el sistema completamente
          delay(1);
        }

        for (uint16_t i = 0; i < length_to_write; i++)
        {
          z80_memoria[start_address + i] = Serial4.read();
        }

        Serial.println("CMD_WRITE_Z80_MEMORY completado");
      }
      else
      {
        Serial.println("CMD_WRITE_Z80_MEMORY error: rango inválido");
      }

      break;
    }

    case CMD_RUN_Z80:
    {
      // Poner WAIT en alto para que se pueda ya ejecutar
      // digitalWrite(WAIT_PIN, HIGH);

      uint16_t start_address_run = (data_bytes[0] << 8) | data_bytes[1];
      execution_start_address = start_address_run;
      currentState = STATE_RUNNING;
      break;
    }

    case CMD_RESET_Z80:
    {
      TrigguerReset();
      ClearRAM();
      currentState = STATE_IDLE;
      break;
    }
    case CMD_RESET_SIN_MODIFICAR_Z80:
    {

      // Se ejecutará el código que habia en memoria apartir de 0x0000
      TrigguerReset();
      currentState = STATE_RUNNING;
      break;
    }
    case CMD_START_CAPTURE:
    {
      // capturar = true;
      enviar_datos_ciclo();

      break;
    }
    case CMD_STOP_CAPTURE:
    {
      capturar = false;
      break;
    }
    case CMD_QUITAR_WAIT:
    {
      digitalWrite(WAIT_PIN, HIGH);
      Serial.println("CMD_QUITAR_WAIT");
      break;
    }
    case CMD_PONER_WAIT:
    {
      digitalWrite(WAIT_PIN, LOW);
      Serial.println("CMD_PONER_WAIT");
      break;
    }
    default:
    {
      break;
    }
    }
  }
}

void inicializar_hardware()
{
  // Definicion de los tipos de pines
  pinMode(RESET_PIN, OUTPUT);
  pinMode(CLOCK_PIN, INPUT);

  // No se si usar INPUT_PULLUP ???
  pinMode(RD_PIN, INPUT);
  pinMode(WR_PIN, INPUT_PULLUP);
  pinMode(MREQ_PIN, INPUT);
  pinMode(RFSH_PIN, INPUT);
  // pinMode(IOREQ_PIN, INPUT);
  pinMode(HALT_PIN, INPUT);
  pinMode(PBAS_PIN, OUTPUT);

  // Bus de direcciones siempre será de entrada
  for (int i = 0; i < 16; i++)
    pinMode(AddrBus16[i], INPUT); // A0-A15
  // Bus de datos por ahora de salida (puede cambiar)
  for (int i = 0; i < 8; i++)
    pinMode(DataBus8[i], OUTPUT); // D0-D7

  TrigguerReset(); // Resetea y ejecuta WAIT() hasta nueva orden
  inicializaSieteSeg();
  ClearRAM();
}

void TrigguerRunLeerMemoria()
{

  direccion = leerBusDireccion();

  for (int i = 0; i < 8; i++)
    pinMode(DataBus8[i], OUTPUT);
  escribeBusDatos(z80_memoria[direccion]);
}

void ejecutar_z80()
{
  if (currentState == STATE_RUNNING)
  {
    if ((digitalRead(MREQ_PIN) == LOW) && (digitalRead(RFSH_PIN) == HIGH))
    {
      if (digitalRead(RD_PIN) == LOW)
      {
        for (int i = 0; i < 8; i++)
          pinMode(DataBus8[i], OUTPUT); // Asegurar salida para lectura

        Serial.println("Z80 LEYENDO EN MEMORIA");
        leerDatoEnDireccion();
        escribeBusDatos(dato);
        delayMicroseconds(10);
        //vTaskDelay();

      }
      else if (digitalRead(WR_PIN) == LOW)
      {
        for (int i = 0; i < 8; i++)
          pinMode(DataBus8[i], INPUT); // Asegurar entrada para escritura
        Serial.println("Z80 ESCRIBIENDO EN MEMORIA");

        leerBusDatos();
        z80_memoria[leerBusDireccion()] = dato;
        delayMicroseconds(10);
      }
    }

    delayMicroseconds(10); // Control de la velocidad del reloj (ajusta según necesidad)
  }
}

void presentaDebug()
{

  if (digitalRead(MREQ_PIN) == LOW)
  {
    if (digitalRead(RD_PIN) == LOW)
    {
      direccion = leerBusDireccion(); // Ahora direccion tiene lo que hay en bus de direcciones
      leerDatoEnDireccion();          // Ahora la variable dato tiene lo que hay en la direccion

      char dato_str[20];

      snprintf(dato_str, sizeof(dato_str), "Dato = 0x%02X", dato);
      Serial.println(dato_str);
      Serial.println("Estado Read() ");

      // char addr_str0[40];
      // snprintf(addr_str0, sizeof(addr_str0), "digitalRead(RFSH_PIN) = %d", digitalRead(RFSH_PIN));
      // Serial.println(addr_str0); // Print the value to use the variable

      // char addr_str1[20]; // Buffer para la dirección formateada
      //  uint16_t direccion = leerDireccion();

      // snprintf(addr_str1, sizeof(addr_str1), "Direccion = 0x%04X", direccion); // Formatea la dirección
      // Serial.println(addr_str1);

      apagaSieteSeg();
      presentaSieteSeg(direccion, dato);
      delay(20);
    }

    if (digitalRead(WR_PIN) == LOW && digitalRead(RFSH_PIN) == HIGH)
    {
      char dato_str[20];

      snprintf(dato_str, sizeof(dato_str), "Dato = 0x%02X", dato);
      Serial.println(dato_str);
      Serial.println("Estado Write() ");

      // char addr_str0[40];
      // snprintf(addr_str0, sizeof(addr_str0), "digitalRead(RFSH_PIN) = %d", digitalRead(RFSH_PIN));
      // Serial.println(addr_str0); // Print the value to use the variable

      char addr_str1[20]; // Buffer para la dirección formateada

      snprintf(addr_str1, sizeof(addr_str1), "Direccion = 0x%04X", direccion); // Formatea la dirección
      Serial.println(addr_str1);

      // apagaSieteSeg();
      // presentaSieteSeg(direccion, dato);
      delay(20);
    }
  }
}



void setup()
{
  // Inicialización de periféricos

  digitalWrite(WAIT_PIN, LOW);
  Serial.begin(SERIAL_BAUD_RATE_DEBUG); // Puerto USB para debug y carga de programas
  Serial4.begin(SERIAL_BAUD_RATE_RUST); // Puerto serie UART para comandos del PC
  delay(500);                           // Esperar a que el puerto serie se conecte. Solo es necesario para el puerto USB nativo.

  //probarLecturaDirectaPuertos();

  Serial.println("Iniciando comunicación serie a RUST " + String(SERIAL_BAUD_RATE_RUST) + " bps.");
  Serial.println("Iniciando comunicación serie a DEBUG " + String(SERIAL_BAUD_RATE_DEBUG) + " bps.");

  Wire.begin();

  inicializar_hardware();

  attachInterrupt(digitalPinToInterrupt(CLOCK_PIN), TrigguerClock, LOW);
  // attachInterrupt(digitalPinToInterrupt(RD_PIN), TrigguerRunLeerMemoria, FALLING);
  // attachInterrupt(digitalPinToInterrupt(WR_PIN), TrigguerRunEscribirMemoria, FALLING);
  // attachInterrupt(digitalPinToInterrupt(RESET_PIN), TrigguerReset, FALLING);

  //  ClearRAM();

  TrigguerReset();
  Serial.println("Pasado setup()");
}

void loop()
{
  //probarLecturaDirectaPuertos();
  manejar_comando();

  presentaDebug();

  bool bandera = statePba;
  while (statePba == bandera)
  { // No ha cambiado...esperar
    delayMicroseconds(10);
  }

  ejecutar_z80();
}
