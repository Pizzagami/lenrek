use crate::exceptions::interrupts::InterruptIndex;
use crate::exceptions::interrupts::{
	alignment_check, bound_range_exceeded, breakpoint, coprocessor_not_available,
	coprocessor_segment_overrun, debug, divide_by_zero, double_fault, general_protection_fault,
	invalid_opcode, invalid_task_state_segment, keybrd_intp, machine_check, math_fault,
	non_maskable_intp, overflow, page_fault, reserved, segment_not_present,
	simd_float_exception, stack_fault, syscall_intp, timer_intp,
	virtualization_exception,
};
use crate::tools::debug::LogLevel;
use core::arch::asm;

#[derive(Debug, Clone, Copy)]
#[repr(C, packed)]
pub struct IdtDesc {
	off_low: u16,
	selector: u16,
	reserved: u8,
	type_attributes: u8,
	off_high: u16,
}

macro_rules! idt_entry {
	($off:expr, $selector:expr, $type_attributes:expr) => {
		IdtDesc {
			off_low: ($off & 0xffff) as u16,
			selector: $selector,
			reserved: 0,
			type_attributes: $type_attributes,
			off_high: (($off >> 16) & 0xffff) as u16,
		}
	};
}

static DIVIDE_BY_ZERO: extern "C" fn() = handler!(divide_by_zero);

static DEBUGG: extern "C" fn() = handler!(debug);

static NON_MASKABLE_INTP: extern "C" fn() = handler!(non_maskable_intp);

static BREAKPOINT: extern "C" fn() = handler!(breakpoint);

static OVERFLOW: extern "C" fn() = handler!(overflow);

static BOUND_RANGE_EXCEEDED: extern "C" fn() = handler!(bound_range_exceeded);

static INVALID_OPCODE: extern "C" fn() = handler!(invalid_opcode);

static COPROCESSOR_NOT_AVAILABLE: extern "C" fn() = handler!(coprocessor_not_available);

static DOUBLE_FAULT: extern "C" fn() = handler!(double_fault);

static COPROCESSOR_SEGMENT_OVERRUN: extern "C" fn() = handler!(coprocessor_segment_overrun);


static INVALID_TASK_STATE_SEGMENT: extern "C" fn() = handler!(invalid_task_state_segment);

static SEGMENT_NOT_PRESENT: extern "C" fn() = handler!(segment_not_present);

static STACK_FAULT: extern "C" fn() = handler!(stack_fault);

static GENERAL_PROTECTION_FAULT: extern "C" fn() = handler!(general_protection_fault);

static PAGE_FAULT: extern "C" fn() = handler_with_error_code!(page_fault);

static RESERVED: extern "C" fn() = handler!(reserved);

static MATH_FAULT: extern "C" fn() = handler!(math_fault);

static ALIGNMENT_CHECK: extern "C" fn() = handler!(alignment_check);

static MACHINE_CHECK: extern "C" fn() = handler!(machine_check);

static SIMD_FLOAT_EXCEPTION: extern "C" fn() = handler!(simd_float_exception);

static VIRTUALIZATION_EXCEPTION: extern "C" fn() = handler!(virtualization_exception);

static TIMER_INTP: extern "C" fn() = handler!(timer_intp);

static KEYBRD_INTP: extern "C" fn() = handler!(keybrd_intp);

static SYSCALL: extern "C" fn() = handler!(syscall_intp);

#[link_section = ".idt"]
static LOW_IDT: [IdtDesc; 256] = {
	let idt = [idt_entry!(0, 0, 0); 256];
	idt
};

pub static mut IDT: *mut [IdtDesc; 256] = core::ptr::null_mut();
#[repr(C, packed)]
struct IdtRegister {
	size: u16,
	off: u32,
}

unsafe fn fill_idt() {
    unsafe {
        IDT = (&LOW_IDT as *const _ as usize + 0xc0000000) as *mut [IdtDesc; 256];
    }
	let idt = unsafe { &mut *IDT };

	idt[0] = idt_entry!(DIVIDE_BY_ZERO as u32, 0x08, 0x8e);
	idt[1] = idt_entry!(DEBUGG as u32, 0x08, 0x8e);
	idt[2] = idt_entry!(NON_MASKABLE_INTP as u32, 0x08, 0x8e);
	idt[3] = idt_entry!(BREAKPOINT as u32, 0x08, 0x8e);
	idt[4] = idt_entry!(OVERFLOW as u32, 0x08, 0x8e);
	idt[5] = idt_entry!(BOUND_RANGE_EXCEEDED as u32, 0x08, 0x8e);
	idt[6] = idt_entry!(INVALID_OPCODE as u32, 0x08, 0x8e);
	idt[7] = idt_entry!(COPROCESSOR_NOT_AVAILABLE as u32, 0x08, 0x8e);
	idt[8] = idt_entry!(DOUBLE_FAULT as u32, 0x08, 0x8e);
	idt[9] = idt_entry!(COPROCESSOR_SEGMENT_OVERRUN as u32, 0x08, 0x8e);
	idt[10] = idt_entry!(INVALID_TASK_STATE_SEGMENT as u32, 0x08, 0x8e);
	idt[11] = idt_entry!(SEGMENT_NOT_PRESENT as u32, 0x08, 0x8e);
	idt[12] = idt_entry!(STACK_FAULT as u32, 0x08, 0x8e);
	idt[13] = idt_entry!(GENERAL_PROTECTION_FAULT as u32, 0x08, 0x8e);
	idt[14] = idt_entry!(PAGE_FAULT as u32, 0x08, 0x8e);
	idt[15] = idt_entry!(RESERVED as u32, 0x08, 0x8e);
	idt[16] = idt_entry!(MATH_FAULT as u32, 0x08, 0x8e);
	idt[17] = idt_entry!(ALIGNMENT_CHECK as u32, 0x08, 0x8e);
	idt[18] = idt_entry!(MACHINE_CHECK as u32, 0x08, 0x8e);
	idt[19] = idt_entry!(SIMD_FLOAT_EXCEPTION as u32, 0x08, 0x8e);
	idt[20] = idt_entry!(VIRTUALIZATION_EXCEPTION as u32, 0x08, 0x8e);
	idt[InterruptIndex::Timer.as_usize()] = idt_entry!(TIMER_INTP as u32, 0x08, 0x8e);
	idt[InterruptIndex::Keyboard.as_usize()] =
		idt_entry!(KEYBRD_INTP as u32, 0x08, 0x8e);
	idt[0x80] = idt_entry!(SYSCALL as u32, 0x08, 0xee);
}

pub fn init() {
	unsafe {
		fill_idt();

		let idt_register = IdtRegister {
			size: (core::mem::size_of::<[IdtDesc; 256]>() - 1) as u16,
			off: IDT as u32,
		};

		asm!("lidt [{}]", in(reg) &idt_register, options(readonly, nostack, preserves_flags));

		log!(
			LogLevel::Info,
			"IDT successfully loaded at 0x{:08x}",
			IDT as u32
		);
	}
}