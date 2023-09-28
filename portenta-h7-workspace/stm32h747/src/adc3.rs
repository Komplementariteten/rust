#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - ADC interrupt and status register"]
    pub isr: ISR,
    #[doc = "0x04 - ADC interrupt enable register"]
    pub ier: IER,
    #[doc = "0x08 - ADC control register"]
    pub cr: CR,
    #[doc = "0x0c - ADC configuration register 1"]
    pub cfgr: CFGR,
    #[doc = "0x10 - ADC configuration register 2"]
    pub cfgr2: CFGR2,
    #[doc = "0x14 - ADC sampling time register 1"]
    pub smpr1: SMPR1,
    #[doc = "0x18 - ADC sampling time register 2"]
    pub smpr2: SMPR2,
    #[doc = "0x1c - ADC pre channel selection register"]
    pub pcsel: PCSEL,
    #[doc = "0x20 - ADC analog watchdog 1 threshold register"]
    pub ltr1: LTR1,
    #[doc = "0x24 - ADC analog watchdog 2 threshold register"]
    pub lhtr1: LHTR1,
    _reserved10: [u8; 0x08],
    #[doc = "0x30 - ADC group regular sequencer ranks register 1"]
    pub sqr1: SQR1,
    #[doc = "0x34 - ADC group regular sequencer ranks register 2"]
    pub sqr2: SQR2,
    #[doc = "0x38 - ADC group regular sequencer ranks register 3"]
    pub sqr3: SQR3,
    #[doc = "0x3c - ADC group regular sequencer ranks register 4"]
    pub sqr4: SQR4,
    #[doc = "0x40 - ADC group regular conversion data register"]
    pub dr: DR,
    _reserved15: [u8; 0x08],
    #[doc = "0x4c - ADC group injected sequencer register"]
    pub jsqr: JSQR,
    _reserved16: [u8; 0x10],
    #[doc = "0x60 - ADC offset number 1 register"]
    pub ofr1: OFR1,
    #[doc = "0x64 - ADC offset number 2 register"]
    pub ofr2: OFR2,
    #[doc = "0x68 - ADC offset number 3 register"]
    pub ofr3: OFR3,
    #[doc = "0x6c - ADC offset number 4 register"]
    pub ofr4: OFR4,
    _reserved20: [u8; 0x10],
    #[doc = "0x80 - ADC group injected sequencer rank 1 register"]
    pub jdr1: JDR1,
    #[doc = "0x84 - ADC group injected sequencer rank 2 register"]
    pub jdr2: JDR2,
    #[doc = "0x88 - ADC group injected sequencer rank 3 register"]
    pub jdr3: JDR3,
    #[doc = "0x8c - ADC group injected sequencer rank 4 register"]
    pub jdr4: JDR4,
    _reserved24: [u8; 0x10],
    #[doc = "0xa0 - ADC analog watchdog 2 configuration register"]
    pub awd2cr: AWD2CR,
    #[doc = "0xa4 - ADC analog watchdog 3 configuration register"]
    pub awd3cr: AWD3CR,
    _reserved26: [u8; 0x08],
    #[doc = "0xb0 - ADC watchdog lower threshold register 2"]
    pub ltr2: LTR2,
    #[doc = "0xb4 - ADC watchdog higher threshold register 2"]
    pub htr2: HTR2,
    #[doc = "0xb8 - ADC watchdog lower threshold register 3"]
    pub ltr3: LTR3,
    #[doc = "0xbc - ADC watchdog higher threshold register 3"]
    pub htr3: HTR3,
    #[doc = "0xc0 - ADC channel differential or single-ended mode selection register"]
    pub difsel: DIFSEL,
    #[doc = "0xc4 - ADC calibration factors register"]
    pub calfact: CALFACT,
    #[doc = "0xc8 - ADC Calibration Factor register 2"]
    pub calfact2: CALFACT2,
}
#[doc = "ISR (rw) register accessor: ADC interrupt and status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`isr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`isr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`isr`]
module"]
pub type ISR = crate::Reg<isr::ISR_SPEC>;
#[doc = "ADC interrupt and status register"]
pub mod isr;
#[doc = "IER (rw) register accessor: ADC interrupt enable register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ier::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ier::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`ier`]
module"]
pub type IER = crate::Reg<ier::IER_SPEC>;
#[doc = "ADC interrupt enable register"]
pub mod ier;
#[doc = "CR (rw) register accessor: ADC control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`cr`]
module"]
pub type CR = crate::Reg<cr::CR_SPEC>;
#[doc = "ADC control register"]
pub mod cr;
#[doc = "CFGR (rw) register accessor: ADC configuration register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfgr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfgr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`cfgr`]
module"]
pub type CFGR = crate::Reg<cfgr::CFGR_SPEC>;
#[doc = "ADC configuration register 1"]
pub mod cfgr;
#[doc = "CFGR2 (rw) register accessor: ADC configuration register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfgr2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfgr2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`cfgr2`]
module"]
pub type CFGR2 = crate::Reg<cfgr2::CFGR2_SPEC>;
#[doc = "ADC configuration register 2"]
pub mod cfgr2;
#[doc = "SMPR1 (rw) register accessor: ADC sampling time register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`smpr1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`smpr1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`smpr1`]
module"]
pub type SMPR1 = crate::Reg<smpr1::SMPR1_SPEC>;
#[doc = "ADC sampling time register 1"]
pub mod smpr1;
#[doc = "SMPR2 (rw) register accessor: ADC sampling time register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`smpr2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`smpr2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`smpr2`]
module"]
pub type SMPR2 = crate::Reg<smpr2::SMPR2_SPEC>;
#[doc = "ADC sampling time register 2"]
pub mod smpr2;
#[doc = "LTR1 (rw) register accessor: ADC analog watchdog 1 threshold register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ltr1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ltr1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`ltr1`]
module"]
pub type LTR1 = crate::Reg<ltr1::LTR1_SPEC>;
#[doc = "ADC analog watchdog 1 threshold register"]
pub mod ltr1;
#[doc = "LHTR1 (rw) register accessor: ADC analog watchdog 2 threshold register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lhtr1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lhtr1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`lhtr1`]
module"]
pub type LHTR1 = crate::Reg<lhtr1::LHTR1_SPEC>;
#[doc = "ADC analog watchdog 2 threshold register"]
pub mod lhtr1;
#[doc = "SQR1 (rw) register accessor: ADC group regular sequencer ranks register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sqr1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sqr1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`sqr1`]
module"]
pub type SQR1 = crate::Reg<sqr1::SQR1_SPEC>;
#[doc = "ADC group regular sequencer ranks register 1"]
pub mod sqr1;
#[doc = "SQR2 (rw) register accessor: ADC group regular sequencer ranks register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sqr2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sqr2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`sqr2`]
module"]
pub type SQR2 = crate::Reg<sqr2::SQR2_SPEC>;
#[doc = "ADC group regular sequencer ranks register 2"]
pub mod sqr2;
#[doc = "SQR3 (rw) register accessor: ADC group regular sequencer ranks register 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sqr3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sqr3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`sqr3`]
module"]
pub type SQR3 = crate::Reg<sqr3::SQR3_SPEC>;
#[doc = "ADC group regular sequencer ranks register 3"]
pub mod sqr3;
#[doc = "SQR4 (rw) register accessor: ADC group regular sequencer ranks register 4\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sqr4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sqr4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`sqr4`]
module"]
pub type SQR4 = crate::Reg<sqr4::SQR4_SPEC>;
#[doc = "ADC group regular sequencer ranks register 4"]
pub mod sqr4;
#[doc = "DR (r) register accessor: ADC group regular conversion data register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`dr`]
module"]
pub type DR = crate::Reg<dr::DR_SPEC>;
#[doc = "ADC group regular conversion data register"]
pub mod dr;
#[doc = "JSQR (rw) register accessor: ADC group injected sequencer register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`jsqr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`jsqr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`jsqr`]
module"]
pub type JSQR = crate::Reg<jsqr::JSQR_SPEC>;
#[doc = "ADC group injected sequencer register"]
pub mod jsqr;
#[doc = "OFR1 (rw) register accessor: ADC offset number 1 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ofr1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ofr1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`ofr1`]
module"]
pub type OFR1 = crate::Reg<ofr1::OFR1_SPEC>;
#[doc = "ADC offset number 1 register"]
pub mod ofr1;
#[doc = "OFR2 (rw) register accessor: ADC offset number 2 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ofr2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ofr2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`ofr2`]
module"]
pub type OFR2 = crate::Reg<ofr2::OFR2_SPEC>;
#[doc = "ADC offset number 2 register"]
pub mod ofr2;
#[doc = "OFR3 (rw) register accessor: ADC offset number 3 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ofr3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ofr3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`ofr3`]
module"]
pub type OFR3 = crate::Reg<ofr3::OFR3_SPEC>;
#[doc = "ADC offset number 3 register"]
pub mod ofr3;
#[doc = "OFR4 (rw) register accessor: ADC offset number 4 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ofr4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ofr4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`ofr4`]
module"]
pub type OFR4 = crate::Reg<ofr4::OFR4_SPEC>;
#[doc = "ADC offset number 4 register"]
pub mod ofr4;
#[doc = "JDR1 (r) register accessor: ADC group injected sequencer rank 1 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`jdr1::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`jdr1`]
module"]
pub type JDR1 = crate::Reg<jdr1::JDR1_SPEC>;
#[doc = "ADC group injected sequencer rank 1 register"]
pub mod jdr1;
#[doc = "JDR2 (r) register accessor: ADC group injected sequencer rank 2 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`jdr2::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`jdr2`]
module"]
pub type JDR2 = crate::Reg<jdr2::JDR2_SPEC>;
#[doc = "ADC group injected sequencer rank 2 register"]
pub mod jdr2;
#[doc = "JDR3 (r) register accessor: ADC group injected sequencer rank 3 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`jdr3::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`jdr3`]
module"]
pub type JDR3 = crate::Reg<jdr3::JDR3_SPEC>;
#[doc = "ADC group injected sequencer rank 3 register"]
pub mod jdr3;
#[doc = "JDR4 (r) register accessor: ADC group injected sequencer rank 4 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`jdr4::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`jdr4`]
module"]
pub type JDR4 = crate::Reg<jdr4::JDR4_SPEC>;
#[doc = "ADC group injected sequencer rank 4 register"]
pub mod jdr4;
#[doc = "AWD2CR (rw) register accessor: ADC analog watchdog 2 configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`awd2cr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`awd2cr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`awd2cr`]
module"]
pub type AWD2CR = crate::Reg<awd2cr::AWD2CR_SPEC>;
#[doc = "ADC analog watchdog 2 configuration register"]
pub mod awd2cr;
#[doc = "AWD3CR (rw) register accessor: ADC analog watchdog 3 configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`awd3cr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`awd3cr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`awd3cr`]
module"]
pub type AWD3CR = crate::Reg<awd3cr::AWD3CR_SPEC>;
#[doc = "ADC analog watchdog 3 configuration register"]
pub mod awd3cr;
#[doc = "DIFSEL (rw) register accessor: ADC channel differential or single-ended mode selection register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`difsel::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`difsel::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`difsel`]
module"]
pub type DIFSEL = crate::Reg<difsel::DIFSEL_SPEC>;
#[doc = "ADC channel differential or single-ended mode selection register"]
pub mod difsel;
#[doc = "CALFACT (rw) register accessor: ADC calibration factors register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`calfact::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`calfact::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`calfact`]
module"]
pub type CALFACT = crate::Reg<calfact::CALFACT_SPEC>;
#[doc = "ADC calibration factors register"]
pub mod calfact;
#[doc = "PCSEL (rw) register accessor: ADC pre channel selection register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcsel::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pcsel::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`pcsel`]
module"]
pub type PCSEL = crate::Reg<pcsel::PCSEL_SPEC>;
#[doc = "ADC pre channel selection register"]
pub mod pcsel;
#[doc = "LTR2 (rw) register accessor: ADC watchdog lower threshold register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ltr2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ltr2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`ltr2`]
module"]
pub type LTR2 = crate::Reg<ltr2::LTR2_SPEC>;
#[doc = "ADC watchdog lower threshold register 2"]
pub mod ltr2;
#[doc = "HTR2 (rw) register accessor: ADC watchdog higher threshold register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`htr2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`htr2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`htr2`]
module"]
pub type HTR2 = crate::Reg<htr2::HTR2_SPEC>;
#[doc = "ADC watchdog higher threshold register 2"]
pub mod htr2;
#[doc = "LTR3 (rw) register accessor: ADC watchdog lower threshold register 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ltr3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ltr3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`ltr3`]
module"]
pub type LTR3 = crate::Reg<ltr3::LTR3_SPEC>;
#[doc = "ADC watchdog lower threshold register 3"]
pub mod ltr3;
#[doc = "HTR3 (rw) register accessor: ADC watchdog higher threshold register 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`htr3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`htr3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`htr3`]
module"]
pub type HTR3 = crate::Reg<htr3::HTR3_SPEC>;
#[doc = "ADC watchdog higher threshold register 3"]
pub mod htr3;
#[doc = "CALFACT2 (rw) register accessor: ADC Calibration Factor register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`calfact2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`calfact2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`calfact2`]
module"]
pub type CALFACT2 = crate::Reg<calfact2::CALFACT2_SPEC>;
#[doc = "ADC Calibration Factor register 2"]
pub mod calfact2;
