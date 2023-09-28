#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - The RTC_TR is the calendar time shadow register. This register must be written in initialization mode only. Refer to Calendar initialization and configuration on page9 and Reading the calendar on page10.This register is write protected. The write access procedure is described in RTC register write protection on page9."]
    pub rtc_tr: RTC_TR,
    #[doc = "0x04 - The RTC_DR is the calendar date shadow register. This register must be written in initialization mode only. Refer to Calendar initialization and configuration on page9 and Reading the calendar on page10.This register is write protected. The write access procedure is described in RTC register write protection on page9."]
    pub rtc_dr: RTC_DR,
    #[doc = "0x08 - RTC control register"]
    pub rtc_cr: RTC_CR,
    #[doc = "0x0c - This register is write protected (except for RTC_ISR\\[13:8\\]
bits). The write access procedure is described in RTC register write protection on page9."]
    pub rtc_isr: RTC_ISR,
    #[doc = "0x10 - This register must be written in initialization mode only. The initialization must be performed in two separate write accesses. Refer to Calendar initialization and configuration on page9.This register is write protected. The write access procedure is described in RTC register write protection on page9."]
    pub rtc_prer: RTC_PRER,
    #[doc = "0x14 - This register can be written only when WUTWF is set to 1 in RTC_ISR.This register is write protected. The write access procedure is described in RTC register write protection on page9."]
    pub rtc_wutr: RTC_WUTR,
    _reserved6: [u8; 0x04],
    #[doc = "0x1c - This register can be written only when ALRAWF is set to 1 in RTC_ISR, or in initialization mode.This register is write protected. The write access procedure is described in RTC register write protection on page9."]
    pub rtc_alrmar: RTC_ALRMAR,
    #[doc = "0x20 - This register can be written only when ALRBWF is set to 1 in RTC_ISR, or in initialization mode.This register is write protected. The write access procedure is described in RTC register write protection on page9."]
    pub rtc_alrmbr: RTC_ALRMBR,
    #[doc = "0x24 - RTC write protection register"]
    pub rtc_wpr: RTC_WPR,
    #[doc = "0x28 - RTC sub second register"]
    pub rtc_ssr: RTC_SSR,
    #[doc = "0x2c - This register is write protected. The write access procedure is described in RTC register write protection on page9."]
    pub rtc_shiftr: RTC_SHIFTR,
    #[doc = "0x30 - The content of this register is valid only when TSF is set to 1 in RTC_ISR. It is cleared when TSF bit is reset."]
    pub rtc_tstr: RTC_TSTR,
    #[doc = "0x34 - The content of this register is valid only when TSF is set to 1 in RTC_ISR. It is cleared when TSF bit is reset."]
    pub rtc_tsdr: RTC_TSDR,
    #[doc = "0x38 - The content of this register is valid only when RTC_ISR/TSF is set. It is cleared when the RTC_ISR/TSF bit is reset."]
    pub rtc_tsssr: RTC_TSSSR,
    #[doc = "0x3c - This register is write protected. The write access procedure is described in RTC register write protection on page9."]
    pub rtc_calr: RTC_CALR,
    #[doc = "0x40 - RTC tamper and alternate function configuration register"]
    pub rtc_tampcr: RTC_TAMPCR,
    #[doc = "0x44 - This register can be written only when ALRAE is reset in RTC_CR register, or in initialization mode.This register is write protected. The write access procedure is described in RTC register write protection on page9"]
    pub rtc_alrmassr: RTC_ALRMASSR,
    #[doc = "0x48 - This register can be written only when ALRBE is reset in RTC_CR register, or in initialization mode.This register is write protected.The write access procedure is described in Section: RTC register write protection."]
    pub rtc_alrmbssr: RTC_ALRMBSSR,
    #[doc = "0x4c - RTC option register"]
    pub rtc_or: RTC_OR,
    #[doc = "0x50 - RTC backup registers"]
    pub rtc_bkp0r: RTC_BKP0R,
    #[doc = "0x54 - RTC backup registers"]
    pub rtc_bkp1r: RTC_BKP1R,
    #[doc = "0x58 - RTC backup registers"]
    pub rtc_bkp2r: RTC_BKP2R,
    #[doc = "0x5c - RTC backup registers"]
    pub rtc_bkp3r: RTC_BKP3R,
    #[doc = "0x60 - RTC backup registers"]
    pub rtc_bkp4r: RTC_BKP4R,
    #[doc = "0x64 - RTC backup registers"]
    pub rtc_bkp5r: RTC_BKP5R,
    #[doc = "0x68 - RTC backup registers"]
    pub rtc_bkp6r: RTC_BKP6R,
    #[doc = "0x6c - RTC backup registers"]
    pub rtc_bkp7r: RTC_BKP7R,
    #[doc = "0x70 - RTC backup registers"]
    pub rtc_bkp8r: RTC_BKP8R,
    #[doc = "0x74 - RTC backup registers"]
    pub rtc_bkp9r: RTC_BKP9R,
    #[doc = "0x78 - RTC backup registers"]
    pub rtc_bkp10r: RTC_BKP10R,
    #[doc = "0x7c - RTC backup registers"]
    pub rtc_bkp11r: RTC_BKP11R,
    #[doc = "0x80 - RTC backup registers"]
    pub rtc_bkp12r: RTC_BKP12R,
    #[doc = "0x84 - RTC backup registers"]
    pub rtc_bkp13r: RTC_BKP13R,
    #[doc = "0x88 - RTC backup registers"]
    pub rtc_bkp14r: RTC_BKP14R,
    #[doc = "0x8c - RTC backup registers"]
    pub rtc_bkp15r: RTC_BKP15R,
    #[doc = "0x90 - RTC backup registers"]
    pub rtc_bkp16r: RTC_BKP16R,
    #[doc = "0x94 - RTC backup registers"]
    pub rtc_bkp17r: RTC_BKP17R,
    #[doc = "0x98 - RTC backup registers"]
    pub rtc_bkp18r: RTC_BKP18R,
    #[doc = "0x9c - RTC backup registers"]
    pub rtc_bkp19r: RTC_BKP19R,
    #[doc = "0xa0 - RTC backup registers"]
    pub rtc_bkp20r: RTC_BKP20R,
    #[doc = "0xa4 - RTC backup registers"]
    pub rtc_bkp21r: RTC_BKP21R,
    #[doc = "0xa8 - RTC backup registers"]
    pub rtc_bkp22r: RTC_BKP22R,
    #[doc = "0xac - RTC backup registers"]
    pub rtc_bkp23r: RTC_BKP23R,
    #[doc = "0xb0 - RTC backup registers"]
    pub rtc_bkp24r: RTC_BKP24R,
    #[doc = "0xb4 - RTC backup registers"]
    pub rtc_bkp25r: RTC_BKP25R,
    #[doc = "0xb8 - RTC backup registers"]
    pub rtc_bkp26r: RTC_BKP26R,
    #[doc = "0xbc - RTC backup registers"]
    pub rtc_bkp27r: RTC_BKP27R,
    #[doc = "0xc0 - RTC backup registers"]
    pub rtc_bkp28r: RTC_BKP28R,
    #[doc = "0xc4 - RTC backup registers"]
    pub rtc_bkp29r: RTC_BKP29R,
    #[doc = "0xc8 - RTC backup registers"]
    pub rtc_bkp30r: RTC_BKP30R,
    #[doc = "0xcc - RTC backup registers"]
    pub rtc_bkp31r: RTC_BKP31R,
}
#[doc = "RTC_TR (rw) register accessor: The RTC_TR is the calendar time shadow register. This register must be written in initialization mode only. Refer to Calendar initialization and configuration on page9 and Reading the calendar on page10.This register is write protected. The write access procedure is described in RTC register write protection on page9.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rtc_tr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rtc_tr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`rtc_tr`]
module"]
pub type RTC_TR = crate::Reg<rtc_tr::RTC_TR_SPEC>;
#[doc = "The RTC_TR is the calendar time shadow register. This register must be written in initialization mode only. Refer to Calendar initialization and configuration on page9 and Reading the calendar on page10.This register is write protected. The write access procedure is described in RTC register write protection on page9."]
pub mod rtc_tr;
#[doc = "RTC_DR (rw) register accessor: The RTC_DR is the calendar date shadow register. This register must be written in initialization mode only. Refer to Calendar initialization and configuration on page9 and Reading the calendar on page10.This register is write protected. The write access procedure is described in RTC register write protection on page9.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rtc_dr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rtc_dr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`rtc_dr`]
module"]
pub type RTC_DR = crate::Reg<rtc_dr::RTC_DR_SPEC>;
#[doc = "The RTC_DR is the calendar date shadow register. This register must be written in initialization mode only. Refer to Calendar initialization and configuration on page9 and Reading the calendar on page10.This register is write protected. The write access procedure is described in RTC register write protection on page9."]
pub mod rtc_dr;
#[doc = "RTC_CR (rw) register accessor: RTC control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rtc_cr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rtc_cr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`rtc_cr`]
module"]
pub type RTC_CR = crate::Reg<rtc_cr::RTC_CR_SPEC>;
#[doc = "RTC control register"]
pub mod rtc_cr;
#[doc = "RTC_ISR (rw) register accessor: This register is write protected (except for RTC_ISR\\[13:8\\]
bits). The write access procedure is described in RTC register write protection on page9.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rtc_isr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rtc_isr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`rtc_isr`]
module"]
pub type RTC_ISR = crate::Reg<rtc_isr::RTC_ISR_SPEC>;
#[doc = "This register is write protected (except for RTC_ISR\\[13:8\\]
bits). The write access procedure is described in RTC register write protection on page9."]
pub mod rtc_isr;
#[doc = "RTC_PRER (rw) register accessor: This register must be written in initialization mode only. The initialization must be performed in two separate write accesses. Refer to Calendar initialization and configuration on page9.This register is write protected. The write access procedure is described in RTC register write protection on page9.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rtc_prer::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rtc_prer::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`rtc_prer`]
module"]
pub type RTC_PRER = crate::Reg<rtc_prer::RTC_PRER_SPEC>;
#[doc = "This register must be written in initialization mode only. The initialization must be performed in two separate write accesses. Refer to Calendar initialization and configuration on page9.This register is write protected. The write access procedure is described in RTC register write protection on page9."]
pub mod rtc_prer;
#[doc = "RTC_WUTR (rw) register accessor: This register can be written only when WUTWF is set to 1 in RTC_ISR.This register is write protected. The write access procedure is described in RTC register write protection on page9.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rtc_wutr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rtc_wutr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`rtc_wutr`]
module"]
pub type RTC_WUTR = crate::Reg<rtc_wutr::RTC_WUTR_SPEC>;
#[doc = "This register can be written only when WUTWF is set to 1 in RTC_ISR.This register is write protected. The write access procedure is described in RTC register write protection on page9."]
pub mod rtc_wutr;
#[doc = "RTC_ALRMAR (rw) register accessor: This register can be written only when ALRAWF is set to 1 in RTC_ISR, or in initialization mode.This register is write protected. The write access procedure is described in RTC register write protection on page9.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rtc_alrmar::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rtc_alrmar::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`rtc_alrmar`]
module"]
pub type RTC_ALRMAR = crate::Reg<rtc_alrmar::RTC_ALRMAR_SPEC>;
#[doc = "This register can be written only when ALRAWF is set to 1 in RTC_ISR, or in initialization mode.This register is write protected. The write access procedure is described in RTC register write protection on page9."]
pub mod rtc_alrmar;
#[doc = "RTC_ALRMBR (rw) register accessor: This register can be written only when ALRBWF is set to 1 in RTC_ISR, or in initialization mode.This register is write protected. The write access procedure is described in RTC register write protection on page9.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rtc_alrmbr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rtc_alrmbr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`rtc_alrmbr`]
module"]
pub type RTC_ALRMBR = crate::Reg<rtc_alrmbr::RTC_ALRMBR_SPEC>;
#[doc = "This register can be written only when ALRBWF is set to 1 in RTC_ISR, or in initialization mode.This register is write protected. The write access procedure is described in RTC register write protection on page9."]
pub mod rtc_alrmbr;
#[doc = "RTC_WPR (w) register accessor: RTC write protection register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rtc_wpr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`rtc_wpr`]
module"]
pub type RTC_WPR = crate::Reg<rtc_wpr::RTC_WPR_SPEC>;
#[doc = "RTC write protection register"]
pub mod rtc_wpr;
#[doc = "RTC_SSR (r) register accessor: RTC sub second register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rtc_ssr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`rtc_ssr`]
module"]
pub type RTC_SSR = crate::Reg<rtc_ssr::RTC_SSR_SPEC>;
#[doc = "RTC sub second register"]
pub mod rtc_ssr;
#[doc = "RTC_SHIFTR (w) register accessor: This register is write protected. The write access procedure is described in RTC register write protection on page9.\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rtc_shiftr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`rtc_shiftr`]
module"]
pub type RTC_SHIFTR = crate::Reg<rtc_shiftr::RTC_SHIFTR_SPEC>;
#[doc = "This register is write protected. The write access procedure is described in RTC register write protection on page9."]
pub mod rtc_shiftr;
#[doc = "RTC_TSTR (r) register accessor: The content of this register is valid only when TSF is set to 1 in RTC_ISR. It is cleared when TSF bit is reset.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rtc_tstr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`rtc_tstr`]
module"]
pub type RTC_TSTR = crate::Reg<rtc_tstr::RTC_TSTR_SPEC>;
#[doc = "The content of this register is valid only when TSF is set to 1 in RTC_ISR. It is cleared when TSF bit is reset."]
pub mod rtc_tstr;
#[doc = "RTC_TSDR (r) register accessor: The content of this register is valid only when TSF is set to 1 in RTC_ISR. It is cleared when TSF bit is reset.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rtc_tsdr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`rtc_tsdr`]
module"]
pub type RTC_TSDR = crate::Reg<rtc_tsdr::RTC_TSDR_SPEC>;
#[doc = "The content of this register is valid only when TSF is set to 1 in RTC_ISR. It is cleared when TSF bit is reset."]
pub mod rtc_tsdr;
#[doc = "RTC_TSSSR (r) register accessor: The content of this register is valid only when RTC_ISR/TSF is set. It is cleared when the RTC_ISR/TSF bit is reset.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rtc_tsssr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`rtc_tsssr`]
module"]
pub type RTC_TSSSR = crate::Reg<rtc_tsssr::RTC_TSSSR_SPEC>;
#[doc = "The content of this register is valid only when RTC_ISR/TSF is set. It is cleared when the RTC_ISR/TSF bit is reset."]
pub mod rtc_tsssr;
#[doc = "RTC_CALR (rw) register accessor: This register is write protected. The write access procedure is described in RTC register write protection on page9.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rtc_calr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rtc_calr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`rtc_calr`]
module"]
pub type RTC_CALR = crate::Reg<rtc_calr::RTC_CALR_SPEC>;
#[doc = "This register is write protected. The write access procedure is described in RTC register write protection on page9."]
pub mod rtc_calr;
#[doc = "RTC_TAMPCR (rw) register accessor: RTC tamper and alternate function configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rtc_tampcr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rtc_tampcr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`rtc_tampcr`]
module"]
pub type RTC_TAMPCR = crate::Reg<rtc_tampcr::RTC_TAMPCR_SPEC>;
#[doc = "RTC tamper and alternate function configuration register"]
pub mod rtc_tampcr;
#[doc = "RTC_ALRMASSR (rw) register accessor: This register can be written only when ALRAE is reset in RTC_CR register, or in initialization mode.This register is write protected. The write access procedure is described in RTC register write protection on page9\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rtc_alrmassr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rtc_alrmassr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`rtc_alrmassr`]
module"]
pub type RTC_ALRMASSR = crate::Reg<rtc_alrmassr::RTC_ALRMASSR_SPEC>;
#[doc = "This register can be written only when ALRAE is reset in RTC_CR register, or in initialization mode.This register is write protected. The write access procedure is described in RTC register write protection on page9"]
pub mod rtc_alrmassr;
#[doc = "RTC_ALRMBSSR (rw) register accessor: This register can be written only when ALRBE is reset in RTC_CR register, or in initialization mode.This register is write protected.The write access procedure is described in Section: RTC register write protection.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rtc_alrmbssr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rtc_alrmbssr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`rtc_alrmbssr`]
module"]
pub type RTC_ALRMBSSR = crate::Reg<rtc_alrmbssr::RTC_ALRMBSSR_SPEC>;
#[doc = "This register can be written only when ALRBE is reset in RTC_CR register, or in initialization mode.This register is write protected.The write access procedure is described in Section: RTC register write protection."]
pub mod rtc_alrmbssr;
#[doc = "RTC_BKP0R (rw) register accessor: RTC backup registers\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rtc_bkp0r::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rtc_bkp0r::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`rtc_bkp0r`]
module"]
pub type RTC_BKP0R = crate::Reg<rtc_bkp0r::RTC_BKP0R_SPEC>;
#[doc = "RTC backup registers"]
pub mod rtc_bkp0r;
#[doc = "RTC_BKP1R (rw) register accessor: RTC backup registers\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rtc_bkp1r::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rtc_bkp1r::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`rtc_bkp1r`]
module"]
pub type RTC_BKP1R = crate::Reg<rtc_bkp1r::RTC_BKP1R_SPEC>;
#[doc = "RTC backup registers"]
pub mod rtc_bkp1r;
#[doc = "RTC_BKP2R (rw) register accessor: RTC backup registers\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rtc_bkp2r::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rtc_bkp2r::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`rtc_bkp2r`]
module"]
pub type RTC_BKP2R = crate::Reg<rtc_bkp2r::RTC_BKP2R_SPEC>;
#[doc = "RTC backup registers"]
pub mod rtc_bkp2r;
#[doc = "RTC_BKP3R (rw) register accessor: RTC backup registers\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rtc_bkp3r::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rtc_bkp3r::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`rtc_bkp3r`]
module"]
pub type RTC_BKP3R = crate::Reg<rtc_bkp3r::RTC_BKP3R_SPEC>;
#[doc = "RTC backup registers"]
pub mod rtc_bkp3r;
#[doc = "RTC_BKP4R (rw) register accessor: RTC backup registers\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rtc_bkp4r::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rtc_bkp4r::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`rtc_bkp4r`]
module"]
pub type RTC_BKP4R = crate::Reg<rtc_bkp4r::RTC_BKP4R_SPEC>;
#[doc = "RTC backup registers"]
pub mod rtc_bkp4r;
#[doc = "RTC_BKP5R (rw) register accessor: RTC backup registers\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rtc_bkp5r::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rtc_bkp5r::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`rtc_bkp5r`]
module"]
pub type RTC_BKP5R = crate::Reg<rtc_bkp5r::RTC_BKP5R_SPEC>;
#[doc = "RTC backup registers"]
pub mod rtc_bkp5r;
#[doc = "RTC_BKP6R (rw) register accessor: RTC backup registers\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rtc_bkp6r::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rtc_bkp6r::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`rtc_bkp6r`]
module"]
pub type RTC_BKP6R = crate::Reg<rtc_bkp6r::RTC_BKP6R_SPEC>;
#[doc = "RTC backup registers"]
pub mod rtc_bkp6r;
#[doc = "RTC_BKP7R (rw) register accessor: RTC backup registers\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rtc_bkp7r::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rtc_bkp7r::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`rtc_bkp7r`]
module"]
pub type RTC_BKP7R = crate::Reg<rtc_bkp7r::RTC_BKP7R_SPEC>;
#[doc = "RTC backup registers"]
pub mod rtc_bkp7r;
#[doc = "RTC_BKP8R (rw) register accessor: RTC backup registers\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rtc_bkp8r::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rtc_bkp8r::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`rtc_bkp8r`]
module"]
pub type RTC_BKP8R = crate::Reg<rtc_bkp8r::RTC_BKP8R_SPEC>;
#[doc = "RTC backup registers"]
pub mod rtc_bkp8r;
#[doc = "RTC_BKP9R (rw) register accessor: RTC backup registers\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rtc_bkp9r::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rtc_bkp9r::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`rtc_bkp9r`]
module"]
pub type RTC_BKP9R = crate::Reg<rtc_bkp9r::RTC_BKP9R_SPEC>;
#[doc = "RTC backup registers"]
pub mod rtc_bkp9r;
#[doc = "RTC_BKP10R (rw) register accessor: RTC backup registers\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rtc_bkp10r::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rtc_bkp10r::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`rtc_bkp10r`]
module"]
pub type RTC_BKP10R = crate::Reg<rtc_bkp10r::RTC_BKP10R_SPEC>;
#[doc = "RTC backup registers"]
pub mod rtc_bkp10r;
#[doc = "RTC_BKP11R (rw) register accessor: RTC backup registers\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rtc_bkp11r::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rtc_bkp11r::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`rtc_bkp11r`]
module"]
pub type RTC_BKP11R = crate::Reg<rtc_bkp11r::RTC_BKP11R_SPEC>;
#[doc = "RTC backup registers"]
pub mod rtc_bkp11r;
#[doc = "RTC_BKP12R (rw) register accessor: RTC backup registers\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rtc_bkp12r::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rtc_bkp12r::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`rtc_bkp12r`]
module"]
pub type RTC_BKP12R = crate::Reg<rtc_bkp12r::RTC_BKP12R_SPEC>;
#[doc = "RTC backup registers"]
pub mod rtc_bkp12r;
#[doc = "RTC_BKP13R (rw) register accessor: RTC backup registers\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rtc_bkp13r::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rtc_bkp13r::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`rtc_bkp13r`]
module"]
pub type RTC_BKP13R = crate::Reg<rtc_bkp13r::RTC_BKP13R_SPEC>;
#[doc = "RTC backup registers"]
pub mod rtc_bkp13r;
#[doc = "RTC_BKP14R (rw) register accessor: RTC backup registers\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rtc_bkp14r::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rtc_bkp14r::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`rtc_bkp14r`]
module"]
pub type RTC_BKP14R = crate::Reg<rtc_bkp14r::RTC_BKP14R_SPEC>;
#[doc = "RTC backup registers"]
pub mod rtc_bkp14r;
#[doc = "RTC_BKP15R (rw) register accessor: RTC backup registers\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rtc_bkp15r::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rtc_bkp15r::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`rtc_bkp15r`]
module"]
pub type RTC_BKP15R = crate::Reg<rtc_bkp15r::RTC_BKP15R_SPEC>;
#[doc = "RTC backup registers"]
pub mod rtc_bkp15r;
#[doc = "RTC_OR (rw) register accessor: RTC option register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rtc_or::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rtc_or::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`rtc_or`]
module"]
pub type RTC_OR = crate::Reg<rtc_or::RTC_OR_SPEC>;
#[doc = "RTC option register"]
pub mod rtc_or;
#[doc = "RTC_BKP16R (rw) register accessor: RTC backup registers\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rtc_bkp16r::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rtc_bkp16r::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`rtc_bkp16r`]
module"]
pub type RTC_BKP16R = crate::Reg<rtc_bkp16r::RTC_BKP16R_SPEC>;
#[doc = "RTC backup registers"]
pub mod rtc_bkp16r;
#[doc = "RTC_BKP17R (rw) register accessor: RTC backup registers\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rtc_bkp17r::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rtc_bkp17r::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`rtc_bkp17r`]
module"]
pub type RTC_BKP17R = crate::Reg<rtc_bkp17r::RTC_BKP17R_SPEC>;
#[doc = "RTC backup registers"]
pub mod rtc_bkp17r;
#[doc = "RTC_BKP18R (rw) register accessor: RTC backup registers\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rtc_bkp18r::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rtc_bkp18r::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`rtc_bkp18r`]
module"]
pub type RTC_BKP18R = crate::Reg<rtc_bkp18r::RTC_BKP18R_SPEC>;
#[doc = "RTC backup registers"]
pub mod rtc_bkp18r;
#[doc = "RTC_BKP19R (rw) register accessor: RTC backup registers\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rtc_bkp19r::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rtc_bkp19r::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`rtc_bkp19r`]
module"]
pub type RTC_BKP19R = crate::Reg<rtc_bkp19r::RTC_BKP19R_SPEC>;
#[doc = "RTC backup registers"]
pub mod rtc_bkp19r;
#[doc = "RTC_BKP20R (rw) register accessor: RTC backup registers\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rtc_bkp20r::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rtc_bkp20r::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`rtc_bkp20r`]
module"]
pub type RTC_BKP20R = crate::Reg<rtc_bkp20r::RTC_BKP20R_SPEC>;
#[doc = "RTC backup registers"]
pub mod rtc_bkp20r;
#[doc = "RTC_BKP21R (rw) register accessor: RTC backup registers\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rtc_bkp21r::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rtc_bkp21r::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`rtc_bkp21r`]
module"]
pub type RTC_BKP21R = crate::Reg<rtc_bkp21r::RTC_BKP21R_SPEC>;
#[doc = "RTC backup registers"]
pub mod rtc_bkp21r;
#[doc = "RTC_BKP22R (rw) register accessor: RTC backup registers\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rtc_bkp22r::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rtc_bkp22r::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`rtc_bkp22r`]
module"]
pub type RTC_BKP22R = crate::Reg<rtc_bkp22r::RTC_BKP22R_SPEC>;
#[doc = "RTC backup registers"]
pub mod rtc_bkp22r;
#[doc = "RTC_BKP23R (rw) register accessor: RTC backup registers\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rtc_bkp23r::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rtc_bkp23r::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`rtc_bkp23r`]
module"]
pub type RTC_BKP23R = crate::Reg<rtc_bkp23r::RTC_BKP23R_SPEC>;
#[doc = "RTC backup registers"]
pub mod rtc_bkp23r;
#[doc = "RTC_BKP24R (rw) register accessor: RTC backup registers\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rtc_bkp24r::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rtc_bkp24r::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`rtc_bkp24r`]
module"]
pub type RTC_BKP24R = crate::Reg<rtc_bkp24r::RTC_BKP24R_SPEC>;
#[doc = "RTC backup registers"]
pub mod rtc_bkp24r;
#[doc = "RTC_BKP25R (rw) register accessor: RTC backup registers\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rtc_bkp25r::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rtc_bkp25r::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`rtc_bkp25r`]
module"]
pub type RTC_BKP25R = crate::Reg<rtc_bkp25r::RTC_BKP25R_SPEC>;
#[doc = "RTC backup registers"]
pub mod rtc_bkp25r;
#[doc = "RTC_BKP26R (rw) register accessor: RTC backup registers\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rtc_bkp26r::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rtc_bkp26r::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`rtc_bkp26r`]
module"]
pub type RTC_BKP26R = crate::Reg<rtc_bkp26r::RTC_BKP26R_SPEC>;
#[doc = "RTC backup registers"]
pub mod rtc_bkp26r;
#[doc = "RTC_BKP27R (rw) register accessor: RTC backup registers\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rtc_bkp27r::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rtc_bkp27r::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`rtc_bkp27r`]
module"]
pub type RTC_BKP27R = crate::Reg<rtc_bkp27r::RTC_BKP27R_SPEC>;
#[doc = "RTC backup registers"]
pub mod rtc_bkp27r;
#[doc = "RTC_BKP28R (rw) register accessor: RTC backup registers\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rtc_bkp28r::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rtc_bkp28r::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`rtc_bkp28r`]
module"]
pub type RTC_BKP28R = crate::Reg<rtc_bkp28r::RTC_BKP28R_SPEC>;
#[doc = "RTC backup registers"]
pub mod rtc_bkp28r;
#[doc = "RTC_BKP29R (rw) register accessor: RTC backup registers\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rtc_bkp29r::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rtc_bkp29r::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`rtc_bkp29r`]
module"]
pub type RTC_BKP29R = crate::Reg<rtc_bkp29r::RTC_BKP29R_SPEC>;
#[doc = "RTC backup registers"]
pub mod rtc_bkp29r;
#[doc = "RTC_BKP30R (rw) register accessor: RTC backup registers\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rtc_bkp30r::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rtc_bkp30r::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`rtc_bkp30r`]
module"]
pub type RTC_BKP30R = crate::Reg<rtc_bkp30r::RTC_BKP30R_SPEC>;
#[doc = "RTC backup registers"]
pub mod rtc_bkp30r;
#[doc = "RTC_BKP31R (rw) register accessor: RTC backup registers\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rtc_bkp31r::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rtc_bkp31r::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`rtc_bkp31r`]
module"]
pub type RTC_BKP31R = crate::Reg<rtc_bkp31r::RTC_BKP31R_SPEC>;
#[doc = "RTC backup registers"]
pub mod rtc_bkp31r;
