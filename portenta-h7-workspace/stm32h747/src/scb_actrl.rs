#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Auxiliary control register"]
    pub actrl: ACTRL,
}
#[doc = "ACTRL (rw) register accessor: Auxiliary control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`actrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`actrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`actrl`]
module"]
pub type ACTRL = crate::Reg<actrl::ACTRL_SPEC>;
#[doc = "Auxiliary control register"]
pub mod actrl;
