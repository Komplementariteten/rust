#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Access control register"]
    pub acr: ACR,
    #[doc = "0x04 - FLASH key register for bank 1"]
    pub keyr1: KEYR1,
    #[doc = "0x08 - FLASH option key register"]
    pub optkeyr: OPTKEYR,
    #[doc = "0x0c - FLASH control register for bank 1"]
    pub cr1: CR1,
    #[doc = "0x10 - FLASH status register for bank 1"]
    pub sr1: SR1,
    #[doc = "0x14 - FLASH clear control register for bank 1"]
    pub ccr1: CCR1,
    #[doc = "0x18 - FLASH option control register"]
    pub optcr: OPTCR,
    #[doc = "0x1c - FLASH option status register"]
    pub optsr_cur: OPTSR_CUR,
    #[doc = "0x20 - FLASH option status register"]
    pub optsr_prg: OPTSR_PRG,
    #[doc = "0x24 - FLASH option clear control register"]
    pub optccr: OPTCCR,
    #[doc = "0x28 - FLASH protection address for bank 1"]
    pub prar_cur1: PRAR_CUR1,
    _reserved_11_prar_prg: [u8; 0x04],
    #[doc = "0x30 - FLASH secure address for bank 1"]
    pub scar_cur1: SCAR_CUR1,
    #[doc = "0x34 - FLASH secure address for bank 1"]
    pub scar_prg1: SCAR_PRG1,
    #[doc = "0x38 - FLASH write sector protection for bank 1"]
    pub wpsn_cur1r: WPSN_CUR1R,
    #[doc = "0x3c - FLASH write sector protection for bank 1"]
    pub wpsn_prg1r: WPSN_PRG1R,
    #[doc = "0x40 - FLASH register with boot address"]
    pub boot_curr: BOOT_CURR,
    #[doc = "0x44 - FLASH register with boot address"]
    pub boot_prgr: BOOT_PRGR,
    _reserved18: [u8; 0x08],
    #[doc = "0x50 - FLASH CRC control register for bank 1"]
    pub crccr1: CRCCR1,
    #[doc = "0x54 - FLASH CRC start address register for bank 1"]
    pub crcsadd1r: CRCSADD1R,
    #[doc = "0x58 - FLASH CRC end address register for bank 1"]
    pub crceadd1r: CRCEADD1R,
    #[doc = "0x5c - FLASH CRC data register"]
    pub crcdatar: CRCDATAR,
    #[doc = "0x60 - FLASH ECC fail address for bank 1"]
    pub ecc_fa1r: ECC_FA1R,
    _reserved23: [u8; 0x9c],
    #[doc = "0x100 - Access control register"]
    pub acr_: ACR_,
    #[doc = "0x104 - FLASH key register for bank 2"]
    pub keyr2: KEYR2,
    #[doc = "0x108 - FLASH option key register"]
    pub optkeyr_: OPTKEYR_,
    #[doc = "0x10c - FLASH control register for bank 2"]
    pub cr2: CR2,
    #[doc = "0x110 - FLASH status register for bank 2"]
    pub sr2: SR2,
    #[doc = "0x114 - FLASH clear control register for bank 2"]
    pub ccr2: CCR2,
    #[doc = "0x118 - FLASH option control register"]
    pub optcr_: OPTCR_,
    #[doc = "0x11c - FLASH option status register"]
    pub optsr_cur_: OPTSR_CUR_,
    #[doc = "0x120 - FLASH option status register"]
    pub optsr_prg_: OPTSR_PRG_,
    #[doc = "0x124 - FLASH option clear control register"]
    pub optccr_: OPTCCR_,
    #[doc = "0x128 - FLASH protection address for bank 1"]
    pub prar_cur2: PRAR_CUR2,
    _reserved34: [u8; 0x04],
    #[doc = "0x130 - FLASH secure address for bank 2"]
    pub scar_cur2: SCAR_CUR2,
    #[doc = "0x134 - FLASH secure address for bank 2"]
    pub scar_prg2: SCAR_PRG2,
    #[doc = "0x138 - FLASH write sector protection for bank 2"]
    pub wpsn_cur2r: WPSN_CUR2R,
    #[doc = "0x13c - FLASH write sector protection for bank 2"]
    pub wpsn_prg2r: WPSN_PRG2R,
    _reserved38: [u8; 0x10],
    #[doc = "0x150 - FLASH CRC control register for bank 1"]
    pub crccr2: CRCCR2,
    #[doc = "0x154 - FLASH CRC start address register for bank 2"]
    pub crcsadd2r: CRCSADD2R,
    #[doc = "0x158 - FLASH CRC end address register for bank 2"]
    pub crceadd2r: CRCEADD2R,
    _reserved41: [u8; 0x04],
    #[doc = "0x160 - FLASH ECC fail address for bank 2"]
    pub ecc_fa2r: ECC_FA2R,
}
impl RegisterBlock {
    #[doc = "0x2c - FLASH protection address for bank 2"]
    #[inline(always)]
    pub const fn prar_prg2(&self) -> &PRAR_PRG2 {
        unsafe { &*(self as *const Self).cast::<u8>().add(44usize).cast() }
    }
    #[doc = "0x2c - FLASH protection address for bank 1"]
    #[inline(always)]
    pub const fn prar_prg1(&self) -> &PRAR_PRG1 {
        unsafe { &*(self as *const Self).cast::<u8>().add(44usize).cast() }
    }
}
#[doc = "ACR (rw) register accessor: Access control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`acr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`acr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`acr`]
module"]
pub type ACR = crate::Reg<acr::ACR_SPEC>;
#[doc = "Access control register"]
pub mod acr;
#[doc = "ACR_ (rw) register accessor: Access control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`acr_::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`acr_::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`acr_`]
module"]
pub type ACR_ = crate::Reg<acr_::ACR__SPEC>;
#[doc = "Access control register"]
pub mod acr_;
#[doc = "KEYR1 (rw) register accessor: FLASH key register for bank 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`keyr1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`keyr1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`keyr1`]
module"]
pub type KEYR1 = crate::Reg<keyr1::KEYR1_SPEC>;
#[doc = "FLASH key register for bank 1"]
pub mod keyr1;
#[doc = "OPTKEYR (rw) register accessor: FLASH option key register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`optkeyr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`optkeyr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`optkeyr`]
module"]
pub type OPTKEYR = crate::Reg<optkeyr::OPTKEYR_SPEC>;
#[doc = "FLASH option key register"]
pub mod optkeyr;
#[doc = "OPTKEYR_ (rw) register accessor: FLASH option key register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`optkeyr_::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`optkeyr_::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`optkeyr_`]
module"]
pub type OPTKEYR_ = crate::Reg<optkeyr_::OPTKEYR__SPEC>;
#[doc = "FLASH option key register"]
pub mod optkeyr_;
#[doc = "CR1 (rw) register accessor: FLASH control register for bank 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cr1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cr1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`cr1`]
module"]
pub type CR1 = crate::Reg<cr1::CR1_SPEC>;
#[doc = "FLASH control register for bank 1"]
pub mod cr1;
#[doc = "SR1 (rw) register accessor: FLASH status register for bank 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sr1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sr1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`sr1`]
module"]
pub type SR1 = crate::Reg<sr1::SR1_SPEC>;
#[doc = "FLASH status register for bank 1"]
pub mod sr1;
#[doc = "CCR1 (rw) register accessor: FLASH clear control register for bank 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ccr1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ccr1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`ccr1`]
module"]
pub type CCR1 = crate::Reg<ccr1::CCR1_SPEC>;
#[doc = "FLASH clear control register for bank 1"]
pub mod ccr1;
#[doc = "OPTCR (rw) register accessor: FLASH option control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`optcr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`optcr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`optcr`]
module"]
pub type OPTCR = crate::Reg<optcr::OPTCR_SPEC>;
#[doc = "FLASH option control register"]
pub mod optcr;
#[doc = "OPTCR_ (rw) register accessor: FLASH option control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`optcr_::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`optcr_::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`optcr_`]
module"]
pub type OPTCR_ = crate::Reg<optcr_::OPTCR__SPEC>;
#[doc = "FLASH option control register"]
pub mod optcr_;
#[doc = "OPTSR_CUR_ (rw) register accessor: FLASH option status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`optsr_cur_::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`optsr_cur_::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`optsr_cur_`]
module"]
pub type OPTSR_CUR_ = crate::Reg<optsr_cur_::OPTSR_CUR__SPEC>;
#[doc = "FLASH option status register"]
pub mod optsr_cur_;
#[doc = "OPTSR_CUR (rw) register accessor: FLASH option status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`optsr_cur::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`optsr_cur::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`optsr_cur`]
module"]
pub type OPTSR_CUR = crate::Reg<optsr_cur::OPTSR_CUR_SPEC>;
#[doc = "FLASH option status register"]
pub mod optsr_cur;
#[doc = "OPTSR_PRG (rw) register accessor: FLASH option status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`optsr_prg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`optsr_prg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`optsr_prg`]
module"]
pub type OPTSR_PRG = crate::Reg<optsr_prg::OPTSR_PRG_SPEC>;
#[doc = "FLASH option status register"]
pub mod optsr_prg;
#[doc = "OPTSR_PRG_ (rw) register accessor: FLASH option status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`optsr_prg_::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`optsr_prg_::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`optsr_prg_`]
module"]
pub type OPTSR_PRG_ = crate::Reg<optsr_prg_::OPTSR_PRG__SPEC>;
#[doc = "FLASH option status register"]
pub mod optsr_prg_;
#[doc = "OPTCCR_ (w) register accessor: FLASH option clear control register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`optccr_::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`optccr_`]
module"]
pub type OPTCCR_ = crate::Reg<optccr_::OPTCCR__SPEC>;
#[doc = "FLASH option clear control register"]
pub mod optccr_;
#[doc = "OPTCCR (w) register accessor: FLASH option clear control register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`optccr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`optccr`]
module"]
pub type OPTCCR = crate::Reg<optccr::OPTCCR_SPEC>;
#[doc = "FLASH option clear control register"]
pub mod optccr;
#[doc = "PRAR_CUR1 (r) register accessor: FLASH protection address for bank 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`prar_cur1::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`prar_cur1`]
module"]
pub type PRAR_CUR1 = crate::Reg<prar_cur1::PRAR_CUR1_SPEC>;
#[doc = "FLASH protection address for bank 1"]
pub mod prar_cur1;
#[doc = "PRAR_PRG1 (rw) register accessor: FLASH protection address for bank 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`prar_prg1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`prar_prg1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`prar_prg1`]
module"]
pub type PRAR_PRG1 = crate::Reg<prar_prg1::PRAR_PRG1_SPEC>;
#[doc = "FLASH protection address for bank 1"]
pub mod prar_prg1;
#[doc = "SCAR_CUR1 (rw) register accessor: FLASH secure address for bank 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`scar_cur1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`scar_cur1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`scar_cur1`]
module"]
pub type SCAR_CUR1 = crate::Reg<scar_cur1::SCAR_CUR1_SPEC>;
#[doc = "FLASH secure address for bank 1"]
pub mod scar_cur1;
#[doc = "SCAR_PRG1 (rw) register accessor: FLASH secure address for bank 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`scar_prg1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`scar_prg1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`scar_prg1`]
module"]
pub type SCAR_PRG1 = crate::Reg<scar_prg1::SCAR_PRG1_SPEC>;
#[doc = "FLASH secure address for bank 1"]
pub mod scar_prg1;
#[doc = "WPSN_CUR1R (r) register accessor: FLASH write sector protection for bank 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wpsn_cur1r::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`wpsn_cur1r`]
module"]
pub type WPSN_CUR1R = crate::Reg<wpsn_cur1r::WPSN_CUR1R_SPEC>;
#[doc = "FLASH write sector protection for bank 1"]
pub mod wpsn_cur1r;
#[doc = "WPSN_PRG1R (rw) register accessor: FLASH write sector protection for bank 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wpsn_prg1r::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wpsn_prg1r::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`wpsn_prg1r`]
module"]
pub type WPSN_PRG1R = crate::Reg<wpsn_prg1r::WPSN_PRG1R_SPEC>;
#[doc = "FLASH write sector protection for bank 1"]
pub mod wpsn_prg1r;
#[doc = "BOOT_CURR (r) register accessor: FLASH register with boot address\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`boot_curr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`boot_curr`]
module"]
pub type BOOT_CURR = crate::Reg<boot_curr::BOOT_CURR_SPEC>;
#[doc = "FLASH register with boot address"]
pub mod boot_curr;
#[doc = "BOOT_PRGR (r) register accessor: FLASH register with boot address\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`boot_prgr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`boot_prgr`]
module"]
pub type BOOT_PRGR = crate::Reg<boot_prgr::BOOT_PRGR_SPEC>;
#[doc = "FLASH register with boot address"]
pub mod boot_prgr;
#[doc = "CRCCR1 (rw) register accessor: FLASH CRC control register for bank 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`crccr1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`crccr1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`crccr1`]
module"]
pub type CRCCR1 = crate::Reg<crccr1::CRCCR1_SPEC>;
#[doc = "FLASH CRC control register for bank 1"]
pub mod crccr1;
#[doc = "CRCSADD1R (rw) register accessor: FLASH CRC start address register for bank 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`crcsadd1r::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`crcsadd1r::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`crcsadd1r`]
module"]
pub type CRCSADD1R = crate::Reg<crcsadd1r::CRCSADD1R_SPEC>;
#[doc = "FLASH CRC start address register for bank 1"]
pub mod crcsadd1r;
#[doc = "CRCEADD1R (rw) register accessor: FLASH CRC end address register for bank 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`crceadd1r::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`crceadd1r::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`crceadd1r`]
module"]
pub type CRCEADD1R = crate::Reg<crceadd1r::CRCEADD1R_SPEC>;
#[doc = "FLASH CRC end address register for bank 1"]
pub mod crceadd1r;
#[doc = "CRCDATAR (rw) register accessor: FLASH CRC data register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`crcdatar::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`crcdatar::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`crcdatar`]
module"]
pub type CRCDATAR = crate::Reg<crcdatar::CRCDATAR_SPEC>;
#[doc = "FLASH CRC data register"]
pub mod crcdatar;
#[doc = "ECC_FA1R (r) register accessor: FLASH ECC fail address for bank 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ecc_fa1r::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`ecc_fa1r`]
module"]
pub type ECC_FA1R = crate::Reg<ecc_fa1r::ECC_FA1R_SPEC>;
#[doc = "FLASH ECC fail address for bank 1"]
pub mod ecc_fa1r;
#[doc = "KEYR2 (r) register accessor: FLASH key register for bank 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`keyr2::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`keyr2`]
module"]
pub type KEYR2 = crate::Reg<keyr2::KEYR2_SPEC>;
#[doc = "FLASH key register for bank 2"]
pub mod keyr2;
#[doc = "CR2 (rw) register accessor: FLASH control register for bank 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cr2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cr2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`cr2`]
module"]
pub type CR2 = crate::Reg<cr2::CR2_SPEC>;
#[doc = "FLASH control register for bank 2"]
pub mod cr2;
#[doc = "SR2 (rw) register accessor: FLASH status register for bank 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sr2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sr2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`sr2`]
module"]
pub type SR2 = crate::Reg<sr2::SR2_SPEC>;
#[doc = "FLASH status register for bank 2"]
pub mod sr2;
#[doc = "CCR2 (rw) register accessor: FLASH clear control register for bank 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ccr2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ccr2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`ccr2`]
module"]
pub type CCR2 = crate::Reg<ccr2::CCR2_SPEC>;
#[doc = "FLASH clear control register for bank 2"]
pub mod ccr2;
#[doc = "PRAR_CUR2 (r) register accessor: FLASH protection address for bank 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`prar_cur2::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`prar_cur2`]
module"]
pub type PRAR_CUR2 = crate::Reg<prar_cur2::PRAR_CUR2_SPEC>;
#[doc = "FLASH protection address for bank 1"]
pub mod prar_cur2;
#[doc = "PRAR_PRG2 (rw) register accessor: FLASH protection address for bank 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`prar_prg2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`prar_prg2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`prar_prg2`]
module"]
pub type PRAR_PRG2 = crate::Reg<prar_prg2::PRAR_PRG2_SPEC>;
#[doc = "FLASH protection address for bank 2"]
pub mod prar_prg2;
#[doc = "SCAR_CUR2 (rw) register accessor: FLASH secure address for bank 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`scar_cur2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`scar_cur2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`scar_cur2`]
module"]
pub type SCAR_CUR2 = crate::Reg<scar_cur2::SCAR_CUR2_SPEC>;
#[doc = "FLASH secure address for bank 2"]
pub mod scar_cur2;
#[doc = "SCAR_PRG2 (rw) register accessor: FLASH secure address for bank 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`scar_prg2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`scar_prg2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`scar_prg2`]
module"]
pub type SCAR_PRG2 = crate::Reg<scar_prg2::SCAR_PRG2_SPEC>;
#[doc = "FLASH secure address for bank 2"]
pub mod scar_prg2;
#[doc = "WPSN_CUR2R (r) register accessor: FLASH write sector protection for bank 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wpsn_cur2r::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`wpsn_cur2r`]
module"]
pub type WPSN_CUR2R = crate::Reg<wpsn_cur2r::WPSN_CUR2R_SPEC>;
#[doc = "FLASH write sector protection for bank 2"]
pub mod wpsn_cur2r;
#[doc = "WPSN_PRG2R (rw) register accessor: FLASH write sector protection for bank 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wpsn_prg2r::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wpsn_prg2r::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`wpsn_prg2r`]
module"]
pub type WPSN_PRG2R = crate::Reg<wpsn_prg2r::WPSN_PRG2R_SPEC>;
#[doc = "FLASH write sector protection for bank 2"]
pub mod wpsn_prg2r;
#[doc = "CRCCR2 (rw) register accessor: FLASH CRC control register for bank 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`crccr2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`crccr2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`crccr2`]
module"]
pub type CRCCR2 = crate::Reg<crccr2::CRCCR2_SPEC>;
#[doc = "FLASH CRC control register for bank 1"]
pub mod crccr2;
#[doc = "CRCSADD2R (rw) register accessor: FLASH CRC start address register for bank 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`crcsadd2r::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`crcsadd2r::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`crcsadd2r`]
module"]
pub type CRCSADD2R = crate::Reg<crcsadd2r::CRCSADD2R_SPEC>;
#[doc = "FLASH CRC start address register for bank 2"]
pub mod crcsadd2r;
#[doc = "CRCEADD2R (rw) register accessor: FLASH CRC end address register for bank 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`crceadd2r::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`crceadd2r::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`crceadd2r`]
module"]
pub type CRCEADD2R = crate::Reg<crceadd2r::CRCEADD2R_SPEC>;
#[doc = "FLASH CRC end address register for bank 2"]
pub mod crceadd2r;
#[doc = "ECC_FA2R (r) register accessor: FLASH ECC fail address for bank 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ecc_fa2r::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`ecc_fa2r`]
module"]
pub type ECC_FA2R = crate::Reg<ecc_fa2r::ECC_FA2R_SPEC>;
#[doc = "FLASH ECC fail address for bank 2"]
pub mod ecc_fa2r;
