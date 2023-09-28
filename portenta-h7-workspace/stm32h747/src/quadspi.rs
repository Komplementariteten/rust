#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - QUADSPI control register"]
    pub cr: CR,
    #[doc = "0x04 - QUADSPI device configuration register"]
    pub dcr: DCR,
    #[doc = "0x08 - QUADSPI status register"]
    pub sr: SR,
    #[doc = "0x0c - QUADSPI flag clear register"]
    pub fcr: FCR,
    #[doc = "0x10 - QUADSPI data length register"]
    pub dlr: DLR,
    #[doc = "0x14 - QUADSPI communication configuration register"]
    pub ccr: CCR,
    #[doc = "0x18 - QUADSPI address register"]
    pub ar: AR,
    #[doc = "0x1c - QUADSPI alternate bytes registers"]
    pub abr: ABR,
    #[doc = "0x20 - QUADSPI data register"]
    pub dr: DR,
    #[doc = "0x24 - QUADSPI polling status mask register"]
    pub psmkr: PSMKR,
    #[doc = "0x28 - QUADSPI polling status match register"]
    pub psmar: PSMAR,
    #[doc = "0x2c - QUADSPI polling interval register"]
    pub pir: PIR,
    #[doc = "0x30 - QUADSPI low-power timeout register"]
    pub lptr: LPTR,
}
#[doc = "CR (rw) register accessor: QUADSPI control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`cr`]
module"]
pub type CR = crate::Reg<cr::CR_SPEC>;
#[doc = "QUADSPI control register"]
pub mod cr;
#[doc = "DCR (rw) register accessor: QUADSPI device configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dcr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dcr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`dcr`]
module"]
pub type DCR = crate::Reg<dcr::DCR_SPEC>;
#[doc = "QUADSPI device configuration register"]
pub mod dcr;
#[doc = "SR (r) register accessor: QUADSPI status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`sr`]
module"]
pub type SR = crate::Reg<sr::SR_SPEC>;
#[doc = "QUADSPI status register"]
pub mod sr;
#[doc = "FCR (rw) register accessor: QUADSPI flag clear register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fcr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fcr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`fcr`]
module"]
pub type FCR = crate::Reg<fcr::FCR_SPEC>;
#[doc = "QUADSPI flag clear register"]
pub mod fcr;
#[doc = "DLR (rw) register accessor: QUADSPI data length register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dlr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dlr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`dlr`]
module"]
pub type DLR = crate::Reg<dlr::DLR_SPEC>;
#[doc = "QUADSPI data length register"]
pub mod dlr;
#[doc = "CCR (rw) register accessor: QUADSPI communication configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ccr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ccr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`ccr`]
module"]
pub type CCR = crate::Reg<ccr::CCR_SPEC>;
#[doc = "QUADSPI communication configuration register"]
pub mod ccr;
#[doc = "AR (rw) register accessor: QUADSPI address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ar::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ar::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`ar`]
module"]
pub type AR = crate::Reg<ar::AR_SPEC>;
#[doc = "QUADSPI address register"]
pub mod ar;
#[doc = "ABR (rw) register accessor: QUADSPI alternate bytes registers\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`abr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`abr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`abr`]
module"]
pub type ABR = crate::Reg<abr::ABR_SPEC>;
#[doc = "QUADSPI alternate bytes registers"]
pub mod abr;
#[doc = "DR (rw) register accessor: QUADSPI data register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`dr`]
module"]
pub type DR = crate::Reg<dr::DR_SPEC>;
#[doc = "QUADSPI data register"]
pub mod dr;
#[doc = "PSMKR (rw) register accessor: QUADSPI polling status mask register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`psmkr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`psmkr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`psmkr`]
module"]
pub type PSMKR = crate::Reg<psmkr::PSMKR_SPEC>;
#[doc = "QUADSPI polling status mask register"]
pub mod psmkr;
#[doc = "PSMAR (rw) register accessor: QUADSPI polling status match register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`psmar::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`psmar::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`psmar`]
module"]
pub type PSMAR = crate::Reg<psmar::PSMAR_SPEC>;
#[doc = "QUADSPI polling status match register"]
pub mod psmar;
#[doc = "PIR (rw) register accessor: QUADSPI polling interval register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pir::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pir::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`pir`]
module"]
pub type PIR = crate::Reg<pir::PIR_SPEC>;
#[doc = "QUADSPI polling interval register"]
pub mod pir;
#[doc = "LPTR (rw) register accessor: QUADSPI low-power timeout register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lptr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lptr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`lptr`]
module"]
pub type LPTR = crate::Reg<lptr::LPTR_SPEC>;
#[doc = "QUADSPI low-power timeout register"]
pub mod lptr;
