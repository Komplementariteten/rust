#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Control register"]
    pub cr: CR,
    #[doc = "0x04 - Interrupt mask register"]
    pub imr: IMR,
    #[doc = "0x08 - Status register"]
    pub sr: SR,
    #[doc = "0x0c - Interrupt Flag Clear register"]
    pub ifcr: IFCR,
    _reserved_4_dr_: [u8; 0x04],
    #[doc = "0x14 - Channel Status register"]
    pub csr: CSR,
    #[doc = "0x18 - Debug Information register"]
    pub dir: DIR,
    _reserved7: [u8; 0x03d8],
    #[doc = "0x3f4 - SPDIFRX version register"]
    pub verr: VERR,
    #[doc = "0x3f8 - SPDIFRX identification register"]
    pub idr: IDR,
    #[doc = "0x3fc - SPDIFRX size identification register"]
    pub sidr: SIDR,
}
impl RegisterBlock {
    #[doc = "0x10 - Data input register"]
    #[inline(always)]
    pub const fn dr_10(&self) -> &DR_10 {
        unsafe { &*(self as *const Self).cast::<u8>().add(16usize).cast() }
    }
    #[doc = "0x10 - Data input register"]
    #[inline(always)]
    pub const fn dr_01(&self) -> &DR_01 {
        unsafe { &*(self as *const Self).cast::<u8>().add(16usize).cast() }
    }
    #[doc = "0x10 - Data input register"]
    #[inline(always)]
    pub const fn dr_00(&self) -> &DR_00 {
        unsafe { &*(self as *const Self).cast::<u8>().add(16usize).cast() }
    }
}
#[doc = "CR (rw) register accessor: Control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`cr`]
module"]
pub type CR = crate::Reg<cr::CR_SPEC>;
#[doc = "Control register"]
pub mod cr;
#[doc = "IMR (rw) register accessor: Interrupt mask register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`imr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`imr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`imr`]
module"]
pub type IMR = crate::Reg<imr::IMR_SPEC>;
#[doc = "Interrupt mask register"]
pub mod imr;
#[doc = "SR (r) register accessor: Status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`sr`]
module"]
pub type SR = crate::Reg<sr::SR_SPEC>;
#[doc = "Status register"]
pub mod sr;
#[doc = "IFCR (w) register accessor: Interrupt Flag Clear register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ifcr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`ifcr`]
module"]
pub type IFCR = crate::Reg<ifcr::IFCR_SPEC>;
#[doc = "Interrupt Flag Clear register"]
pub mod ifcr;
#[doc = "DR_00 (r) register accessor: Data input register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dr_00::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`dr_00`]
module"]
pub type DR_00 = crate::Reg<dr_00::DR_00_SPEC>;
#[doc = "Data input register"]
pub mod dr_00;
#[doc = "CSR (r) register accessor: Channel Status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`csr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`csr`]
module"]
pub type CSR = crate::Reg<csr::CSR_SPEC>;
#[doc = "Channel Status register"]
pub mod csr;
#[doc = "DIR (r) register accessor: Debug Information register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dir::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`dir`]
module"]
pub type DIR = crate::Reg<dir::DIR_SPEC>;
#[doc = "Debug Information register"]
pub mod dir;
#[doc = "VERR (r) register accessor: SPDIFRX version register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`verr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`verr`]
module"]
pub type VERR = crate::Reg<verr::VERR_SPEC>;
#[doc = "SPDIFRX version register"]
pub mod verr;
#[doc = "IDR (r) register accessor: SPDIFRX identification register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`idr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`idr`]
module"]
pub type IDR = crate::Reg<idr::IDR_SPEC>;
#[doc = "SPDIFRX identification register"]
pub mod idr;
#[doc = "SIDR (r) register accessor: SPDIFRX size identification register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sidr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`sidr`]
module"]
pub type SIDR = crate::Reg<sidr::SIDR_SPEC>;
#[doc = "SPDIFRX size identification register"]
pub mod sidr;
#[doc = "DR_01 (r) register accessor: Data input register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dr_01::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`dr_01`]
module"]
pub type DR_01 = crate::Reg<dr_01::DR_01_SPEC>;
#[doc = "Data input register"]
pub mod dr_01;
#[doc = "DR_10 (r) register accessor: Data input register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dr_10::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`dr_10`]
module"]
pub type DR_10 = crate::Reg<dr_10::DR_10_SPEC>;
#[doc = "Data input register"]
pub mod dr_10;
