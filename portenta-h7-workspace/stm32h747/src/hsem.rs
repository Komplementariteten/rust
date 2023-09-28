#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - HSEM register HSEM_R0 HSEM_R31"]
    pub hsem_r0: HSEM_R0,
    #[doc = "0x04 - HSEM register HSEM_R0 HSEM_R31"]
    pub hsem_r1: HSEM_R1,
    #[doc = "0x08 - HSEM register HSEM_R0 HSEM_R31"]
    pub hsem_r2: HSEM_R2,
    #[doc = "0x0c - HSEM register HSEM_R0 HSEM_R31"]
    pub hsem_r3: HSEM_R3,
    #[doc = "0x10 - HSEM register HSEM_R0 HSEM_R31"]
    pub hsem_r4: HSEM_R4,
    #[doc = "0x14 - HSEM register HSEM_R0 HSEM_R31"]
    pub hsem_r5: HSEM_R5,
    #[doc = "0x18 - HSEM register HSEM_R0 HSEM_R31"]
    pub hsem_r6: HSEM_R6,
    #[doc = "0x1c - HSEM register HSEM_R0 HSEM_R31"]
    pub hsem_r7: HSEM_R7,
    #[doc = "0x20 - HSEM register HSEM_R0 HSEM_R31"]
    pub hsem_r8: HSEM_R8,
    #[doc = "0x24 - HSEM register HSEM_R0 HSEM_R31"]
    pub hsem_r9: HSEM_R9,
    #[doc = "0x28 - HSEM register HSEM_R0 HSEM_R31"]
    pub hsem_r10: HSEM_R10,
    #[doc = "0x2c - HSEM register HSEM_R0 HSEM_R31"]
    pub hsem_r11: HSEM_R11,
    #[doc = "0x30 - HSEM register HSEM_R0 HSEM_R31"]
    pub hsem_r12: HSEM_R12,
    #[doc = "0x34 - HSEM register HSEM_R0 HSEM_R31"]
    pub hsem_r13: HSEM_R13,
    #[doc = "0x38 - HSEM register HSEM_R0 HSEM_R31"]
    pub hsem_r14: HSEM_R14,
    #[doc = "0x3c - HSEM register HSEM_R0 HSEM_R31"]
    pub hsem_r15: HSEM_R15,
    #[doc = "0x40 - HSEM register HSEM_R0 HSEM_R31"]
    pub hsem_r16: HSEM_R16,
    #[doc = "0x44 - HSEM register HSEM_R0 HSEM_R31"]
    pub hsem_r17: HSEM_R17,
    #[doc = "0x48 - HSEM register HSEM_R0 HSEM_R31"]
    pub hsem_r18: HSEM_R18,
    #[doc = "0x4c - HSEM register HSEM_R0 HSEM_R31"]
    pub hsem_r19: HSEM_R19,
    #[doc = "0x50 - HSEM register HSEM_R0 HSEM_R31"]
    pub hsem_r20: HSEM_R20,
    #[doc = "0x54 - HSEM register HSEM_R0 HSEM_R31"]
    pub hsem_r21: HSEM_R21,
    #[doc = "0x58 - HSEM register HSEM_R0 HSEM_R31"]
    pub hsem_r22: HSEM_R22,
    #[doc = "0x5c - HSEM register HSEM_R0 HSEM_R31"]
    pub hsem_r23: HSEM_R23,
    #[doc = "0x60 - HSEM register HSEM_R0 HSEM_R31"]
    pub hsem_r24: HSEM_R24,
    #[doc = "0x64 - HSEM register HSEM_R0 HSEM_R31"]
    pub hsem_r25: HSEM_R25,
    #[doc = "0x68 - HSEM register HSEM_R0 HSEM_R31"]
    pub hsem_r26: HSEM_R26,
    #[doc = "0x6c - HSEM register HSEM_R0 HSEM_R31"]
    pub hsem_r27: HSEM_R27,
    #[doc = "0x70 - HSEM register HSEM_R0 HSEM_R31"]
    pub hsem_r28: HSEM_R28,
    #[doc = "0x74 - HSEM register HSEM_R0 HSEM_R31"]
    pub hsem_r29: HSEM_R29,
    #[doc = "0x78 - HSEM register HSEM_R0 HSEM_R31"]
    pub hsem_r30: HSEM_R30,
    #[doc = "0x7c - HSEM register HSEM_R0 HSEM_R31"]
    pub hsem_r31: HSEM_R31,
    #[doc = "0x80 - HSEM Read lock register"]
    pub hsem_rlr0: HSEM_RLR0,
    #[doc = "0x84 - HSEM Read lock register"]
    pub hsem_rlr1: HSEM_RLR1,
    #[doc = "0x88 - HSEM Read lock register"]
    pub hsem_rlr2: HSEM_RLR2,
    #[doc = "0x8c - HSEM Read lock register"]
    pub hsem_rlr3: HSEM_RLR3,
    #[doc = "0x90 - HSEM Read lock register"]
    pub hsem_rlr4: HSEM_RLR4,
    #[doc = "0x94 - HSEM Read lock register"]
    pub hsem_rlr5: HSEM_RLR5,
    #[doc = "0x98 - HSEM Read lock register"]
    pub hsem_rlr6: HSEM_RLR6,
    #[doc = "0x9c - HSEM Read lock register"]
    pub hsem_rlr7: HSEM_RLR7,
    #[doc = "0xa0 - HSEM Read lock register"]
    pub hsem_rlr8: HSEM_RLR8,
    #[doc = "0xa4 - HSEM Read lock register"]
    pub hsem_rlr9: HSEM_RLR9,
    #[doc = "0xa8 - HSEM Read lock register"]
    pub hsem_rlr10: HSEM_RLR10,
    #[doc = "0xac - HSEM Read lock register"]
    pub hsem_rlr11: HSEM_RLR11,
    #[doc = "0xb0 - HSEM Read lock register"]
    pub hsem_rlr12: HSEM_RLR12,
    #[doc = "0xb4 - HSEM Read lock register"]
    pub hsem_rlr13: HSEM_RLR13,
    #[doc = "0xb8 - HSEM Read lock register"]
    pub hsem_rlr14: HSEM_RLR14,
    #[doc = "0xbc - HSEM Read lock register"]
    pub hsem_rlr15: HSEM_RLR15,
    #[doc = "0xc0 - HSEM Read lock register"]
    pub hsem_rlr16: HSEM_RLR16,
    #[doc = "0xc4 - HSEM Read lock register"]
    pub hsem_rlr17: HSEM_RLR17,
    #[doc = "0xc8 - HSEM Read lock register"]
    pub hsem_rlr18: HSEM_RLR18,
    #[doc = "0xcc - HSEM Read lock register"]
    pub hsem_rlr19: HSEM_RLR19,
    #[doc = "0xd0 - HSEM Read lock register"]
    pub hsem_rlr20: HSEM_RLR20,
    #[doc = "0xd4 - HSEM Read lock register"]
    pub hsem_rlr21: HSEM_RLR21,
    #[doc = "0xd8 - HSEM Read lock register"]
    pub hsem_rlr22: HSEM_RLR22,
    #[doc = "0xdc - HSEM Read lock register"]
    pub hsem_rlr23: HSEM_RLR23,
    #[doc = "0xe0 - HSEM Read lock register"]
    pub hsem_rlr24: HSEM_RLR24,
    #[doc = "0xe4 - HSEM Read lock register"]
    pub hsem_rlr25: HSEM_RLR25,
    #[doc = "0xe8 - HSEM Read lock register"]
    pub hsem_rlr26: HSEM_RLR26,
    #[doc = "0xec - HSEM Read lock register"]
    pub hsem_rlr27: HSEM_RLR27,
    #[doc = "0xf0 - HSEM Read lock register"]
    pub hsem_rlr28: HSEM_RLR28,
    #[doc = "0xf4 - HSEM Read lock register"]
    pub hsem_rlr29: HSEM_RLR29,
    #[doc = "0xf8 - HSEM Read lock register"]
    pub hsem_rlr30: HSEM_RLR30,
    #[doc = "0xfc - HSEM Read lock register"]
    pub hsem_rlr31: HSEM_RLR31,
    #[doc = "0x100 - HSEM Interrupt enable register"]
    pub hsem_c1ier: HSEM_C1IER,
    #[doc = "0x104 - HSEM Interrupt clear register"]
    pub hsem_c1icr: HSEM_C1ICR,
    #[doc = "0x108 - HSEM Interrupt status register"]
    pub hsem_c1isr: HSEM_C1ISR,
    #[doc = "0x10c - HSEM Masked interrupt status register"]
    pub hsem_c1misr: HSEM_C1MISR,
    #[doc = "0x110 - HSEM Interrupt enable register"]
    pub hsem_c2ier: HSEM_C2IER,
    #[doc = "0x114 - HSEM Interrupt clear register"]
    pub hsem_c2icr: HSEM_C2ICR,
    #[doc = "0x118 - HSEM Interrupt status register"]
    pub hsem_c2isr: HSEM_C2ISR,
    #[doc = "0x11c - HSEM Masked interrupt status register"]
    pub hsem_c2misr: HSEM_C2MISR,
    _reserved72: [u8; 0x20],
    #[doc = "0x140 - HSEM Clear register"]
    pub hsem_cr: HSEM_CR,
    #[doc = "0x144 - HSEM Interrupt clear register"]
    pub hsem_keyr: HSEM_KEYR,
}
#[doc = "HSEM_R0 (rw) register accessor: HSEM register HSEM_R0 HSEM_R31\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hsem_r0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hsem_r0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`hsem_r0`]
module"]
pub type HSEM_R0 = crate::Reg<hsem_r0::HSEM_R0_SPEC>;
#[doc = "HSEM register HSEM_R0 HSEM_R31"]
pub mod hsem_r0;
#[doc = "HSEM_R1 (rw) register accessor: HSEM register HSEM_R0 HSEM_R31\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hsem_r1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hsem_r1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`hsem_r1`]
module"]
pub type HSEM_R1 = crate::Reg<hsem_r1::HSEM_R1_SPEC>;
#[doc = "HSEM register HSEM_R0 HSEM_R31"]
pub mod hsem_r1;
#[doc = "HSEM_R2 (rw) register accessor: HSEM register HSEM_R0 HSEM_R31\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hsem_r2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hsem_r2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`hsem_r2`]
module"]
pub type HSEM_R2 = crate::Reg<hsem_r2::HSEM_R2_SPEC>;
#[doc = "HSEM register HSEM_R0 HSEM_R31"]
pub mod hsem_r2;
#[doc = "HSEM_R3 (rw) register accessor: HSEM register HSEM_R0 HSEM_R31\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hsem_r3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hsem_r3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`hsem_r3`]
module"]
pub type HSEM_R3 = crate::Reg<hsem_r3::HSEM_R3_SPEC>;
#[doc = "HSEM register HSEM_R0 HSEM_R31"]
pub mod hsem_r3;
#[doc = "HSEM_R4 (rw) register accessor: HSEM register HSEM_R0 HSEM_R31\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hsem_r4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hsem_r4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`hsem_r4`]
module"]
pub type HSEM_R4 = crate::Reg<hsem_r4::HSEM_R4_SPEC>;
#[doc = "HSEM register HSEM_R0 HSEM_R31"]
pub mod hsem_r4;
#[doc = "HSEM_R5 (rw) register accessor: HSEM register HSEM_R0 HSEM_R31\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hsem_r5::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hsem_r5::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`hsem_r5`]
module"]
pub type HSEM_R5 = crate::Reg<hsem_r5::HSEM_R5_SPEC>;
#[doc = "HSEM register HSEM_R0 HSEM_R31"]
pub mod hsem_r5;
#[doc = "HSEM_R6 (rw) register accessor: HSEM register HSEM_R0 HSEM_R31\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hsem_r6::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hsem_r6::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`hsem_r6`]
module"]
pub type HSEM_R6 = crate::Reg<hsem_r6::HSEM_R6_SPEC>;
#[doc = "HSEM register HSEM_R0 HSEM_R31"]
pub mod hsem_r6;
#[doc = "HSEM_R7 (rw) register accessor: HSEM register HSEM_R0 HSEM_R31\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hsem_r7::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hsem_r7::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`hsem_r7`]
module"]
pub type HSEM_R7 = crate::Reg<hsem_r7::HSEM_R7_SPEC>;
#[doc = "HSEM register HSEM_R0 HSEM_R31"]
pub mod hsem_r7;
#[doc = "HSEM_R8 (rw) register accessor: HSEM register HSEM_R0 HSEM_R31\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hsem_r8::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hsem_r8::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`hsem_r8`]
module"]
pub type HSEM_R8 = crate::Reg<hsem_r8::HSEM_R8_SPEC>;
#[doc = "HSEM register HSEM_R0 HSEM_R31"]
pub mod hsem_r8;
#[doc = "HSEM_R9 (rw) register accessor: HSEM register HSEM_R0 HSEM_R31\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hsem_r9::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hsem_r9::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`hsem_r9`]
module"]
pub type HSEM_R9 = crate::Reg<hsem_r9::HSEM_R9_SPEC>;
#[doc = "HSEM register HSEM_R0 HSEM_R31"]
pub mod hsem_r9;
#[doc = "HSEM_R10 (rw) register accessor: HSEM register HSEM_R0 HSEM_R31\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hsem_r10::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hsem_r10::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`hsem_r10`]
module"]
pub type HSEM_R10 = crate::Reg<hsem_r10::HSEM_R10_SPEC>;
#[doc = "HSEM register HSEM_R0 HSEM_R31"]
pub mod hsem_r10;
#[doc = "HSEM_R11 (rw) register accessor: HSEM register HSEM_R0 HSEM_R31\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hsem_r11::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hsem_r11::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`hsem_r11`]
module"]
pub type HSEM_R11 = crate::Reg<hsem_r11::HSEM_R11_SPEC>;
#[doc = "HSEM register HSEM_R0 HSEM_R31"]
pub mod hsem_r11;
#[doc = "HSEM_R12 (rw) register accessor: HSEM register HSEM_R0 HSEM_R31\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hsem_r12::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hsem_r12::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`hsem_r12`]
module"]
pub type HSEM_R12 = crate::Reg<hsem_r12::HSEM_R12_SPEC>;
#[doc = "HSEM register HSEM_R0 HSEM_R31"]
pub mod hsem_r12;
#[doc = "HSEM_R13 (rw) register accessor: HSEM register HSEM_R0 HSEM_R31\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hsem_r13::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hsem_r13::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`hsem_r13`]
module"]
pub type HSEM_R13 = crate::Reg<hsem_r13::HSEM_R13_SPEC>;
#[doc = "HSEM register HSEM_R0 HSEM_R31"]
pub mod hsem_r13;
#[doc = "HSEM_R14 (rw) register accessor: HSEM register HSEM_R0 HSEM_R31\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hsem_r14::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hsem_r14::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`hsem_r14`]
module"]
pub type HSEM_R14 = crate::Reg<hsem_r14::HSEM_R14_SPEC>;
#[doc = "HSEM register HSEM_R0 HSEM_R31"]
pub mod hsem_r14;
#[doc = "HSEM_R15 (rw) register accessor: HSEM register HSEM_R0 HSEM_R31\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hsem_r15::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hsem_r15::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`hsem_r15`]
module"]
pub type HSEM_R15 = crate::Reg<hsem_r15::HSEM_R15_SPEC>;
#[doc = "HSEM register HSEM_R0 HSEM_R31"]
pub mod hsem_r15;
#[doc = "HSEM_R16 (rw) register accessor: HSEM register HSEM_R0 HSEM_R31\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hsem_r16::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hsem_r16::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`hsem_r16`]
module"]
pub type HSEM_R16 = crate::Reg<hsem_r16::HSEM_R16_SPEC>;
#[doc = "HSEM register HSEM_R0 HSEM_R31"]
pub mod hsem_r16;
#[doc = "HSEM_R17 (rw) register accessor: HSEM register HSEM_R0 HSEM_R31\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hsem_r17::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hsem_r17::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`hsem_r17`]
module"]
pub type HSEM_R17 = crate::Reg<hsem_r17::HSEM_R17_SPEC>;
#[doc = "HSEM register HSEM_R0 HSEM_R31"]
pub mod hsem_r17;
#[doc = "HSEM_R18 (rw) register accessor: HSEM register HSEM_R0 HSEM_R31\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hsem_r18::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hsem_r18::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`hsem_r18`]
module"]
pub type HSEM_R18 = crate::Reg<hsem_r18::HSEM_R18_SPEC>;
#[doc = "HSEM register HSEM_R0 HSEM_R31"]
pub mod hsem_r18;
#[doc = "HSEM_R19 (rw) register accessor: HSEM register HSEM_R0 HSEM_R31\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hsem_r19::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hsem_r19::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`hsem_r19`]
module"]
pub type HSEM_R19 = crate::Reg<hsem_r19::HSEM_R19_SPEC>;
#[doc = "HSEM register HSEM_R0 HSEM_R31"]
pub mod hsem_r19;
#[doc = "HSEM_R20 (rw) register accessor: HSEM register HSEM_R0 HSEM_R31\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hsem_r20::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hsem_r20::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`hsem_r20`]
module"]
pub type HSEM_R20 = crate::Reg<hsem_r20::HSEM_R20_SPEC>;
#[doc = "HSEM register HSEM_R0 HSEM_R31"]
pub mod hsem_r20;
#[doc = "HSEM_R21 (rw) register accessor: HSEM register HSEM_R0 HSEM_R31\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hsem_r21::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hsem_r21::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`hsem_r21`]
module"]
pub type HSEM_R21 = crate::Reg<hsem_r21::HSEM_R21_SPEC>;
#[doc = "HSEM register HSEM_R0 HSEM_R31"]
pub mod hsem_r21;
#[doc = "HSEM_R22 (rw) register accessor: HSEM register HSEM_R0 HSEM_R31\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hsem_r22::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hsem_r22::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`hsem_r22`]
module"]
pub type HSEM_R22 = crate::Reg<hsem_r22::HSEM_R22_SPEC>;
#[doc = "HSEM register HSEM_R0 HSEM_R31"]
pub mod hsem_r22;
#[doc = "HSEM_R23 (rw) register accessor: HSEM register HSEM_R0 HSEM_R31\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hsem_r23::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hsem_r23::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`hsem_r23`]
module"]
pub type HSEM_R23 = crate::Reg<hsem_r23::HSEM_R23_SPEC>;
#[doc = "HSEM register HSEM_R0 HSEM_R31"]
pub mod hsem_r23;
#[doc = "HSEM_R24 (rw) register accessor: HSEM register HSEM_R0 HSEM_R31\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hsem_r24::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hsem_r24::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`hsem_r24`]
module"]
pub type HSEM_R24 = crate::Reg<hsem_r24::HSEM_R24_SPEC>;
#[doc = "HSEM register HSEM_R0 HSEM_R31"]
pub mod hsem_r24;
#[doc = "HSEM_R25 (rw) register accessor: HSEM register HSEM_R0 HSEM_R31\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hsem_r25::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hsem_r25::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`hsem_r25`]
module"]
pub type HSEM_R25 = crate::Reg<hsem_r25::HSEM_R25_SPEC>;
#[doc = "HSEM register HSEM_R0 HSEM_R31"]
pub mod hsem_r25;
#[doc = "HSEM_R26 (rw) register accessor: HSEM register HSEM_R0 HSEM_R31\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hsem_r26::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hsem_r26::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`hsem_r26`]
module"]
pub type HSEM_R26 = crate::Reg<hsem_r26::HSEM_R26_SPEC>;
#[doc = "HSEM register HSEM_R0 HSEM_R31"]
pub mod hsem_r26;
#[doc = "HSEM_R27 (rw) register accessor: HSEM register HSEM_R0 HSEM_R31\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hsem_r27::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hsem_r27::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`hsem_r27`]
module"]
pub type HSEM_R27 = crate::Reg<hsem_r27::HSEM_R27_SPEC>;
#[doc = "HSEM register HSEM_R0 HSEM_R31"]
pub mod hsem_r27;
#[doc = "HSEM_R28 (rw) register accessor: HSEM register HSEM_R0 HSEM_R31\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hsem_r28::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hsem_r28::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`hsem_r28`]
module"]
pub type HSEM_R28 = crate::Reg<hsem_r28::HSEM_R28_SPEC>;
#[doc = "HSEM register HSEM_R0 HSEM_R31"]
pub mod hsem_r28;
#[doc = "HSEM_R29 (rw) register accessor: HSEM register HSEM_R0 HSEM_R31\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hsem_r29::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hsem_r29::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`hsem_r29`]
module"]
pub type HSEM_R29 = crate::Reg<hsem_r29::HSEM_R29_SPEC>;
#[doc = "HSEM register HSEM_R0 HSEM_R31"]
pub mod hsem_r29;
#[doc = "HSEM_R30 (rw) register accessor: HSEM register HSEM_R0 HSEM_R31\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hsem_r30::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hsem_r30::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`hsem_r30`]
module"]
pub type HSEM_R30 = crate::Reg<hsem_r30::HSEM_R30_SPEC>;
#[doc = "HSEM register HSEM_R0 HSEM_R31"]
pub mod hsem_r30;
#[doc = "HSEM_R31 (rw) register accessor: HSEM register HSEM_R0 HSEM_R31\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hsem_r31::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hsem_r31::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`hsem_r31`]
module"]
pub type HSEM_R31 = crate::Reg<hsem_r31::HSEM_R31_SPEC>;
#[doc = "HSEM register HSEM_R0 HSEM_R31"]
pub mod hsem_r31;
#[doc = "HSEM_RLR0 (r) register accessor: HSEM Read lock register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hsem_rlr0::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`hsem_rlr0`]
module"]
pub type HSEM_RLR0 = crate::Reg<hsem_rlr0::HSEM_RLR0_SPEC>;
#[doc = "HSEM Read lock register"]
pub mod hsem_rlr0;
#[doc = "HSEM_RLR1 (r) register accessor: HSEM Read lock register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hsem_rlr1::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`hsem_rlr1`]
module"]
pub type HSEM_RLR1 = crate::Reg<hsem_rlr1::HSEM_RLR1_SPEC>;
#[doc = "HSEM Read lock register"]
pub mod hsem_rlr1;
#[doc = "HSEM_RLR2 (r) register accessor: HSEM Read lock register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hsem_rlr2::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`hsem_rlr2`]
module"]
pub type HSEM_RLR2 = crate::Reg<hsem_rlr2::HSEM_RLR2_SPEC>;
#[doc = "HSEM Read lock register"]
pub mod hsem_rlr2;
#[doc = "HSEM_RLR3 (r) register accessor: HSEM Read lock register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hsem_rlr3::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`hsem_rlr3`]
module"]
pub type HSEM_RLR3 = crate::Reg<hsem_rlr3::HSEM_RLR3_SPEC>;
#[doc = "HSEM Read lock register"]
pub mod hsem_rlr3;
#[doc = "HSEM_RLR4 (r) register accessor: HSEM Read lock register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hsem_rlr4::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`hsem_rlr4`]
module"]
pub type HSEM_RLR4 = crate::Reg<hsem_rlr4::HSEM_RLR4_SPEC>;
#[doc = "HSEM Read lock register"]
pub mod hsem_rlr4;
#[doc = "HSEM_RLR5 (r) register accessor: HSEM Read lock register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hsem_rlr5::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`hsem_rlr5`]
module"]
pub type HSEM_RLR5 = crate::Reg<hsem_rlr5::HSEM_RLR5_SPEC>;
#[doc = "HSEM Read lock register"]
pub mod hsem_rlr5;
#[doc = "HSEM_RLR6 (r) register accessor: HSEM Read lock register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hsem_rlr6::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`hsem_rlr6`]
module"]
pub type HSEM_RLR6 = crate::Reg<hsem_rlr6::HSEM_RLR6_SPEC>;
#[doc = "HSEM Read lock register"]
pub mod hsem_rlr6;
#[doc = "HSEM_RLR7 (r) register accessor: HSEM Read lock register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hsem_rlr7::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`hsem_rlr7`]
module"]
pub type HSEM_RLR7 = crate::Reg<hsem_rlr7::HSEM_RLR7_SPEC>;
#[doc = "HSEM Read lock register"]
pub mod hsem_rlr7;
#[doc = "HSEM_RLR8 (r) register accessor: HSEM Read lock register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hsem_rlr8::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`hsem_rlr8`]
module"]
pub type HSEM_RLR8 = crate::Reg<hsem_rlr8::HSEM_RLR8_SPEC>;
#[doc = "HSEM Read lock register"]
pub mod hsem_rlr8;
#[doc = "HSEM_RLR9 (r) register accessor: HSEM Read lock register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hsem_rlr9::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`hsem_rlr9`]
module"]
pub type HSEM_RLR9 = crate::Reg<hsem_rlr9::HSEM_RLR9_SPEC>;
#[doc = "HSEM Read lock register"]
pub mod hsem_rlr9;
#[doc = "HSEM_RLR10 (r) register accessor: HSEM Read lock register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hsem_rlr10::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`hsem_rlr10`]
module"]
pub type HSEM_RLR10 = crate::Reg<hsem_rlr10::HSEM_RLR10_SPEC>;
#[doc = "HSEM Read lock register"]
pub mod hsem_rlr10;
#[doc = "HSEM_RLR11 (r) register accessor: HSEM Read lock register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hsem_rlr11::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`hsem_rlr11`]
module"]
pub type HSEM_RLR11 = crate::Reg<hsem_rlr11::HSEM_RLR11_SPEC>;
#[doc = "HSEM Read lock register"]
pub mod hsem_rlr11;
#[doc = "HSEM_RLR12 (r) register accessor: HSEM Read lock register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hsem_rlr12::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`hsem_rlr12`]
module"]
pub type HSEM_RLR12 = crate::Reg<hsem_rlr12::HSEM_RLR12_SPEC>;
#[doc = "HSEM Read lock register"]
pub mod hsem_rlr12;
#[doc = "HSEM_RLR13 (r) register accessor: HSEM Read lock register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hsem_rlr13::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`hsem_rlr13`]
module"]
pub type HSEM_RLR13 = crate::Reg<hsem_rlr13::HSEM_RLR13_SPEC>;
#[doc = "HSEM Read lock register"]
pub mod hsem_rlr13;
#[doc = "HSEM_RLR14 (r) register accessor: HSEM Read lock register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hsem_rlr14::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`hsem_rlr14`]
module"]
pub type HSEM_RLR14 = crate::Reg<hsem_rlr14::HSEM_RLR14_SPEC>;
#[doc = "HSEM Read lock register"]
pub mod hsem_rlr14;
#[doc = "HSEM_RLR15 (r) register accessor: HSEM Read lock register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hsem_rlr15::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`hsem_rlr15`]
module"]
pub type HSEM_RLR15 = crate::Reg<hsem_rlr15::HSEM_RLR15_SPEC>;
#[doc = "HSEM Read lock register"]
pub mod hsem_rlr15;
#[doc = "HSEM_RLR16 (r) register accessor: HSEM Read lock register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hsem_rlr16::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`hsem_rlr16`]
module"]
pub type HSEM_RLR16 = crate::Reg<hsem_rlr16::HSEM_RLR16_SPEC>;
#[doc = "HSEM Read lock register"]
pub mod hsem_rlr16;
#[doc = "HSEM_RLR17 (r) register accessor: HSEM Read lock register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hsem_rlr17::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`hsem_rlr17`]
module"]
pub type HSEM_RLR17 = crate::Reg<hsem_rlr17::HSEM_RLR17_SPEC>;
#[doc = "HSEM Read lock register"]
pub mod hsem_rlr17;
#[doc = "HSEM_RLR18 (r) register accessor: HSEM Read lock register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hsem_rlr18::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`hsem_rlr18`]
module"]
pub type HSEM_RLR18 = crate::Reg<hsem_rlr18::HSEM_RLR18_SPEC>;
#[doc = "HSEM Read lock register"]
pub mod hsem_rlr18;
#[doc = "HSEM_RLR19 (r) register accessor: HSEM Read lock register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hsem_rlr19::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`hsem_rlr19`]
module"]
pub type HSEM_RLR19 = crate::Reg<hsem_rlr19::HSEM_RLR19_SPEC>;
#[doc = "HSEM Read lock register"]
pub mod hsem_rlr19;
#[doc = "HSEM_RLR20 (r) register accessor: HSEM Read lock register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hsem_rlr20::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`hsem_rlr20`]
module"]
pub type HSEM_RLR20 = crate::Reg<hsem_rlr20::HSEM_RLR20_SPEC>;
#[doc = "HSEM Read lock register"]
pub mod hsem_rlr20;
#[doc = "HSEM_RLR21 (r) register accessor: HSEM Read lock register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hsem_rlr21::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`hsem_rlr21`]
module"]
pub type HSEM_RLR21 = crate::Reg<hsem_rlr21::HSEM_RLR21_SPEC>;
#[doc = "HSEM Read lock register"]
pub mod hsem_rlr21;
#[doc = "HSEM_RLR22 (r) register accessor: HSEM Read lock register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hsem_rlr22::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`hsem_rlr22`]
module"]
pub type HSEM_RLR22 = crate::Reg<hsem_rlr22::HSEM_RLR22_SPEC>;
#[doc = "HSEM Read lock register"]
pub mod hsem_rlr22;
#[doc = "HSEM_RLR23 (r) register accessor: HSEM Read lock register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hsem_rlr23::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`hsem_rlr23`]
module"]
pub type HSEM_RLR23 = crate::Reg<hsem_rlr23::HSEM_RLR23_SPEC>;
#[doc = "HSEM Read lock register"]
pub mod hsem_rlr23;
#[doc = "HSEM_RLR24 (r) register accessor: HSEM Read lock register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hsem_rlr24::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`hsem_rlr24`]
module"]
pub type HSEM_RLR24 = crate::Reg<hsem_rlr24::HSEM_RLR24_SPEC>;
#[doc = "HSEM Read lock register"]
pub mod hsem_rlr24;
#[doc = "HSEM_RLR25 (r) register accessor: HSEM Read lock register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hsem_rlr25::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`hsem_rlr25`]
module"]
pub type HSEM_RLR25 = crate::Reg<hsem_rlr25::HSEM_RLR25_SPEC>;
#[doc = "HSEM Read lock register"]
pub mod hsem_rlr25;
#[doc = "HSEM_RLR26 (r) register accessor: HSEM Read lock register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hsem_rlr26::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`hsem_rlr26`]
module"]
pub type HSEM_RLR26 = crate::Reg<hsem_rlr26::HSEM_RLR26_SPEC>;
#[doc = "HSEM Read lock register"]
pub mod hsem_rlr26;
#[doc = "HSEM_RLR27 (r) register accessor: HSEM Read lock register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hsem_rlr27::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`hsem_rlr27`]
module"]
pub type HSEM_RLR27 = crate::Reg<hsem_rlr27::HSEM_RLR27_SPEC>;
#[doc = "HSEM Read lock register"]
pub mod hsem_rlr27;
#[doc = "HSEM_RLR28 (r) register accessor: HSEM Read lock register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hsem_rlr28::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`hsem_rlr28`]
module"]
pub type HSEM_RLR28 = crate::Reg<hsem_rlr28::HSEM_RLR28_SPEC>;
#[doc = "HSEM Read lock register"]
pub mod hsem_rlr28;
#[doc = "HSEM_RLR29 (r) register accessor: HSEM Read lock register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hsem_rlr29::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`hsem_rlr29`]
module"]
pub type HSEM_RLR29 = crate::Reg<hsem_rlr29::HSEM_RLR29_SPEC>;
#[doc = "HSEM Read lock register"]
pub mod hsem_rlr29;
#[doc = "HSEM_RLR30 (r) register accessor: HSEM Read lock register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hsem_rlr30::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`hsem_rlr30`]
module"]
pub type HSEM_RLR30 = crate::Reg<hsem_rlr30::HSEM_RLR30_SPEC>;
#[doc = "HSEM Read lock register"]
pub mod hsem_rlr30;
#[doc = "HSEM_RLR31 (r) register accessor: HSEM Read lock register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hsem_rlr31::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`hsem_rlr31`]
module"]
pub type HSEM_RLR31 = crate::Reg<hsem_rlr31::HSEM_RLR31_SPEC>;
#[doc = "HSEM Read lock register"]
pub mod hsem_rlr31;
#[doc = "HSEM_C1IER (rw) register accessor: HSEM Interrupt enable register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hsem_c1ier::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hsem_c1ier::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`hsem_c1ier`]
module"]
pub type HSEM_C1IER = crate::Reg<hsem_c1ier::HSEM_C1IER_SPEC>;
#[doc = "HSEM Interrupt enable register"]
pub mod hsem_c1ier;
#[doc = "HSEM_C1ICR (rw) register accessor: HSEM Interrupt clear register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hsem_c1icr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hsem_c1icr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`hsem_c1icr`]
module"]
pub type HSEM_C1ICR = crate::Reg<hsem_c1icr::HSEM_C1ICR_SPEC>;
#[doc = "HSEM Interrupt clear register"]
pub mod hsem_c1icr;
#[doc = "HSEM_C1ISR (r) register accessor: HSEM Interrupt status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hsem_c1isr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`hsem_c1isr`]
module"]
pub type HSEM_C1ISR = crate::Reg<hsem_c1isr::HSEM_C1ISR_SPEC>;
#[doc = "HSEM Interrupt status register"]
pub mod hsem_c1isr;
#[doc = "HSEM_C1MISR (r) register accessor: HSEM Masked interrupt status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hsem_c1misr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`hsem_c1misr`]
module"]
pub type HSEM_C1MISR = crate::Reg<hsem_c1misr::HSEM_C1MISR_SPEC>;
#[doc = "HSEM Masked interrupt status register"]
pub mod hsem_c1misr;
#[doc = "HSEM_C2IER (rw) register accessor: HSEM Interrupt enable register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hsem_c2ier::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hsem_c2ier::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`hsem_c2ier`]
module"]
pub type HSEM_C2IER = crate::Reg<hsem_c2ier::HSEM_C2IER_SPEC>;
#[doc = "HSEM Interrupt enable register"]
pub mod hsem_c2ier;
#[doc = "HSEM_C2ICR (rw) register accessor: HSEM Interrupt clear register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hsem_c2icr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hsem_c2icr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`hsem_c2icr`]
module"]
pub type HSEM_C2ICR = crate::Reg<hsem_c2icr::HSEM_C2ICR_SPEC>;
#[doc = "HSEM Interrupt clear register"]
pub mod hsem_c2icr;
#[doc = "HSEM_C2ISR (r) register accessor: HSEM Interrupt status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hsem_c2isr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`hsem_c2isr`]
module"]
pub type HSEM_C2ISR = crate::Reg<hsem_c2isr::HSEM_C2ISR_SPEC>;
#[doc = "HSEM Interrupt status register"]
pub mod hsem_c2isr;
#[doc = "HSEM_C2MISR (r) register accessor: HSEM Masked interrupt status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hsem_c2misr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`hsem_c2misr`]
module"]
pub type HSEM_C2MISR = crate::Reg<hsem_c2misr::HSEM_C2MISR_SPEC>;
#[doc = "HSEM Masked interrupt status register"]
pub mod hsem_c2misr;
#[doc = "HSEM_CR (w) register accessor: HSEM Clear register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hsem_cr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`hsem_cr`]
module"]
pub type HSEM_CR = crate::Reg<hsem_cr::HSEM_CR_SPEC>;
#[doc = "HSEM Clear register"]
pub mod hsem_cr;
#[doc = "HSEM_KEYR (rw) register accessor: HSEM Interrupt clear register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hsem_keyr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hsem_keyr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`hsem_keyr`]
module"]
pub type HSEM_KEYR = crate::Reg<hsem_keyr::HSEM_KEYR_SPEC>;
#[doc = "HSEM Interrupt clear register"]
pub mod hsem_keyr;
