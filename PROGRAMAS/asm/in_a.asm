            LD  SP,0xFF00
            LD  B,0xDE
LOOP:      IN  A,(0x07)
            CP  B
            JP  NZ,LOOP
            RET
