#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Cache Level ID register"]
    pub clidr: CLIDR,
    #[doc = "0x04 - Cache Type register"]
    pub ctr: CTR,
    #[doc = "0x08 - Cache Size ID register"]
    pub ccsidr: CCSIDR,
}
#[doc = "CLIDR (r) register accessor: Cache Level ID register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clidr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`clidr`]
module"]
pub type CLIDR = crate::Reg<clidr::CLIDR_SPEC>;
#[doc = "Cache Level ID register"]
pub mod clidr;
#[doc = "CTR (r) register accessor: Cache Type register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`ctr`]
module"]
pub type CTR = crate::Reg<ctr::CTR_SPEC>;
#[doc = "Cache Type register"]
pub mod ctr;
#[doc = "CCSIDR (r) register accessor: Cache Size ID register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ccsidr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`ccsidr`]
module"]
pub type CCSIDR = crate::Reg<ccsidr::CCSIDR_SPEC>;
#[doc = "Cache Size ID register"]
pub mod ccsidr;
