#pragma once

#define MAX7219_CS 10    // Pin Chip Select para el MAX7219
#define MAX7219_MOSI 51  // Pin MOSI (Master Out Slave In) para la comunicación SPI con el MAX7219
#define MAX7219_CLK 52   // Pin CLK (Clock) para la comunicación SPI con el MAX7219


// Bus Z80
const uint8_t AddrBus16[] = {53,51,49,47,45,43,41,39,37,35,33,31,29,27,25,23};
const uint8_t DataBus8[] = {36,34,32,30,28,26,24,22};

// Configuración MAX7219 (SPI nativo)
const uint32_t SPI_CLK = 8000000;

// Tabla hexadecimal para display de 7 segmentos (cátodo común)
const byte tabla7seg[16] = {
    0b01111110, // 0
    0b00110000, // 1
    0b01101101, // 2
    0b01111001, // 3
    0b00110011, // 4
    0b01011011, // 5
    0b01011111, // 6
    0b01110000, // 7
    0b01111111, // 8
    0b01111011, // 9
    0b01110111, // A
    0b00011111, // b (minúscula)
    0b01001110, // C
    0b00111101, // d (minúscula)
    0b01001111, // E
    0b01000111  // F
};
#define MAX7219_DATA_IN 12      // Pin Data In para el MAX7219
#define MAX7219_CHIP_SELECT 10  // Alias para el pin Chip Select del MAX7219
#define MAX7219_CLOCK 13        // Pin Clock para el MAX7219




// Pines de control del Z80
#define CLOCK_PIN 38
#define RESET_PIN 42
#define RD_PIN 48
#define WR_PIN 46
#define MREQ_PIN 52
// #define IOREQ_PIN 26
#define HALT_PIN 50
#define RFSH_PIN 40
#define WAIT_PIN 44
#define PBAS_PIN 3


uint8_t dato = 0; // Variable donde se guarda el valor leído del bus de datos
int i = 0;
uint16_t direccion = 0; // Variable para almacenar la dirección leída del bus de direcciones

// Constantes comunicación con PC
const uint8_t CMD_ECHO = 0x01;             // Prueba de comunicación básica
const uint8_t CMD_SEND_BYTE = 0x02;        // Envío de un byte de prueba
const uint8_t CMD_GET_STATUS = 0x03;       // Obtener un estado genérico
const uint8_t CMD_WRITE_Z80_MEMORY = 0x04; // Escribir un bloque en la memoria Z80
const uint8_t CMD_RUN_Z80 = 0x05;          // Iniciar la ejecución del Z80
const uint8_t CMD_READ_Z80_MEMORY = 0x06;  // Leer un bloque de la memoria Z80
const uint8_t CMD_RESET_Z80 = 0x07;        // Resetear el Z80
const uint8_t CMD_RESET_SIN_MODIFICAR_Z80 = 0x08;
const uint8_t CMD_GET_REGISTERS = 0x09; // (Futuro) Obtener el estado de registros
const uint8_t CMD_START_CAPTURE = 0x0A;
const uint8_t CMD_STOP_CAPTURE = 0x0B;
const uint8_t CMD_QUITAR_WAIT = 0x0C;
const uint8_t CMD_PONER_WAIT = 0x0D;

const uint8_t ACK_SUCCESS = 0xDD; // Confirmación no lo uso por ahora
const uint8_t ACK_ERROR = 0x00;   // Indicador de error no lo uso por ahora

// Constantes de velocidad de comunicación serie
const uint32_t SERIAL_BAUD_RATE_RUST = 115200;
const uint32_t SERIAL_BAUD_RATE_DEBUG = 1000000;

// Estados del sistema
typedef enum
{
  STATE_IDLE,
  STATE_RUNNING,
  STATE_HALTED
} SystemState;

// Variables globales
volatile SystemState currentState = STATE_IDLE;       // Estado actual del sistema
uint16_t execution_start_address = 0x00; // Dirección de memoria donde se iniciará la ejecución del Z80
bool capturar = false;                   // Flag para indicar si se deben capturar los ciclos del Z80 (no utilizado completamente)
volatile uint16_t ciclo = 0;              // Contador de ciclos del Z80

// Memoria RAM del Z80 (64KB) inicializada a cero
uint8_t z80_memoria[65536] = {0};

// Arreglos para el estado de los puertos de E/S (entrada y salida)
uint8_t io_input[10] = {0};
uint8_t io_output[10] = {0};

volatile bool clock_tick = false; // Flag volátil para indicar la ocurrencia de un flanco de reloj (para uso en interrupciones)
volatile uint16_t int_direccion = 0; // Variable volátil para almacenar la dirección capturada por la interrupción
volatile uint8_t int_databus = 0;  // Variable volátil para almacenar el dato capturado por la interrupción
volatile bool int_rd = HIGH;      // Variable volátil para almacenar el estado de la señal /RD capturada por la interrupción
volatile bool int_wr = HIGH;      // Variable volátil para almacenar el estado de la señal /WR capturada por la interrupción
volatile bool int_mreq = HIGH;    // Variable volátil para almacenar el estado de la señal /MREQ capturada por la interrupción
volatile bool int_rfsh = HIGH;    // Variable volátil para almacenar el estado de la señal /RFSH capturada por la interrupción
volatile byte statePba = LOW;    // Variable volátil para el experimento de trigger de reloj
