lorom

org $007FD7 : db $0A ; db $09 | expand rom size to 1MB

; load level data changes
org $00C1CE : lda #$10 ; lda #$02 | load wave data from bank 10 instead of 02
org $00C1D7 : lda.w $8000 ; lda.w $B5F0 | change level data offset
org $00C1DC : lda.w $8001 ; lda.w $B5F1 | ^
