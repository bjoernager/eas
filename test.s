_start:
	nop ; This is a comment.
	nop @ This is GNU-style comment.
	eor r0, r0, r0
	add r0, r0, #0x1

.string: ; Labels and directives may share names.
	.string "Þið sjáið snæri!"
