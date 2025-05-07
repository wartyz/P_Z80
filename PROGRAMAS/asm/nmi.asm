        LD  SP,0xFF00
        LD  A,0x0F
        JP  START

INT:     .ORG 0x0066
        LD  B,A
        RETN

START:   .ORG 0x0070
        EI
LOOP:   CP  B
        JP  NZ,LOOP
        RET
.END
