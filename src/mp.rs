use axplat::mem::{PhysAddr, phys_to_virt};
use loongArch64::ipi::{csr_mail_send, send_ipi_single};

const ACTION_BOOT_CPU: u32 = 1;

/// Starts the given secondary CPU with its boot stack.
pub fn start_secondary_cpu(cpu_id: usize, stack_top: PhysAddr) {
    let stack_top_virt_addr = phys_to_virt(stack_top).as_usize();
    csr_mail_send(crate::boot::_start_secondary as usize as _, cpu_id, 0);
    csr_mail_send(stack_top_virt_addr as _, cpu_id, 1);
    send_ipi_single(cpu_id, ACTION_BOOT_CPU);
}
