#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Timerx Control Register"]
    pub timbcr: TIMBCR,
    #[doc = "0x04 - Timerx Interrupt Status Register"]
    pub timbisr: TIMBISR,
    #[doc = "0x08 - Timerx Interrupt Clear Register"]
    pub timbicr: TIMBICR,
    #[doc = "0x0c - TIMxDIER5"]
    pub timbdier5: TIMBDIER5,
    #[doc = "0x10 - Timerx Counter Register"]
    pub cntr: CNTR,
    #[doc = "0x14 - Timerx Period Register"]
    pub perbr: PERBR,
    #[doc = "0x18 - Timerx Repetition Register"]
    pub repbr: REPBR,
    #[doc = "0x1c - Timerx Compare 1 Register"]
    pub cmp1br: CMP1BR,
    #[doc = "0x20 - Timerx Compare 1 Compound Register"]
    pub cmp1cbr: CMP1CBR,
    #[doc = "0x24 - Timerx Compare 2 Register"]
    pub cmp2br: CMP2BR,
    #[doc = "0x28 - Timerx Compare 3 Register"]
    pub cmp3br: CMP3BR,
    #[doc = "0x2c - Timerx Compare 4 Register"]
    pub cmp4br: CMP4BR,
    #[doc = "0x30 - Timerx Capture 1 Register"]
    pub cpt1br: CPT1BR,
    #[doc = "0x34 - Timerx Capture 2 Register"]
    pub cpt2br: CPT2BR,
    #[doc = "0x38 - Timerx Deadtime Register"]
    pub dtbr: DTBR,
    #[doc = "0x3c - Timerx Output1 Set Register"]
    pub setb1r: SETB1R,
    #[doc = "0x40 - Timerx Output1 Reset Register"]
    pub rstb1r: RSTB1R,
    #[doc = "0x44 - Timerx Output2 Set Register"]
    pub setb2r: SETB2R,
    #[doc = "0x48 - Timerx Output2 Reset Register"]
    pub rstb2r: RSTB2R,
    #[doc = "0x4c - Timerx External Event Filtering Register 1"]
    pub eefbr1: EEFBR1,
    #[doc = "0x50 - Timerx External Event Filtering Register 2"]
    pub eefbr2: EEFBR2,
    #[doc = "0x54 - TimerA Reset Register"]
    pub rstbr: RSTBR,
    #[doc = "0x58 - Timerx Chopper Register"]
    pub chpbr: CHPBR,
    #[doc = "0x5c - Timerx Capture 2 Control Register"]
    pub cpt1bcr: CPT1BCR,
    #[doc = "0x60 - CPT2xCR"]
    pub cpt2bcr: CPT2BCR,
    #[doc = "0x64 - Timerx Output Register"]
    pub outbr: OUTBR,
    #[doc = "0x68 - Timerx Fault Register"]
    pub fltbr: FLTBR,
}
#[doc = "TIMBCR (rw) register accessor: Timerx Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`timbcr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`timbcr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`timbcr`]
module"]
pub type TIMBCR = crate::Reg<timbcr::TIMBCR_SPEC>;
#[doc = "Timerx Control Register"]
pub mod timbcr;
#[doc = "TIMBISR (r) register accessor: Timerx Interrupt Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`timbisr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`timbisr`]
module"]
pub type TIMBISR = crate::Reg<timbisr::TIMBISR_SPEC>;
#[doc = "Timerx Interrupt Status Register"]
pub mod timbisr;
#[doc = "TIMBICR (w) register accessor: Timerx Interrupt Clear Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`timbicr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`timbicr`]
module"]
pub type TIMBICR = crate::Reg<timbicr::TIMBICR_SPEC>;
#[doc = "Timerx Interrupt Clear Register"]
pub mod timbicr;
#[doc = "TIMBDIER5 (rw) register accessor: TIMxDIER5\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`timbdier5::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`timbdier5::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`timbdier5`]
module"]
pub type TIMBDIER5 = crate::Reg<timbdier5::TIMBDIER5_SPEC>;
#[doc = "TIMxDIER5"]
pub mod timbdier5;
#[doc = "CNTR (rw) register accessor: Timerx Counter Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cntr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cntr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`cntr`]
module"]
pub type CNTR = crate::Reg<cntr::CNTR_SPEC>;
#[doc = "Timerx Counter Register"]
pub mod cntr;
#[doc = "PERBR (rw) register accessor: Timerx Period Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`perbr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`perbr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`perbr`]
module"]
pub type PERBR = crate::Reg<perbr::PERBR_SPEC>;
#[doc = "Timerx Period Register"]
pub mod perbr;
#[doc = "REPBR (rw) register accessor: Timerx Repetition Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`repbr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`repbr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`repbr`]
module"]
pub type REPBR = crate::Reg<repbr::REPBR_SPEC>;
#[doc = "Timerx Repetition Register"]
pub mod repbr;
#[doc = "CMP1BR (rw) register accessor: Timerx Compare 1 Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cmp1br::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cmp1br::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`cmp1br`]
module"]
pub type CMP1BR = crate::Reg<cmp1br::CMP1BR_SPEC>;
#[doc = "Timerx Compare 1 Register"]
pub mod cmp1br;
#[doc = "CMP1CBR (rw) register accessor: Timerx Compare 1 Compound Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cmp1cbr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cmp1cbr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`cmp1cbr`]
module"]
pub type CMP1CBR = crate::Reg<cmp1cbr::CMP1CBR_SPEC>;
#[doc = "Timerx Compare 1 Compound Register"]
pub mod cmp1cbr;
#[doc = "CMP2BR (rw) register accessor: Timerx Compare 2 Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cmp2br::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cmp2br::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`cmp2br`]
module"]
pub type CMP2BR = crate::Reg<cmp2br::CMP2BR_SPEC>;
#[doc = "Timerx Compare 2 Register"]
pub mod cmp2br;
#[doc = "CMP3BR (rw) register accessor: Timerx Compare 3 Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cmp3br::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cmp3br::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`cmp3br`]
module"]
pub type CMP3BR = crate::Reg<cmp3br::CMP3BR_SPEC>;
#[doc = "Timerx Compare 3 Register"]
pub mod cmp3br;
#[doc = "CMP4BR (rw) register accessor: Timerx Compare 4 Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cmp4br::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cmp4br::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`cmp4br`]
module"]
pub type CMP4BR = crate::Reg<cmp4br::CMP4BR_SPEC>;
#[doc = "Timerx Compare 4 Register"]
pub mod cmp4br;
#[doc = "CPT1BR (r) register accessor: Timerx Capture 1 Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cpt1br::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`cpt1br`]
module"]
pub type CPT1BR = crate::Reg<cpt1br::CPT1BR_SPEC>;
#[doc = "Timerx Capture 1 Register"]
pub mod cpt1br;
#[doc = "CPT2BR (r) register accessor: Timerx Capture 2 Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cpt2br::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`cpt2br`]
module"]
pub type CPT2BR = crate::Reg<cpt2br::CPT2BR_SPEC>;
#[doc = "Timerx Capture 2 Register"]
pub mod cpt2br;
#[doc = "DTBR (rw) register accessor: Timerx Deadtime Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dtbr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dtbr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`dtbr`]
module"]
pub type DTBR = crate::Reg<dtbr::DTBR_SPEC>;
#[doc = "Timerx Deadtime Register"]
pub mod dtbr;
#[doc = "SETB1R (rw) register accessor: Timerx Output1 Set Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`setb1r::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`setb1r::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`setb1r`]
module"]
pub type SETB1R = crate::Reg<setb1r::SETB1R_SPEC>;
#[doc = "Timerx Output1 Set Register"]
pub mod setb1r;
#[doc = "RSTB1R (rw) register accessor: Timerx Output1 Reset Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rstb1r::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rstb1r::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`rstb1r`]
module"]
pub type RSTB1R = crate::Reg<rstb1r::RSTB1R_SPEC>;
#[doc = "Timerx Output1 Reset Register"]
pub mod rstb1r;
#[doc = "SETB2R (rw) register accessor: Timerx Output2 Set Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`setb2r::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`setb2r::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`setb2r`]
module"]
pub type SETB2R = crate::Reg<setb2r::SETB2R_SPEC>;
#[doc = "Timerx Output2 Set Register"]
pub mod setb2r;
#[doc = "RSTB2R (rw) register accessor: Timerx Output2 Reset Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rstb2r::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rstb2r::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`rstb2r`]
module"]
pub type RSTB2R = crate::Reg<rstb2r::RSTB2R_SPEC>;
#[doc = "Timerx Output2 Reset Register"]
pub mod rstb2r;
#[doc = "EEFBR1 (rw) register accessor: Timerx External Event Filtering Register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`eefbr1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`eefbr1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`eefbr1`]
module"]
pub type EEFBR1 = crate::Reg<eefbr1::EEFBR1_SPEC>;
#[doc = "Timerx External Event Filtering Register 1"]
pub mod eefbr1;
#[doc = "EEFBR2 (rw) register accessor: Timerx External Event Filtering Register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`eefbr2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`eefbr2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`eefbr2`]
module"]
pub type EEFBR2 = crate::Reg<eefbr2::EEFBR2_SPEC>;
#[doc = "Timerx External Event Filtering Register 2"]
pub mod eefbr2;
#[doc = "RSTBR (rw) register accessor: TimerA Reset Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rstbr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rstbr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`rstbr`]
module"]
pub type RSTBR = crate::Reg<rstbr::RSTBR_SPEC>;
#[doc = "TimerA Reset Register"]
pub mod rstbr;
#[doc = "CHPBR (rw) register accessor: Timerx Chopper Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`chpbr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`chpbr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`chpbr`]
module"]
pub type CHPBR = crate::Reg<chpbr::CHPBR_SPEC>;
#[doc = "Timerx Chopper Register"]
pub mod chpbr;
#[doc = "CPT1BCR (rw) register accessor: Timerx Capture 2 Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cpt1bcr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cpt1bcr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`cpt1bcr`]
module"]
pub type CPT1BCR = crate::Reg<cpt1bcr::CPT1BCR_SPEC>;
#[doc = "Timerx Capture 2 Control Register"]
pub mod cpt1bcr;
#[doc = "CPT2BCR (rw) register accessor: CPT2xCR\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cpt2bcr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cpt2bcr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`cpt2bcr`]
module"]
pub type CPT2BCR = crate::Reg<cpt2bcr::CPT2BCR_SPEC>;
#[doc = "CPT2xCR"]
pub mod cpt2bcr;
#[doc = "OUTBR (rw) register accessor: Timerx Output Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`outbr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`outbr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`outbr`]
module"]
pub type OUTBR = crate::Reg<outbr::OUTBR_SPEC>;
#[doc = "Timerx Output Register"]
pub mod outbr;
#[doc = "FLTBR (rw) register accessor: Timerx Fault Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fltbr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fltbr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`fltbr`]
module"]
pub type FLTBR = crate::Reg<fltbr::FLTBR_SPEC>;
#[doc = "Timerx Fault Register"]
pub mod fltbr;
