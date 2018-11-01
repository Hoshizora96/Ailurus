#[macro_use]
pub mod util;
pub mod handler;
pub mod idt;

pub fn init_idt() {
    unsafe {
        use self::idt::{lidt, DescriptorTablePointer, Idt, IDT};
        use core::mem::size_of;
        IDT[0].set_handler_fn(handler::divide_by_zero);

        let ptr = DescriptorTablePointer {
            base: &IDT as *const _ as u64,
            limit: (size_of::<Idt>() - 1) as u16,
        };

        lidt(&ptr);
    }
}