use crate::exceptions::interrupts::InterruptIndex;
use crate::exceptions::interrupts::{
	alignment_check, bound_range_exceeded, breakpoint, coprocessor_not_available,
	coprocessor_segment_overrun, debug, divide_by_zero, double_fault, general_protection_fault,
	invalid_opcode, invalid_task_state_segment, keyboard_interrupt, machine_check, math_fault,
	non_maskable_interrupt, overflow, page_fault, reserved, segment_not_present,
	simd_floating_point_exception, stack_fault, syscall_interrupt, timer_interrupt,
	virtualization_exception,
};
use crate::tools::debug::LogLevel;
use core::arch::asm;

#[derive(Debug, Clone, Copy)]
#[repr(C, packed)]
pub struct IdtDescriptor {
	offset_low: u16,
	selector: u16,
	reserved: u8,
	type_attributes: u8,
	offset_high: u16,
}

macro_rules! create_idt_entry {
	($offset:expr, $selector:expr, $type_attributes:expr) => {
		IdtDescriptor {
			offset_low: ($offset & 0xffff) as u16,
			selector: $selector,
			reserved: 0,
			type_attributes: $type_attributes,
			offset_high: (($offset >> 16) & 0xffff) as u16,
		}
	};
}

static DIVIDE_BY_ZERO: extern "C" fn() = handler!(divide_by_zero);

static DEBUGG: extern "C" fn() = handler!(debug);

static NON_MASKABLE_INTERRUPT: extern "C" fn() = handler!(non_maskable_interrupt);

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

static SIMD_FLOATING_POINT_EXCEPTION: extern "C" fn() = handler!(simd_floating_point_exception);

static VIRTUALIZATION_EXCEPTION: extern "C" fn() = handler!(virtualization_exception);

static TIMER_INTERRUPT: extern "C" fn() = handler!(timer_interrupt);

static KEYBOARD_INTERRUPT: extern "C" fn() = handler!(keyboard_interrupt);

static SYSCALL: extern "C" fn() = handler!(syscall_interrupt);

#[link_section = ".idt"]
static LOW_IDT: [IdtDescriptor; 256] = {
	let idt = [create_idt_entry!(0, 0, 0); 256];
	idt
};

pub static mut IDT: *mut [IdtDescriptor; 256] = core::ptr::null_mut();
#[repr(C, packed)]
struct IdtRegister {
	size: u16,
	offset: u32,
}

unsafe fn fill_idt() {
    unsafe {
        IDT = (&LOW_IDT as *const _ as usize + 0xc0000000) as *mut [IdtDescriptor; 256];
    }
	let idt = unsafe { &mut *IDT };

	idt[0] = create_idt_entry!(DIVIDE_BY_ZERO as u32, 0x08, 0x8e);
	idt[1] = create_idt_entry!(DEBUGG as u32, 0x08, 0x8e);
	idt[2] = create_idt_entry!(NON_MASKABLE_INTERRUPT as u32, 0x08, 0x8e);
	idt[3] = create_idt_entry!(BREAKPOINT as u32, 0x08, 0x8e);
	idt[4] = create_idt_entry!(OVERFLOW as u32, 0x08, 0x8e);
	idt[5] = create_idt_entry!(BOUND_RANGE_EXCEEDED as u32, 0x08, 0x8e);
	idt[6] = create_idt_entry!(INVALID_OPCODE as u32, 0x08, 0x8e);
	idt[7] = create_idt_entry!(COPROCESSOR_NOT_AVAILABLE as u32, 0x08, 0x8e);
	idt[8] = create_idt_entry!(DOUBLE_FAULT as u32, 0x08, 0x8e);
	idt[9] = create_idt_entry!(COPROCESSOR_SEGMENT_OVERRUN as u32, 0x08, 0x8e);
	idt[10] = create_idt_entry!(INVALID_TASK_STATE_SEGMENT as u32, 0x08, 0x8e);
	idt[11] = create_idt_entry!(SEGMENT_NOT_PRESENT as u32, 0x08, 0x8e);
	idt[12] = create_idt_entry!(STACK_FAULT as u32, 0x08, 0x8e);
	idt[13] = create_idt_entry!(GENERAL_PROTECTION_FAULT as u32, 0x08, 0x8e);
	idt[14] = create_idt_entry!(PAGE_FAULT as u32, 0x08, 0x8e);
	idt[15] = create_idt_entry!(RESERVED as u32, 0x08, 0x8e);
	idt[16] = create_idt_entry!(MATH_FAULT as u32, 0x08, 0x8e);
	idt[17] = create_idt_entry!(ALIGNMENT_CHECK as u32, 0x08, 0x8e);
	idt[18] = create_idt_entry!(MACHINE_CHECK as u32, 0x08, 0x8e);
	idt[19] = create_idt_entry!(SIMD_FLOATING_POINT_EXCEPTION as u32, 0x08, 0x8e);
	idt[20] = create_idt_entry!(VIRTUALIZATION_EXCEPTION as u32, 0x08, 0x8e);
	idt[InterruptIndex::Timer.as_usize()] = create_idt_entry!(TIMER_INTERRUPT as u32, 0x08, 0x8e);
	idt[InterruptIndex::Keyboard.as_usize()] =
		create_idt_entry!(KEYBOARD_INTERRUPT as u32, 0x08, 0x8e);
	idt[0x80] = create_idt_entry!(SYSCALL as u32, 0x08, 0xee);
}

pub fn init() {
	unsafe {
		fill_idt();

		let idt_register = IdtRegister {
			size: (core::mem::size_of::<[IdtDescriptor; 256]>() - 1) as u16,
			offset: IDT as u32,
		};

		asm!("lidt [{}]", in(reg) &idt_register, options(readonly, nostack, preserves_flags));

		log!(
			LogLevel::Info,
			"IDT successfully loaded at 0x{:08x}",
			IDT as u32
		);
	}
}