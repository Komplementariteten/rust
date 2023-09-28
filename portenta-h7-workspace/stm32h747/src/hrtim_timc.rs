#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Timerx Control Register"]
    pub timccr: TIMCCR,
    #[doc = "0x04 - Timerx Interrupt Status Register"]
    pub timcisr: TIMCISR,
    #[doc = "0x08 - Timerx Interrupt Clear Register"]
    pub timcicr: TIMCICR,
    #[doc = "0x0c - TIMxDIER5"]
    pub timcdier5: TIMCDIER5,
    #[doc = "0x10 - Timerx Counter Register"]
    pub cntcr: CNTCR,
    #[doc = "0x14 - Timerx Period Register"]
    pub percr: PERCR,
    #[doc = "0x18 - Timerx Repetition Register"]
    pub repcr: REPCR,
    #[doc = "0x1c - Timerx Compare 1 Register"]
    pub cmp1cr: CMP1CR,
    #[doc = "0x20 - Timerx Compare 1 Compound Register"]
    pub cmp1ccr: CMP1CCR,
    #[doc = "0x24 - Timerx Compare 2 Register"]
    pub cmp2cr: CMP2CR,
    #[doc = "0x28 - Timerx Compare 3 Register"]
    pub cmp3cr: CMP3CR,
    #[doc = "0x2c - Timerx Compare 4 Register"]
    pub cmp4cr: CMP4CR,
    #[doc = "0x30 - Timerx Capture 1 Register"]
    pub cpt1cr: CPT1CR,
    #[doc = "0x34 - Timerx Capture 2 Register"]
    pub cpt2cr: CPT2CR,
    #[doc = "0x38 - Timerx Deadtime Register"]
    pub dtcr: DTCR,
    #[doc = "0x3c - Timerx Output1 Set Register"]
    pub setc1r: SETC1R,
    #[doc = "0x40 - Timerx Output1 Reset Register"]
    pub rstc1r: RSTC1R,
    #[doc = "0x44 - Timerx Output2 Set Register"]
    pub setc2r: SETC2R,
    #[doc = "0x48 - Timerx Output2 Reset Register"]
    pub rstc2r: RSTC2R,
    #[doc = "0x4c - Timerx External Event Filtering Register 1"]
    pub eefcr1: EEFCR1,
    #[doc = "0x50 - Timerx External Event Filtering Register 2"]
    pub eefcr2: EEFCR2,
    #[doc = "0x54 - TimerA Reset Register"]
    pub rstcr: RSTCR,
    #[doc = "0x58 - Timerx Chopper Register"]
    pub chpcr: CHPCR,
    #[doc = "0x5c - Timerx Capture 2 Control Register"]
    pub cpt1ccr: CPT1CCR,
    #[doc = "0x60 - CPT2xCR"]
    pub cpt2ccr: CPT2CCR,
    #[doc = "0x64 - Timerx Output Register"]
    pub outcr: OUTCR,
    #[doc = "0x68 - Timerx Fault Register"]
    pub fltcr: FLTCR,
}
#[doc = "TIMCCR (rw) register accessor: Timerx Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`timccr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`timccr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`timccr`]
module"]
pub type TIMCCR = crate::Reg<timccr::TIMCCR_SPEC>;
#[doc = "Timerx Control Register"]
pub mod timccr;
#[doc = "TIMCISR (r) register accessor: Timerx Interrupt Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`timcisr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`timcisr`]
module"]
pub type TIMCISR = crate::Reg<timcisr::TIMCISR_SPEC>;
#[doc = "Timerx Interrupt Status Register"]
pub mod timcisr;
#[doc = "TIMCICR (w) register accessor: Timerx Interrupt Clear Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`timcicr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`timcicr`]
module"]
pub type TIMCICR = crate::Reg<timcicr::TIMCICR_SPEC>;
#[doc = "Timerx Interrupt Clear Register"]
pub mod timcicr;
#[doc = "TIMCDIER5 (rw) register accessor: TIMxDIER5\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`timcdier5::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`timcdier5::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`timcdier5`]
module"]
pub type TIMCDIER5 = crate::Reg<timcdier5::TIMCDIER5_SPEC>;
#[doc = "TIMxDIER5"]
pub mod timcdier5;
#[doc = "CNTCR (rw) register accessor: Timerx Counter Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cntcr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cntcr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`cntcr`]
module"]
pub type CNTCR = crate::Reg<cntcr::CNTCR_SPEC>;
#[doc = "Timerx Counter Register"]
pub mod cntcr;
#[doc = "PERCR (rw) register accessor: Timerx Period Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`percr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`percr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`percr`]
module"]
pub type PERCR = crate::Reg<percr::PERCR_SPEC>;
#[doc = "Timerx Period Register"]
pub mod percr;
#[doc = "REPCR (rw) register accessor: Timerx Repetition Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`repcr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`repcr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`repcr`]
module"]
pub type REPCR = crate::Reg<repcr::REPCR_SPEC>;
#[doc = "Timerx Repetition Register"]
pub mod repcr;
#[doc = "CMP1CR (rw) register accessor: Timerx Compare 1 Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cmp1cr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cmp1cr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`cmp1cr`]
module"]
pub type CMP1CR = crate::Reg<cmp1cr::CMP1CR_SPEC>;
#[doc = "Timerx Compare 1 Register"]
pub mod cmp1cr;
#[doc = "CMP1CCR (rw) register accessor: Timerx Compare 1 Compound Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cmp1ccr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cmp1ccr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`cmp1ccr`]
module"]
pub type CMP1CCR = crate::Reg<cmp1ccr::CMP1CCR_SPEC>;
#[doc = "Timerx Compare 1 Compound Register"]
pub mod cmp1ccr;
#[doc = "CMP2CR (rw) register accessor: Timerx Compare 2 Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cmp2cr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cmp2cr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`cmp2cr`]
module"]
pub type CMP2CR = crate::Reg<cmp2cr::CMP2CR_SPEC>;
#[doc = "Timerx Compare 2 Register"]
pub mod cmp2cr;
#[doc = "CMP3CR (rw) register accessor: Timerx Compare 3 Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cmp3cr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cmp3cr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`cmp3cr`]
module"]
pub type CMP3CR = crate::Reg<cmp3cr::CMP3CR_SPEC>;
#[doc = "Timerx Compare 3 Register"]
pub mod cmp3cr;
#[doc = "CMP4CR (rw) register accessor: Timerx Compare 4 Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cmp4cr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cmp4cr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`cmp4cr`]
module"]
pub type CMP4CR = crate::Reg<cmp4cr::CMP4CR_SPEC>;
#[doc = "Timerx Compare 4 Register"]
pub mod cmp4cr;
#[doc = "CPT1CR (r) register accessor: Timerx Capture 1 Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cpt1cr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`cpt1cr`]
module"]
pub type CPT1CR = crate::Reg<cpt1cr::CPT1CR_SPEC>;
#[doc = "Timerx Capture 1 Register"]
pub mod cpt1cr;
#[doc = "CPT2CR (r) register accessor: Timerx Capture 2 Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cpt2cr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`cpt2cr`]
module"]
pub type CPT2CR = crate::Reg<cpt2cr::CPT2CR_SPEC>;
#[doc = "Timerx Capture 2 Register"]
pub mod cpt2cr;
#[doc = "DTCR (rw) register accessor: Timerx Deadtime Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dtcr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dtcr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`dtcr`]
module"]
pub type DTCR = crate::Reg<dtcr::DTCR_SPEC>;
#[doc = "Timerx Deadtime Register"]
pub mod dtcr;
#[doc = "SETC1R (rw) register accessor: Timerx Output1 Set Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`setc1r::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`setc1r::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`setc1r`]
module"]
pub type SETC1R = crate::Reg<setc1r::SETC1R_SPEC>;
#[doc = "Timerx Output1 Set Register"]
pub mod setc1r;
#[doc = "RSTC1R (rw) register accessor: Timerx Output1 Reset Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rstc1r::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rstc1r::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`rstc1r`]
module"]
pub type RSTC1R = crate::Reg<rstc1r::RSTC1R_SPEC>;
#[doc = "Timerx Output1 Reset Register"]
pub mod rstc1r;
#[doc = "SETC2R (rw) register accessor: Timerx Output2 Set Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`setc2r::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`setc2r::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`setc2r`]
module"]
pub type SETC2R = crate::Reg<setc2r::SETC2R_SPEC>;
#[doc = "Timerx Output2 Set Register"]
pub mod setc2r;
#[doc = "RSTC2R (rw) register accessor: Timerx Output2 Reset Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rstc2r::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rstc2r::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`rstc2r`]
module"]
pub type RSTC2R = crate::Reg<rstc2r::RSTC2R_SPEC>;
#[doc = "Timerx Output2 Reset Register"]
pub mod rstc2r;
#[doc = "EEFCR1 (rw) register accessor: Timerx External Event Filtering Register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`eefcr1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`eefcr1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`eefcr1`]
module"]
pub type EEFCR1 = crate::Reg<eefcr1::EEFCR1_SPEC>;
#[doc = "Timerx External Event Filtering Register 1"]
pub mod eefcr1;
#[doc = "EEFCR2 (rw) register accessor: Timerx External Event Filtering Register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`eefcr2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`eefcr2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`eefcr2`]
module"]
pub type EEFCR2 = crate::Reg<eefcr2::EEFCR2_SPEC>;
#[doc = "Timerx External Event Filtering Register 2"]
pub mod eefcr2;
#[doc = "RSTCR (rw) register accessor: TimerA Reset Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rstcr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rstcr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`rstcr`]
module"]
pub type RSTCR = crate::Reg<rstcr::RSTCR_SPEC>;
#[doc = "TimerA Reset Register"]
pub mod rstcr;
#[doc = "CHPCR (rw) register accessor: Timerx Chopper Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`chpcr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`chpcr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`chpcr`]
module"]
pub type CHPCR = crate::Reg<chpcr::CHPCR_SPEC>;
#[doc = "Timerx Chopper Register"]
pub mod chpcr;
#[doc = "CPT1CCR (rw) register accessor: Timerx Capture 2 Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cpt1ccr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cpt1ccr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`cpt1ccr`]
module"]
pub type CPT1CCR = crate::Reg<cpt1ccr::CPT1CCR_SPEC>;
#[doc = "Timerx Capture 2 Control Register"]
pub mod cpt1ccr;
#[doc = "CPT2CCR (rw) register accessor: CPT2xCR\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cpt2ccr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cpt2ccr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`cpt2ccr`]
module"]
pub type CPT2CCR = crate::Reg<cpt2ccr::CPT2CCR_SPEC>;
#[doc = "CPT2xCR"]
pub mod cpt2ccr;
#[doc = "OUTCR (rw) register accessor: Timerx Output Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`outcr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`outcr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`outcr`]
module"]
pub type OUTCR = crate::Reg<outcr::OUTCR_SPEC>;
#[doc = "Timerx Output Register"]
pub mod outcr;
#[doc = "FLTCR (rw) register accessor: Timerx Fault Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fltcr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fltcr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`fltcr`]
module"]
pub type FLTCR = crate::Reg<fltcr::FLTCR_SPEC>;
#[doc = "Timerx Fault Register"]
pub mod fltcr;
