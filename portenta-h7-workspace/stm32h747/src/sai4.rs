#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Global configuration register"]
    pub sai_gcr: SAI_GCR,
    #[doc = "0x04 - Configuration register 1"]
    pub sai_acr1: SAI_ACR1,
    #[doc = "0x08 - Configuration register 2"]
    pub sai_acr2: SAI_ACR2,
    #[doc = "0x0c - This register has no meaning in AC97 and SPDIF audio protocol"]
    pub sai_afrcr: SAI_AFRCR,
    #[doc = "0x10 - This register has no meaning in AC97 and SPDIF audio protocol"]
    pub sai_aslotr: SAI_ASLOTR,
    #[doc = "0x14 - Interrupt mask register 2"]
    pub sai_aim: SAI_AIM,
    #[doc = "0x18 - Status register"]
    pub sai_asr: SAI_ASR,
    #[doc = "0x1c - Clear flag register"]
    pub sai_aclrfr: SAI_ACLRFR,
    #[doc = "0x20 - Data register"]
    pub sai_adr: SAI_ADR,
    #[doc = "0x24 - Configuration register 1"]
    pub sai_bcr1: SAI_BCR1,
    #[doc = "0x28 - Configuration register 2"]
    pub sai_bcr2: SAI_BCR2,
    #[doc = "0x2c - This register has no meaning in AC97 and SPDIF audio protocol"]
    pub sai_bfrcr: SAI_BFRCR,
    #[doc = "0x30 - This register has no meaning in AC97 and SPDIF audio protocol"]
    pub sai_bslotr: SAI_BSLOTR,
    #[doc = "0x34 - Interrupt mask register 2"]
    pub sai_bim: SAI_BIM,
    #[doc = "0x38 - Status register"]
    pub sai_bsr: SAI_BSR,
    #[doc = "0x3c - Clear flag register"]
    pub sai_bclrfr: SAI_BCLRFR,
    #[doc = "0x40 - Data register"]
    pub sai_bdr: SAI_BDR,
    #[doc = "0x44 - PDM control register"]
    pub sai_pdmcr: SAI_PDMCR,
    #[doc = "0x48 - PDM delay register"]
    pub sai_pdmdly: SAI_PDMDLY,
}
#[doc = "SAI_GCR (rw) register accessor: Global configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sai_gcr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sai_gcr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`sai_gcr`]
module"]
pub type SAI_GCR = crate::Reg<sai_gcr::SAI_GCR_SPEC>;
#[doc = "Global configuration register"]
pub mod sai_gcr;
#[doc = "SAI_ACR1 (rw) register accessor: Configuration register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sai_acr1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sai_acr1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`sai_acr1`]
module"]
pub type SAI_ACR1 = crate::Reg<sai_acr1::SAI_ACR1_SPEC>;
#[doc = "Configuration register 1"]
pub mod sai_acr1;
#[doc = "SAI_ACR2 (rw) register accessor: Configuration register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sai_acr2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sai_acr2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`sai_acr2`]
module"]
pub type SAI_ACR2 = crate::Reg<sai_acr2::SAI_ACR2_SPEC>;
#[doc = "Configuration register 2"]
pub mod sai_acr2;
#[doc = "SAI_AFRCR (rw) register accessor: This register has no meaning in AC97 and SPDIF audio protocol\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sai_afrcr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sai_afrcr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`sai_afrcr`]
module"]
pub type SAI_AFRCR = crate::Reg<sai_afrcr::SAI_AFRCR_SPEC>;
#[doc = "This register has no meaning in AC97 and SPDIF audio protocol"]
pub mod sai_afrcr;
#[doc = "SAI_ASLOTR (rw) register accessor: This register has no meaning in AC97 and SPDIF audio protocol\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sai_aslotr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sai_aslotr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`sai_aslotr`]
module"]
pub type SAI_ASLOTR = crate::Reg<sai_aslotr::SAI_ASLOTR_SPEC>;
#[doc = "This register has no meaning in AC97 and SPDIF audio protocol"]
pub mod sai_aslotr;
#[doc = "SAI_AIM (rw) register accessor: Interrupt mask register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sai_aim::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sai_aim::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`sai_aim`]
module"]
pub type SAI_AIM = crate::Reg<sai_aim::SAI_AIM_SPEC>;
#[doc = "Interrupt mask register 2"]
pub mod sai_aim;
#[doc = "SAI_ASR (r) register accessor: Status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sai_asr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`sai_asr`]
module"]
pub type SAI_ASR = crate::Reg<sai_asr::SAI_ASR_SPEC>;
#[doc = "Status register"]
pub mod sai_asr;
#[doc = "SAI_ACLRFR (w) register accessor: Clear flag register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sai_aclrfr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`sai_aclrfr`]
module"]
pub type SAI_ACLRFR = crate::Reg<sai_aclrfr::SAI_ACLRFR_SPEC>;
#[doc = "Clear flag register"]
pub mod sai_aclrfr;
#[doc = "SAI_ADR (rw) register accessor: Data register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sai_adr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sai_adr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`sai_adr`]
module"]
pub type SAI_ADR = crate::Reg<sai_adr::SAI_ADR_SPEC>;
#[doc = "Data register"]
pub mod sai_adr;
#[doc = "SAI_BCR1 (rw) register accessor: Configuration register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sai_bcr1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sai_bcr1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`sai_bcr1`]
module"]
pub type SAI_BCR1 = crate::Reg<sai_bcr1::SAI_BCR1_SPEC>;
#[doc = "Configuration register 1"]
pub mod sai_bcr1;
#[doc = "SAI_BCR2 (rw) register accessor: Configuration register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sai_bcr2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sai_bcr2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`sai_bcr2`]
module"]
pub type SAI_BCR2 = crate::Reg<sai_bcr2::SAI_BCR2_SPEC>;
#[doc = "Configuration register 2"]
pub mod sai_bcr2;
#[doc = "SAI_BFRCR (rw) register accessor: This register has no meaning in AC97 and SPDIF audio protocol\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sai_bfrcr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sai_bfrcr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`sai_bfrcr`]
module"]
pub type SAI_BFRCR = crate::Reg<sai_bfrcr::SAI_BFRCR_SPEC>;
#[doc = "This register has no meaning in AC97 and SPDIF audio protocol"]
pub mod sai_bfrcr;
#[doc = "SAI_BSLOTR (rw) register accessor: This register has no meaning in AC97 and SPDIF audio protocol\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sai_bslotr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sai_bslotr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`sai_bslotr`]
module"]
pub type SAI_BSLOTR = crate::Reg<sai_bslotr::SAI_BSLOTR_SPEC>;
#[doc = "This register has no meaning in AC97 and SPDIF audio protocol"]
pub mod sai_bslotr;
#[doc = "SAI_BIM (rw) register accessor: Interrupt mask register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sai_bim::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sai_bim::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`sai_bim`]
module"]
pub type SAI_BIM = crate::Reg<sai_bim::SAI_BIM_SPEC>;
#[doc = "Interrupt mask register 2"]
pub mod sai_bim;
#[doc = "SAI_BSR (r) register accessor: Status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sai_bsr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`sai_bsr`]
module"]
pub type SAI_BSR = crate::Reg<sai_bsr::SAI_BSR_SPEC>;
#[doc = "Status register"]
pub mod sai_bsr;
#[doc = "SAI_BCLRFR (w) register accessor: Clear flag register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sai_bclrfr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`sai_bclrfr`]
module"]
pub type SAI_BCLRFR = crate::Reg<sai_bclrfr::SAI_BCLRFR_SPEC>;
#[doc = "Clear flag register"]
pub mod sai_bclrfr;
#[doc = "SAI_BDR (rw) register accessor: Data register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sai_bdr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sai_bdr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`sai_bdr`]
module"]
pub type SAI_BDR = crate::Reg<sai_bdr::SAI_BDR_SPEC>;
#[doc = "Data register"]
pub mod sai_bdr;
#[doc = "SAI_PDMCR (rw) register accessor: PDM control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sai_pdmcr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sai_pdmcr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`sai_pdmcr`]
module"]
pub type SAI_PDMCR = crate::Reg<sai_pdmcr::SAI_PDMCR_SPEC>;
#[doc = "PDM control register"]
pub mod sai_pdmcr;
#[doc = "SAI_PDMDLY (rw) register accessor: PDM delay register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sai_pdmdly::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sai_pdmdly::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`sai_pdmdly`]
module"]
pub type SAI_PDMDLY = crate::Reg<sai_pdmdly::SAI_PDMDLY_SPEC>;
#[doc = "PDM delay register"]
pub mod sai_pdmdly;
