            LD      SP,0xFF00
            LD      A,0x01
            LD      I,A
            LD      A,0x0F
            JP      START

            .ORG    0x0038
            LD      D,A
            RET

START:      .ORG    0x0050
            IM      2
            EI
LOOP:       CP      B
            JP      NZ,LOOP
            RET




            .ORG    0x0106
            LD      B,A
            RET
            .END