#include "p10f200.inc"

    __CONFIG _WDT_OFF & _CP_OFF & _MCLRE_OFF
    ORG 0x0000

   MOVLW  ~(1<<T0CS)      ;Enable GPIO2
   OPTION    
   MOVLW ~(1 << GP1)           ;Set and GP1 as an output
   TRIS GPIO
LOOP: 
   BCF GPIO, GP1          ;Reset GP1 
   BCF GPIO, GP1          ;Reset GP1 
   BCF GPIO, GP1          ;Reset GP1 
   BCF GPIO, GP1          ;Reset GP1 
   BCF GPIO, GP1          ;Reset GP1 
   BSF GPIO, GP1          ;Set GP1
   BCF GPIO, GP1          ;Reset GP1 
   BCF GPIO, GP1          ;Reset GP1 
   BCF GPIO, GP1          ;Reset GP1 
   BCF GPIO, GP1          ;Reset GP1 
   BCF GPIO, GP1          ;Reset GP1 


;   BSF GPIO, GP1          ;Reset GP1 
;   BSF GPIO, GP1          ;Reset GP1 
;   BSF GPIO, GP1          ;Reset GP1 
;   BSF GPIO, GP1          ;Reset GP1 
;   BSF GPIO, GP1          ;Reset GP1 
;   BCF GPIO, GP1          ;Set GP1
;   BSF GPIO, GP1          ;Reset GP1 
;   BSF GPIO, GP1          ;Reset GP1 
;   BSF GPIO, GP1          ;Reset GP1 
;   BSF GPIO, GP1          ;Reset GP1 
;   BSF GPIO, GP1          ;Reset GP1 
   
   GOTO LOOP                   ;loop forever
   GOTO LOOP                   ;loop forever
   
    END
