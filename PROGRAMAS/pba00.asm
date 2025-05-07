            ORG    0x0000 ; Inicio del programa

; --- PROGRAMA DE PRUEBA (ejemplo) ---
            LD      A,0x01 ; Código de prueba
            LD      B,0x02 
            ADD     A,B ; A = 0x03

; --- VOLCADO DE REGISTROS (se ejecuta a continuación) ---
            LD      (0xC006),HL 
            PUSH    AF 
            POP     HL 
            LD      (0xC000),HL 
            LD      (0xC002),BC 
            LD      (0xC004),DE 
            LD      (0xC008),SP 
            EXX      
            LD      (0xC010),HL 
            PUSH    AF 
            POP     HL 
            LD      (0xC00A),HL 
            LD      (0xC00C),BC 
            LD      (0xC00E),DE 
            LD      (0xC012),SP 
            HALT     ; Fin del volcado, aqui el ESP32 detecta la señal HALT
