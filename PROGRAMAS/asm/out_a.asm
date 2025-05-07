        LD  SP,0xFF00
        LD  A,0xBB
        LD  B,0xFF

LOOP:   DEC B
        JP  NZ,LOOP
        OUT (0x07),A
        RET
