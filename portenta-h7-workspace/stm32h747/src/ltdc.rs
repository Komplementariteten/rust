#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 0x08],
    #[doc = "0x08 - Synchronization Size Configuration Register"]
    pub sscr: SSCR,
    #[doc = "0x0c - Back Porch Configuration Register"]
    pub bpcr: BPCR,
    #[doc = "0x10 - Active Width Configuration Register"]
    pub awcr: AWCR,
    #[doc = "0x14 - Total Width Configuration Register"]
    pub twcr: TWCR,
    #[doc = "0x18 - Global Control Register"]
    pub gcr: GCR,
    _reserved5: [u8; 0x08],
    #[doc = "0x24 - Shadow Reload Configuration Register"]
    pub srcr: SRCR,
    _reserved6: [u8; 0x04],
    #[doc = "0x2c - Background Color Configuration Register"]
    pub bccr: BCCR,
    _reserved7: [u8; 0x04],
    #[doc = "0x34 - Interrupt Enable Register"]
    pub ier: IER,
    #[doc = "0x38 - Interrupt Status Register"]
    pub isr: ISR,
    #[doc = "0x3c - Interrupt Clear Register"]
    pub icr: ICR,
    #[doc = "0x40 - Line Interrupt Position Configuration Register"]
    pub lipcr: LIPCR,
    #[doc = "0x44 - Current Position Status Register"]
    pub cpsr: CPSR,
    #[doc = "0x48 - Current Display Status Register"]
    pub cdsr: CDSR,
    _reserved13: [u8; 0x38],
    #[doc = "0x84 - Layerx Control Register"]
    pub l1cr: L1CR,
    #[doc = "0x88 - Layerx Window Horizontal Position Configuration Register"]
    pub l1whpcr: L1WHPCR,
    #[doc = "0x8c - Layerx Window Vertical Position Configuration Register"]
    pub l1wvpcr: L1WVPCR,
    #[doc = "0x90 - Layerx Color Keying Configuration Register"]
    pub l1ckcr: L1CKCR,
    #[doc = "0x94 - Layerx Pixel Format Configuration Register"]
    pub l1pfcr: L1PFCR,
    #[doc = "0x98 - Layerx Constant Alpha Configuration Register"]
    pub l1cacr: L1CACR,
    #[doc = "0x9c - Layerx Default Color Configuration Register"]
    pub l1dccr: L1DCCR,
    #[doc = "0xa0 - Layerx Blending Factors Configuration Register"]
    pub l1bfcr: L1BFCR,
    _reserved21: [u8; 0x08],
    #[doc = "0xac - Layerx Color Frame Buffer Address Register"]
    pub l1cfbar: L1CFBAR,
    #[doc = "0xb0 - Layerx Color Frame Buffer Length Register"]
    pub l1cfblr: L1CFBLR,
    #[doc = "0xb4 - Layerx ColorFrame Buffer Line Number Register"]
    pub l1cfblnr: L1CFBLNR,
    _reserved24: [u8; 0x0c],
    #[doc = "0xc4 - Layerx CLUT Write Register"]
    pub l1clutwr: L1CLUTWR,
    _reserved25: [u8; 0x3c],
    #[doc = "0x104 - Layerx Control Register"]
    pub l2cr: L2CR,
    #[doc = "0x108 - Layerx Window Horizontal Position Configuration Register"]
    pub l2whpcr: L2WHPCR,
    #[doc = "0x10c - Layerx Window Vertical Position Configuration Register"]
    pub l2wvpcr: L2WVPCR,
    #[doc = "0x110 - Layerx Color Keying Configuration Register"]
    pub l2ckcr: L2CKCR,
    #[doc = "0x114 - Layerx Pixel Format Configuration Register"]
    pub l2pfcr: L2PFCR,
    #[doc = "0x118 - Layerx Constant Alpha Configuration Register"]
    pub l2cacr: L2CACR,
    #[doc = "0x11c - Layerx Default Color Configuration Register"]
    pub l2dccr: L2DCCR,
    #[doc = "0x120 - Layerx Blending Factors Configuration Register"]
    pub l2bfcr: L2BFCR,
    _reserved33: [u8; 0x08],
    #[doc = "0x12c - Layerx Color Frame Buffer Address Register"]
    pub l2cfbar: L2CFBAR,
    #[doc = "0x130 - Layerx Color Frame Buffer Length Register"]
    pub l2cfblr: L2CFBLR,
    #[doc = "0x134 - Layerx ColorFrame Buffer Line Number Register"]
    pub l2cfblnr: L2CFBLNR,
    _reserved36: [u8; 0x0c],
    #[doc = "0x144 - Layerx CLUT Write Register"]
    pub l2clutwr: L2CLUTWR,
}
#[doc = "SSCR (rw) register accessor: Synchronization Size Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sscr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sscr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`sscr`]
module"]
pub type SSCR = crate::Reg<sscr::SSCR_SPEC>;
#[doc = "Synchronization Size Configuration Register"]
pub mod sscr;
#[doc = "BPCR (rw) register accessor: Back Porch Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bpcr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bpcr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`bpcr`]
module"]
pub type BPCR = crate::Reg<bpcr::BPCR_SPEC>;
#[doc = "Back Porch Configuration Register"]
pub mod bpcr;
#[doc = "AWCR (rw) register accessor: Active Width Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`awcr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`awcr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`awcr`]
module"]
pub type AWCR = crate::Reg<awcr::AWCR_SPEC>;
#[doc = "Active Width Configuration Register"]
pub mod awcr;
#[doc = "TWCR (rw) register accessor: Total Width Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`twcr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`twcr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`twcr`]
module"]
pub type TWCR = crate::Reg<twcr::TWCR_SPEC>;
#[doc = "Total Width Configuration Register"]
pub mod twcr;
#[doc = "GCR (rw) register accessor: Global Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gcr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gcr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`gcr`]
module"]
pub type GCR = crate::Reg<gcr::GCR_SPEC>;
#[doc = "Global Control Register"]
pub mod gcr;
#[doc = "SRCR (rw) register accessor: Shadow Reload Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`srcr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`srcr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`srcr`]
module"]
pub type SRCR = crate::Reg<srcr::SRCR_SPEC>;
#[doc = "Shadow Reload Configuration Register"]
pub mod srcr;
#[doc = "BCCR (rw) register accessor: Background Color Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bccr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bccr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`bccr`]
module"]
pub type BCCR = crate::Reg<bccr::BCCR_SPEC>;
#[doc = "Background Color Configuration Register"]
pub mod bccr;
#[doc = "IER (rw) register accessor: Interrupt Enable Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ier::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ier::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`ier`]
module"]
pub type IER = crate::Reg<ier::IER_SPEC>;
#[doc = "Interrupt Enable Register"]
pub mod ier;
#[doc = "ISR (r) register accessor: Interrupt Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`isr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`isr`]
module"]
pub type ISR = crate::Reg<isr::ISR_SPEC>;
#[doc = "Interrupt Status Register"]
pub mod isr;
#[doc = "ICR (w) register accessor: Interrupt Clear Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`icr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`icr`]
module"]
pub type ICR = crate::Reg<icr::ICR_SPEC>;
#[doc = "Interrupt Clear Register"]
pub mod icr;
#[doc = "LIPCR (rw) register accessor: Line Interrupt Position Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lipcr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lipcr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`lipcr`]
module"]
pub type LIPCR = crate::Reg<lipcr::LIPCR_SPEC>;
#[doc = "Line Interrupt Position Configuration Register"]
pub mod lipcr;
#[doc = "CPSR (r) register accessor: Current Position Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cpsr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`cpsr`]
module"]
pub type CPSR = crate::Reg<cpsr::CPSR_SPEC>;
#[doc = "Current Position Status Register"]
pub mod cpsr;
#[doc = "CDSR (r) register accessor: Current Display Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cdsr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`cdsr`]
module"]
pub type CDSR = crate::Reg<cdsr::CDSR_SPEC>;
#[doc = "Current Display Status Register"]
pub mod cdsr;
#[doc = "L1CR (rw) register accessor: Layerx Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`l1cr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`l1cr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`l1cr`]
module"]
pub type L1CR = crate::Reg<l1cr::L1CR_SPEC>;
#[doc = "Layerx Control Register"]
pub mod l1cr;
#[doc = "L1WHPCR (rw) register accessor: Layerx Window Horizontal Position Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`l1whpcr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`l1whpcr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`l1whpcr`]
module"]
pub type L1WHPCR = crate::Reg<l1whpcr::L1WHPCR_SPEC>;
#[doc = "Layerx Window Horizontal Position Configuration Register"]
pub mod l1whpcr;
#[doc = "L1WVPCR (rw) register accessor: Layerx Window Vertical Position Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`l1wvpcr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`l1wvpcr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`l1wvpcr`]
module"]
pub type L1WVPCR = crate::Reg<l1wvpcr::L1WVPCR_SPEC>;
#[doc = "Layerx Window Vertical Position Configuration Register"]
pub mod l1wvpcr;
#[doc = "L1CKCR (rw) register accessor: Layerx Color Keying Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`l1ckcr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`l1ckcr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`l1ckcr`]
module"]
pub type L1CKCR = crate::Reg<l1ckcr::L1CKCR_SPEC>;
#[doc = "Layerx Color Keying Configuration Register"]
pub mod l1ckcr;
#[doc = "L1PFCR (rw) register accessor: Layerx Pixel Format Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`l1pfcr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`l1pfcr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`l1pfcr`]
module"]
pub type L1PFCR = crate::Reg<l1pfcr::L1PFCR_SPEC>;
#[doc = "Layerx Pixel Format Configuration Register"]
pub mod l1pfcr;
#[doc = "L1CACR (rw) register accessor: Layerx Constant Alpha Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`l1cacr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`l1cacr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`l1cacr`]
module"]
pub type L1CACR = crate::Reg<l1cacr::L1CACR_SPEC>;
#[doc = "Layerx Constant Alpha Configuration Register"]
pub mod l1cacr;
#[doc = "L1DCCR (rw) register accessor: Layerx Default Color Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`l1dccr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`l1dccr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`l1dccr`]
module"]
pub type L1DCCR = crate::Reg<l1dccr::L1DCCR_SPEC>;
#[doc = "Layerx Default Color Configuration Register"]
pub mod l1dccr;
#[doc = "L1BFCR (rw) register accessor: Layerx Blending Factors Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`l1bfcr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`l1bfcr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`l1bfcr`]
module"]
pub type L1BFCR = crate::Reg<l1bfcr::L1BFCR_SPEC>;
#[doc = "Layerx Blending Factors Configuration Register"]
pub mod l1bfcr;
#[doc = "L1CFBAR (rw) register accessor: Layerx Color Frame Buffer Address Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`l1cfbar::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`l1cfbar::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`l1cfbar`]
module"]
pub type L1CFBAR = crate::Reg<l1cfbar::L1CFBAR_SPEC>;
#[doc = "Layerx Color Frame Buffer Address Register"]
pub mod l1cfbar;
#[doc = "L1CFBLR (rw) register accessor: Layerx Color Frame Buffer Length Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`l1cfblr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`l1cfblr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`l1cfblr`]
module"]
pub type L1CFBLR = crate::Reg<l1cfblr::L1CFBLR_SPEC>;
#[doc = "Layerx Color Frame Buffer Length Register"]
pub mod l1cfblr;
#[doc = "L1CFBLNR (rw) register accessor: Layerx ColorFrame Buffer Line Number Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`l1cfblnr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`l1cfblnr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`l1cfblnr`]
module"]
pub type L1CFBLNR = crate::Reg<l1cfblnr::L1CFBLNR_SPEC>;
#[doc = "Layerx ColorFrame Buffer Line Number Register"]
pub mod l1cfblnr;
#[doc = "L1CLUTWR (w) register accessor: Layerx CLUT Write Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`l1clutwr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`l1clutwr`]
module"]
pub type L1CLUTWR = crate::Reg<l1clutwr::L1CLUTWR_SPEC>;
#[doc = "Layerx CLUT Write Register"]
pub mod l1clutwr;
#[doc = "L2CR (rw) register accessor: Layerx Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`l2cr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`l2cr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`l2cr`]
module"]
pub type L2CR = crate::Reg<l2cr::L2CR_SPEC>;
#[doc = "Layerx Control Register"]
pub mod l2cr;
#[doc = "L2WHPCR (rw) register accessor: Layerx Window Horizontal Position Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`l2whpcr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`l2whpcr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`l2whpcr`]
module"]
pub type L2WHPCR = crate::Reg<l2whpcr::L2WHPCR_SPEC>;
#[doc = "Layerx Window Horizontal Position Configuration Register"]
pub mod l2whpcr;
#[doc = "L2WVPCR (rw) register accessor: Layerx Window Vertical Position Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`l2wvpcr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`l2wvpcr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`l2wvpcr`]
module"]
pub type L2WVPCR = crate::Reg<l2wvpcr::L2WVPCR_SPEC>;
#[doc = "Layerx Window Vertical Position Configuration Register"]
pub mod l2wvpcr;
#[doc = "L2CKCR (rw) register accessor: Layerx Color Keying Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`l2ckcr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`l2ckcr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`l2ckcr`]
module"]
pub type L2CKCR = crate::Reg<l2ckcr::L2CKCR_SPEC>;
#[doc = "Layerx Color Keying Configuration Register"]
pub mod l2ckcr;
#[doc = "L2PFCR (rw) register accessor: Layerx Pixel Format Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`l2pfcr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`l2pfcr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`l2pfcr`]
module"]
pub type L2PFCR = crate::Reg<l2pfcr::L2PFCR_SPEC>;
#[doc = "Layerx Pixel Format Configuration Register"]
pub mod l2pfcr;
#[doc = "L2CACR (rw) register accessor: Layerx Constant Alpha Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`l2cacr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`l2cacr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`l2cacr`]
module"]
pub type L2CACR = crate::Reg<l2cacr::L2CACR_SPEC>;
#[doc = "Layerx Constant Alpha Configuration Register"]
pub mod l2cacr;
#[doc = "L2DCCR (rw) register accessor: Layerx Default Color Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`l2dccr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`l2dccr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`l2dccr`]
module"]
pub type L2DCCR = crate::Reg<l2dccr::L2DCCR_SPEC>;
#[doc = "Layerx Default Color Configuration Register"]
pub mod l2dccr;
#[doc = "L2BFCR (rw) register accessor: Layerx Blending Factors Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`l2bfcr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`l2bfcr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`l2bfcr`]
module"]
pub type L2BFCR = crate::Reg<l2bfcr::L2BFCR_SPEC>;
#[doc = "Layerx Blending Factors Configuration Register"]
pub mod l2bfcr;
#[doc = "L2CFBAR (rw) register accessor: Layerx Color Frame Buffer Address Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`l2cfbar::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`l2cfbar::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`l2cfbar`]
module"]
pub type L2CFBAR = crate::Reg<l2cfbar::L2CFBAR_SPEC>;
#[doc = "Layerx Color Frame Buffer Address Register"]
pub mod l2cfbar;
#[doc = "L2CFBLR (rw) register accessor: Layerx Color Frame Buffer Length Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`l2cfblr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`l2cfblr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`l2cfblr`]
module"]
pub type L2CFBLR = crate::Reg<l2cfblr::L2CFBLR_SPEC>;
#[doc = "Layerx Color Frame Buffer Length Register"]
pub mod l2cfblr;
#[doc = "L2CFBLNR (rw) register accessor: Layerx ColorFrame Buffer Line Number Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`l2cfblnr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`l2cfblnr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`l2cfblnr`]
module"]
pub type L2CFBLNR = crate::Reg<l2cfblnr::L2CFBLNR_SPEC>;
#[doc = "Layerx ColorFrame Buffer Line Number Register"]
pub mod l2cfblnr;
#[doc = "L2CLUTWR (w) register accessor: Layerx CLUT Write Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`l2clutwr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`l2clutwr`]
module"]
pub type L2CLUTWR = crate::Reg<l2clutwr::L2CLUTWR_SPEC>;
#[doc = "Layerx CLUT Write Register"]
pub mod l2clutwr;
