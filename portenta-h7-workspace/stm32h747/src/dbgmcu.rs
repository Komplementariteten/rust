#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - DBGMCU Identity Code Register"]
    pub idc: IDC,
    #[doc = "0x04 - DBGMCU Configuration Register"]
    pub cr: CR,
    _reserved2: [u8; 0x2c],
    #[doc = "0x34 - DBGMCU APB3 peripheral freeze register CPU1"]
    pub apb3fz1: APB3FZ1,
    #[doc = "0x38 - DBGMCU APB3 peripheral freeze register CPU2"]
    pub apb3fz2: APB3FZ2,
    #[doc = "0x3c - DBGMCU APB1L peripheral freeze register"]
    pub apb1lfz1: APB1LFZ1,
    #[doc = "0x40 - DBGMCU APB1L peripheral freeze register CPU2"]
    pub apb1lfz2: APB1LFZ2,
    _reserved6: [u8; 0x08],
    #[doc = "0x4c - DBGMCU APB2 peripheral freeze register"]
    pub apb2fz1: APB2FZ1,
    #[doc = "0x50 - DBGMCU APB2 peripheral freeze register CPU2"]
    pub apb2fz2: APB2FZ2,
    #[doc = "0x54 - DBGMCU APB4 peripheral freeze register"]
    pub apb4fz1: APB4FZ1,
    #[doc = "0x58 - DBGMCU APB4 peripheral freeze register CPU2"]
    pub apb4fz2: APB4FZ2,
}
#[doc = "IDC (r) register accessor: DBGMCU Identity Code Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`idc::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`idc`]
module"]
pub type IDC = crate::Reg<idc::IDC_SPEC>;
#[doc = "DBGMCU Identity Code Register"]
pub mod idc;
#[doc = "CR (rw) register accessor: DBGMCU Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`cr`]
module"]
pub type CR = crate::Reg<cr::CR_SPEC>;
#[doc = "DBGMCU Configuration Register"]
pub mod cr;
#[doc = "APB3FZ1 (rw) register accessor: DBGMCU APB3 peripheral freeze register CPU1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apb3fz1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apb3fz1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`apb3fz1`]
module"]
pub type APB3FZ1 = crate::Reg<apb3fz1::APB3FZ1_SPEC>;
#[doc = "DBGMCU APB3 peripheral freeze register CPU1"]
pub mod apb3fz1;
#[doc = "APB3FZ2 (rw) register accessor: DBGMCU APB3 peripheral freeze register CPU2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apb3fz2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apb3fz2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`apb3fz2`]
module"]
pub type APB3FZ2 = crate::Reg<apb3fz2::APB3FZ2_SPEC>;
#[doc = "DBGMCU APB3 peripheral freeze register CPU2"]
pub mod apb3fz2;
#[doc = "APB1LFZ1 (rw) register accessor: DBGMCU APB1L peripheral freeze register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apb1lfz1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apb1lfz1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`apb1lfz1`]
module"]
pub type APB1LFZ1 = crate::Reg<apb1lfz1::APB1LFZ1_SPEC>;
#[doc = "DBGMCU APB1L peripheral freeze register"]
pub mod apb1lfz1;
#[doc = "APB1LFZ2 (rw) register accessor: DBGMCU APB1L peripheral freeze register CPU2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apb1lfz2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apb1lfz2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`apb1lfz2`]
module"]
pub type APB1LFZ2 = crate::Reg<apb1lfz2::APB1LFZ2_SPEC>;
#[doc = "DBGMCU APB1L peripheral freeze register CPU2"]
pub mod apb1lfz2;
#[doc = "APB2FZ1 (rw) register accessor: DBGMCU APB2 peripheral freeze register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apb2fz1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apb2fz1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`apb2fz1`]
module"]
pub type APB2FZ1 = crate::Reg<apb2fz1::APB2FZ1_SPEC>;
#[doc = "DBGMCU APB2 peripheral freeze register"]
pub mod apb2fz1;
#[doc = "APB2FZ2 (rw) register accessor: DBGMCU APB2 peripheral freeze register CPU2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apb2fz2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apb2fz2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`apb2fz2`]
module"]
pub type APB2FZ2 = crate::Reg<apb2fz2::APB2FZ2_SPEC>;
#[doc = "DBGMCU APB2 peripheral freeze register CPU2"]
pub mod apb2fz2;
#[doc = "APB4FZ1 (rw) register accessor: DBGMCU APB4 peripheral freeze register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apb4fz1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apb4fz1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`apb4fz1`]
module"]
pub type APB4FZ1 = crate::Reg<apb4fz1::APB4FZ1_SPEC>;
#[doc = "DBGMCU APB4 peripheral freeze register"]
pub mod apb4fz1;
#[doc = "APB4FZ2 (rw) register accessor: DBGMCU APB4 peripheral freeze register CPU2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apb4fz2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apb4fz2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`apb4fz2`]
module"]
pub type APB4FZ2 = crate::Reg<apb4fz2::APB4FZ2_SPEC>;
#[doc = "DBGMCU APB4 peripheral freeze register CPU2"]
pub mod apb4fz2;
