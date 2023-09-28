#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - SWPMI Configuration/Control register"]
    pub cr: CR,
    #[doc = "0x04 - SWPMI Bitrate register"]
    pub brr: BRR,
    _reserved2: [u8; 0x04],
    #[doc = "0x0c - SWPMI Interrupt and Status register"]
    pub isr: ISR,
    #[doc = "0x10 - SWPMI Interrupt Flag Clear register"]
    pub icr: ICR,
    #[doc = "0x14 - SWPMI Interrupt Enable register"]
    pub ier: IER,
    #[doc = "0x18 - SWPMI Receive Frame Length register"]
    pub rfl: RFL,
    #[doc = "0x1c - SWPMI Transmit data register"]
    pub tdr: TDR,
    #[doc = "0x20 - SWPMI Receive data register"]
    pub rdr: RDR,
    #[doc = "0x24 - SWPMI Option register"]
    pub or: OR,
}
#[doc = "CR (rw) register accessor: SWPMI Configuration/Control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`cr`]
module"]
pub type CR = crate::Reg<cr::CR_SPEC>;
#[doc = "SWPMI Configuration/Control register"]
pub mod cr;
#[doc = "BRR (rw) register accessor: SWPMI Bitrate register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`brr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`brr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`brr`]
module"]
pub type BRR = crate::Reg<brr::BRR_SPEC>;
#[doc = "SWPMI Bitrate register"]
pub mod brr;
#[doc = "ISR (r) register accessor: SWPMI Interrupt and Status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`isr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`isr`]
module"]
pub type ISR = crate::Reg<isr::ISR_SPEC>;
#[doc = "SWPMI Interrupt and Status register"]
pub mod isr;
#[doc = "ICR (w) register accessor: SWPMI Interrupt Flag Clear register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`icr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`icr`]
module"]
pub type ICR = crate::Reg<icr::ICR_SPEC>;
#[doc = "SWPMI Interrupt Flag Clear register"]
pub mod icr;
#[doc = "IER (rw) register accessor: SWPMI Interrupt Enable register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ier::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ier::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`ier`]
module"]
pub type IER = crate::Reg<ier::IER_SPEC>;
#[doc = "SWPMI Interrupt Enable register"]
pub mod ier;
#[doc = "RFL (r) register accessor: SWPMI Receive Frame Length register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rfl::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`rfl`]
module"]
pub type RFL = crate::Reg<rfl::RFL_SPEC>;
#[doc = "SWPMI Receive Frame Length register"]
pub mod rfl;
#[doc = "TDR (w) register accessor: SWPMI Transmit data register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tdr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`tdr`]
module"]
pub type TDR = crate::Reg<tdr::TDR_SPEC>;
#[doc = "SWPMI Transmit data register"]
pub mod tdr;
#[doc = "RDR (r) register accessor: SWPMI Receive data register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rdr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`rdr`]
module"]
pub type RDR = crate::Reg<rdr::RDR_SPEC>;
#[doc = "SWPMI Receive data register"]
pub mod rdr;
#[doc = "OR (rw) register accessor: SWPMI Option register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`or::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`or::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`or`]
module"]
pub type OR = crate::Reg<or::OR_SPEC>;
#[doc = "SWPMI Option register"]
pub mod or;
