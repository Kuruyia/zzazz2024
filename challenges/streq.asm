SECTION "WRAM0", WRAM0[$C000]
wPasswordToCheck:
    ds $800
wEqualities:
    ds $800

SECTION "WRAM1", WRAMX[$D000]
wStack:
    ; D000
    ds $100
    ; D100
wStackEnd:

SECTION "ROM", ROM0[$00FE]
    ; Make sure crashes don't reset.
    ld b, b
    rst $38
    ; This is where execution starts.
    di
    jp EntryPoint
    ds $150 - @, 0

EntryPoint:
    ; Clear HRAM
    xor a
    ld c, 16
    ld hl, $FF80
.clearHRAM
    ld [hli], a
    dec c
    jr nz, .clearHRAM
    ; Set up the stack
    ld sp, wStackEnd
    ; Clear all memory
    call ClearWRAM
.copyValidPassword
    ; Copy the valid password to the WRAM
    ld hl, .validPassword
    ld de, wPasswordToCheck
    call _StrCpy_
    ; Prepare the equality tests
    ld hl, wEqualities
    xor c
.equals
    ; Push the current index of the equalities
    push hl
    ; Set the second character of the password to check to the byte to check
    ld hl, wPasswordToCheck
    add hl, 2
    ld [hl], c
    ; Check the password against the valid one
    ld de, wPasswordToCheck
    ld hl, .validPassword
    call _StrEq_
    ; Write a byte depending on if both strings are equals
    ld b, 0
    jr c, .wrongPW
.correctPW
    ld b, 1
.wrongPW
    ; Pop the current index of the equalities
    pop hl
    ; Write the result of the current iteration
    ld [hl], b
    ; Check the next equality
    inc hl
    inc c
    jr nz, .equals
.infiniteLoop
    jr .infiniteLoop
.validPassword
    db "p=Pa$$W0rD123", 0

; Clears all WRAM with 0, excluding stack
ClearWRAM:
    ld bc, wStack - $c000
    ld hl, $c000
.clear
    xor a
    ld [hli], a
    dec bc
    ld a, c
    or b
    jr nz, .clear
    ret
    
; Copy string HL to DE.
_StrCpy_:
    ld a, [hli]
    ld [de], a
    inc de
    and a
    ret z
    jr _StrCpy_

; Test if strings at HL and DE are equal.
; Set carry if not equal, unset carry if equal
_StrEq_:
    push bc
.test
    ld b, [hl]
    ld a, [de]
    cp b
    jr z, .eq
    jr .resultNeq
.chk
    inc hl
    inc de
    jr .test
.eq
    and a
    jr z, .resultEq
    jr .chk
.resultEq
    pop bc
    or a
    ret
.resultNeq
    pop bc
    scf
    ret