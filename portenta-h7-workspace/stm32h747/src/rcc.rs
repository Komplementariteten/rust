#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - clock control register"]
    pub cr: CR,
    #[doc = "0x04 - RCC Internal Clock Source Calibration Register"]
    pub icscr: ICSCR,
    #[doc = "0x08 - RCC Clock Recovery RC Register"]
    pub crrcr: CRRCR,
    _reserved3: [u8; 0x04],
    #[doc = "0x10 - RCC Clock Configuration Register"]
    pub cfgr: CFGR,
    _reserved4: [u8; 0x04],
    #[doc = "0x18 - RCC Domain 1 Clock Configuration Register"]
    pub d1cfgr: D1CFGR,
    #[doc = "0x1c - RCC Domain 2 Clock Configuration Register"]
    pub d2cfgr: D2CFGR,
    #[doc = "0x20 - RCC Domain 3 Clock Configuration Register"]
    pub d3cfgr: D3CFGR,
    _reserved7: [u8; 0x04],
    #[doc = "0x28 - RCC PLLs Clock Source Selection Register"]
    pub pllckselr: PLLCKSELR,
    #[doc = "0x2c - RCC PLLs Configuration Register"]
    pub pllcfgr: PLLCFGR,
    #[doc = "0x30 - RCC PLL1 Dividers Configuration Register"]
    pub pll1divr: PLL1DIVR,
    #[doc = "0x34 - RCC PLL1 Fractional Divider Register"]
    pub pll1fracr: PLL1FRACR,
    #[doc = "0x38 - RCC PLL2 Dividers Configuration Register"]
    pub pll2divr: PLL2DIVR,
    #[doc = "0x3c - RCC PLL2 Fractional Divider Register"]
    pub pll2fracr: PLL2FRACR,
    #[doc = "0x40 - RCC PLL3 Dividers Configuration Register"]
    pub pll3divr: PLL3DIVR,
    #[doc = "0x44 - RCC PLL3 Fractional Divider Register"]
    pub pll3fracr: PLL3FRACR,
    _reserved15: [u8; 0x04],
    #[doc = "0x4c - RCC Domain 1 Kernel Clock Configuration Register"]
    pub d1ccipr: D1CCIPR,
    #[doc = "0x50 - RCC Domain 2 Kernel Clock Configuration Register"]
    pub d2ccip1r: D2CCIP1R,
    #[doc = "0x54 - RCC Domain 2 Kernel Clock Configuration Register"]
    pub d2ccip2r: D2CCIP2R,
    #[doc = "0x58 - RCC Domain 3 Kernel Clock Configuration Register"]
    pub d3ccipr: D3CCIPR,
    _reserved19: [u8; 0x04],
    #[doc = "0x60 - RCC Clock Source Interrupt Enable Register"]
    pub cier: CIER,
    #[doc = "0x64 - RCC Clock Source Interrupt Flag Register"]
    pub cifr: CIFR,
    #[doc = "0x68 - RCC Clock Source Interrupt Clear Register"]
    pub cicr: CICR,
    _reserved22: [u8; 0x04],
    #[doc = "0x70 - RCC Backup Domain Control Register"]
    pub bdcr: BDCR,
    #[doc = "0x74 - RCC Clock Control and Status Register"]
    pub csr: CSR,
    _reserved24: [u8; 0x04],
    #[doc = "0x7c - RCC AHB3 Reset Register"]
    pub ahb3rstr: AHB3RSTR,
    #[doc = "0x80 - RCC AHB1 Peripheral Reset Register"]
    pub ahb1rstr: AHB1RSTR,
    #[doc = "0x84 - RCC AHB2 Peripheral Reset Register"]
    pub ahb2rstr: AHB2RSTR,
    #[doc = "0x88 - RCC AHB4 Peripheral Reset Register"]
    pub ahb4rstr: AHB4RSTR,
    #[doc = "0x8c - RCC APB3 Peripheral Reset Register"]
    pub apb3rstr: APB3RSTR,
    #[doc = "0x90 - RCC APB1 Peripheral Reset Register"]
    pub apb1lrstr: APB1LRSTR,
    #[doc = "0x94 - RCC APB1 Peripheral Reset Register"]
    pub apb1hrstr: APB1HRSTR,
    #[doc = "0x98 - RCC APB2 Peripheral Reset Register"]
    pub apb2rstr: APB2RSTR,
    #[doc = "0x9c - RCC APB4 Peripheral Reset Register"]
    pub apb4rstr: APB4RSTR,
    #[doc = "0xa0 - RCC Global Control Register"]
    pub gcr: GCR,
    _reserved34: [u8; 0x04],
    #[doc = "0xa8 - RCC D3 Autonomous mode Register"]
    pub d3amr: D3AMR,
    _reserved35: [u8; 0x24],
    #[doc = "0xd0 - RCC Reset Status Register"]
    pub rsr: RSR,
    #[doc = "0xd4 - RCC AHB3 Clock Register"]
    pub ahb3enr: AHB3ENR,
    #[doc = "0xd8 - RCC AHB1 Clock Register"]
    pub ahb1enr: AHB1ENR,
    #[doc = "0xdc - RCC AHB2 Clock Register"]
    pub ahb2enr: AHB2ENR,
    #[doc = "0xe0 - RCC AHB4 Clock Register"]
    pub ahb4enr: AHB4ENR,
    #[doc = "0xe4 - RCC APB3 Clock Register"]
    pub apb3enr: APB3ENR,
    #[doc = "0xe8 - RCC APB1 Clock Register"]
    pub apb1lenr: APB1LENR,
    #[doc = "0xec - RCC APB1 Clock Register"]
    pub apb1henr: APB1HENR,
    #[doc = "0xf0 - RCC APB2 Clock Register"]
    pub apb2enr: APB2ENR,
    #[doc = "0xf4 - RCC APB4 Clock Register"]
    pub apb4enr: APB4ENR,
    _reserved45: [u8; 0x04],
    #[doc = "0xfc - RCC AHB3 Sleep Clock Register"]
    pub ahb3lpenr: AHB3LPENR,
    #[doc = "0x100 - RCC AHB1 Sleep Clock Register"]
    pub ahb1lpenr: AHB1LPENR,
    #[doc = "0x104 - RCC AHB2 Sleep Clock Register"]
    pub ahb2lpenr: AHB2LPENR,
    #[doc = "0x108 - RCC AHB4 Sleep Clock Register"]
    pub ahb4lpenr: AHB4LPENR,
    #[doc = "0x10c - RCC APB3 Sleep Clock Register"]
    pub apb3lpenr: APB3LPENR,
    #[doc = "0x110 - RCC APB1 Low Sleep Clock Register"]
    pub apb1llpenr: APB1LLPENR,
    #[doc = "0x114 - RCC APB1 High Sleep Clock Register"]
    pub apb1hlpenr: APB1HLPENR,
    #[doc = "0x118 - RCC APB2 Sleep Clock Register"]
    pub apb2lpenr: APB2LPENR,
    #[doc = "0x11c - RCC APB4 Sleep Clock Register"]
    pub apb4lpenr: APB4LPENR,
    _reserved54: [u8; 0x10],
    #[doc = "0x130 - RCC Reset Status Register"]
    pub c1_rsr: C1_RSR,
    #[doc = "0x134 - RCC AHB3 Clock Register"]
    pub c1_ahb3enr: C1_AHB3ENR,
    #[doc = "0x138 - RCC AHB1 Clock Register"]
    pub c1_ahb1enr: C1_AHB1ENR,
    #[doc = "0x13c - RCC AHB2 Clock Register"]
    pub c1_ahb2enr: C1_AHB2ENR,
    #[doc = "0x140 - RCC AHB4 Clock Register"]
    pub c1_ahb4enr: C1_AHB4ENR,
    #[doc = "0x144 - RCC APB3 Clock Register"]
    pub c1_apb3enr: C1_APB3ENR,
    #[doc = "0x148 - RCC APB1 Clock Register"]
    pub c1_apb1lenr: C1_APB1LENR,
    #[doc = "0x14c - RCC APB1 Clock Register"]
    pub c1_apb1henr: C1_APB1HENR,
    #[doc = "0x150 - RCC APB2 Clock Register"]
    pub c1_apb2enr: C1_APB2ENR,
    #[doc = "0x154 - RCC APB4 Clock Register"]
    pub c1_apb4enr: C1_APB4ENR,
    _reserved64: [u8; 0x04],
    #[doc = "0x15c - RCC AHB3 Sleep Clock Register"]
    pub c1_ahb3lpenr: C1_AHB3LPENR,
    #[doc = "0x160 - RCC AHB1 Sleep Clock Register"]
    pub c1_ahb1lpenr: C1_AHB1LPENR,
    #[doc = "0x164 - RCC AHB2 Sleep Clock Register"]
    pub c1_ahb2lpenr: C1_AHB2LPENR,
    #[doc = "0x168 - RCC AHB4 Sleep Clock Register"]
    pub c1_ahb4lpenr: C1_AHB4LPENR,
    #[doc = "0x16c - RCC APB3 Sleep Clock Register"]
    pub c1_apb3lpenr: C1_APB3LPENR,
    #[doc = "0x170 - RCC APB1 Low Sleep Clock Register"]
    pub c1_apb1llpenr: C1_APB1LLPENR,
    #[doc = "0x174 - RCC APB1 High Sleep Clock Register"]
    pub c1_apb1hlpenr: C1_APB1HLPENR,
    #[doc = "0x178 - RCC APB2 Sleep Clock Register"]
    pub c1_apb2lpenr: C1_APB2LPENR,
    #[doc = "0x17c - RCC APB4 Sleep Clock Register"]
    pub c1_apb4lpenr: C1_APB4LPENR,
}
#[doc = "CR (rw) register accessor: clock control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`cr`]
module"]
pub type CR = crate::Reg<cr::CR_SPEC>;
#[doc = "clock control register"]
pub mod cr;
#[doc = "ICSCR (rw) register accessor: RCC Internal Clock Source Calibration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`icscr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`icscr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`icscr`]
module"]
pub type ICSCR = crate::Reg<icscr::ICSCR_SPEC>;
#[doc = "RCC Internal Clock Source Calibration Register"]
pub mod icscr;
#[doc = "CRRCR (r) register accessor: RCC Clock Recovery RC Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`crrcr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`crrcr`]
module"]
pub type CRRCR = crate::Reg<crrcr::CRRCR_SPEC>;
#[doc = "RCC Clock Recovery RC Register"]
pub mod crrcr;
#[doc = "CFGR (rw) register accessor: RCC Clock Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfgr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfgr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`cfgr`]
module"]
pub type CFGR = crate::Reg<cfgr::CFGR_SPEC>;
#[doc = "RCC Clock Configuration Register"]
pub mod cfgr;
#[doc = "D1CFGR (rw) register accessor: RCC Domain 1 Clock Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`d1cfgr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`d1cfgr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`d1cfgr`]
module"]
pub type D1CFGR = crate::Reg<d1cfgr::D1CFGR_SPEC>;
#[doc = "RCC Domain 1 Clock Configuration Register"]
pub mod d1cfgr;
#[doc = "D2CFGR (rw) register accessor: RCC Domain 2 Clock Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`d2cfgr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`d2cfgr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`d2cfgr`]
module"]
pub type D2CFGR = crate::Reg<d2cfgr::D2CFGR_SPEC>;
#[doc = "RCC Domain 2 Clock Configuration Register"]
pub mod d2cfgr;
#[doc = "D3CFGR (rw) register accessor: RCC Domain 3 Clock Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`d3cfgr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`d3cfgr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`d3cfgr`]
module"]
pub type D3CFGR = crate::Reg<d3cfgr::D3CFGR_SPEC>;
#[doc = "RCC Domain 3 Clock Configuration Register"]
pub mod d3cfgr;
#[doc = "PLLCKSELR (rw) register accessor: RCC PLLs Clock Source Selection Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pllckselr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pllckselr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`pllckselr`]
module"]
pub type PLLCKSELR = crate::Reg<pllckselr::PLLCKSELR_SPEC>;
#[doc = "RCC PLLs Clock Source Selection Register"]
pub mod pllckselr;
#[doc = "PLLCFGR (rw) register accessor: RCC PLLs Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pllcfgr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pllcfgr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`pllcfgr`]
module"]
pub type PLLCFGR = crate::Reg<pllcfgr::PLLCFGR_SPEC>;
#[doc = "RCC PLLs Configuration Register"]
pub mod pllcfgr;
#[doc = "PLL1DIVR (rw) register accessor: RCC PLL1 Dividers Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pll1divr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pll1divr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`pll1divr`]
module"]
pub type PLL1DIVR = crate::Reg<pll1divr::PLL1DIVR_SPEC>;
#[doc = "RCC PLL1 Dividers Configuration Register"]
pub mod pll1divr;
#[doc = "PLL1FRACR (rw) register accessor: RCC PLL1 Fractional Divider Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pll1fracr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pll1fracr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`pll1fracr`]
module"]
pub type PLL1FRACR = crate::Reg<pll1fracr::PLL1FRACR_SPEC>;
#[doc = "RCC PLL1 Fractional Divider Register"]
pub mod pll1fracr;
#[doc = "PLL2DIVR (rw) register accessor: RCC PLL2 Dividers Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pll2divr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pll2divr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`pll2divr`]
module"]
pub type PLL2DIVR = crate::Reg<pll2divr::PLL2DIVR_SPEC>;
#[doc = "RCC PLL2 Dividers Configuration Register"]
pub mod pll2divr;
#[doc = "PLL2FRACR (rw) register accessor: RCC PLL2 Fractional Divider Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pll2fracr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pll2fracr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`pll2fracr`]
module"]
pub type PLL2FRACR = crate::Reg<pll2fracr::PLL2FRACR_SPEC>;
#[doc = "RCC PLL2 Fractional Divider Register"]
pub mod pll2fracr;
#[doc = "PLL3DIVR (rw) register accessor: RCC PLL3 Dividers Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pll3divr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pll3divr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`pll3divr`]
module"]
pub type PLL3DIVR = crate::Reg<pll3divr::PLL3DIVR_SPEC>;
#[doc = "RCC PLL3 Dividers Configuration Register"]
pub mod pll3divr;
#[doc = "PLL3FRACR (rw) register accessor: RCC PLL3 Fractional Divider Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pll3fracr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pll3fracr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`pll3fracr`]
module"]
pub type PLL3FRACR = crate::Reg<pll3fracr::PLL3FRACR_SPEC>;
#[doc = "RCC PLL3 Fractional Divider Register"]
pub mod pll3fracr;
#[doc = "D1CCIPR (rw) register accessor: RCC Domain 1 Kernel Clock Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`d1ccipr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`d1ccipr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`d1ccipr`]
module"]
pub type D1CCIPR = crate::Reg<d1ccipr::D1CCIPR_SPEC>;
#[doc = "RCC Domain 1 Kernel Clock Configuration Register"]
pub mod d1ccipr;
#[doc = "D2CCIP1R (rw) register accessor: RCC Domain 2 Kernel Clock Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`d2ccip1r::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`d2ccip1r::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`d2ccip1r`]
module"]
pub type D2CCIP1R = crate::Reg<d2ccip1r::D2CCIP1R_SPEC>;
#[doc = "RCC Domain 2 Kernel Clock Configuration Register"]
pub mod d2ccip1r;
#[doc = "D2CCIP2R (rw) register accessor: RCC Domain 2 Kernel Clock Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`d2ccip2r::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`d2ccip2r::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`d2ccip2r`]
module"]
pub type D2CCIP2R = crate::Reg<d2ccip2r::D2CCIP2R_SPEC>;
#[doc = "RCC Domain 2 Kernel Clock Configuration Register"]
pub mod d2ccip2r;
#[doc = "D3CCIPR (rw) register accessor: RCC Domain 3 Kernel Clock Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`d3ccipr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`d3ccipr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`d3ccipr`]
module"]
pub type D3CCIPR = crate::Reg<d3ccipr::D3CCIPR_SPEC>;
#[doc = "RCC Domain 3 Kernel Clock Configuration Register"]
pub mod d3ccipr;
#[doc = "CIER (rw) register accessor: RCC Clock Source Interrupt Enable Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cier::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cier::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`cier`]
module"]
pub type CIER = crate::Reg<cier::CIER_SPEC>;
#[doc = "RCC Clock Source Interrupt Enable Register"]
pub mod cier;
#[doc = "CIFR (rw) register accessor: RCC Clock Source Interrupt Flag Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cifr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cifr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`cifr`]
module"]
pub type CIFR = crate::Reg<cifr::CIFR_SPEC>;
#[doc = "RCC Clock Source Interrupt Flag Register"]
pub mod cifr;
#[doc = "CICR (rw) register accessor: RCC Clock Source Interrupt Clear Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cicr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cicr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`cicr`]
module"]
pub type CICR = crate::Reg<cicr::CICR_SPEC>;
#[doc = "RCC Clock Source Interrupt Clear Register"]
pub mod cicr;
#[doc = "BDCR (rw) register accessor: RCC Backup Domain Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bdcr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bdcr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`bdcr`]
module"]
pub type BDCR = crate::Reg<bdcr::BDCR_SPEC>;
#[doc = "RCC Backup Domain Control Register"]
pub mod bdcr;
#[doc = "CSR (rw) register accessor: RCC Clock Control and Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`csr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`csr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`csr`]
module"]
pub type CSR = crate::Reg<csr::CSR_SPEC>;
#[doc = "RCC Clock Control and Status Register"]
pub mod csr;
#[doc = "AHB3RSTR (rw) register accessor: RCC AHB3 Reset Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ahb3rstr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ahb3rstr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`ahb3rstr`]
module"]
pub type AHB3RSTR = crate::Reg<ahb3rstr::AHB3RSTR_SPEC>;
#[doc = "RCC AHB3 Reset Register"]
pub mod ahb3rstr;
#[doc = "AHB1RSTR (rw) register accessor: RCC AHB1 Peripheral Reset Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ahb1rstr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ahb1rstr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`ahb1rstr`]
module"]
pub type AHB1RSTR = crate::Reg<ahb1rstr::AHB1RSTR_SPEC>;
#[doc = "RCC AHB1 Peripheral Reset Register"]
pub mod ahb1rstr;
#[doc = "AHB2RSTR (rw) register accessor: RCC AHB2 Peripheral Reset Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ahb2rstr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ahb2rstr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`ahb2rstr`]
module"]
pub type AHB2RSTR = crate::Reg<ahb2rstr::AHB2RSTR_SPEC>;
#[doc = "RCC AHB2 Peripheral Reset Register"]
pub mod ahb2rstr;
#[doc = "AHB4RSTR (rw) register accessor: RCC AHB4 Peripheral Reset Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ahb4rstr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ahb4rstr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`ahb4rstr`]
module"]
pub type AHB4RSTR = crate::Reg<ahb4rstr::AHB4RSTR_SPEC>;
#[doc = "RCC AHB4 Peripheral Reset Register"]
pub mod ahb4rstr;
#[doc = "APB3RSTR (rw) register accessor: RCC APB3 Peripheral Reset Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apb3rstr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apb3rstr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`apb3rstr`]
module"]
pub type APB3RSTR = crate::Reg<apb3rstr::APB3RSTR_SPEC>;
#[doc = "RCC APB3 Peripheral Reset Register"]
pub mod apb3rstr;
#[doc = "APB1LRSTR (rw) register accessor: RCC APB1 Peripheral Reset Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apb1lrstr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apb1lrstr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`apb1lrstr`]
module"]
pub type APB1LRSTR = crate::Reg<apb1lrstr::APB1LRSTR_SPEC>;
#[doc = "RCC APB1 Peripheral Reset Register"]
pub mod apb1lrstr;
#[doc = "APB1HRSTR (rw) register accessor: RCC APB1 Peripheral Reset Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apb1hrstr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apb1hrstr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`apb1hrstr`]
module"]
pub type APB1HRSTR = crate::Reg<apb1hrstr::APB1HRSTR_SPEC>;
#[doc = "RCC APB1 Peripheral Reset Register"]
pub mod apb1hrstr;
#[doc = "APB2RSTR (rw) register accessor: RCC APB2 Peripheral Reset Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apb2rstr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apb2rstr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`apb2rstr`]
module"]
pub type APB2RSTR = crate::Reg<apb2rstr::APB2RSTR_SPEC>;
#[doc = "RCC APB2 Peripheral Reset Register"]
pub mod apb2rstr;
#[doc = "APB4RSTR (rw) register accessor: RCC APB4 Peripheral Reset Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apb4rstr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apb4rstr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`apb4rstr`]
module"]
pub type APB4RSTR = crate::Reg<apb4rstr::APB4RSTR_SPEC>;
#[doc = "RCC APB4 Peripheral Reset Register"]
pub mod apb4rstr;
#[doc = "GCR (rw) register accessor: RCC Global Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gcr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gcr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`gcr`]
module"]
pub type GCR = crate::Reg<gcr::GCR_SPEC>;
#[doc = "RCC Global Control Register"]
pub mod gcr;
#[doc = "D3AMR (rw) register accessor: RCC D3 Autonomous mode Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`d3amr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`d3amr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`d3amr`]
module"]
pub type D3AMR = crate::Reg<d3amr::D3AMR_SPEC>;
#[doc = "RCC D3 Autonomous mode Register"]
pub mod d3amr;
#[doc = "RSR (rw) register accessor: RCC Reset Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rsr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rsr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`rsr`]
module"]
pub type RSR = crate::Reg<rsr::RSR_SPEC>;
#[doc = "RCC Reset Status Register"]
pub mod rsr;
#[doc = "C1_RSR (rw) register accessor: RCC Reset Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`c1_rsr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`c1_rsr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`c1_rsr`]
module"]
pub type C1_RSR = crate::Reg<c1_rsr::C1_RSR_SPEC>;
#[doc = "RCC Reset Status Register"]
pub mod c1_rsr;
#[doc = "C1_AHB3ENR (rw) register accessor: RCC AHB3 Clock Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`c1_ahb3enr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`c1_ahb3enr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`c1_ahb3enr`]
module"]
pub type C1_AHB3ENR = crate::Reg<c1_ahb3enr::C1_AHB3ENR_SPEC>;
#[doc = "RCC AHB3 Clock Register"]
pub mod c1_ahb3enr;
#[doc = "AHB3ENR (rw) register accessor: RCC AHB3 Clock Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ahb3enr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ahb3enr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`ahb3enr`]
module"]
pub type AHB3ENR = crate::Reg<ahb3enr::AHB3ENR_SPEC>;
#[doc = "RCC AHB3 Clock Register"]
pub mod ahb3enr;
#[doc = "AHB1ENR (rw) register accessor: RCC AHB1 Clock Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ahb1enr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ahb1enr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`ahb1enr`]
module"]
pub type AHB1ENR = crate::Reg<ahb1enr::AHB1ENR_SPEC>;
#[doc = "RCC AHB1 Clock Register"]
pub mod ahb1enr;
#[doc = "C1_AHB1ENR (rw) register accessor: RCC AHB1 Clock Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`c1_ahb1enr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`c1_ahb1enr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`c1_ahb1enr`]
module"]
pub type C1_AHB1ENR = crate::Reg<c1_ahb1enr::C1_AHB1ENR_SPEC>;
#[doc = "RCC AHB1 Clock Register"]
pub mod c1_ahb1enr;
#[doc = "C1_AHB2ENR (rw) register accessor: RCC AHB2 Clock Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`c1_ahb2enr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`c1_ahb2enr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`c1_ahb2enr`]
module"]
pub type C1_AHB2ENR = crate::Reg<c1_ahb2enr::C1_AHB2ENR_SPEC>;
#[doc = "RCC AHB2 Clock Register"]
pub mod c1_ahb2enr;
#[doc = "AHB2ENR (rw) register accessor: RCC AHB2 Clock Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ahb2enr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ahb2enr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`ahb2enr`]
module"]
pub type AHB2ENR = crate::Reg<ahb2enr::AHB2ENR_SPEC>;
#[doc = "RCC AHB2 Clock Register"]
pub mod ahb2enr;
#[doc = "AHB4ENR (rw) register accessor: RCC AHB4 Clock Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ahb4enr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ahb4enr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`ahb4enr`]
module"]
pub type AHB4ENR = crate::Reg<ahb4enr::AHB4ENR_SPEC>;
#[doc = "RCC AHB4 Clock Register"]
pub mod ahb4enr;
#[doc = "C1_AHB4ENR (rw) register accessor: RCC AHB4 Clock Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`c1_ahb4enr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`c1_ahb4enr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`c1_ahb4enr`]
module"]
pub type C1_AHB4ENR = crate::Reg<c1_ahb4enr::C1_AHB4ENR_SPEC>;
#[doc = "RCC AHB4 Clock Register"]
pub mod c1_ahb4enr;
#[doc = "C1_APB3ENR (rw) register accessor: RCC APB3 Clock Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`c1_apb3enr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`c1_apb3enr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`c1_apb3enr`]
module"]
pub type C1_APB3ENR = crate::Reg<c1_apb3enr::C1_APB3ENR_SPEC>;
#[doc = "RCC APB3 Clock Register"]
pub mod c1_apb3enr;
#[doc = "APB3ENR (rw) register accessor: RCC APB3 Clock Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apb3enr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apb3enr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`apb3enr`]
module"]
pub type APB3ENR = crate::Reg<apb3enr::APB3ENR_SPEC>;
#[doc = "RCC APB3 Clock Register"]
pub mod apb3enr;
#[doc = "APB1LENR (rw) register accessor: RCC APB1 Clock Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apb1lenr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apb1lenr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`apb1lenr`]
module"]
pub type APB1LENR = crate::Reg<apb1lenr::APB1LENR_SPEC>;
#[doc = "RCC APB1 Clock Register"]
pub mod apb1lenr;
#[doc = "C1_APB1LENR (rw) register accessor: RCC APB1 Clock Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`c1_apb1lenr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`c1_apb1lenr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`c1_apb1lenr`]
module"]
pub type C1_APB1LENR = crate::Reg<c1_apb1lenr::C1_APB1LENR_SPEC>;
#[doc = "RCC APB1 Clock Register"]
pub mod c1_apb1lenr;
#[doc = "APB1HENR (rw) register accessor: RCC APB1 Clock Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apb1henr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apb1henr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`apb1henr`]
module"]
pub type APB1HENR = crate::Reg<apb1henr::APB1HENR_SPEC>;
#[doc = "RCC APB1 Clock Register"]
pub mod apb1henr;
#[doc = "C1_APB1HENR (rw) register accessor: RCC APB1 Clock Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`c1_apb1henr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`c1_apb1henr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`c1_apb1henr`]
module"]
pub type C1_APB1HENR = crate::Reg<c1_apb1henr::C1_APB1HENR_SPEC>;
#[doc = "RCC APB1 Clock Register"]
pub mod c1_apb1henr;
#[doc = "C1_APB2ENR (rw) register accessor: RCC APB2 Clock Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`c1_apb2enr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`c1_apb2enr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`c1_apb2enr`]
module"]
pub type C1_APB2ENR = crate::Reg<c1_apb2enr::C1_APB2ENR_SPEC>;
#[doc = "RCC APB2 Clock Register"]
pub mod c1_apb2enr;
#[doc = "APB2ENR (rw) register accessor: RCC APB2 Clock Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apb2enr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apb2enr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`apb2enr`]
module"]
pub type APB2ENR = crate::Reg<apb2enr::APB2ENR_SPEC>;
#[doc = "RCC APB2 Clock Register"]
pub mod apb2enr;
#[doc = "APB4ENR (rw) register accessor: RCC APB4 Clock Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apb4enr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apb4enr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`apb4enr`]
module"]
pub type APB4ENR = crate::Reg<apb4enr::APB4ENR_SPEC>;
#[doc = "RCC APB4 Clock Register"]
pub mod apb4enr;
#[doc = "C1_APB4ENR (rw) register accessor: RCC APB4 Clock Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`c1_apb4enr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`c1_apb4enr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`c1_apb4enr`]
module"]
pub type C1_APB4ENR = crate::Reg<c1_apb4enr::C1_APB4ENR_SPEC>;
#[doc = "RCC APB4 Clock Register"]
pub mod c1_apb4enr;
#[doc = "C1_AHB3LPENR (rw) register accessor: RCC AHB3 Sleep Clock Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`c1_ahb3lpenr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`c1_ahb3lpenr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`c1_ahb3lpenr`]
module"]
pub type C1_AHB3LPENR = crate::Reg<c1_ahb3lpenr::C1_AHB3LPENR_SPEC>;
#[doc = "RCC AHB3 Sleep Clock Register"]
pub mod c1_ahb3lpenr;
#[doc = "AHB3LPENR (rw) register accessor: RCC AHB3 Sleep Clock Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ahb3lpenr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ahb3lpenr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`ahb3lpenr`]
module"]
pub type AHB3LPENR = crate::Reg<ahb3lpenr::AHB3LPENR_SPEC>;
#[doc = "RCC AHB3 Sleep Clock Register"]
pub mod ahb3lpenr;
#[doc = "AHB1LPENR (rw) register accessor: RCC AHB1 Sleep Clock Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ahb1lpenr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ahb1lpenr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`ahb1lpenr`]
module"]
pub type AHB1LPENR = crate::Reg<ahb1lpenr::AHB1LPENR_SPEC>;
#[doc = "RCC AHB1 Sleep Clock Register"]
pub mod ahb1lpenr;
#[doc = "C1_AHB1LPENR (rw) register accessor: RCC AHB1 Sleep Clock Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`c1_ahb1lpenr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`c1_ahb1lpenr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`c1_ahb1lpenr`]
module"]
pub type C1_AHB1LPENR = crate::Reg<c1_ahb1lpenr::C1_AHB1LPENR_SPEC>;
#[doc = "RCC AHB1 Sleep Clock Register"]
pub mod c1_ahb1lpenr;
#[doc = "C1_AHB2LPENR (rw) register accessor: RCC AHB2 Sleep Clock Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`c1_ahb2lpenr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`c1_ahb2lpenr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`c1_ahb2lpenr`]
module"]
pub type C1_AHB2LPENR = crate::Reg<c1_ahb2lpenr::C1_AHB2LPENR_SPEC>;
#[doc = "RCC AHB2 Sleep Clock Register"]
pub mod c1_ahb2lpenr;
#[doc = "AHB2LPENR (rw) register accessor: RCC AHB2 Sleep Clock Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ahb2lpenr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ahb2lpenr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`ahb2lpenr`]
module"]
pub type AHB2LPENR = crate::Reg<ahb2lpenr::AHB2LPENR_SPEC>;
#[doc = "RCC AHB2 Sleep Clock Register"]
pub mod ahb2lpenr;
#[doc = "AHB4LPENR (rw) register accessor: RCC AHB4 Sleep Clock Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ahb4lpenr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ahb4lpenr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`ahb4lpenr`]
module"]
pub type AHB4LPENR = crate::Reg<ahb4lpenr::AHB4LPENR_SPEC>;
#[doc = "RCC AHB4 Sleep Clock Register"]
pub mod ahb4lpenr;
#[doc = "C1_AHB4LPENR (rw) register accessor: RCC AHB4 Sleep Clock Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`c1_ahb4lpenr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`c1_ahb4lpenr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`c1_ahb4lpenr`]
module"]
pub type C1_AHB4LPENR = crate::Reg<c1_ahb4lpenr::C1_AHB4LPENR_SPEC>;
#[doc = "RCC AHB4 Sleep Clock Register"]
pub mod c1_ahb4lpenr;
#[doc = "C1_APB3LPENR (rw) register accessor: RCC APB3 Sleep Clock Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`c1_apb3lpenr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`c1_apb3lpenr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`c1_apb3lpenr`]
module"]
pub type C1_APB3LPENR = crate::Reg<c1_apb3lpenr::C1_APB3LPENR_SPEC>;
#[doc = "RCC APB3 Sleep Clock Register"]
pub mod c1_apb3lpenr;
#[doc = "APB3LPENR (rw) register accessor: RCC APB3 Sleep Clock Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apb3lpenr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apb3lpenr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`apb3lpenr`]
module"]
pub type APB3LPENR = crate::Reg<apb3lpenr::APB3LPENR_SPEC>;
#[doc = "RCC APB3 Sleep Clock Register"]
pub mod apb3lpenr;
#[doc = "APB1LLPENR (rw) register accessor: RCC APB1 Low Sleep Clock Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apb1llpenr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apb1llpenr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`apb1llpenr`]
module"]
pub type APB1LLPENR = crate::Reg<apb1llpenr::APB1LLPENR_SPEC>;
#[doc = "RCC APB1 Low Sleep Clock Register"]
pub mod apb1llpenr;
#[doc = "C1_APB1LLPENR (rw) register accessor: RCC APB1 Low Sleep Clock Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`c1_apb1llpenr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`c1_apb1llpenr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`c1_apb1llpenr`]
module"]
pub type C1_APB1LLPENR = crate::Reg<c1_apb1llpenr::C1_APB1LLPENR_SPEC>;
#[doc = "RCC APB1 Low Sleep Clock Register"]
pub mod c1_apb1llpenr;
#[doc = "C1_APB1HLPENR (rw) register accessor: RCC APB1 High Sleep Clock Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`c1_apb1hlpenr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`c1_apb1hlpenr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`c1_apb1hlpenr`]
module"]
pub type C1_APB1HLPENR = crate::Reg<c1_apb1hlpenr::C1_APB1HLPENR_SPEC>;
#[doc = "RCC APB1 High Sleep Clock Register"]
pub mod c1_apb1hlpenr;
#[doc = "APB1HLPENR (rw) register accessor: RCC APB1 High Sleep Clock Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apb1hlpenr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apb1hlpenr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`apb1hlpenr`]
module"]
pub type APB1HLPENR = crate::Reg<apb1hlpenr::APB1HLPENR_SPEC>;
#[doc = "RCC APB1 High Sleep Clock Register"]
pub mod apb1hlpenr;
#[doc = "APB2LPENR (rw) register accessor: RCC APB2 Sleep Clock Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apb2lpenr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apb2lpenr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`apb2lpenr`]
module"]
pub type APB2LPENR = crate::Reg<apb2lpenr::APB2LPENR_SPEC>;
#[doc = "RCC APB2 Sleep Clock Register"]
pub mod apb2lpenr;
#[doc = "C1_APB2LPENR (rw) register accessor: RCC APB2 Sleep Clock Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`c1_apb2lpenr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`c1_apb2lpenr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`c1_apb2lpenr`]
module"]
pub type C1_APB2LPENR = crate::Reg<c1_apb2lpenr::C1_APB2LPENR_SPEC>;
#[doc = "RCC APB2 Sleep Clock Register"]
pub mod c1_apb2lpenr;
#[doc = "C1_APB4LPENR (rw) register accessor: RCC APB4 Sleep Clock Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`c1_apb4lpenr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`c1_apb4lpenr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`c1_apb4lpenr`]
module"]
pub type C1_APB4LPENR = crate::Reg<c1_apb4lpenr::C1_APB4LPENR_SPEC>;
#[doc = "RCC APB4 Sleep Clock Register"]
pub mod c1_apb4lpenr;
#[doc = "APB4LPENR (rw) register accessor: RCC APB4 Sleep Clock Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apb4lpenr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apb4lpenr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`apb4lpenr`]
module"]
pub type APB4LPENR = crate::Reg<apb4lpenr::APB4LPENR_SPEC>;
#[doc = "RCC APB4 Sleep Clock Register"]
pub mod apb4lpenr;
