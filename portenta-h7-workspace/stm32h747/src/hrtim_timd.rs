#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Timerx Control Register"]
    pub timdcr: TIMDCR,
    #[doc = "0x04 - Timerx Interrupt Status Register"]
    pub timdisr: TIMDISR,
    #[doc = "0x08 - Timerx Interrupt Clear Register"]
    pub timdicr: TIMDICR,
    #[doc = "0x0c - TIMxDIER5"]
    pub timddier5: TIMDDIER5,
    #[doc = "0x10 - Timerx Counter Register"]
    pub cntdr: CNTDR,
    #[doc = "0x14 - Timerx Period Register"]
    pub perdr: PERDR,
    #[doc = "0x18 - Timerx Repetition Register"]
    pub repdr: REPDR,
    #[doc = "0x1c - Timerx Compare 1 Register"]
    pub cmp1dr: CMP1DR,
    #[doc = "0x20 - Timerx Compare 1 Compound Register"]
    pub cmp1cdr: CMP1CDR,
    #[doc = "0x24 - Timerx Compare 2 Register"]
    pub cmp2dr: CMP2DR,
    #[doc = "0x28 - Timerx Compare 3 Register"]
    pub cmp3dr: CMP3DR,
    #[doc = "0x2c - Timerx Compare 4 Register"]
    pub cmp4dr: CMP4DR,
    #[doc = "0x30 - Timerx Capture 1 Register"]
    pub cpt1dr: CPT1DR,
    #[doc = "0x34 - Timerx Capture 2 Register"]
    pub cpt2dr: CPT2DR,
    #[doc = "0x38 - Timerx Deadtime Register"]
    pub dtdr: DTDR,
    #[doc = "0x3c - Timerx Output1 Set Register"]
    pub setd1r: SETD1R,
    #[doc = "0x40 - Timerx Output1 Reset Register"]
    pub rstd1r: RSTD1R,
    #[doc = "0x44 - Timerx Output2 Set Register"]
    pub setd2r: SETD2R,
    #[doc = "0x48 - Timerx Output2 Reset Register"]
    pub rstd2r: RSTD2R,
    #[doc = "0x4c - Timerx External Event Filtering Register 1"]
    pub eefdr1: EEFDR1,
    #[doc = "0x50 - Timerx External Event Filtering Register 2"]
    pub eefdr2: EEFDR2,
    #[doc = "0x54 - TimerA Reset Register"]
    pub rstdr: RSTDR,
    #[doc = "0x58 - Timerx Chopper Register"]
    pub chpdr: CHPDR,
    #[doc = "0x5c - Timerx Capture 2 Control Register"]
    pub cpt1dcr: CPT1DCR,
    #[doc = "0x60 - CPT2xCR"]
    pub cpt2dcr: CPT2DCR,
    #[doc = "0x64 - Timerx Output Register"]
    pub outdr: OUTDR,
    #[doc = "0x68 - Timerx Fault Register"]
    pub fltdr: FLTDR,
}
#[doc = "TIMDCR (rw) register accessor: Timerx Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`timdcr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`timdcr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`timdcr`]
module"]
pub type TIMDCR = crate::Reg<timdcr::TIMDCR_SPEC>;
#[doc = "Timerx Control Register"]
pub mod timdcr;
#[doc = "TIMDISR (r) register accessor: Timerx Interrupt Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`timdisr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`timdisr`]
module"]
pub type TIMDISR = crate::Reg<timdisr::TIMDISR_SPEC>;
#[doc = "Timerx Interrupt Status Register"]
pub mod timdisr;
#[doc = "TIMDICR (w) register accessor: Timerx Interrupt Clear Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`timdicr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`timdicr`]
module"]
pub type TIMDICR = crate::Reg<timdicr::TIMDICR_SPEC>;
#[doc = "Timerx Interrupt Clear Register"]
pub mod timdicr;
#[doc = "TIMDDIER5 (rw) register accessor: TIMxDIER5\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`timddier5::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`timddier5::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`timddier5`]
module"]
pub type TIMDDIER5 = crate::Reg<timddier5::TIMDDIER5_SPEC>;
#[doc = "TIMxDIER5"]
pub mod timddier5;
#[doc = "CNTDR (rw) register accessor: Timerx Counter Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cntdr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cntdr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`cntdr`]
module"]
pub type CNTDR = crate::Reg<cntdr::CNTDR_SPEC>;
#[doc = "Timerx Counter Register"]
pub mod cntdr;
#[doc = "PERDR (rw) register accessor: Timerx Period Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`perdr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`perdr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`perdr`]
module"]
pub type PERDR = crate::Reg<perdr::PERDR_SPEC>;
#[doc = "Timerx Period Register"]
pub mod perdr;
#[doc = "REPDR (rw) register accessor: Timerx Repetition Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`repdr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`repdr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`repdr`]
module"]
pub type REPDR = crate::Reg<repdr::REPDR_SPEC>;
#[doc = "Timerx Repetition Register"]
pub mod repdr;
#[doc = "CMP1DR (rw) register accessor: Timerx Compare 1 Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cmp1dr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cmp1dr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`cmp1dr`]
module"]
pub type CMP1DR = crate::Reg<cmp1dr::CMP1DR_SPEC>;
#[doc = "Timerx Compare 1 Register"]
pub mod cmp1dr;
#[doc = "CMP1CDR (rw) register accessor: Timerx Compare 1 Compound Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cmp1cdr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cmp1cdr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`cmp1cdr`]
module"]
pub type CMP1CDR = crate::Reg<cmp1cdr::CMP1CDR_SPEC>;
#[doc = "Timerx Compare 1 Compound Register"]
pub mod cmp1cdr;
#[doc = "CMP2DR (rw) register accessor: Timerx Compare 2 Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cmp2dr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cmp2dr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`cmp2dr`]
module"]
pub type CMP2DR = crate::Reg<cmp2dr::CMP2DR_SPEC>;
#[doc = "Timerx Compare 2 Register"]
pub mod cmp2dr;
#[doc = "CMP3DR (rw) register accessor: Timerx Compare 3 Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cmp3dr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cmp3dr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`cmp3dr`]
module"]
pub type CMP3DR = crate::Reg<cmp3dr::CMP3DR_SPEC>;
#[doc = "Timerx Compare 3 Register"]
pub mod cmp3dr;
#[doc = "CMP4DR (rw) register accessor: Timerx Compare 4 Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cmp4dr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cmp4dr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`cmp4dr`]
module"]
pub type CMP4DR = crate::Reg<cmp4dr::CMP4DR_SPEC>;
#[doc = "Timerx Compare 4 Register"]
pub mod cmp4dr;
#[doc = "CPT1DR (r) register accessor: Timerx Capture 1 Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cpt1dr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`cpt1dr`]
module"]
pub type CPT1DR = crate::Reg<cpt1dr::CPT1DR_SPEC>;
#[doc = "Timerx Capture 1 Register"]
pub mod cpt1dr;
#[doc = "CPT2DR (r) register accessor: Timerx Capture 2 Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cpt2dr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`cpt2dr`]
module"]
pub type CPT2DR = crate::Reg<cpt2dr::CPT2DR_SPEC>;
#[doc = "Timerx Capture 2 Register"]
pub mod cpt2dr;
#[doc = "DTDR (rw) register accessor: Timerx Deadtime Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dtdr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dtdr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`dtdr`]
module"]
pub type DTDR = crate::Reg<dtdr::DTDR_SPEC>;
#[doc = "Timerx Deadtime Register"]
pub mod dtdr;
#[doc = "SETD1R (rw) register accessor: Timerx Output1 Set Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`setd1r::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`setd1r::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`setd1r`]
module"]
pub type SETD1R = crate::Reg<setd1r::SETD1R_SPEC>;
#[doc = "Timerx Output1 Set Register"]
pub mod setd1r;
#[doc = "RSTD1R (rw) register accessor: Timerx Output1 Reset Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rstd1r::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rstd1r::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`rstd1r`]
module"]
pub type RSTD1R = crate::Reg<rstd1r::RSTD1R_SPEC>;
#[doc = "Timerx Output1 Reset Register"]
pub mod rstd1r;
#[doc = "SETD2R (rw) register accessor: Timerx Output2 Set Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`setd2r::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`setd2r::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`setd2r`]
module"]
pub type SETD2R = crate::Reg<setd2r::SETD2R_SPEC>;
#[doc = "Timerx Output2 Set Register"]
pub mod setd2r;
#[doc = "RSTD2R (rw) register accessor: Timerx Output2 Reset Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rstd2r::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rstd2r::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`rstd2r`]
module"]
pub type RSTD2R = crate::Reg<rstd2r::RSTD2R_SPEC>;
#[doc = "Timerx Output2 Reset Register"]
pub mod rstd2r;
#[doc = "EEFDR1 (rw) register accessor: Timerx External Event Filtering Register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`eefdr1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`eefdr1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`eefdr1`]
module"]
pub type EEFDR1 = crate::Reg<eefdr1::EEFDR1_SPEC>;
#[doc = "Timerx External Event Filtering Register 1"]
pub mod eefdr1;
#[doc = "EEFDR2 (rw) register accessor: Timerx External Event Filtering Register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`eefdr2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`eefdr2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`eefdr2`]
module"]
pub type EEFDR2 = crate::Reg<eefdr2::EEFDR2_SPEC>;
#[doc = "Timerx External Event Filtering Register 2"]
pub mod eefdr2;
#[doc = "RSTDR (rw) register accessor: TimerA Reset Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rstdr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rstdr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`rstdr`]
module"]
pub type RSTDR = crate::Reg<rstdr::RSTDR_SPEC>;
#[doc = "TimerA Reset Register"]
pub mod rstdr;
#[doc = "CHPDR (rw) register accessor: Timerx Chopper Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`chpdr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`chpdr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`chpdr`]
module"]
pub type CHPDR = crate::Reg<chpdr::CHPDR_SPEC>;
#[doc = "Timerx Chopper Register"]
pub mod chpdr;
#[doc = "CPT1DCR (rw) register accessor: Timerx Capture 2 Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cpt1dcr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cpt1dcr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`cpt1dcr`]
module"]
pub type CPT1DCR = crate::Reg<cpt1dcr::CPT1DCR_SPEC>;
#[doc = "Timerx Capture 2 Control Register"]
pub mod cpt1dcr;
#[doc = "CPT2DCR (rw) register accessor: CPT2xCR\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cpt2dcr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cpt2dcr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`cpt2dcr`]
module"]
pub type CPT2DCR = crate::Reg<cpt2dcr::CPT2DCR_SPEC>;
#[doc = "CPT2xCR"]
pub mod cpt2dcr;
#[doc = "OUTDR (rw) register accessor: Timerx Output Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`outdr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`outdr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`outdr`]
module"]
pub type OUTDR = crate::Reg<outdr::OUTDR_SPEC>;
#[doc = "Timerx Output Register"]
pub mod outdr;
#[doc = "FLTDR (rw) register accessor: Timerx Fault Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fltdr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fltdr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`fltdr`]
module"]
pub type FLTDR = crate::Reg<fltdr::FLTDR_SPEC>;
#[doc = "Timerx Fault Register"]
pub mod fltdr;
