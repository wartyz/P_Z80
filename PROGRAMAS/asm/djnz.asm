        LD B,0x03
        SUB A
loop:   INC A
        DJNZ loop
        NOP
