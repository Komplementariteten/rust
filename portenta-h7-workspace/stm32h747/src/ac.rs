#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Instruction and Data Tightly-Coupled Memory Control Registers"]
    pub itcmcr: ITCMCR,
    #[doc = "0x04 - Instruction and Data Tightly-Coupled Memory Control Registers"]
    pub dtcmcr: DTCMCR,
    #[doc = "0x08 - AHBP Control register"]
    pub ahbpcr: AHBPCR,
    #[doc = "0x0c - Auxiliary Cache Control register"]
    pub cacr: CACR,
    #[doc = "0x10 - AHB Slave Control register"]
    pub ahbscr: AHBSCR,
    _reserved5: [u8; 0x04],
    #[doc = "0x18 - Auxiliary Bus Fault Status register"]
    pub abfsr: ABFSR,
}
#[doc = "ITCMCR (rw) register accessor: Instruction and Data Tightly-Coupled Memory Control Registers\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`itcmcr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`itcmcr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`itcmcr`]
module"]
pub type ITCMCR = crate::Reg<itcmcr::ITCMCR_SPEC>;
#[doc = "Instruction and Data Tightly-Coupled Memory Control Registers"]
pub mod itcmcr;
#[doc = "DTCMCR (rw) register accessor: Instruction and Data Tightly-Coupled Memory Control Registers\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dtcmcr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dtcmcr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`dtcmcr`]
module"]
pub type DTCMCR = crate::Reg<dtcmcr::DTCMCR_SPEC>;
#[doc = "Instruction and Data Tightly-Coupled Memory Control Registers"]
pub mod dtcmcr;
#[doc = "AHBPCR (rw) register accessor: AHBP Control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ahbpcr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ahbpcr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`ahbpcr`]
module"]
pub type AHBPCR = crate::Reg<ahbpcr::AHBPCR_SPEC>;
#[doc = "AHBP Control register"]
pub mod ahbpcr;
#[doc = "CACR (rw) register accessor: Auxiliary Cache Control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cacr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cacr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`cacr`]
module"]
pub type CACR = crate::Reg<cacr::CACR_SPEC>;
#[doc = "Auxiliary Cache Control register"]
pub mod cacr;
#[doc = "AHBSCR (rw) register accessor: AHB Slave Control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ahbscr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ahbscr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`ahbscr`]
module"]
pub type AHBSCR = crate::Reg<ahbscr::AHBSCR_SPEC>;
#[doc = "AHB Slave Control register"]
pub mod ahbscr;
#[doc = "ABFSR (rw) register accessor: Auxiliary Bus Fault Status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`abfsr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`abfsr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`abfsr`]
module"]
pub type ABFSR = crate::Reg<abfsr::ABFSR_SPEC>;
#[doc = "Auxiliary Bus Fault Status register"]
pub mod abfsr;
