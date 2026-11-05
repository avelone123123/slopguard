.code
PUBLIC sg_asm_rdtsc
sg_asm_rdtsc PROC
    lfence
    rdtsc
    shl rdx, 32
    or rax, rdx
    ret
sg_asm_rdtsc ENDP
PUBLIC sg_asm_fast_hash
sg_asm_fast_hash PROC
    mov rax, 0cbf29ce484222325h
    mov r8, 0100000001b3h
    test rdx, rdx
    jz hash_done
hash_loop:
    movzx r9d, byte ptr [rcx]
    xor rax, r9
    imul rax, r8
    inc rcx
    dec rdx
    jnz hash_loop
hash_done:
    ret
sg_asm_fast_hash ENDP
PUBLIC sg_asm_count_newlines
sg_asm_count_newlines PROC
    xor rax, rax
    test rdx, rdx
    jz nl_done
    mov rax, 1
nl_loop:
    movzx r8d, byte ptr [rcx]
    cmp r8b, 10
    jne nl_next
    inc rax
nl_next:
    inc rcx
    dec rdx
    jnz nl_loop
nl_done:
    ret
sg_asm_count_newlines ENDP
END

