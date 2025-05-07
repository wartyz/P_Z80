            .ORG    0 
            LD      SP,0xFF00 
            LD      A,0x0F 
            JP      start 

            .ORG    0x0018 
            LD      C,A 
            RET      

            .ORG    0x0038 
            LD      B,A 
            RET      

START:      .ORG    0x0050 
            IM      1 
            EI       
LOOP:       CP      B 
            JP      NZ,loop 
            RET      
            .END     

