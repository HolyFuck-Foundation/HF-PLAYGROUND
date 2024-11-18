Plan:

```nasm
; total memory: r11
; end of allocated memory: r10

cmp r8, r10
jl +2
mov r9w, 10
call grow

lea r8, [r8+10]


ret ; end of program


grow:
	; r8  is the current ptr
	; r10 is the end
	; r11 is the total

	; figure out r8's offset from the beginning of our memory
	; store that in r12

	lea r11, [r11+r9w]
	; CHECK that r11 isn't > some number -> go to death
	; mmap syscall to grow to r11, either by a page or a growth factor
	lea r10, [rax+r11]

	; move r8 to the right offset in the new memory
	ret



death:
	; write syscall "OOM"
	; exit syscall
```
