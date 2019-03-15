		ADR		R0, Num
		LDR		R1, [R0]
		ADR		R0, Array
		MOV		R2, #0 ; R2 -> Loop1 Counter
Loop1
		LDRB		R4, [R0, R2] ; R4 -> Initial Min Value
		MOV		R6, R2 ; R6 -> Min Index
		MOV		R3, R2 ; R3 -> Loop2 Counter
		
Loop2
		LDRB		R5, [R0, R3]
		CMP		R4, R5
		MOVGT	R4, R5
		MOVGT	R6, R3
		ADD		R3, R3, #1
		CMP		R3, R1
		BNE		Loop2
		
Swap
		LDRB		R7, [R0, R6]
		LDRB		R8, [R0, R2]
		STRB		R7, [R0, R2]
		STRB		R8, [R0, R6]
		
		ADD		R2, R2, #1
		CMP		R2, R1
		BNE		Loop1
		
Num		DCD		10
Array	DCB		3, 1, 2, 6, 7, 9, 5, 0, 8, 4
		END
