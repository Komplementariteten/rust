#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - control register 1"]
    pub cr: CR,
    #[doc = "0x04 - status register"]
    pub sr: SR,
    #[doc = "0x08 - raw interrupt status register"]
    pub ris: RIS,
    #[doc = "0x0c - interrupt enable register"]
    pub ier: IER,
    #[doc = "0x10 - masked interrupt status register"]
    pub mis: MIS,
    #[doc = "0x14 - interrupt clear register"]
    pub icr: ICR,
    #[doc = "0x18 - embedded synchronization code register"]
    pub escr: ESCR,
    #[doc = "0x1c - embedded synchronization unmask register"]
    pub esur: ESUR,
    #[doc = "0x20 - crop window start"]
    pub cwstrt: CWSTRT,
    #[doc = "0x24 - crop window size"]
    pub cwsize: CWSIZE,
    #[doc = "0x28 - data register"]
    pub dr: DR,
}
#[doc = "CR (rw) register accessor: control register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`cr`]
module"]
pub type CR = crate::Reg<cr::CR_SPEC>;
#[doc = "control register 1"]
pub mod cr;
#[doc = "SR (r) register accessor: status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`sr`]
module"]
pub type SR = crate::Reg<sr::SR_SPEC>;
#[doc = "status register"]
pub mod sr;
#[doc = "RIS (r) register accessor: raw interrupt status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ris::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`ris`]
module"]
pub type RIS = crate::Reg<ris::RIS_SPEC>;
#[doc = "raw interrupt status register"]
pub mod ris;
#[doc = "IER (rw) register accessor: interrupt enable register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ier::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ier::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`ier`]
module"]
pub type IER = crate::Reg<ier::IER_SPEC>;
#[doc = "interrupt enable register"]
pub mod ier;
#[doc = "MIS (r) register accessor: masked interrupt status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mis::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mis`]
module"]
pub type MIS = crate::Reg<mis::MIS_SPEC>;
#[doc = "masked interrupt status register"]
pub mod mis;
#[doc = "ICR (w) register accessor: interrupt clear register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`icr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`icr`]
module"]
pub type ICR = crate::Reg<icr::ICR_SPEC>;
#[doc = "interrupt clear register"]
pub mod icr;
#[doc = "ESCR (rw) register accessor: embedded synchronization code register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`escr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`escr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`escr`]
module"]
pub type ESCR = crate::Reg<escr::ESCR_SPEC>;
#[doc = "embedded synchronization code register"]
pub mod escr;
#[doc = "ESUR (rw) register accessor: embedded synchronization unmask register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`esur::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`esur::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`esur`]
module"]
pub type ESUR = crate::Reg<esur::ESUR_SPEC>;
#[doc = "embedded synchronization unmask register"]
pub mod esur;
#[doc = "CWSTRT (rw) register accessor: crop window start\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cwstrt::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cwstrt::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`cwstrt`]
module"]
pub type CWSTRT = crate::Reg<cwstrt::CWSTRT_SPEC>;
#[doc = "crop window start"]
pub mod cwstrt;
#[doc = "CWSIZE (rw) register accessor: crop window size\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cwsize::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cwsize::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`cwsize`]
module"]
pub type CWSIZE = crate::Reg<cwsize::CWSIZE_SPEC>;
#[doc = "crop window size"]
pub mod cwsize;
#[doc = "DR (r) register accessor: data register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`dr`]
module"]
pub type DR = crate::Reg<dr::DR_SPEC>;
#[doc = "data register"]
pub mod dr;
