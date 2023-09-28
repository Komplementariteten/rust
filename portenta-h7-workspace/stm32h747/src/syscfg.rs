#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 0x04],
    #[doc = "0x04 - peripheral mode configuration register"]
    pub pmcr: PMCR,
    #[doc = "0x08 - external interrupt configuration register 1"]
    pub exticr1: EXTICR1,
    #[doc = "0x0c - external interrupt configuration register 2"]
    pub exticr2: EXTICR2,
    #[doc = "0x10 - external interrupt configuration register 3"]
    pub exticr3: EXTICR3,
    #[doc = "0x14 - external interrupt configuration register 4"]
    pub exticr4: EXTICR4,
    #[doc = "0x18 - configuration register"]
    pub cfgr: CFGR,
    _reserved6: [u8; 0x04],
    #[doc = "0x20 - compensation cell control/status register"]
    pub cccsr: CCCSR,
    #[doc = "0x24 - SYSCFG compensation cell value register"]
    pub ccvr: CCVR,
    #[doc = "0x28 - SYSCFG compensation cell code register"]
    pub cccr: CCCR,
    #[doc = "0x2c - SYSCFG power control register"]
    pub pwrcr: PWRCR,
    _reserved10: [u8; 0xf4],
    #[doc = "0x124 - SYSCFG package register"]
    pub pkgr: PKGR,
    _reserved11: [u8; 0x01d8],
    #[doc = "0x300 - SYSCFG user register 0"]
    pub ur0: UR0,
    #[doc = "0x304 - SYSCFG user register 1"]
    pub ur1: UR1,
    #[doc = "0x308 - SYSCFG user register 2"]
    pub ur2: UR2,
    #[doc = "0x30c - SYSCFG user register 3"]
    pub ur3: UR3,
    #[doc = "0x310 - SYSCFG user register 4"]
    pub ur4: UR4,
    #[doc = "0x314 - SYSCFG user register 5"]
    pub ur5: UR5,
    #[doc = "0x318 - SYSCFG user register 6"]
    pub ur6: UR6,
    #[doc = "0x31c - SYSCFG user register 7"]
    pub ur7: UR7,
    #[doc = "0x320 - SYSCFG user register 8"]
    pub ur8: UR8,
    #[doc = "0x324 - SYSCFG user register 9"]
    pub ur9: UR9,
    #[doc = "0x328 - SYSCFG user register 10"]
    pub ur10: UR10,
    #[doc = "0x32c - SYSCFG user register 11"]
    pub ur11: UR11,
    #[doc = "0x330 - SYSCFG user register 12"]
    pub ur12: UR12,
    #[doc = "0x334 - SYSCFG user register 13"]
    pub ur13: UR13,
    #[doc = "0x338 - SYSCFG user register 14"]
    pub ur14: UR14,
    #[doc = "0x33c - SYSCFG user register 15"]
    pub ur15: UR15,
    #[doc = "0x340 - SYSCFG user register 16"]
    pub ur16: UR16,
    #[doc = "0x344 - SYSCFG user register 17"]
    pub ur17: UR17,
}
#[doc = "PMCR (rw) register accessor: peripheral mode configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pmcr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pmcr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`pmcr`]
module"]
pub type PMCR = crate::Reg<pmcr::PMCR_SPEC>;
#[doc = "peripheral mode configuration register"]
pub mod pmcr;
#[doc = "EXTICR1 (rw) register accessor: external interrupt configuration register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`exticr1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`exticr1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`exticr1`]
module"]
pub type EXTICR1 = crate::Reg<exticr1::EXTICR1_SPEC>;
#[doc = "external interrupt configuration register 1"]
pub mod exticr1;
#[doc = "EXTICR2 (rw) register accessor: external interrupt configuration register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`exticr2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`exticr2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`exticr2`]
module"]
pub type EXTICR2 = crate::Reg<exticr2::EXTICR2_SPEC>;
#[doc = "external interrupt configuration register 2"]
pub mod exticr2;
#[doc = "EXTICR3 (rw) register accessor: external interrupt configuration register 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`exticr3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`exticr3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`exticr3`]
module"]
pub type EXTICR3 = crate::Reg<exticr3::EXTICR3_SPEC>;
#[doc = "external interrupt configuration register 3"]
pub mod exticr3;
#[doc = "EXTICR4 (rw) register accessor: external interrupt configuration register 4\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`exticr4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`exticr4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`exticr4`]
module"]
pub type EXTICR4 = crate::Reg<exticr4::EXTICR4_SPEC>;
#[doc = "external interrupt configuration register 4"]
pub mod exticr4;
#[doc = "CFGR (rw) register accessor: configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfgr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfgr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`cfgr`]
module"]
pub type CFGR = crate::Reg<cfgr::CFGR_SPEC>;
#[doc = "configuration register"]
pub mod cfgr;
#[doc = "CCCSR (rw) register accessor: compensation cell control/status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cccsr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cccsr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`cccsr`]
module"]
pub type CCCSR = crate::Reg<cccsr::CCCSR_SPEC>;
#[doc = "compensation cell control/status register"]
pub mod cccsr;
#[doc = "CCVR (r) register accessor: SYSCFG compensation cell value register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ccvr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`ccvr`]
module"]
pub type CCVR = crate::Reg<ccvr::CCVR_SPEC>;
#[doc = "SYSCFG compensation cell value register"]
pub mod ccvr;
#[doc = "CCCR (rw) register accessor: SYSCFG compensation cell code register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cccr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cccr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`cccr`]
module"]
pub type CCCR = crate::Reg<cccr::CCCR_SPEC>;
#[doc = "SYSCFG compensation cell code register"]
pub mod cccr;
#[doc = "PWRCR (rw) register accessor: SYSCFG power control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pwrcr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pwrcr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`pwrcr`]
module"]
pub type PWRCR = crate::Reg<pwrcr::PWRCR_SPEC>;
#[doc = "SYSCFG power control register"]
pub mod pwrcr;
#[doc = "PKGR (r) register accessor: SYSCFG package register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pkgr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`pkgr`]
module"]
pub type PKGR = crate::Reg<pkgr::PKGR_SPEC>;
#[doc = "SYSCFG package register"]
pub mod pkgr;
#[doc = "UR0 (r) register accessor: SYSCFG user register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ur0::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`ur0`]
module"]
pub type UR0 = crate::Reg<ur0::UR0_SPEC>;
#[doc = "SYSCFG user register 0"]
pub mod ur0;
#[doc = "UR1 (rw) register accessor: SYSCFG user register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ur1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ur1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`ur1`]
module"]
pub type UR1 = crate::Reg<ur1::UR1_SPEC>;
#[doc = "SYSCFG user register 1"]
pub mod ur1;
#[doc = "UR2 (rw) register accessor: SYSCFG user register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ur2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ur2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`ur2`]
module"]
pub type UR2 = crate::Reg<ur2::UR2_SPEC>;
#[doc = "SYSCFG user register 2"]
pub mod ur2;
#[doc = "UR3 (rw) register accessor: SYSCFG user register 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ur3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ur3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`ur3`]
module"]
pub type UR3 = crate::Reg<ur3::UR3_SPEC>;
#[doc = "SYSCFG user register 3"]
pub mod ur3;
#[doc = "UR4 (rw) register accessor: SYSCFG user register 4\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ur4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ur4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`ur4`]
module"]
pub type UR4 = crate::Reg<ur4::UR4_SPEC>;
#[doc = "SYSCFG user register 4"]
pub mod ur4;
#[doc = "UR5 (r) register accessor: SYSCFG user register 5\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ur5::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`ur5`]
module"]
pub type UR5 = crate::Reg<ur5::UR5_SPEC>;
#[doc = "SYSCFG user register 5"]
pub mod ur5;
#[doc = "UR6 (r) register accessor: SYSCFG user register 6\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ur6::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`ur6`]
module"]
pub type UR6 = crate::Reg<ur6::UR6_SPEC>;
#[doc = "SYSCFG user register 6"]
pub mod ur6;
#[doc = "UR7 (r) register accessor: SYSCFG user register 7\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ur7::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`ur7`]
module"]
pub type UR7 = crate::Reg<ur7::UR7_SPEC>;
#[doc = "SYSCFG user register 7"]
pub mod ur7;
#[doc = "UR8 (r) register accessor: SYSCFG user register 8\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ur8::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`ur8`]
module"]
pub type UR8 = crate::Reg<ur8::UR8_SPEC>;
#[doc = "SYSCFG user register 8"]
pub mod ur8;
#[doc = "UR9 (r) register accessor: SYSCFG user register 9\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ur9::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`ur9`]
module"]
pub type UR9 = crate::Reg<ur9::UR9_SPEC>;
#[doc = "SYSCFG user register 9"]
pub mod ur9;
#[doc = "UR10 (r) register accessor: SYSCFG user register 10\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ur10::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`ur10`]
module"]
pub type UR10 = crate::Reg<ur10::UR10_SPEC>;
#[doc = "SYSCFG user register 10"]
pub mod ur10;
#[doc = "UR11 (r) register accessor: SYSCFG user register 11\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ur11::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`ur11`]
module"]
pub type UR11 = crate::Reg<ur11::UR11_SPEC>;
#[doc = "SYSCFG user register 11"]
pub mod ur11;
#[doc = "UR12 (r) register accessor: SYSCFG user register 12\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ur12::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`ur12`]
module"]
pub type UR12 = crate::Reg<ur12::UR12_SPEC>;
#[doc = "SYSCFG user register 12"]
pub mod ur12;
#[doc = "UR13 (r) register accessor: SYSCFG user register 13\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ur13::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`ur13`]
module"]
pub type UR13 = crate::Reg<ur13::UR13_SPEC>;
#[doc = "SYSCFG user register 13"]
pub mod ur13;
#[doc = "UR14 (rw) register accessor: SYSCFG user register 14\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ur14::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ur14::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`ur14`]
module"]
pub type UR14 = crate::Reg<ur14::UR14_SPEC>;
#[doc = "SYSCFG user register 14"]
pub mod ur14;
#[doc = "UR15 (rw) register accessor: SYSCFG user register 15\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ur15::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ur15::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`ur15`]
module"]
pub type UR15 = crate::Reg<ur15::UR15_SPEC>;
#[doc = "SYSCFG user register 15"]
pub mod ur15;
#[doc = "UR16 (r) register accessor: SYSCFG user register 16\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ur16::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`ur16`]
module"]
pub type UR16 = crate::Reg<ur16::UR16_SPEC>;
#[doc = "SYSCFG user register 16"]
pub mod ur16;
#[doc = "UR17 (r) register accessor: SYSCFG user register 17\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ur17::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`ur17`]
module"]
pub type UR17 = crate::Reg<ur17::UR17_SPEC>;
#[doc = "SYSCFG user register 17"]
pub mod ur17;
