#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - MDIOS configuration register"]
    pub mdios_cr: MDIOS_CR,
    #[doc = "0x04 - MDIOS write flag register"]
    pub mdios_wrfr: MDIOS_WRFR,
    #[doc = "0x08 - MDIOS clear write flag register"]
    pub mdios_cwrfr: MDIOS_CWRFR,
    #[doc = "0x0c - MDIOS read flag register"]
    pub mdios_rdfr: MDIOS_RDFR,
    #[doc = "0x10 - MDIOS clear read flag register"]
    pub mdios_crdfr: MDIOS_CRDFR,
    #[doc = "0x14 - MDIOS status register"]
    pub mdios_sr: MDIOS_SR,
    #[doc = "0x18 - MDIOS clear flag register"]
    pub mdios_clrfr: MDIOS_CLRFR,
    #[doc = "0x1c - MDIOS input data register 0"]
    pub mdios_dinr0: MDIOS_DINR0,
    #[doc = "0x20 - MDIOS input data register 1"]
    pub mdios_dinr1: MDIOS_DINR1,
    #[doc = "0x24 - MDIOS input data register 2"]
    pub mdios_dinr2: MDIOS_DINR2,
    #[doc = "0x28 - MDIOS input data register 3"]
    pub mdios_dinr3: MDIOS_DINR3,
    #[doc = "0x2c - MDIOS input data register 4"]
    pub mdios_dinr4: MDIOS_DINR4,
    #[doc = "0x30 - MDIOS input data register 5"]
    pub mdios_dinr5: MDIOS_DINR5,
    #[doc = "0x34 - MDIOS input data register 6"]
    pub mdios_dinr6: MDIOS_DINR6,
    #[doc = "0x38 - MDIOS input data register 7"]
    pub mdios_dinr7: MDIOS_DINR7,
    #[doc = "0x3c - MDIOS input data register 8"]
    pub mdios_dinr8: MDIOS_DINR8,
    #[doc = "0x40 - MDIOS input data register 9"]
    pub mdios_dinr9: MDIOS_DINR9,
    #[doc = "0x44 - MDIOS input data register 10"]
    pub mdios_dinr10: MDIOS_DINR10,
    #[doc = "0x48 - MDIOS input data register 11"]
    pub mdios_dinr11: MDIOS_DINR11,
    #[doc = "0x4c - MDIOS input data register 12"]
    pub mdios_dinr12: MDIOS_DINR12,
    #[doc = "0x50 - MDIOS input data register 13"]
    pub mdios_dinr13: MDIOS_DINR13,
    #[doc = "0x54 - MDIOS input data register 14"]
    pub mdios_dinr14: MDIOS_DINR14,
    #[doc = "0x58 - MDIOS input data register 15"]
    pub mdios_dinr15: MDIOS_DINR15,
    #[doc = "0x5c - MDIOS input data register 16"]
    pub mdios_dinr16: MDIOS_DINR16,
    #[doc = "0x60 - MDIOS input data register 17"]
    pub mdios_dinr17: MDIOS_DINR17,
    #[doc = "0x64 - MDIOS input data register 18"]
    pub mdios_dinr18: MDIOS_DINR18,
    #[doc = "0x68 - MDIOS input data register 19"]
    pub mdios_dinr19: MDIOS_DINR19,
    #[doc = "0x6c - MDIOS input data register 20"]
    pub mdios_dinr20: MDIOS_DINR20,
    #[doc = "0x70 - MDIOS input data register 21"]
    pub mdios_dinr21: MDIOS_DINR21,
    #[doc = "0x74 - MDIOS input data register 22"]
    pub mdios_dinr22: MDIOS_DINR22,
    #[doc = "0x78 - MDIOS input data register 23"]
    pub mdios_dinr23: MDIOS_DINR23,
    #[doc = "0x7c - MDIOS input data register 24"]
    pub mdios_dinr24: MDIOS_DINR24,
    #[doc = "0x80 - MDIOS input data register 25"]
    pub mdios_dinr25: MDIOS_DINR25,
    #[doc = "0x84 - MDIOS input data register 26"]
    pub mdios_dinr26: MDIOS_DINR26,
    #[doc = "0x88 - MDIOS input data register 27"]
    pub mdios_dinr27: MDIOS_DINR27,
    #[doc = "0x8c - MDIOS input data register 28"]
    pub mdios_dinr28: MDIOS_DINR28,
    #[doc = "0x90 - MDIOS input data register 29"]
    pub mdios_dinr29: MDIOS_DINR29,
    #[doc = "0x94 - MDIOS input data register 30"]
    pub mdios_dinr30: MDIOS_DINR30,
    #[doc = "0x98 - MDIOS input data register 31"]
    pub mdios_dinr31: MDIOS_DINR31,
    #[doc = "0x9c - MDIOS output data register 0"]
    pub mdios_doutr0: MDIOS_DOUTR0,
    #[doc = "0xa0 - MDIOS output data register 1"]
    pub mdios_doutr1: MDIOS_DOUTR1,
    #[doc = "0xa4 - MDIOS output data register 2"]
    pub mdios_doutr2: MDIOS_DOUTR2,
    #[doc = "0xa8 - MDIOS output data register 3"]
    pub mdios_doutr3: MDIOS_DOUTR3,
    #[doc = "0xac - MDIOS output data register 4"]
    pub mdios_doutr4: MDIOS_DOUTR4,
    #[doc = "0xb0 - MDIOS output data register 5"]
    pub mdios_doutr5: MDIOS_DOUTR5,
    #[doc = "0xb4 - MDIOS output data register 6"]
    pub mdios_doutr6: MDIOS_DOUTR6,
    #[doc = "0xb8 - MDIOS output data register 7"]
    pub mdios_doutr7: MDIOS_DOUTR7,
    #[doc = "0xbc - MDIOS output data register 8"]
    pub mdios_doutr8: MDIOS_DOUTR8,
    #[doc = "0xc0 - MDIOS output data register 9"]
    pub mdios_doutr9: MDIOS_DOUTR9,
    #[doc = "0xc4 - MDIOS output data register 10"]
    pub mdios_doutr10: MDIOS_DOUTR10,
    #[doc = "0xc8 - MDIOS output data register 11"]
    pub mdios_doutr11: MDIOS_DOUTR11,
    #[doc = "0xcc - MDIOS output data register 12"]
    pub mdios_doutr12: MDIOS_DOUTR12,
    #[doc = "0xd0 - MDIOS output data register 13"]
    pub mdios_doutr13: MDIOS_DOUTR13,
    #[doc = "0xd4 - MDIOS output data register 14"]
    pub mdios_doutr14: MDIOS_DOUTR14,
    #[doc = "0xd8 - MDIOS output data register 15"]
    pub mdios_doutr15: MDIOS_DOUTR15,
    #[doc = "0xdc - MDIOS output data register 16"]
    pub mdios_doutr16: MDIOS_DOUTR16,
    #[doc = "0xe0 - MDIOS output data register 17"]
    pub mdios_doutr17: MDIOS_DOUTR17,
    #[doc = "0xe4 - MDIOS output data register 18"]
    pub mdios_doutr18: MDIOS_DOUTR18,
    #[doc = "0xe8 - MDIOS output data register 19"]
    pub mdios_doutr19: MDIOS_DOUTR19,
    #[doc = "0xec - MDIOS output data register 20"]
    pub mdios_doutr20: MDIOS_DOUTR20,
    #[doc = "0xf0 - MDIOS output data register 21"]
    pub mdios_doutr21: MDIOS_DOUTR21,
    #[doc = "0xf4 - MDIOS output data register 22"]
    pub mdios_doutr22: MDIOS_DOUTR22,
    #[doc = "0xf8 - MDIOS output data register 23"]
    pub mdios_doutr23: MDIOS_DOUTR23,
    #[doc = "0xfc - MDIOS output data register 24"]
    pub mdios_doutr24: MDIOS_DOUTR24,
    #[doc = "0x100 - MDIOS output data register 25"]
    pub mdios_doutr25: MDIOS_DOUTR25,
    #[doc = "0x104 - MDIOS output data register 26"]
    pub mdios_doutr26: MDIOS_DOUTR26,
    #[doc = "0x108 - MDIOS output data register 27"]
    pub mdios_doutr27: MDIOS_DOUTR27,
    #[doc = "0x10c - MDIOS output data register 28"]
    pub mdios_doutr28: MDIOS_DOUTR28,
    #[doc = "0x110 - MDIOS output data register 29"]
    pub mdios_doutr29: MDIOS_DOUTR29,
    #[doc = "0x114 - MDIOS output data register 30"]
    pub mdios_doutr30: MDIOS_DOUTR30,
    #[doc = "0x118 - MDIOS output data register 31"]
    pub mdios_doutr31: MDIOS_DOUTR31,
}
#[doc = "MDIOS_CR (rw) register accessor: MDIOS configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mdios_cr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mdios_cr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mdios_cr`]
module"]
pub type MDIOS_CR = crate::Reg<mdios_cr::MDIOS_CR_SPEC>;
#[doc = "MDIOS configuration register"]
pub mod mdios_cr;
#[doc = "MDIOS_WRFR (r) register accessor: MDIOS write flag register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mdios_wrfr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mdios_wrfr`]
module"]
pub type MDIOS_WRFR = crate::Reg<mdios_wrfr::MDIOS_WRFR_SPEC>;
#[doc = "MDIOS write flag register"]
pub mod mdios_wrfr;
#[doc = "MDIOS_CWRFR (rw) register accessor: MDIOS clear write flag register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mdios_cwrfr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mdios_cwrfr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mdios_cwrfr`]
module"]
pub type MDIOS_CWRFR = crate::Reg<mdios_cwrfr::MDIOS_CWRFR_SPEC>;
#[doc = "MDIOS clear write flag register"]
pub mod mdios_cwrfr;
#[doc = "MDIOS_RDFR (r) register accessor: MDIOS read flag register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mdios_rdfr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mdios_rdfr`]
module"]
pub type MDIOS_RDFR = crate::Reg<mdios_rdfr::MDIOS_RDFR_SPEC>;
#[doc = "MDIOS read flag register"]
pub mod mdios_rdfr;
#[doc = "MDIOS_CRDFR (rw) register accessor: MDIOS clear read flag register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mdios_crdfr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mdios_crdfr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mdios_crdfr`]
module"]
pub type MDIOS_CRDFR = crate::Reg<mdios_crdfr::MDIOS_CRDFR_SPEC>;
#[doc = "MDIOS clear read flag register"]
pub mod mdios_crdfr;
#[doc = "MDIOS_SR (r) register accessor: MDIOS status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mdios_sr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mdios_sr`]
module"]
pub type MDIOS_SR = crate::Reg<mdios_sr::MDIOS_SR_SPEC>;
#[doc = "MDIOS status register"]
pub mod mdios_sr;
#[doc = "MDIOS_CLRFR (rw) register accessor: MDIOS clear flag register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mdios_clrfr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mdios_clrfr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mdios_clrfr`]
module"]
pub type MDIOS_CLRFR = crate::Reg<mdios_clrfr::MDIOS_CLRFR_SPEC>;
#[doc = "MDIOS clear flag register"]
pub mod mdios_clrfr;
#[doc = "MDIOS_DINR0 (r) register accessor: MDIOS input data register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mdios_dinr0::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mdios_dinr0`]
module"]
pub type MDIOS_DINR0 = crate::Reg<mdios_dinr0::MDIOS_DINR0_SPEC>;
#[doc = "MDIOS input data register 0"]
pub mod mdios_dinr0;
#[doc = "MDIOS_DINR1 (r) register accessor: MDIOS input data register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mdios_dinr1::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mdios_dinr1`]
module"]
pub type MDIOS_DINR1 = crate::Reg<mdios_dinr1::MDIOS_DINR1_SPEC>;
#[doc = "MDIOS input data register 1"]
pub mod mdios_dinr1;
#[doc = "MDIOS_DINR2 (r) register accessor: MDIOS input data register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mdios_dinr2::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mdios_dinr2`]
module"]
pub type MDIOS_DINR2 = crate::Reg<mdios_dinr2::MDIOS_DINR2_SPEC>;
#[doc = "MDIOS input data register 2"]
pub mod mdios_dinr2;
#[doc = "MDIOS_DINR3 (r) register accessor: MDIOS input data register 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mdios_dinr3::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mdios_dinr3`]
module"]
pub type MDIOS_DINR3 = crate::Reg<mdios_dinr3::MDIOS_DINR3_SPEC>;
#[doc = "MDIOS input data register 3"]
pub mod mdios_dinr3;
#[doc = "MDIOS_DINR4 (r) register accessor: MDIOS input data register 4\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mdios_dinr4::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mdios_dinr4`]
module"]
pub type MDIOS_DINR4 = crate::Reg<mdios_dinr4::MDIOS_DINR4_SPEC>;
#[doc = "MDIOS input data register 4"]
pub mod mdios_dinr4;
#[doc = "MDIOS_DINR5 (r) register accessor: MDIOS input data register 5\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mdios_dinr5::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mdios_dinr5`]
module"]
pub type MDIOS_DINR5 = crate::Reg<mdios_dinr5::MDIOS_DINR5_SPEC>;
#[doc = "MDIOS input data register 5"]
pub mod mdios_dinr5;
#[doc = "MDIOS_DINR6 (r) register accessor: MDIOS input data register 6\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mdios_dinr6::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mdios_dinr6`]
module"]
pub type MDIOS_DINR6 = crate::Reg<mdios_dinr6::MDIOS_DINR6_SPEC>;
#[doc = "MDIOS input data register 6"]
pub mod mdios_dinr6;
#[doc = "MDIOS_DINR7 (r) register accessor: MDIOS input data register 7\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mdios_dinr7::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mdios_dinr7`]
module"]
pub type MDIOS_DINR7 = crate::Reg<mdios_dinr7::MDIOS_DINR7_SPEC>;
#[doc = "MDIOS input data register 7"]
pub mod mdios_dinr7;
#[doc = "MDIOS_DINR8 (r) register accessor: MDIOS input data register 8\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mdios_dinr8::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mdios_dinr8`]
module"]
pub type MDIOS_DINR8 = crate::Reg<mdios_dinr8::MDIOS_DINR8_SPEC>;
#[doc = "MDIOS input data register 8"]
pub mod mdios_dinr8;
#[doc = "MDIOS_DINR9 (r) register accessor: MDIOS input data register 9\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mdios_dinr9::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mdios_dinr9`]
module"]
pub type MDIOS_DINR9 = crate::Reg<mdios_dinr9::MDIOS_DINR9_SPEC>;
#[doc = "MDIOS input data register 9"]
pub mod mdios_dinr9;
#[doc = "MDIOS_DINR10 (r) register accessor: MDIOS input data register 10\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mdios_dinr10::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mdios_dinr10`]
module"]
pub type MDIOS_DINR10 = crate::Reg<mdios_dinr10::MDIOS_DINR10_SPEC>;
#[doc = "MDIOS input data register 10"]
pub mod mdios_dinr10;
#[doc = "MDIOS_DINR11 (r) register accessor: MDIOS input data register 11\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mdios_dinr11::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mdios_dinr11`]
module"]
pub type MDIOS_DINR11 = crate::Reg<mdios_dinr11::MDIOS_DINR11_SPEC>;
#[doc = "MDIOS input data register 11"]
pub mod mdios_dinr11;
#[doc = "MDIOS_DINR12 (r) register accessor: MDIOS input data register 12\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mdios_dinr12::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mdios_dinr12`]
module"]
pub type MDIOS_DINR12 = crate::Reg<mdios_dinr12::MDIOS_DINR12_SPEC>;
#[doc = "MDIOS input data register 12"]
pub mod mdios_dinr12;
#[doc = "MDIOS_DINR13 (r) register accessor: MDIOS input data register 13\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mdios_dinr13::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mdios_dinr13`]
module"]
pub type MDIOS_DINR13 = crate::Reg<mdios_dinr13::MDIOS_DINR13_SPEC>;
#[doc = "MDIOS input data register 13"]
pub mod mdios_dinr13;
#[doc = "MDIOS_DINR14 (r) register accessor: MDIOS input data register 14\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mdios_dinr14::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mdios_dinr14`]
module"]
pub type MDIOS_DINR14 = crate::Reg<mdios_dinr14::MDIOS_DINR14_SPEC>;
#[doc = "MDIOS input data register 14"]
pub mod mdios_dinr14;
#[doc = "MDIOS_DINR15 (r) register accessor: MDIOS input data register 15\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mdios_dinr15::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mdios_dinr15`]
module"]
pub type MDIOS_DINR15 = crate::Reg<mdios_dinr15::MDIOS_DINR15_SPEC>;
#[doc = "MDIOS input data register 15"]
pub mod mdios_dinr15;
#[doc = "MDIOS_DINR16 (r) register accessor: MDIOS input data register 16\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mdios_dinr16::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mdios_dinr16`]
module"]
pub type MDIOS_DINR16 = crate::Reg<mdios_dinr16::MDIOS_DINR16_SPEC>;
#[doc = "MDIOS input data register 16"]
pub mod mdios_dinr16;
#[doc = "MDIOS_DINR17 (r) register accessor: MDIOS input data register 17\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mdios_dinr17::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mdios_dinr17`]
module"]
pub type MDIOS_DINR17 = crate::Reg<mdios_dinr17::MDIOS_DINR17_SPEC>;
#[doc = "MDIOS input data register 17"]
pub mod mdios_dinr17;
#[doc = "MDIOS_DINR18 (r) register accessor: MDIOS input data register 18\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mdios_dinr18::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mdios_dinr18`]
module"]
pub type MDIOS_DINR18 = crate::Reg<mdios_dinr18::MDIOS_DINR18_SPEC>;
#[doc = "MDIOS input data register 18"]
pub mod mdios_dinr18;
#[doc = "MDIOS_DINR19 (r) register accessor: MDIOS input data register 19\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mdios_dinr19::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mdios_dinr19`]
module"]
pub type MDIOS_DINR19 = crate::Reg<mdios_dinr19::MDIOS_DINR19_SPEC>;
#[doc = "MDIOS input data register 19"]
pub mod mdios_dinr19;
#[doc = "MDIOS_DINR20 (r) register accessor: MDIOS input data register 20\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mdios_dinr20::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mdios_dinr20`]
module"]
pub type MDIOS_DINR20 = crate::Reg<mdios_dinr20::MDIOS_DINR20_SPEC>;
#[doc = "MDIOS input data register 20"]
pub mod mdios_dinr20;
#[doc = "MDIOS_DINR21 (r) register accessor: MDIOS input data register 21\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mdios_dinr21::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mdios_dinr21`]
module"]
pub type MDIOS_DINR21 = crate::Reg<mdios_dinr21::MDIOS_DINR21_SPEC>;
#[doc = "MDIOS input data register 21"]
pub mod mdios_dinr21;
#[doc = "MDIOS_DINR22 (r) register accessor: MDIOS input data register 22\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mdios_dinr22::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mdios_dinr22`]
module"]
pub type MDIOS_DINR22 = crate::Reg<mdios_dinr22::MDIOS_DINR22_SPEC>;
#[doc = "MDIOS input data register 22"]
pub mod mdios_dinr22;
#[doc = "MDIOS_DINR23 (r) register accessor: MDIOS input data register 23\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mdios_dinr23::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mdios_dinr23`]
module"]
pub type MDIOS_DINR23 = crate::Reg<mdios_dinr23::MDIOS_DINR23_SPEC>;
#[doc = "MDIOS input data register 23"]
pub mod mdios_dinr23;
#[doc = "MDIOS_DINR24 (r) register accessor: MDIOS input data register 24\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mdios_dinr24::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mdios_dinr24`]
module"]
pub type MDIOS_DINR24 = crate::Reg<mdios_dinr24::MDIOS_DINR24_SPEC>;
#[doc = "MDIOS input data register 24"]
pub mod mdios_dinr24;
#[doc = "MDIOS_DINR25 (r) register accessor: MDIOS input data register 25\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mdios_dinr25::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mdios_dinr25`]
module"]
pub type MDIOS_DINR25 = crate::Reg<mdios_dinr25::MDIOS_DINR25_SPEC>;
#[doc = "MDIOS input data register 25"]
pub mod mdios_dinr25;
#[doc = "MDIOS_DINR26 (r) register accessor: MDIOS input data register 26\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mdios_dinr26::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mdios_dinr26`]
module"]
pub type MDIOS_DINR26 = crate::Reg<mdios_dinr26::MDIOS_DINR26_SPEC>;
#[doc = "MDIOS input data register 26"]
pub mod mdios_dinr26;
#[doc = "MDIOS_DINR27 (r) register accessor: MDIOS input data register 27\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mdios_dinr27::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mdios_dinr27`]
module"]
pub type MDIOS_DINR27 = crate::Reg<mdios_dinr27::MDIOS_DINR27_SPEC>;
#[doc = "MDIOS input data register 27"]
pub mod mdios_dinr27;
#[doc = "MDIOS_DINR28 (r) register accessor: MDIOS input data register 28\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mdios_dinr28::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mdios_dinr28`]
module"]
pub type MDIOS_DINR28 = crate::Reg<mdios_dinr28::MDIOS_DINR28_SPEC>;
#[doc = "MDIOS input data register 28"]
pub mod mdios_dinr28;
#[doc = "MDIOS_DINR29 (r) register accessor: MDIOS input data register 29\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mdios_dinr29::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mdios_dinr29`]
module"]
pub type MDIOS_DINR29 = crate::Reg<mdios_dinr29::MDIOS_DINR29_SPEC>;
#[doc = "MDIOS input data register 29"]
pub mod mdios_dinr29;
#[doc = "MDIOS_DINR30 (r) register accessor: MDIOS input data register 30\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mdios_dinr30::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mdios_dinr30`]
module"]
pub type MDIOS_DINR30 = crate::Reg<mdios_dinr30::MDIOS_DINR30_SPEC>;
#[doc = "MDIOS input data register 30"]
pub mod mdios_dinr30;
#[doc = "MDIOS_DINR31 (r) register accessor: MDIOS input data register 31\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mdios_dinr31::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mdios_dinr31`]
module"]
pub type MDIOS_DINR31 = crate::Reg<mdios_dinr31::MDIOS_DINR31_SPEC>;
#[doc = "MDIOS input data register 31"]
pub mod mdios_dinr31;
#[doc = "MDIOS_DOUTR0 (rw) register accessor: MDIOS output data register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mdios_doutr0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mdios_doutr0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mdios_doutr0`]
module"]
pub type MDIOS_DOUTR0 = crate::Reg<mdios_doutr0::MDIOS_DOUTR0_SPEC>;
#[doc = "MDIOS output data register 0"]
pub mod mdios_doutr0;
#[doc = "MDIOS_DOUTR1 (rw) register accessor: MDIOS output data register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mdios_doutr1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mdios_doutr1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mdios_doutr1`]
module"]
pub type MDIOS_DOUTR1 = crate::Reg<mdios_doutr1::MDIOS_DOUTR1_SPEC>;
#[doc = "MDIOS output data register 1"]
pub mod mdios_doutr1;
#[doc = "MDIOS_DOUTR2 (rw) register accessor: MDIOS output data register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mdios_doutr2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mdios_doutr2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mdios_doutr2`]
module"]
pub type MDIOS_DOUTR2 = crate::Reg<mdios_doutr2::MDIOS_DOUTR2_SPEC>;
#[doc = "MDIOS output data register 2"]
pub mod mdios_doutr2;
#[doc = "MDIOS_DOUTR3 (rw) register accessor: MDIOS output data register 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mdios_doutr3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mdios_doutr3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mdios_doutr3`]
module"]
pub type MDIOS_DOUTR3 = crate::Reg<mdios_doutr3::MDIOS_DOUTR3_SPEC>;
#[doc = "MDIOS output data register 3"]
pub mod mdios_doutr3;
#[doc = "MDIOS_DOUTR4 (rw) register accessor: MDIOS output data register 4\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mdios_doutr4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mdios_doutr4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mdios_doutr4`]
module"]
pub type MDIOS_DOUTR4 = crate::Reg<mdios_doutr4::MDIOS_DOUTR4_SPEC>;
#[doc = "MDIOS output data register 4"]
pub mod mdios_doutr4;
#[doc = "MDIOS_DOUTR5 (rw) register accessor: MDIOS output data register 5\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mdios_doutr5::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mdios_doutr5::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mdios_doutr5`]
module"]
pub type MDIOS_DOUTR5 = crate::Reg<mdios_doutr5::MDIOS_DOUTR5_SPEC>;
#[doc = "MDIOS output data register 5"]
pub mod mdios_doutr5;
#[doc = "MDIOS_DOUTR6 (rw) register accessor: MDIOS output data register 6\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mdios_doutr6::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mdios_doutr6::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mdios_doutr6`]
module"]
pub type MDIOS_DOUTR6 = crate::Reg<mdios_doutr6::MDIOS_DOUTR6_SPEC>;
#[doc = "MDIOS output data register 6"]
pub mod mdios_doutr6;
#[doc = "MDIOS_DOUTR7 (rw) register accessor: MDIOS output data register 7\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mdios_doutr7::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mdios_doutr7::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mdios_doutr7`]
module"]
pub type MDIOS_DOUTR7 = crate::Reg<mdios_doutr7::MDIOS_DOUTR7_SPEC>;
#[doc = "MDIOS output data register 7"]
pub mod mdios_doutr7;
#[doc = "MDIOS_DOUTR8 (rw) register accessor: MDIOS output data register 8\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mdios_doutr8::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mdios_doutr8::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mdios_doutr8`]
module"]
pub type MDIOS_DOUTR8 = crate::Reg<mdios_doutr8::MDIOS_DOUTR8_SPEC>;
#[doc = "MDIOS output data register 8"]
pub mod mdios_doutr8;
#[doc = "MDIOS_DOUTR9 (rw) register accessor: MDIOS output data register 9\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mdios_doutr9::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mdios_doutr9::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mdios_doutr9`]
module"]
pub type MDIOS_DOUTR9 = crate::Reg<mdios_doutr9::MDIOS_DOUTR9_SPEC>;
#[doc = "MDIOS output data register 9"]
pub mod mdios_doutr9;
#[doc = "MDIOS_DOUTR10 (rw) register accessor: MDIOS output data register 10\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mdios_doutr10::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mdios_doutr10::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mdios_doutr10`]
module"]
pub type MDIOS_DOUTR10 = crate::Reg<mdios_doutr10::MDIOS_DOUTR10_SPEC>;
#[doc = "MDIOS output data register 10"]
pub mod mdios_doutr10;
#[doc = "MDIOS_DOUTR11 (rw) register accessor: MDIOS output data register 11\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mdios_doutr11::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mdios_doutr11::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mdios_doutr11`]
module"]
pub type MDIOS_DOUTR11 = crate::Reg<mdios_doutr11::MDIOS_DOUTR11_SPEC>;
#[doc = "MDIOS output data register 11"]
pub mod mdios_doutr11;
#[doc = "MDIOS_DOUTR12 (rw) register accessor: MDIOS output data register 12\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mdios_doutr12::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mdios_doutr12::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mdios_doutr12`]
module"]
pub type MDIOS_DOUTR12 = crate::Reg<mdios_doutr12::MDIOS_DOUTR12_SPEC>;
#[doc = "MDIOS output data register 12"]
pub mod mdios_doutr12;
#[doc = "MDIOS_DOUTR13 (rw) register accessor: MDIOS output data register 13\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mdios_doutr13::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mdios_doutr13::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mdios_doutr13`]
module"]
pub type MDIOS_DOUTR13 = crate::Reg<mdios_doutr13::MDIOS_DOUTR13_SPEC>;
#[doc = "MDIOS output data register 13"]
pub mod mdios_doutr13;
#[doc = "MDIOS_DOUTR14 (rw) register accessor: MDIOS output data register 14\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mdios_doutr14::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mdios_doutr14::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mdios_doutr14`]
module"]
pub type MDIOS_DOUTR14 = crate::Reg<mdios_doutr14::MDIOS_DOUTR14_SPEC>;
#[doc = "MDIOS output data register 14"]
pub mod mdios_doutr14;
#[doc = "MDIOS_DOUTR15 (rw) register accessor: MDIOS output data register 15\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mdios_doutr15::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mdios_doutr15::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mdios_doutr15`]
module"]
pub type MDIOS_DOUTR15 = crate::Reg<mdios_doutr15::MDIOS_DOUTR15_SPEC>;
#[doc = "MDIOS output data register 15"]
pub mod mdios_doutr15;
#[doc = "MDIOS_DOUTR16 (rw) register accessor: MDIOS output data register 16\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mdios_doutr16::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mdios_doutr16::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mdios_doutr16`]
module"]
pub type MDIOS_DOUTR16 = crate::Reg<mdios_doutr16::MDIOS_DOUTR16_SPEC>;
#[doc = "MDIOS output data register 16"]
pub mod mdios_doutr16;
#[doc = "MDIOS_DOUTR17 (rw) register accessor: MDIOS output data register 17\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mdios_doutr17::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mdios_doutr17::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mdios_doutr17`]
module"]
pub type MDIOS_DOUTR17 = crate::Reg<mdios_doutr17::MDIOS_DOUTR17_SPEC>;
#[doc = "MDIOS output data register 17"]
pub mod mdios_doutr17;
#[doc = "MDIOS_DOUTR18 (rw) register accessor: MDIOS output data register 18\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mdios_doutr18::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mdios_doutr18::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mdios_doutr18`]
module"]
pub type MDIOS_DOUTR18 = crate::Reg<mdios_doutr18::MDIOS_DOUTR18_SPEC>;
#[doc = "MDIOS output data register 18"]
pub mod mdios_doutr18;
#[doc = "MDIOS_DOUTR19 (rw) register accessor: MDIOS output data register 19\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mdios_doutr19::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mdios_doutr19::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mdios_doutr19`]
module"]
pub type MDIOS_DOUTR19 = crate::Reg<mdios_doutr19::MDIOS_DOUTR19_SPEC>;
#[doc = "MDIOS output data register 19"]
pub mod mdios_doutr19;
#[doc = "MDIOS_DOUTR20 (rw) register accessor: MDIOS output data register 20\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mdios_doutr20::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mdios_doutr20::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mdios_doutr20`]
module"]
pub type MDIOS_DOUTR20 = crate::Reg<mdios_doutr20::MDIOS_DOUTR20_SPEC>;
#[doc = "MDIOS output data register 20"]
pub mod mdios_doutr20;
#[doc = "MDIOS_DOUTR21 (rw) register accessor: MDIOS output data register 21\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mdios_doutr21::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mdios_doutr21::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mdios_doutr21`]
module"]
pub type MDIOS_DOUTR21 = crate::Reg<mdios_doutr21::MDIOS_DOUTR21_SPEC>;
#[doc = "MDIOS output data register 21"]
pub mod mdios_doutr21;
#[doc = "MDIOS_DOUTR22 (rw) register accessor: MDIOS output data register 22\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mdios_doutr22::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mdios_doutr22::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mdios_doutr22`]
module"]
pub type MDIOS_DOUTR22 = crate::Reg<mdios_doutr22::MDIOS_DOUTR22_SPEC>;
#[doc = "MDIOS output data register 22"]
pub mod mdios_doutr22;
#[doc = "MDIOS_DOUTR23 (rw) register accessor: MDIOS output data register 23\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mdios_doutr23::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mdios_doutr23::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mdios_doutr23`]
module"]
pub type MDIOS_DOUTR23 = crate::Reg<mdios_doutr23::MDIOS_DOUTR23_SPEC>;
#[doc = "MDIOS output data register 23"]
pub mod mdios_doutr23;
#[doc = "MDIOS_DOUTR24 (rw) register accessor: MDIOS output data register 24\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mdios_doutr24::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mdios_doutr24::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mdios_doutr24`]
module"]
pub type MDIOS_DOUTR24 = crate::Reg<mdios_doutr24::MDIOS_DOUTR24_SPEC>;
#[doc = "MDIOS output data register 24"]
pub mod mdios_doutr24;
#[doc = "MDIOS_DOUTR25 (rw) register accessor: MDIOS output data register 25\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mdios_doutr25::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mdios_doutr25::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mdios_doutr25`]
module"]
pub type MDIOS_DOUTR25 = crate::Reg<mdios_doutr25::MDIOS_DOUTR25_SPEC>;
#[doc = "MDIOS output data register 25"]
pub mod mdios_doutr25;
#[doc = "MDIOS_DOUTR26 (rw) register accessor: MDIOS output data register 26\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mdios_doutr26::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mdios_doutr26::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mdios_doutr26`]
module"]
pub type MDIOS_DOUTR26 = crate::Reg<mdios_doutr26::MDIOS_DOUTR26_SPEC>;
#[doc = "MDIOS output data register 26"]
pub mod mdios_doutr26;
#[doc = "MDIOS_DOUTR27 (rw) register accessor: MDIOS output data register 27\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mdios_doutr27::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mdios_doutr27::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mdios_doutr27`]
module"]
pub type MDIOS_DOUTR27 = crate::Reg<mdios_doutr27::MDIOS_DOUTR27_SPEC>;
#[doc = "MDIOS output data register 27"]
pub mod mdios_doutr27;
#[doc = "MDIOS_DOUTR28 (rw) register accessor: MDIOS output data register 28\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mdios_doutr28::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mdios_doutr28::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mdios_doutr28`]
module"]
pub type MDIOS_DOUTR28 = crate::Reg<mdios_doutr28::MDIOS_DOUTR28_SPEC>;
#[doc = "MDIOS output data register 28"]
pub mod mdios_doutr28;
#[doc = "MDIOS_DOUTR29 (rw) register accessor: MDIOS output data register 29\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mdios_doutr29::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mdios_doutr29::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mdios_doutr29`]
module"]
pub type MDIOS_DOUTR29 = crate::Reg<mdios_doutr29::MDIOS_DOUTR29_SPEC>;
#[doc = "MDIOS output data register 29"]
pub mod mdios_doutr29;
#[doc = "MDIOS_DOUTR30 (rw) register accessor: MDIOS output data register 30\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mdios_doutr30::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mdios_doutr30::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mdios_doutr30`]
module"]
pub type MDIOS_DOUTR30 = crate::Reg<mdios_doutr30::MDIOS_DOUTR30_SPEC>;
#[doc = "MDIOS output data register 30"]
pub mod mdios_doutr30;
#[doc = "MDIOS_DOUTR31 (rw) register accessor: MDIOS output data register 31\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mdios_doutr31::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mdios_doutr31::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mdios_doutr31`]
module"]
pub type MDIOS_DOUTR31 = crate::Reg<mdios_doutr31::MDIOS_DOUTR31_SPEC>;
#[doc = "MDIOS output data register 31"]
pub mod mdios_doutr31;
