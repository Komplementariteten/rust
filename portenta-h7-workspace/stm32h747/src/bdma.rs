#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - DMA interrupt status register"]
    pub isr: ISR,
    #[doc = "0x04 - DMA interrupt flag clear register"]
    pub ifcr: IFCR,
    #[doc = "0x08 - DMA channel x configuration register"]
    pub ccr1: CCR1,
    #[doc = "0x0c - DMA channel x number of data register"]
    pub cndtr1: CNDTR1,
    #[doc = "0x10 - This register must not be written when the channel is enabled."]
    pub cpar1: CPAR1,
    #[doc = "0x14 - This register must not be written when the channel is enabled."]
    pub cmar1: CMAR1,
    _reserved6: [u8; 0x04],
    #[doc = "0x1c - DMA channel x configuration register"]
    pub ccr2: CCR2,
    #[doc = "0x20 - DMA channel x number of data register"]
    pub cndtr2: CNDTR2,
    #[doc = "0x24 - This register must not be written when the channel is enabled."]
    pub cpar2: CPAR2,
    #[doc = "0x28 - This register must not be written when the channel is enabled."]
    pub cmar2: CMAR2,
    _reserved10: [u8; 0x04],
    #[doc = "0x30 - DMA channel x configuration register"]
    pub ccr3: CCR3,
    #[doc = "0x34 - DMA channel x number of data register"]
    pub cndtr3: CNDTR3,
    #[doc = "0x38 - This register must not be written when the channel is enabled."]
    pub cpar3: CPAR3,
    #[doc = "0x3c - This register must not be written when the channel is enabled."]
    pub cmar3: CMAR3,
    _reserved14: [u8; 0x04],
    #[doc = "0x44 - DMA channel x configuration register"]
    pub ccr4: CCR4,
    #[doc = "0x48 - DMA channel x number of data register"]
    pub cndtr4: CNDTR4,
    #[doc = "0x4c - This register must not be written when the channel is enabled."]
    pub cpar4: CPAR4,
    #[doc = "0x50 - This register must not be written when the channel is enabled."]
    pub cmar4: CMAR4,
    _reserved18: [u8; 0x04],
    #[doc = "0x58 - DMA channel x configuration register"]
    pub ccr5: CCR5,
    #[doc = "0x5c - DMA channel x number of data register"]
    pub cndtr5: CNDTR5,
    #[doc = "0x60 - This register must not be written when the channel is enabled."]
    pub cpar5: CPAR5,
    #[doc = "0x64 - This register must not be written when the channel is enabled."]
    pub cmar5: CMAR5,
    _reserved22: [u8; 0x04],
    #[doc = "0x6c - DMA channel x configuration register"]
    pub ccr6: CCR6,
    #[doc = "0x70 - DMA channel x number of data register"]
    pub cndtr6: CNDTR6,
    #[doc = "0x74 - This register must not be written when the channel is enabled."]
    pub cpar6: CPAR6,
    #[doc = "0x78 - This register must not be written when the channel is enabled."]
    pub cmar6: CMAR6,
    _reserved26: [u8; 0x04],
    #[doc = "0x80 - DMA channel x configuration register"]
    pub ccr7: CCR7,
    #[doc = "0x84 - DMA channel x number of data register"]
    pub cndtr7: CNDTR7,
    #[doc = "0x88 - This register must not be written when the channel is enabled."]
    pub cpar7: CPAR7,
    #[doc = "0x8c - This register must not be written when the channel is enabled."]
    pub cmar7: CMAR7,
    _reserved30: [u8; 0x04],
    #[doc = "0x94 - DMA channel x configuration register"]
    pub ccr8: CCR8,
    #[doc = "0x98 - DMA channel x number of data register"]
    pub cndtr8: CNDTR8,
    #[doc = "0x9c - This register must not be written when the channel is enabled."]
    pub cpar8: CPAR8,
    #[doc = "0xa0 - This register must not be written when the channel is enabled."]
    pub cmar8: CMAR8,
}
#[doc = "ISR (r) register accessor: DMA interrupt status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`isr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`isr`]
module"]
pub type ISR = crate::Reg<isr::ISR_SPEC>;
#[doc = "DMA interrupt status register"]
pub mod isr;
#[doc = "IFCR (w) register accessor: DMA interrupt flag clear register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ifcr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`ifcr`]
module"]
pub type IFCR = crate::Reg<ifcr::IFCR_SPEC>;
#[doc = "DMA interrupt flag clear register"]
pub mod ifcr;
#[doc = "CCR1 (rw) register accessor: DMA channel x configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ccr1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ccr1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`ccr1`]
module"]
pub type CCR1 = crate::Reg<ccr1::CCR1_SPEC>;
#[doc = "DMA channel x configuration register"]
pub mod ccr1;
#[doc = "CNDTR1 (rw) register accessor: DMA channel x number of data register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cndtr1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cndtr1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`cndtr1`]
module"]
pub type CNDTR1 = crate::Reg<cndtr1::CNDTR1_SPEC>;
#[doc = "DMA channel x number of data register"]
pub mod cndtr1;
#[doc = "CPAR1 (rw) register accessor: This register must not be written when the channel is enabled.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cpar1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cpar1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`cpar1`]
module"]
pub type CPAR1 = crate::Reg<cpar1::CPAR1_SPEC>;
#[doc = "This register must not be written when the channel is enabled."]
pub mod cpar1;
#[doc = "CMAR1 (rw) register accessor: This register must not be written when the channel is enabled.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cmar1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cmar1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`cmar1`]
module"]
pub type CMAR1 = crate::Reg<cmar1::CMAR1_SPEC>;
#[doc = "This register must not be written when the channel is enabled."]
pub mod cmar1;
#[doc = "CCR2 (rw) register accessor: DMA channel x configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ccr2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ccr2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`ccr2`]
module"]
pub type CCR2 = crate::Reg<ccr2::CCR2_SPEC>;
#[doc = "DMA channel x configuration register"]
pub mod ccr2;
#[doc = "CNDTR2 (rw) register accessor: DMA channel x number of data register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cndtr2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cndtr2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`cndtr2`]
module"]
pub type CNDTR2 = crate::Reg<cndtr2::CNDTR2_SPEC>;
#[doc = "DMA channel x number of data register"]
pub mod cndtr2;
#[doc = "CPAR2 (rw) register accessor: This register must not be written when the channel is enabled.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cpar2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cpar2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`cpar2`]
module"]
pub type CPAR2 = crate::Reg<cpar2::CPAR2_SPEC>;
#[doc = "This register must not be written when the channel is enabled."]
pub mod cpar2;
#[doc = "CMAR2 (rw) register accessor: This register must not be written when the channel is enabled.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cmar2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cmar2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`cmar2`]
module"]
pub type CMAR2 = crate::Reg<cmar2::CMAR2_SPEC>;
#[doc = "This register must not be written when the channel is enabled."]
pub mod cmar2;
#[doc = "CCR3 (rw) register accessor: DMA channel x configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ccr3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ccr3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`ccr3`]
module"]
pub type CCR3 = crate::Reg<ccr3::CCR3_SPEC>;
#[doc = "DMA channel x configuration register"]
pub mod ccr3;
#[doc = "CNDTR3 (rw) register accessor: DMA channel x number of data register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cndtr3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cndtr3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`cndtr3`]
module"]
pub type CNDTR3 = crate::Reg<cndtr3::CNDTR3_SPEC>;
#[doc = "DMA channel x number of data register"]
pub mod cndtr3;
#[doc = "CPAR3 (rw) register accessor: This register must not be written when the channel is enabled.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cpar3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cpar3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`cpar3`]
module"]
pub type CPAR3 = crate::Reg<cpar3::CPAR3_SPEC>;
#[doc = "This register must not be written when the channel is enabled."]
pub mod cpar3;
#[doc = "CMAR3 (rw) register accessor: This register must not be written when the channel is enabled.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cmar3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cmar3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`cmar3`]
module"]
pub type CMAR3 = crate::Reg<cmar3::CMAR3_SPEC>;
#[doc = "This register must not be written when the channel is enabled."]
pub mod cmar3;
#[doc = "CCR4 (rw) register accessor: DMA channel x configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ccr4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ccr4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`ccr4`]
module"]
pub type CCR4 = crate::Reg<ccr4::CCR4_SPEC>;
#[doc = "DMA channel x configuration register"]
pub mod ccr4;
#[doc = "CNDTR4 (rw) register accessor: DMA channel x number of data register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cndtr4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cndtr4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`cndtr4`]
module"]
pub type CNDTR4 = crate::Reg<cndtr4::CNDTR4_SPEC>;
#[doc = "DMA channel x number of data register"]
pub mod cndtr4;
#[doc = "CPAR4 (rw) register accessor: This register must not be written when the channel is enabled.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cpar4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cpar4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`cpar4`]
module"]
pub type CPAR4 = crate::Reg<cpar4::CPAR4_SPEC>;
#[doc = "This register must not be written when the channel is enabled."]
pub mod cpar4;
#[doc = "CMAR4 (rw) register accessor: This register must not be written when the channel is enabled.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cmar4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cmar4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`cmar4`]
module"]
pub type CMAR4 = crate::Reg<cmar4::CMAR4_SPEC>;
#[doc = "This register must not be written when the channel is enabled."]
pub mod cmar4;
#[doc = "CCR5 (rw) register accessor: DMA channel x configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ccr5::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ccr5::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`ccr5`]
module"]
pub type CCR5 = crate::Reg<ccr5::CCR5_SPEC>;
#[doc = "DMA channel x configuration register"]
pub mod ccr5;
#[doc = "CNDTR5 (rw) register accessor: DMA channel x number of data register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cndtr5::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cndtr5::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`cndtr5`]
module"]
pub type CNDTR5 = crate::Reg<cndtr5::CNDTR5_SPEC>;
#[doc = "DMA channel x number of data register"]
pub mod cndtr5;
#[doc = "CPAR5 (rw) register accessor: This register must not be written when the channel is enabled.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cpar5::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cpar5::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`cpar5`]
module"]
pub type CPAR5 = crate::Reg<cpar5::CPAR5_SPEC>;
#[doc = "This register must not be written when the channel is enabled."]
pub mod cpar5;
#[doc = "CMAR5 (rw) register accessor: This register must not be written when the channel is enabled.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cmar5::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cmar5::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`cmar5`]
module"]
pub type CMAR5 = crate::Reg<cmar5::CMAR5_SPEC>;
#[doc = "This register must not be written when the channel is enabled."]
pub mod cmar5;
#[doc = "CCR6 (rw) register accessor: DMA channel x configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ccr6::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ccr6::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`ccr6`]
module"]
pub type CCR6 = crate::Reg<ccr6::CCR6_SPEC>;
#[doc = "DMA channel x configuration register"]
pub mod ccr6;
#[doc = "CNDTR6 (rw) register accessor: DMA channel x number of data register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cndtr6::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cndtr6::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`cndtr6`]
module"]
pub type CNDTR6 = crate::Reg<cndtr6::CNDTR6_SPEC>;
#[doc = "DMA channel x number of data register"]
pub mod cndtr6;
#[doc = "CPAR6 (rw) register accessor: This register must not be written when the channel is enabled.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cpar6::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cpar6::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`cpar6`]
module"]
pub type CPAR6 = crate::Reg<cpar6::CPAR6_SPEC>;
#[doc = "This register must not be written when the channel is enabled."]
pub mod cpar6;
#[doc = "CMAR6 (rw) register accessor: This register must not be written when the channel is enabled.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cmar6::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cmar6::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`cmar6`]
module"]
pub type CMAR6 = crate::Reg<cmar6::CMAR6_SPEC>;
#[doc = "This register must not be written when the channel is enabled."]
pub mod cmar6;
#[doc = "CCR7 (rw) register accessor: DMA channel x configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ccr7::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ccr7::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`ccr7`]
module"]
pub type CCR7 = crate::Reg<ccr7::CCR7_SPEC>;
#[doc = "DMA channel x configuration register"]
pub mod ccr7;
#[doc = "CNDTR7 (rw) register accessor: DMA channel x number of data register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cndtr7::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cndtr7::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`cndtr7`]
module"]
pub type CNDTR7 = crate::Reg<cndtr7::CNDTR7_SPEC>;
#[doc = "DMA channel x number of data register"]
pub mod cndtr7;
#[doc = "CPAR7 (rw) register accessor: This register must not be written when the channel is enabled.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cpar7::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cpar7::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`cpar7`]
module"]
pub type CPAR7 = crate::Reg<cpar7::CPAR7_SPEC>;
#[doc = "This register must not be written when the channel is enabled."]
pub mod cpar7;
#[doc = "CMAR7 (rw) register accessor: This register must not be written when the channel is enabled.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cmar7::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cmar7::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`cmar7`]
module"]
pub type CMAR7 = crate::Reg<cmar7::CMAR7_SPEC>;
#[doc = "This register must not be written when the channel is enabled."]
pub mod cmar7;
#[doc = "CCR8 (rw) register accessor: DMA channel x configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ccr8::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ccr8::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`ccr8`]
module"]
pub type CCR8 = crate::Reg<ccr8::CCR8_SPEC>;
#[doc = "DMA channel x configuration register"]
pub mod ccr8;
#[doc = "CNDTR8 (rw) register accessor: DMA channel x number of data register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cndtr8::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cndtr8::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`cndtr8`]
module"]
pub type CNDTR8 = crate::Reg<cndtr8::CNDTR8_SPEC>;
#[doc = "DMA channel x number of data register"]
pub mod cndtr8;
#[doc = "CPAR8 (rw) register accessor: This register must not be written when the channel is enabled.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cpar8::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cpar8::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`cpar8`]
module"]
pub type CPAR8 = crate::Reg<cpar8::CPAR8_SPEC>;
#[doc = "This register must not be written when the channel is enabled."]
pub mod cpar8;
#[doc = "CMAR8 (rw) register accessor: This register must not be written when the channel is enabled.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cmar8::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cmar8::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`cmar8`]
module"]
pub type CMAR8 = crate::Reg<cmar8::CMAR8_SPEC>;
#[doc = "This register must not be written when the channel is enabled."]
pub mod cmar8;
