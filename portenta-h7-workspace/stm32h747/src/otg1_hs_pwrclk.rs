#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Power and clock gating control register"]
    pub otg_hs_pcgcr: OTG_HS_PCGCR,
}
#[doc = "OTG_HS_PCGCR (rw) register accessor: Power and clock gating control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`otg_hs_pcgcr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`otg_hs_pcgcr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`otg_hs_pcgcr`]
module"]
pub type OTG_HS_PCGCR = crate::Reg<otg_hs_pcgcr::OTG_HS_PCGCR_SPEC>;
#[doc = "Power and clock gating control register"]
pub mod otg_hs_pcgcr;
