// System call convention for x86_64:
// - rax: syscall number
// - rdi, rsi, rdx, r10, r8, r9: arguments
// - rax: return value
// - rcx, r11: clobbered by syscall/sysret

/// System call numbers.
#[repr(u64)]
pub enum Syscall {
    Exit = 0,
    Yield = 1,
    Open = 2,
    Read = 3,
    Write = 4,
}

use x86_64::registers::model_specific::{Efer, EferFlags, Lstar};
use x86_64::VirtAddr;
use core::arch::asm;

#[naked]
unsafe extern "C" fn syscall_handler() {
    asm!(
        "
        push rax
        push rcx
        push rdx
        push rsi
        push rdi
        push r8
        push r9
        push r10
        push r11

        mov rdi, rsp // Arg 1: pointer to stack
        call syscall_dispatcher

        pop r11
        pop r10
        pop r9
        pop r8
        pop rdi
        pop rsi
        pop rdx
        pop rcx
        pop rax

        sysretq
        ",
        options(noreturn)
    );
}

#[no_mangle]
extern "C" fn syscall_dispatcher(stack_frame: *mut u64) {
    // The stack frame is a pointer to the saved registers.
    // We can read the syscall number and arguments from here.
    // The order of registers on the stack is:
    // r11, r10, r9, r8, rdi, rsi, rdx, rcx, rax
    let rax = unsafe { *stack_frame.offset(8) };

    let result = match rax {
        x if x == Syscall::Exit as u64 => {
            crate::println!("Task exited.");
            crate::hlt_loop(); // For now, just halt.
        }
        x if x == Syscall::Yield as u64 => {
            // TODO: Implement task yielding.
            0 // Success
        }
        _ => {
            crate::println!("Unknown or unimplemented syscall {}", rax);
            -1 // Return -1 for unimplemented syscalls
        }
    };

    // Write the return value to rax on the stack.
    unsafe {
        *stack_frame.offset(8) = result as u64;
    }
}

pub fn init() {
    unsafe {
        // Enable the SYSCALL/SYSRET instructions.
        Efer::update(|efer| {
            *efer |= EferFlags::SYSTEM_CALL_EXTENSIONS;
        });

        // Set the syscall handler address.
        Lstar::write(VirtAddr::new(syscall_handler as u64));

        // We also need to set the STAR and SFMASK MSRs, but we'll do that later.
        // For now, this is enough to enable the instruction.
    }
}
