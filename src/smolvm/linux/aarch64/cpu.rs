#![allow(dead_code)]
#![allow(non_upper_case_globals)]
#![allow(non_snake_case)]
#![allow(unused_imports)]

//! Copied from arch/arm64/include/uapi/asm/ptrace.h

/**
 * PSR bits
 */
pub const PSR_MODE_EL0t: u32 = 0x00000000;
pub const PSR_MODE_EL1t: u32 = 0x00000004;
pub const PSR_MODE_EL1h: u32 = 0x00000005;
pub const PSR_MODE_EL2t: u32 = 0x00000008;
pub const PSR_MODE_EL2h: u32 = 0x00000009;
pub const PSR_MODE_EL3t: u32 = 0x0000000c;
pub const PSR_MODE_EL3h: u32 = 0x0000000d;
pub const PSR_MODE_MASK: u32 = 0x0000000f;

/* AArch32 CPSR bits */
pub const PSR_MODE32_BIT: u32 = 0x00000010;

/* AArch64 SPSR bits */
pub const PSR_F_BIT: u32 = 0x00000040;
pub const PSR_I_BIT: u32 = 0x00000080;
pub const PSR_A_BIT: u32 = 0x00000100;
pub const PSR_D_BIT: u32 = 0x00000200;
pub const PSR_BTYPE_MASK: u32 = 0x00000c00;
pub const PSR_SSBS_BIT: u32 = 0x00001000;
pub const PSR_PAN_BIT: u32 = 0x00400000;
pub const PSR_UAO_BIT: u32 = 0x00800000;
pub const PSR_DIT_BIT: u32 = 0x01000000;
pub const PSR_TCO_BIT: u32 = 0x02000000;
pub const PSR_V_BIT: u32 = 0x10000000;
pub const PSR_C_BIT: u32 = 0x20000000;
pub const PSR_Z_BIT: u32 = 0x40000000;
pub const PSR_N_BIT: u32 = 0x80000000;

pub const PSR_BTYPE_SHIFT: u32 = 10;

/*
 * Groups of PSR bits
 */
pub const PSR_f: u32 = 0xff000000; /* Flags		*/
pub const PSR_s: u32 = 0x00ff0000; /* Status		*/
pub const PSR_x: u32 = 0x0000ff00; /* Extension		*/
pub const PSR_c: u32 = 0x000000ff; /* Control		*/

/* Convenience names for the values of PSTATE.BTYPE */
pub const PSR_BTYPE_NONE: u32 = 0b00 << PSR_BTYPE_SHIFT;
pub const PSR_BTYPE_JC: u32 = 0b01 << PSR_BTYPE_SHIFT;
pub const PSR_BTYPE_C: u32 = 0b10 << PSR_BTYPE_SHIFT;
pub const PSR_BTYPE_J: u32 = 0b11 << PSR_BTYPE_SHIFT;

pub const REG_ARM_COPROC_SHIFT: u64 = 16;

// Normal registers are mapped as coprocessor 16
pub const REG_ARM_CORE: u64 = 0x0010 << REG_ARM_COPROC_SHIFT;
pub const REG_ARM64_SYSREG: u64 = 0x0013 << REG_ARM_COPROC_SHIFT;

pub const REG_ARM64: u64 = 0x6000000000000000;
pub const REG_ARM64_CORE_BASE: u64 = REG_ARM64 | REG_ARM_CORE;
pub const REG_ARM64_SYSREG_BASE: u64 = REG_ARM64 | REG_ARM64_SYSREG;

pub const REG_SIZE_U8: u64 = 0x0000000000000000;
pub const REG_SIZE_U16: u64 = 0x0010000000000000;
pub const REG_SIZE_U32: u64 = 0x0020000000000000;
pub const REG_SIZE_U64: u64 = 0x0030000000000000;
pub const REG_SIZE_U128: u64 = 0x0040000000000000;
pub const REG_SIZE_U256: u64 = 0x0050000000000000;
pub const REG_SIZE_U512: u64 = 0x0060000000000000;
pub const REG_SIZE_U1024: u64 = 0x0070000000000000;
pub const REG_SIZE_U2048: u64 = 0x0080000000000000;

// https://developer.arm.com/documentation/ddi0595/2021-09/AArch64-Registers/SCTLR-EL1--System-Control-Register--EL1-?lang=en
const SCTLR_RESERVED_MUST_BE_1: u64 = (3 << 28) | (3 << 22) | (1 << 20) | (1 << 11);
const SCTLR_EE_LITTLE_ENDIAN: u64 = 0 << 25;
const SCTLR_EOE_LITTLE_ENDIAN: u64 = 0 << 24;
const SCTLR_TRAP_WFE: u64 = 1 << 18;
const SCTLR_TRAP_WFI: u64 = 1 << 16;
const SCTLR_I_CACHE_DISABLED: u64 = 0 << 12;
const SCTLR_EXCEPTION_EXIT_CONTEXT_SYNC: u64 = 1 << 11;
const SCTLR_STACK_ALIGNMENT_EL0: u64 = 1 << 4;
const SCTLR_STACK_ALIGNMENT: u64 = 1 << 3;
const SCTLR_D_CACHE_DISABLED: u64 = 0 << 2;
const SCTLR_MMU_DISABLED: u64 = 0 << 0;
const SCTLR_MMU_ENABLED: u64 = 1 << 0;

pub const SPSR_INITIAL_VALUE: u64 =
    (PSR_D_BIT | PSR_A_BIT | PSR_I_BIT | PSR_F_BIT | PSR_MODE_EL1h) as u64;

pub const SCTLR_INITIAL_VALUE: u64 = SCTLR_RESERVED_MUST_BE_1
    | SCTLR_EE_LITTLE_ENDIAN
    | SCTLR_TRAP_WFE
    | SCTLR_TRAP_WFI
    | SCTLR_EXCEPTION_EXIT_CONTEXT_SYNC
    | SCTLR_I_CACHE_DISABLED
    | SCTLR_D_CACHE_DISABLED
    | SCTLR_STACK_ALIGNMENT
    | SCTLR_STACK_ALIGNMENT_EL0
    | SCTLR_MMU_DISABLED;

// https://developer.arm.com/documentation/ddi0595/2021-09/AArch64-Registers/MIDR-EL1--Main-ID-Register?lang=en
pub const MIDR_EL1_INITIAL_VALUE: u64 = 0x00000000410fd034;

const OP0_SHIFT: u8 = 19;
const OP0_MASK: u8 = 0x3;
const OP1_SHIFT: u8 = 16;
const OP1_MASK: u8 = 0x7;
const CRN_SHIFT: u8 = 12;
const CRN_MASK: u8 = 0xf;
const CRM_SHIFT: u8 = 8;
const CRM_MASK: u8 = 0xf;
const OP2_SHIFT: u8 = 5;
const OP2_MASK: u8 = 0x7;

// linux/arch/arm64/include/asm/sysreg.h
const fn sys_reg(op0: u8, op1: u8, crn: u8, crm: u8, op2: u8) -> u64 {
    (op0 as u64) << OP0_SHIFT
        | (op1 as u64) << OP1_SHIFT
        | (crn as u64) << CRN_SHIFT
        | (crm as u64) << CRM_SHIFT
        | (op2 as u64) << OP2_SHIFT
}

pub const SYS_MIDR_EL1: u64 = sys_reg(3, 0, 0, 0, 0);
pub const SYS_MPIDR_EL1: u64 = sys_reg(3, 0, 0, 0, 5);
pub const SYS_SCTLR_EL1: u64 = sys_reg(3, 0, 1, 0, 0);
pub const SYS_TTBR0_EL1: u64 = sys_reg(3, 0, 2, 0, 0);
pub const SYS_TTBR1_EL1: u64 = sys_reg(3, 0, 2, 0, 1);
pub const SYS_ESR_EL1: u64 = sys_reg(3, 0, 5, 2, 0);
pub const SYS_SPSR_EL1: u64 = sys_reg(3, 0, 4, 0, 0);
pub const SYS_MAIR_EL1: u64 = sys_reg(3, 0, 10, 2, 0);
