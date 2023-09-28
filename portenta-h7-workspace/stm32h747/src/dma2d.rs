#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - DMA2D control register"]
    pub cr: CR,
    #[doc = "0x04 - DMA2D Interrupt Status Register"]
    pub isr: ISR,
    #[doc = "0x08 - DMA2D interrupt flag clear register"]
    pub ifcr: IFCR,
    #[doc = "0x0c - DMA2D foreground memory address register"]
    pub fgmar: FGMAR,
    #[doc = "0x10 - DMA2D foreground offset register"]
    pub fgor: FGOR,
    #[doc = "0x14 - DMA2D background memory address register"]
    pub bgmar: BGMAR,
    #[doc = "0x18 - DMA2D background offset register"]
    pub bgor: BGOR,
    #[doc = "0x1c - DMA2D foreground PFC control register"]
    pub fgpfccr: FGPFCCR,
    #[doc = "0x20 - DMA2D foreground color register"]
    pub fgcolr: FGCOLR,
    #[doc = "0x24 - DMA2D background PFC control register"]
    pub bgpfccr: BGPFCCR,
    #[doc = "0x28 - DMA2D background color register"]
    pub bgcolr: BGCOLR,
    #[doc = "0x2c - DMA2D foreground CLUT memory address register"]
    pub fgcmar: FGCMAR,
    #[doc = "0x30 - DMA2D background CLUT memory address register"]
    pub bgcmar: BGCMAR,
    #[doc = "0x34 - DMA2D output PFC control register"]
    pub opfccr: OPFCCR,
    #[doc = "0x38 - DMA2D output color register"]
    pub ocolr: OCOLR,
    #[doc = "0x3c - DMA2D output memory address register"]
    pub omar: OMAR,
    #[doc = "0x40 - DMA2D output offset register"]
    pub oor: OOR,
    #[doc = "0x44 - DMA2D number of line register"]
    pub nlr: NLR,
    #[doc = "0x48 - DMA2D line watermark register"]
    pub lwr: LWR,
    #[doc = "0x4c - DMA2D AXI master timer configuration register"]
    pub amtcr: AMTCR,
}
#[doc = "CR (rw) register accessor: DMA2D control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`cr`]
module"]
pub type CR = crate::Reg<cr::CR_SPEC>;
#[doc = "DMA2D control register"]
pub mod cr;
#[doc = "ISR (r) register accessor: DMA2D Interrupt Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`isr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`isr`]
module"]
pub type ISR = crate::Reg<isr::ISR_SPEC>;
#[doc = "DMA2D Interrupt Status Register"]
pub mod isr;
#[doc = "IFCR (rw) register accessor: DMA2D interrupt flag clear register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ifcr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ifcr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`ifcr`]
module"]
pub type IFCR = crate::Reg<ifcr::IFCR_SPEC>;
#[doc = "DMA2D interrupt flag clear register"]
pub mod ifcr;
#[doc = "FGMAR (rw) register accessor: DMA2D foreground memory address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fgmar::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fgmar::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`fgmar`]
module"]
pub type FGMAR = crate::Reg<fgmar::FGMAR_SPEC>;
#[doc = "DMA2D foreground memory address register"]
pub mod fgmar;
#[doc = "FGOR (rw) register accessor: DMA2D foreground offset register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fgor::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fgor::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`fgor`]
module"]
pub type FGOR = crate::Reg<fgor::FGOR_SPEC>;
#[doc = "DMA2D foreground offset register"]
pub mod fgor;
#[doc = "BGMAR (rw) register accessor: DMA2D background memory address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bgmar::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bgmar::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`bgmar`]
module"]
pub type BGMAR = crate::Reg<bgmar::BGMAR_SPEC>;
#[doc = "DMA2D background memory address register"]
pub mod bgmar;
#[doc = "BGOR (rw) register accessor: DMA2D background offset register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bgor::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bgor::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`bgor`]
module"]
pub type BGOR = crate::Reg<bgor::BGOR_SPEC>;
#[doc = "DMA2D background offset register"]
pub mod bgor;
#[doc = "FGPFCCR (rw) register accessor: DMA2D foreground PFC control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fgpfccr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fgpfccr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`fgpfccr`]
module"]
pub type FGPFCCR = crate::Reg<fgpfccr::FGPFCCR_SPEC>;
#[doc = "DMA2D foreground PFC control register"]
pub mod fgpfccr;
#[doc = "FGCOLR (rw) register accessor: DMA2D foreground color register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fgcolr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fgcolr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`fgcolr`]
module"]
pub type FGCOLR = crate::Reg<fgcolr::FGCOLR_SPEC>;
#[doc = "DMA2D foreground color register"]
pub mod fgcolr;
#[doc = "BGPFCCR (rw) register accessor: DMA2D background PFC control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bgpfccr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bgpfccr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`bgpfccr`]
module"]
pub type BGPFCCR = crate::Reg<bgpfccr::BGPFCCR_SPEC>;
#[doc = "DMA2D background PFC control register"]
pub mod bgpfccr;
#[doc = "BGCOLR (rw) register accessor: DMA2D background color register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bgcolr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bgcolr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`bgcolr`]
module"]
pub type BGCOLR = crate::Reg<bgcolr::BGCOLR_SPEC>;
#[doc = "DMA2D background color register"]
pub mod bgcolr;
#[doc = "FGCMAR (rw) register accessor: DMA2D foreground CLUT memory address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fgcmar::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fgcmar::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`fgcmar`]
module"]
pub type FGCMAR = crate::Reg<fgcmar::FGCMAR_SPEC>;
#[doc = "DMA2D foreground CLUT memory address register"]
pub mod fgcmar;
#[doc = "BGCMAR (rw) register accessor: DMA2D background CLUT memory address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bgcmar::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bgcmar::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`bgcmar`]
module"]
pub type BGCMAR = crate::Reg<bgcmar::BGCMAR_SPEC>;
#[doc = "DMA2D background CLUT memory address register"]
pub mod bgcmar;
#[doc = "OPFCCR (rw) register accessor: DMA2D output PFC control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`opfccr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`opfccr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`opfccr`]
module"]
pub type OPFCCR = crate::Reg<opfccr::OPFCCR_SPEC>;
#[doc = "DMA2D output PFC control register"]
pub mod opfccr;
#[doc = "OCOLR (rw) register accessor: DMA2D output color register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ocolr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ocolr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`ocolr`]
module"]
pub type OCOLR = crate::Reg<ocolr::OCOLR_SPEC>;
#[doc = "DMA2D output color register"]
pub mod ocolr;
#[doc = "OMAR (rw) register accessor: DMA2D output memory address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`omar::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`omar::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`omar`]
module"]
pub type OMAR = crate::Reg<omar::OMAR_SPEC>;
#[doc = "DMA2D output memory address register"]
pub mod omar;
#[doc = "OOR (rw) register accessor: DMA2D output offset register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`oor::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`oor::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`oor`]
module"]
pub type OOR = crate::Reg<oor::OOR_SPEC>;
#[doc = "DMA2D output offset register"]
pub mod oor;
#[doc = "NLR (rw) register accessor: DMA2D number of line register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`nlr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`nlr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`nlr`]
module"]
pub type NLR = crate::Reg<nlr::NLR_SPEC>;
#[doc = "DMA2D number of line register"]
pub mod nlr;
#[doc = "LWR (rw) register accessor: DMA2D line watermark register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lwr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lwr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`lwr`]
module"]
pub type LWR = crate::Reg<lwr::LWR_SPEC>;
#[doc = "DMA2D line watermark register"]
pub mod lwr;
#[doc = "AMTCR (rw) register accessor: DMA2D AXI master timer configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`amtcr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`amtcr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`amtcr`]
module"]
pub type AMTCR = crate::Reg<amtcr::AMTCR_SPEC>;
#[doc = "DMA2D AXI master timer configuration register"]
pub mod amtcr;
