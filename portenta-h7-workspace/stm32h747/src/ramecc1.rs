#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - RAMECC interrupt enable register"]
    pub ier: IER,
    _reserved1: [u8; 0x1c],
    #[doc = "0x20 - RAMECC monitor x configuration register"]
    pub m1cr: M1CR,
    #[doc = "0x24 - RAMECC monitor x status register"]
    pub m1sr: M1SR,
    #[doc = "0x28 - RAMECC monitor x failing address register"]
    pub m1far: M1FAR,
    #[doc = "0x2c - RAMECC monitor x failing data low register"]
    pub m1fdrl: M1FDRL,
    #[doc = "0x30 - RAMECC monitor x failing data high register"]
    pub m1fdrh: M1FDRH,
    #[doc = "0x34 - RAMECC monitor x failing ECC error code register"]
    pub m1fecr: M1FECR,
    _reserved7: [u8; 0x08],
    #[doc = "0x40 - RAMECC monitor x configuration register"]
    pub m2cr: M2CR,
    #[doc = "0x44 - RAMECC monitor x status register"]
    pub m2sr: M2SR,
    #[doc = "0x48 - RAMECC monitor x failing address register"]
    pub m2far: M2FAR,
    #[doc = "0x4c - RAMECC monitor x failing data low register"]
    pub m2fdrl: M2FDRL,
    #[doc = "0x50 - RAMECC monitor x failing data high register"]
    pub m2fdrh: M2FDRH,
    _reserved12: [u8; 0x04],
    #[doc = "0x58 - RAMECC monitor x failing ECC error code register"]
    pub m2fecr: M2FECR,
    _reserved13: [u8; 0x04],
    #[doc = "0x60 - RAMECC monitor x configuration register"]
    pub m3cr: M3CR,
    #[doc = "0x64 - RAMECC monitor x status register"]
    pub m3sr: M3SR,
    #[doc = "0x68 - RAMECC monitor x failing address register"]
    pub m3far: M3FAR,
    #[doc = "0x6c - RAMECC monitor x failing data low register"]
    pub m3fdrl: M3FDRL,
    #[doc = "0x70 - RAMECC monitor x failing data high register"]
    pub m3fdrh: M3FDRH,
    _reserved18: [u8; 0x08],
    #[doc = "0x7c - RAMECC monitor x failing ECC error code register"]
    pub m3fecr: M3FECR,
    #[doc = "0x80 - RAMECC monitor x configuration register"]
    pub m4cr: M4CR,
    #[doc = "0x84 - RAMECC monitor x status register"]
    pub m4sr: M4SR,
    #[doc = "0x88 - RAMECC monitor x failing address register"]
    pub m4far: M4FAR,
    #[doc = "0x8c - RAMECC monitor x failing data low register"]
    pub m4fdrl: M4FDRL,
    _reserved_23_m: [u8; 0x04],
    _reserved24: [u8; 0x0c],
    #[doc = "0xa0 - RAMECC monitor x configuration register"]
    pub m5cr: M5CR,
    #[doc = "0xa4 - RAMECC monitor x status register"]
    pub m5sr: M5SR,
    #[doc = "0xa8 - RAMECC monitor x failing address register"]
    pub m5far: M5FAR,
    #[doc = "0xac - RAMECC monitor x failing data low register"]
    pub m5fdrl: M5FDRL,
    #[doc = "0xb0 - RAMECC monitor x failing data high register"]
    pub m5fdrh: M5FDRH,
    #[doc = "0xb4 - RAMECC monitor x failing ECC error code register"]
    pub m5fecr: M5FECR,
}
impl RegisterBlock {
    #[doc = "0x90 - RAMECC monitor x failing ECC error code register"]
    #[inline(always)]
    pub const fn m4fecr(&self) -> &M4FECR {
        unsafe { &*(self as *const Self).cast::<u8>().add(144usize).cast() }
    }
    #[doc = "0x90 - RAMECC monitor x failing data high register"]
    #[inline(always)]
    pub const fn m4fdrh(&self) -> &M4FDRH {
        unsafe { &*(self as *const Self).cast::<u8>().add(144usize).cast() }
    }
}
#[doc = "IER (rw) register accessor: RAMECC interrupt enable register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ier::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ier::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`ier`]
module"]
pub type IER = crate::Reg<ier::IER_SPEC>;
#[doc = "RAMECC interrupt enable register"]
pub mod ier;
#[doc = "M1CR (rw) register accessor: RAMECC monitor x configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`m1cr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`m1cr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`m1cr`]
module"]
pub type M1CR = crate::Reg<m1cr::M1CR_SPEC>;
#[doc = "RAMECC monitor x configuration register"]
pub mod m1cr;
#[doc = "M1SR (rw) register accessor: RAMECC monitor x status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`m1sr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`m1sr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`m1sr`]
module"]
pub type M1SR = crate::Reg<m1sr::M1SR_SPEC>;
#[doc = "RAMECC monitor x status register"]
pub mod m1sr;
#[doc = "M1FAR (rw) register accessor: RAMECC monitor x failing address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`m1far::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`m1far::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`m1far`]
module"]
pub type M1FAR = crate::Reg<m1far::M1FAR_SPEC>;
#[doc = "RAMECC monitor x failing address register"]
pub mod m1far;
#[doc = "M1FDRL (rw) register accessor: RAMECC monitor x failing data low register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`m1fdrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`m1fdrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`m1fdrl`]
module"]
pub type M1FDRL = crate::Reg<m1fdrl::M1FDRL_SPEC>;
#[doc = "RAMECC monitor x failing data low register"]
pub mod m1fdrl;
#[doc = "M1FDRH (rw) register accessor: RAMECC monitor x failing data high register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`m1fdrh::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`m1fdrh::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`m1fdrh`]
module"]
pub type M1FDRH = crate::Reg<m1fdrh::M1FDRH_SPEC>;
#[doc = "RAMECC monitor x failing data high register"]
pub mod m1fdrh;
#[doc = "M1FECR (rw) register accessor: RAMECC monitor x failing ECC error code register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`m1fecr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`m1fecr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`m1fecr`]
module"]
pub type M1FECR = crate::Reg<m1fecr::M1FECR_SPEC>;
#[doc = "RAMECC monitor x failing ECC error code register"]
pub mod m1fecr;
#[doc = "M2CR (rw) register accessor: RAMECC monitor x configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`m2cr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`m2cr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`m2cr`]
module"]
pub type M2CR = crate::Reg<m2cr::M2CR_SPEC>;
#[doc = "RAMECC monitor x configuration register"]
pub mod m2cr;
#[doc = "M2SR (rw) register accessor: RAMECC monitor x status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`m2sr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`m2sr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`m2sr`]
module"]
pub type M2SR = crate::Reg<m2sr::M2SR_SPEC>;
#[doc = "RAMECC monitor x status register"]
pub mod m2sr;
#[doc = "M2FAR (rw) register accessor: RAMECC monitor x failing address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`m2far::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`m2far::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`m2far`]
module"]
pub type M2FAR = crate::Reg<m2far::M2FAR_SPEC>;
#[doc = "RAMECC monitor x failing address register"]
pub mod m2far;
#[doc = "M2FDRL (rw) register accessor: RAMECC monitor x failing data low register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`m2fdrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`m2fdrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`m2fdrl`]
module"]
pub type M2FDRL = crate::Reg<m2fdrl::M2FDRL_SPEC>;
#[doc = "RAMECC monitor x failing data low register"]
pub mod m2fdrl;
#[doc = "M2FDRH (r) register accessor: RAMECC monitor x failing data high register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`m2fdrh::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`m2fdrh`]
module"]
pub type M2FDRH = crate::Reg<m2fdrh::M2FDRH_SPEC>;
#[doc = "RAMECC monitor x failing data high register"]
pub mod m2fdrh;
#[doc = "M2FECR (r) register accessor: RAMECC monitor x failing ECC error code register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`m2fecr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`m2fecr`]
module"]
pub type M2FECR = crate::Reg<m2fecr::M2FECR_SPEC>;
#[doc = "RAMECC monitor x failing ECC error code register"]
pub mod m2fecr;
#[doc = "M3CR (r) register accessor: RAMECC monitor x configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`m3cr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`m3cr`]
module"]
pub type M3CR = crate::Reg<m3cr::M3CR_SPEC>;
#[doc = "RAMECC monitor x configuration register"]
pub mod m3cr;
#[doc = "M3SR (r) register accessor: RAMECC monitor x status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`m3sr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`m3sr`]
module"]
pub type M3SR = crate::Reg<m3sr::M3SR_SPEC>;
#[doc = "RAMECC monitor x status register"]
pub mod m3sr;
#[doc = "M3FAR (rw) register accessor: RAMECC monitor x failing address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`m3far::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`m3far::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`m3far`]
module"]
pub type M3FAR = crate::Reg<m3far::M3FAR_SPEC>;
#[doc = "RAMECC monitor x failing address register"]
pub mod m3far;
#[doc = "M3FDRL (r) register accessor: RAMECC monitor x failing data low register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`m3fdrl::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`m3fdrl`]
module"]
pub type M3FDRL = crate::Reg<m3fdrl::M3FDRL_SPEC>;
#[doc = "RAMECC monitor x failing data low register"]
pub mod m3fdrl;
#[doc = "M3FDRH (r) register accessor: RAMECC monitor x failing data high register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`m3fdrh::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`m3fdrh`]
module"]
pub type M3FDRH = crate::Reg<m3fdrh::M3FDRH_SPEC>;
#[doc = "RAMECC monitor x failing data high register"]
pub mod m3fdrh;
#[doc = "M3FECR (r) register accessor: RAMECC monitor x failing ECC error code register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`m3fecr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`m3fecr`]
module"]
pub type M3FECR = crate::Reg<m3fecr::M3FECR_SPEC>;
#[doc = "RAMECC monitor x failing ECC error code register"]
pub mod m3fecr;
#[doc = "M4CR (r) register accessor: RAMECC monitor x configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`m4cr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`m4cr`]
module"]
pub type M4CR = crate::Reg<m4cr::M4CR_SPEC>;
#[doc = "RAMECC monitor x configuration register"]
pub mod m4cr;
#[doc = "M4SR (r) register accessor: RAMECC monitor x status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`m4sr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`m4sr`]
module"]
pub type M4SR = crate::Reg<m4sr::M4SR_SPEC>;
#[doc = "RAMECC monitor x status register"]
pub mod m4sr;
#[doc = "M4FAR (r) register accessor: RAMECC monitor x failing address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`m4far::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`m4far`]
module"]
pub type M4FAR = crate::Reg<m4far::M4FAR_SPEC>;
#[doc = "RAMECC monitor x failing address register"]
pub mod m4far;
#[doc = "M4FDRL (rw) register accessor: RAMECC monitor x failing data low register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`m4fdrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`m4fdrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`m4fdrl`]
module"]
pub type M4FDRL = crate::Reg<m4fdrl::M4FDRL_SPEC>;
#[doc = "RAMECC monitor x failing data low register"]
pub mod m4fdrl;
#[doc = "M4FDRH (r) register accessor: RAMECC monitor x failing data high register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`m4fdrh::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`m4fdrh`]
module"]
pub type M4FDRH = crate::Reg<m4fdrh::M4FDRH_SPEC>;
#[doc = "RAMECC monitor x failing data high register"]
pub mod m4fdrh;
#[doc = "M4FECR (r) register accessor: RAMECC monitor x failing ECC error code register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`m4fecr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`m4fecr`]
module"]
pub type M4FECR = crate::Reg<m4fecr::M4FECR_SPEC>;
#[doc = "RAMECC monitor x failing ECC error code register"]
pub mod m4fecr;
#[doc = "M5CR (r) register accessor: RAMECC monitor x configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`m5cr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`m5cr`]
module"]
pub type M5CR = crate::Reg<m5cr::M5CR_SPEC>;
#[doc = "RAMECC monitor x configuration register"]
pub mod m5cr;
#[doc = "M5SR (rw) register accessor: RAMECC monitor x status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`m5sr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`m5sr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`m5sr`]
module"]
pub type M5SR = crate::Reg<m5sr::M5SR_SPEC>;
#[doc = "RAMECC monitor x status register"]
pub mod m5sr;
#[doc = "M5FAR (rw) register accessor: RAMECC monitor x failing address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`m5far::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`m5far::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`m5far`]
module"]
pub type M5FAR = crate::Reg<m5far::M5FAR_SPEC>;
#[doc = "RAMECC monitor x failing address register"]
pub mod m5far;
#[doc = "M5FDRL (r) register accessor: RAMECC monitor x failing data low register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`m5fdrl::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`m5fdrl`]
module"]
pub type M5FDRL = crate::Reg<m5fdrl::M5FDRL_SPEC>;
#[doc = "RAMECC monitor x failing data low register"]
pub mod m5fdrl;
#[doc = "M5FDRH (r) register accessor: RAMECC monitor x failing data high register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`m5fdrh::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`m5fdrh`]
module"]
pub type M5FDRH = crate::Reg<m5fdrh::M5FDRH_SPEC>;
#[doc = "RAMECC monitor x failing data high register"]
pub mod m5fdrh;
#[doc = "M5FECR (r) register accessor: RAMECC monitor x failing ECC error code register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`m5fecr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`m5fecr`]
module"]
pub type M5FECR = crate::Reg<m5fecr::M5FECR_SPEC>;
#[doc = "RAMECC monitor x failing ECC error code register"]
pub mod m5fecr;
