use axplat::mem::{pa, phys_to_virt};
use axplat::power::PowerIf;

use crate::config::devices::PMC_PADDR;

struct PowerImpl;

#[impl_plat_interface]
impl PowerIf for PowerImpl {
    /// Bootstraps the given CPU core with the given initial stack (in physical
    /// address).
    ///
    /// Where `cpu_id` is the logical CPU ID (0, 1, ..., N-1, N is the number of
    /// CPU cores on the platform).
    #[cfg(feature = "smp")]
    fn cpu_boot(cpu_id: usize, stack_top_paddr: usize) {
        crate::mp::start_secondary_cpu(cpu_id, pa!(stack_top_paddr));
    }

    /// Shutdown the whole system.
    fn system_off() -> ! {
        info!("Shutting down...");
        unsafe {
            phys_to_virt(pa!(PMC_PADDR + 0x14))
                .as_mut_ptr_of::<u16>()
                .write_volatile(0x3c00)
        };
        axcpu::asm::halt();
        warn!("It should shutdown!");
        loop {
            axcpu::asm::halt();
        }
    }
}
