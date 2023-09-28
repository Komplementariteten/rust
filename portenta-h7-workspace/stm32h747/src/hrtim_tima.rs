#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Timerx Control Register"]
    pub timacr: TIMACR,
    #[doc = "0x04 - Timerx Interrupt Status Register"]
    pub timaisr: TIMAISR,
    #[doc = "0x08 - Timerx Interrupt Clear Register"]
    pub timaicr: TIMAICR,
    #[doc = "0x0c - TIMxDIER5"]
    pub timadier5: TIMADIER5,
    #[doc = "0x10 - Timerx Counter Register"]
    pub cntar: CNTAR,
    #[doc = "0x14 - Timerx Period Register"]
    pub perar: PERAR,
    #[doc = "0x18 - Timerx Repetition Register"]
    pub repar: REPAR,
    #[doc = "0x1c - Timerx Compare 1 Register"]
    pub cmp1ar: CMP1AR,
    #[doc = "0x20 - Timerx Compare 1 Compound Register"]
    pub cmp1car: CMP1CAR,
    #[doc = "0x24 - Timerx Compare 2 Register"]
    pub cmp2ar: CMP2AR,
    #[doc = "0x28 - Timerx Compare 3 Register"]
    pub cmp3ar: CMP3AR,
    #[doc = "0x2c - Timerx Compare 4 Register"]
    pub cmp4ar: CMP4AR,
    #[doc = "0x30 - Timerx Capture 1 Register"]
    pub cpt1ar: CPT1AR,
    #[doc = "0x34 - Timerx Capture 2 Register"]
    pub cpt2ar: CPT2AR,
    #[doc = "0x38 - Timerx Deadtime Register"]
    pub dtar: DTAR,
    #[doc = "0x3c - Timerx Output1 Set Register"]
    pub seta1r: SETA1R,
    #[doc = "0x40 - Timerx Output1 Reset Register"]
    pub rsta1r: RSTA1R,
    #[doc = "0x44 - Timerx Output2 Set Register"]
    pub seta2r: SETA2R,
    #[doc = "0x48 - Timerx Output2 Reset Register"]
    pub rsta2r: RSTA2R,
    #[doc = "0x4c - Timerx External Event Filtering Register 1"]
    pub eefar1: EEFAR1,
    #[doc = "0x50 - Timerx External Event Filtering Register 2"]
    pub eefar2: EEFAR2,
    #[doc = "0x54 - TimerA Reset Register"]
    pub rstar: RSTAR,
    #[doc = "0x58 - Timerx Chopper Register"]
    pub chpar: CHPAR,
    #[doc = "0x5c - Timerx Capture 2 Control Register"]
    pub cpt1acr: CPT1ACR,
    #[doc = "0x60 - CPT2xCR"]
    pub cpt2acr: CPT2ACR,
    #[doc = "0x64 - Timerx Output Register"]
    pub outar: OUTAR,
    #[doc = "0x68 - Timerx Fault Register"]
    pub fltar: FLTAR,
}
#[doc = "TIMACR (rw) register accessor: Timerx Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`timacr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`timacr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`timacr`]
module"]
pub type TIMACR = crate::Reg<timacr::TIMACR_SPEC>;
#[doc = "Timerx Control Register"]
pub mod timacr;
#[doc = "TIMAISR (r) register accessor: Timerx Interrupt Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`timaisr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`timaisr`]
module"]
pub type TIMAISR = crate::Reg<timaisr::TIMAISR_SPEC>;
#[doc = "Timerx Interrupt Status Register"]
pub mod timaisr;
#[doc = "TIMAICR (w) register accessor: Timerx Interrupt Clear Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`timaicr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`timaicr`]
module"]
pub type TIMAICR = crate::Reg<timaicr::TIMAICR_SPEC>;
#[doc = "Timerx Interrupt Clear Register"]
pub mod timaicr;
#[doc = "TIMADIER5 (rw) register accessor: TIMxDIER5\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`timadier5::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`timadier5::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`timadier5`]
module"]
pub type TIMADIER5 = crate::Reg<timadier5::TIMADIER5_SPEC>;
#[doc = "TIMxDIER5"]
pub mod timadier5;
#[doc = "CNTAR (rw) register accessor: Timerx Counter Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cntar::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cntar::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`cntar`]
module"]
pub type CNTAR = crate::Reg<cntar::CNTAR_SPEC>;
#[doc = "Timerx Counter Register"]
pub mod cntar;
#[doc = "PERAR (rw) register accessor: Timerx Period Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`perar::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`perar::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`perar`]
module"]
pub type PERAR = crate::Reg<perar::PERAR_SPEC>;
#[doc = "Timerx Period Register"]
pub mod perar;
#[doc = "REPAR (rw) register accessor: Timerx Repetition Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`repar::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`repar::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`repar`]
module"]
pub type REPAR = crate::Reg<repar::REPAR_SPEC>;
#[doc = "Timerx Repetition Register"]
pub mod repar;
#[doc = "CMP1AR (rw) register accessor: Timerx Compare 1 Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cmp1ar::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cmp1ar::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`cmp1ar`]
module"]
pub type CMP1AR = crate::Reg<cmp1ar::CMP1AR_SPEC>;
#[doc = "Timerx Compare 1 Register"]
pub mod cmp1ar;
#[doc = "CMP1CAR (rw) register accessor: Timerx Compare 1 Compound Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cmp1car::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cmp1car::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`cmp1car`]
module"]
pub type CMP1CAR = crate::Reg<cmp1car::CMP1CAR_SPEC>;
#[doc = "Timerx Compare 1 Compound Register"]
pub mod cmp1car;
#[doc = "CMP2AR (rw) register accessor: Timerx Compare 2 Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cmp2ar::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cmp2ar::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`cmp2ar`]
module"]
pub type CMP2AR = crate::Reg<cmp2ar::CMP2AR_SPEC>;
#[doc = "Timerx Compare 2 Register"]
pub mod cmp2ar;
#[doc = "CMP3AR (rw) register accessor: Timerx Compare 3 Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cmp3ar::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cmp3ar::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`cmp3ar`]
module"]
pub type CMP3AR = crate::Reg<cmp3ar::CMP3AR_SPEC>;
#[doc = "Timerx Compare 3 Register"]
pub mod cmp3ar;
#[doc = "CMP4AR (rw) register accessor: Timerx Compare 4 Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cmp4ar::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cmp4ar::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`cmp4ar`]
module"]
pub type CMP4AR = crate::Reg<cmp4ar::CMP4AR_SPEC>;
#[doc = "Timerx Compare 4 Register"]
pub mod cmp4ar;
#[doc = "CPT1AR (r) register accessor: Timerx Capture 1 Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cpt1ar::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`cpt1ar`]
module"]
pub type CPT1AR = crate::Reg<cpt1ar::CPT1AR_SPEC>;
#[doc = "Timerx Capture 1 Register"]
pub mod cpt1ar;
#[doc = "CPT2AR (r) register accessor: Timerx Capture 2 Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cpt2ar::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`cpt2ar`]
module"]
pub type CPT2AR = crate::Reg<cpt2ar::CPT2AR_SPEC>;
#[doc = "Timerx Capture 2 Register"]
pub mod cpt2ar;
#[doc = "DTAR (rw) register accessor: Timerx Deadtime Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dtar::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dtar::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`dtar`]
module"]
pub type DTAR = crate::Reg<dtar::DTAR_SPEC>;
#[doc = "Timerx Deadtime Register"]
pub mod dtar;
#[doc = "SETA1R (rw) register accessor: Timerx Output1 Set Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`seta1r::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`seta1r::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`seta1r`]
module"]
pub type SETA1R = crate::Reg<seta1r::SETA1R_SPEC>;
#[doc = "Timerx Output1 Set Register"]
pub mod seta1r;
#[doc = "RSTA1R (rw) register accessor: Timerx Output1 Reset Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rsta1r::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rsta1r::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`rsta1r`]
module"]
pub type RSTA1R = crate::Reg<rsta1r::RSTA1R_SPEC>;
#[doc = "Timerx Output1 Reset Register"]
pub mod rsta1r;
#[doc = "SETA2R (rw) register accessor: Timerx Output2 Set Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`seta2r::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`seta2r::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`seta2r`]
module"]
pub type SETA2R = crate::Reg<seta2r::SETA2R_SPEC>;
#[doc = "Timerx Output2 Set Register"]
pub mod seta2r;
#[doc = "RSTA2R (rw) register accessor: Timerx Output2 Reset Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rsta2r::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rsta2r::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`rsta2r`]
module"]
pub type RSTA2R = crate::Reg<rsta2r::RSTA2R_SPEC>;
#[doc = "Timerx Output2 Reset Register"]
pub mod rsta2r;
#[doc = "EEFAR1 (rw) register accessor: Timerx External Event Filtering Register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`eefar1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`eefar1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`eefar1`]
module"]
pub type EEFAR1 = crate::Reg<eefar1::EEFAR1_SPEC>;
#[doc = "Timerx External Event Filtering Register 1"]
pub mod eefar1;
#[doc = "EEFAR2 (rw) register accessor: Timerx External Event Filtering Register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`eefar2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`eefar2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`eefar2`]
module"]
pub type EEFAR2 = crate::Reg<eefar2::EEFAR2_SPEC>;
#[doc = "Timerx External Event Filtering Register 2"]
pub mod eefar2;
#[doc = "RSTAR (rw) register accessor: TimerA Reset Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rstar::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rstar::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`rstar`]
module"]
pub type RSTAR = crate::Reg<rstar::RSTAR_SPEC>;
#[doc = "TimerA Reset Register"]
pub mod rstar;
#[doc = "CHPAR (rw) register accessor: Timerx Chopper Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`chpar::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`chpar::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`chpar`]
module"]
pub type CHPAR = crate::Reg<chpar::CHPAR_SPEC>;
#[doc = "Timerx Chopper Register"]
pub mod chpar;
#[doc = "CPT1ACR (rw) register accessor: Timerx Capture 2 Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cpt1acr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cpt1acr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`cpt1acr`]
module"]
pub type CPT1ACR = crate::Reg<cpt1acr::CPT1ACR_SPEC>;
#[doc = "Timerx Capture 2 Control Register"]
pub mod cpt1acr;
#[doc = "CPT2ACR (rw) register accessor: CPT2xCR\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cpt2acr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cpt2acr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`cpt2acr`]
module"]
pub type CPT2ACR = crate::Reg<cpt2acr::CPT2ACR_SPEC>;
#[doc = "CPT2xCR"]
pub mod cpt2acr;
#[doc = "OUTAR (rw) register accessor: Timerx Output Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`outar::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`outar::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`outar`]
module"]
pub type OUTAR = crate::Reg<outar::OUTAR_SPEC>;
#[doc = "Timerx Output Register"]
pub mod outar;
#[doc = "FLTAR (rw) register accessor: Timerx Fault Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fltar::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fltar::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`fltar`]
module"]
pub type FLTAR = crate::Reg<fltar::FLTAR_SPEC>;
#[doc = "Timerx Fault Register"]
pub mod fltar;
