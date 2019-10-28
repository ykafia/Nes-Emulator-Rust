LDA #$FF  ;Load the hex value $c0 into the A register
TAX       ;Transfer the value in the A register to X
DEX       ;Increment the value in the X register
SBC #$c4  ;Add the hex value $c4 to the A register
BRK       ;Break - we're done