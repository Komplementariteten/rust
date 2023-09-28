#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 0x1fd0],
    #[doc = "0x1fd0 - AXI interconnect - peripheral ID4 register"]
    pub axi_periph_id_4: AXI_PERIPH_ID_4,
    _reserved1: [u8; 0x0c],
    #[doc = "0x1fe0 - AXI interconnect - peripheral ID0 register"]
    pub axi_periph_id_0: AXI_PERIPH_ID_0,
    #[doc = "0x1fe4 - AXI interconnect - peripheral ID1 register"]
    pub axi_periph_id_1: AXI_PERIPH_ID_1,
    #[doc = "0x1fe8 - AXI interconnect - peripheral ID2 register"]
    pub axi_periph_id_2: AXI_PERIPH_ID_2,
    #[doc = "0x1fec - AXI interconnect - peripheral ID3 register"]
    pub axi_periph_id_3: AXI_PERIPH_ID_3,
    #[doc = "0x1ff0 - AXI interconnect - component ID0 register"]
    pub axi_comp_id_0: AXI_COMP_ID_0,
    #[doc = "0x1ff4 - AXI interconnect - component ID1 register"]
    pub axi_comp_id_1: AXI_COMP_ID_1,
    #[doc = "0x1ff8 - AXI interconnect - component ID2 register"]
    pub axi_comp_id_2: AXI_COMP_ID_2,
    #[doc = "0x1ffc - AXI interconnect - component ID3 register"]
    pub axi_comp_id_3: AXI_COMP_ID_3,
    _reserved9: [u8; 0x08],
    #[doc = "0x2008 - AXI interconnect - TARG x bus matrix issuing functionality register"]
    pub axi_targ1_fn_mod_iss_bm: AXI_TARG1_FN_MOD_ISS_BM,
    _reserved10: [u8; 0x18],
    #[doc = "0x2024 - AXI interconnect - TARG x bus matrix functionality 2 register"]
    pub axi_targ1_fn_mod2: AXI_TARG1_FN_MOD2,
    _reserved11: [u8; 0x04],
    #[doc = "0x202c - AXI interconnect - TARG x long burst functionality modification"]
    pub axi_targ1_fn_mod_lb: AXI_TARG1_FN_MOD_LB,
    _reserved12: [u8; 0xd8],
    #[doc = "0x2108 - AXI interconnect - TARG x long burst functionality modification"]
    pub axi_targ1_fn_mod: AXI_TARG1_FN_MOD,
    _reserved13: [u8; 0x0efc],
    #[doc = "0x3008 - AXI interconnect - TARG x bus matrix issuing functionality register"]
    pub axi_targ2_fn_mod_iss_bm: AXI_TARG2_FN_MOD_ISS_BM,
    _reserved14: [u8; 0x18],
    #[doc = "0x3024 - AXI interconnect - TARG x bus matrix functionality 2 register"]
    pub axi_targ2_fn_mod2: AXI_TARG2_FN_MOD2,
    _reserved15: [u8; 0x04],
    #[doc = "0x302c - AXI interconnect - TARG x long burst functionality modification"]
    pub axi_targ2_fn_mod_lb: AXI_TARG2_FN_MOD_LB,
    _reserved16: [u8; 0xd8],
    #[doc = "0x3108 - AXI interconnect - TARG x long burst functionality modification"]
    pub axi_targ2_fn_mod: AXI_TARG2_FN_MOD,
    _reserved17: [u8; 0x0efc],
    #[doc = "0x4008 - AXI interconnect - TARG x bus matrix issuing functionality register"]
    pub axi_targ3_fn_mod_iss_bm: AXI_TARG3_FN_MOD_ISS_BM,
    _reserved18: [u8; 0x0ffc],
    #[doc = "0x5008 - AXI interconnect - TARG x bus matrix issuing functionality register"]
    pub axi_targ4_fn_mod_iss_bm: AXI_TARG4_FN_MOD_ISS_BM,
    _reserved19: [u8; 0x0ffc],
    #[doc = "0x6008 - AXI interconnect - TARG x bus matrix issuing functionality register"]
    pub axi_targ5_fn_mod_iss_bm: AXI_TARG5_FN_MOD_ISS_BM,
    _reserved20: [u8; 0x0ffc],
    #[doc = "0x7008 - AXI interconnect - TARG x bus matrix issuing functionality register"]
    pub axi_targ6_fn_mod_iss_bm: AXI_TARG6_FN_MOD_ISS_BM,
    _reserved21: [u8; 0x1000],
    #[doc = "0x800c - AXI interconnect - TARG x bus matrix issuing functionality register"]
    pub axi_targ7_fn_mod_iss_bm: AXI_TARG7_FN_MOD_ISS_BM,
    _reserved22: [u8; 0x14],
    #[doc = "0x8024 - AXI interconnect - TARG x bus matrix functionality 2 register"]
    pub axi_targ7_fn_mod2: AXI_TARG7_FN_MOD2,
    _reserved23: [u8; 0xe0],
    #[doc = "0x8108 - AXI interconnect - TARG x long burst functionality modification"]
    pub axi_targ7_fn_mod: AXI_TARG7_FN_MOD,
    _reserved24: [u8; 0x0003_9f18],
    #[doc = "0x42024 - AXI interconnect - INI x functionality modification 2 register"]
    pub axi_ini1_fn_mod2: AXI_INI1_FN_MOD2,
    #[doc = "0x42028 - AXI interconnect - INI x AHB functionality modification register"]
    pub axi_ini1_fn_mod_ahb: AXI_INI1_FN_MOD_AHB,
    _reserved26: [u8; 0xd4],
    #[doc = "0x42100 - AXI interconnect - INI x read QoS register"]
    pub axi_ini1_read_qos: AXI_INI1_READ_QOS,
    #[doc = "0x42104 - AXI interconnect - INI x write QoS register"]
    pub axi_ini1_write_qos: AXI_INI1_WRITE_QOS,
    #[doc = "0x42108 - AXI interconnect - INI x issuing functionality modification register"]
    pub axi_ini1_fn_mod: AXI_INI1_FN_MOD,
    _reserved29: [u8; 0x0ff4],
    #[doc = "0x43100 - AXI interconnect - INI x read QoS register"]
    pub axi_ini2_read_qos: AXI_INI2_READ_QOS,
    #[doc = "0x43104 - AXI interconnect - INI x write QoS register"]
    pub axi_ini2_write_qos: AXI_INI2_WRITE_QOS,
    #[doc = "0x43108 - AXI interconnect - INI x issuing functionality modification register"]
    pub axi_ini2_fn_mod: AXI_INI2_FN_MOD,
    _reserved32: [u8; 0x0f18],
    #[doc = "0x44024 - AXI interconnect - INI x functionality modification 2 register"]
    pub axi_ini3_fn_mod2: AXI_INI3_FN_MOD2,
    #[doc = "0x44028 - AXI interconnect - INI x AHB functionality modification register"]
    pub axi_ini3_fn_mod_ahb: AXI_INI3_FN_MOD_AHB,
    _reserved34: [u8; 0xd4],
    #[doc = "0x44100 - AXI interconnect - INI x read QoS register"]
    pub axi_ini3_read_qos: AXI_INI3_READ_QOS,
    #[doc = "0x44104 - AXI interconnect - INI x write QoS register"]
    pub axi_ini3_write_qos: AXI_INI3_WRITE_QOS,
    #[doc = "0x44108 - AXI interconnect - INI x issuing functionality modification register"]
    pub axi_ini3_fn_mod: AXI_INI3_FN_MOD,
    _reserved37: [u8; 0x0ff4],
    #[doc = "0x45100 - AXI interconnect - INI x read QoS register"]
    pub axi_ini4_read_qos: AXI_INI4_READ_QOS,
    #[doc = "0x45104 - AXI interconnect - INI x write QoS register"]
    pub axi_ini4_write_qos: AXI_INI4_WRITE_QOS,
    #[doc = "0x45108 - AXI interconnect - INI x issuing functionality modification register"]
    pub axi_ini4_fn_mod: AXI_INI4_FN_MOD,
    _reserved40: [u8; 0x0ff4],
    #[doc = "0x46100 - AXI interconnect - INI x read QoS register"]
    pub axi_ini5_read_qos: AXI_INI5_READ_QOS,
    #[doc = "0x46104 - AXI interconnect - INI x write QoS register"]
    pub axi_ini5_write_qos: AXI_INI5_WRITE_QOS,
    #[doc = "0x46108 - AXI interconnect - INI x issuing functionality modification register"]
    pub axi_ini5_fn_mod: AXI_INI5_FN_MOD,
    _reserved43: [u8; 0x0ff4],
    #[doc = "0x47100 - AXI interconnect - INI x read QoS register"]
    pub axi_ini6_read_qos: AXI_INI6_READ_QOS,
    #[doc = "0x47104 - AXI interconnect - INI x write QoS register"]
    pub axi_ini6_write_qos: AXI_INI6_WRITE_QOS,
    #[doc = "0x47108 - AXI interconnect - INI x issuing functionality modification register"]
    pub axi_ini6_fn_mod: AXI_INI6_FN_MOD,
}
#[doc = "AXI_PERIPH_ID_4 (r) register accessor: AXI interconnect - peripheral ID4 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`axi_periph_id_4::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`axi_periph_id_4`]
module"]
pub type AXI_PERIPH_ID_4 = crate::Reg<axi_periph_id_4::AXI_PERIPH_ID_4_SPEC>;
#[doc = "AXI interconnect - peripheral ID4 register"]
pub mod axi_periph_id_4;
#[doc = "AXI_PERIPH_ID_0 (r) register accessor: AXI interconnect - peripheral ID0 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`axi_periph_id_0::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`axi_periph_id_0`]
module"]
pub type AXI_PERIPH_ID_0 = crate::Reg<axi_periph_id_0::AXI_PERIPH_ID_0_SPEC>;
#[doc = "AXI interconnect - peripheral ID0 register"]
pub mod axi_periph_id_0;
#[doc = "AXI_PERIPH_ID_1 (r) register accessor: AXI interconnect - peripheral ID1 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`axi_periph_id_1::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`axi_periph_id_1`]
module"]
pub type AXI_PERIPH_ID_1 = crate::Reg<axi_periph_id_1::AXI_PERIPH_ID_1_SPEC>;
#[doc = "AXI interconnect - peripheral ID1 register"]
pub mod axi_periph_id_1;
#[doc = "AXI_PERIPH_ID_2 (r) register accessor: AXI interconnect - peripheral ID2 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`axi_periph_id_2::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`axi_periph_id_2`]
module"]
pub type AXI_PERIPH_ID_2 = crate::Reg<axi_periph_id_2::AXI_PERIPH_ID_2_SPEC>;
#[doc = "AXI interconnect - peripheral ID2 register"]
pub mod axi_periph_id_2;
#[doc = "AXI_PERIPH_ID_3 (r) register accessor: AXI interconnect - peripheral ID3 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`axi_periph_id_3::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`axi_periph_id_3`]
module"]
pub type AXI_PERIPH_ID_3 = crate::Reg<axi_periph_id_3::AXI_PERIPH_ID_3_SPEC>;
#[doc = "AXI interconnect - peripheral ID3 register"]
pub mod axi_periph_id_3;
#[doc = "AXI_COMP_ID_0 (r) register accessor: AXI interconnect - component ID0 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`axi_comp_id_0::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`axi_comp_id_0`]
module"]
pub type AXI_COMP_ID_0 = crate::Reg<axi_comp_id_0::AXI_COMP_ID_0_SPEC>;
#[doc = "AXI interconnect - component ID0 register"]
pub mod axi_comp_id_0;
#[doc = "AXI_COMP_ID_1 (r) register accessor: AXI interconnect - component ID1 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`axi_comp_id_1::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`axi_comp_id_1`]
module"]
pub type AXI_COMP_ID_1 = crate::Reg<axi_comp_id_1::AXI_COMP_ID_1_SPEC>;
#[doc = "AXI interconnect - component ID1 register"]
pub mod axi_comp_id_1;
#[doc = "AXI_COMP_ID_2 (r) register accessor: AXI interconnect - component ID2 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`axi_comp_id_2::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`axi_comp_id_2`]
module"]
pub type AXI_COMP_ID_2 = crate::Reg<axi_comp_id_2::AXI_COMP_ID_2_SPEC>;
#[doc = "AXI interconnect - component ID2 register"]
pub mod axi_comp_id_2;
#[doc = "AXI_COMP_ID_3 (r) register accessor: AXI interconnect - component ID3 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`axi_comp_id_3::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`axi_comp_id_3`]
module"]
pub type AXI_COMP_ID_3 = crate::Reg<axi_comp_id_3::AXI_COMP_ID_3_SPEC>;
#[doc = "AXI interconnect - component ID3 register"]
pub mod axi_comp_id_3;
#[doc = "AXI_TARG1_FN_MOD_ISS_BM (rw) register accessor: AXI interconnect - TARG x bus matrix issuing functionality register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`axi_targ1_fn_mod_iss_bm::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`axi_targ1_fn_mod_iss_bm::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`axi_targ1_fn_mod_iss_bm`]
module"]
pub type AXI_TARG1_FN_MOD_ISS_BM =
    crate::Reg<axi_targ1_fn_mod_iss_bm::AXI_TARG1_FN_MOD_ISS_BM_SPEC>;
#[doc = "AXI interconnect - TARG x bus matrix issuing functionality register"]
pub mod axi_targ1_fn_mod_iss_bm;
#[doc = "AXI_TARG2_FN_MOD_ISS_BM (rw) register accessor: AXI interconnect - TARG x bus matrix issuing functionality register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`axi_targ2_fn_mod_iss_bm::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`axi_targ2_fn_mod_iss_bm::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`axi_targ2_fn_mod_iss_bm`]
module"]
pub type AXI_TARG2_FN_MOD_ISS_BM =
    crate::Reg<axi_targ2_fn_mod_iss_bm::AXI_TARG2_FN_MOD_ISS_BM_SPEC>;
#[doc = "AXI interconnect - TARG x bus matrix issuing functionality register"]
pub mod axi_targ2_fn_mod_iss_bm;
#[doc = "AXI_TARG3_FN_MOD_ISS_BM (rw) register accessor: AXI interconnect - TARG x bus matrix issuing functionality register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`axi_targ3_fn_mod_iss_bm::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`axi_targ3_fn_mod_iss_bm::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`axi_targ3_fn_mod_iss_bm`]
module"]
pub type AXI_TARG3_FN_MOD_ISS_BM =
    crate::Reg<axi_targ3_fn_mod_iss_bm::AXI_TARG3_FN_MOD_ISS_BM_SPEC>;
#[doc = "AXI interconnect - TARG x bus matrix issuing functionality register"]
pub mod axi_targ3_fn_mod_iss_bm;
#[doc = "AXI_TARG4_FN_MOD_ISS_BM (rw) register accessor: AXI interconnect - TARG x bus matrix issuing functionality register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`axi_targ4_fn_mod_iss_bm::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`axi_targ4_fn_mod_iss_bm::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`axi_targ4_fn_mod_iss_bm`]
module"]
pub type AXI_TARG4_FN_MOD_ISS_BM =
    crate::Reg<axi_targ4_fn_mod_iss_bm::AXI_TARG4_FN_MOD_ISS_BM_SPEC>;
#[doc = "AXI interconnect - TARG x bus matrix issuing functionality register"]
pub mod axi_targ4_fn_mod_iss_bm;
#[doc = "AXI_TARG5_FN_MOD_ISS_BM (rw) register accessor: AXI interconnect - TARG x bus matrix issuing functionality register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`axi_targ5_fn_mod_iss_bm::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`axi_targ5_fn_mod_iss_bm::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`axi_targ5_fn_mod_iss_bm`]
module"]
pub type AXI_TARG5_FN_MOD_ISS_BM =
    crate::Reg<axi_targ5_fn_mod_iss_bm::AXI_TARG5_FN_MOD_ISS_BM_SPEC>;
#[doc = "AXI interconnect - TARG x bus matrix issuing functionality register"]
pub mod axi_targ5_fn_mod_iss_bm;
#[doc = "AXI_TARG6_FN_MOD_ISS_BM (rw) register accessor: AXI interconnect - TARG x bus matrix issuing functionality register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`axi_targ6_fn_mod_iss_bm::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`axi_targ6_fn_mod_iss_bm::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`axi_targ6_fn_mod_iss_bm`]
module"]
pub type AXI_TARG6_FN_MOD_ISS_BM =
    crate::Reg<axi_targ6_fn_mod_iss_bm::AXI_TARG6_FN_MOD_ISS_BM_SPEC>;
#[doc = "AXI interconnect - TARG x bus matrix issuing functionality register"]
pub mod axi_targ6_fn_mod_iss_bm;
#[doc = "AXI_TARG7_FN_MOD_ISS_BM (rw) register accessor: AXI interconnect - TARG x bus matrix issuing functionality register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`axi_targ7_fn_mod_iss_bm::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`axi_targ7_fn_mod_iss_bm::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`axi_targ7_fn_mod_iss_bm`]
module"]
pub type AXI_TARG7_FN_MOD_ISS_BM =
    crate::Reg<axi_targ7_fn_mod_iss_bm::AXI_TARG7_FN_MOD_ISS_BM_SPEC>;
#[doc = "AXI interconnect - TARG x bus matrix issuing functionality register"]
pub mod axi_targ7_fn_mod_iss_bm;
#[doc = "AXI_TARG1_FN_MOD2 (rw) register accessor: AXI interconnect - TARG x bus matrix functionality 2 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`axi_targ1_fn_mod2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`axi_targ1_fn_mod2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`axi_targ1_fn_mod2`]
module"]
pub type AXI_TARG1_FN_MOD2 = crate::Reg<axi_targ1_fn_mod2::AXI_TARG1_FN_MOD2_SPEC>;
#[doc = "AXI interconnect - TARG x bus matrix functionality 2 register"]
pub mod axi_targ1_fn_mod2;
#[doc = "AXI_TARG2_FN_MOD2 (rw) register accessor: AXI interconnect - TARG x bus matrix functionality 2 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`axi_targ2_fn_mod2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`axi_targ2_fn_mod2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`axi_targ2_fn_mod2`]
module"]
pub type AXI_TARG2_FN_MOD2 = crate::Reg<axi_targ2_fn_mod2::AXI_TARG2_FN_MOD2_SPEC>;
#[doc = "AXI interconnect - TARG x bus matrix functionality 2 register"]
pub mod axi_targ2_fn_mod2;
#[doc = "AXI_TARG7_FN_MOD2 (rw) register accessor: AXI interconnect - TARG x bus matrix functionality 2 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`axi_targ7_fn_mod2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`axi_targ7_fn_mod2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`axi_targ7_fn_mod2`]
module"]
pub type AXI_TARG7_FN_MOD2 = crate::Reg<axi_targ7_fn_mod2::AXI_TARG7_FN_MOD2_SPEC>;
#[doc = "AXI interconnect - TARG x bus matrix functionality 2 register"]
pub mod axi_targ7_fn_mod2;
#[doc = "AXI_TARG1_FN_MOD_LB (rw) register accessor: AXI interconnect - TARG x long burst functionality modification\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`axi_targ1_fn_mod_lb::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`axi_targ1_fn_mod_lb::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`axi_targ1_fn_mod_lb`]
module"]
pub type AXI_TARG1_FN_MOD_LB = crate::Reg<axi_targ1_fn_mod_lb::AXI_TARG1_FN_MOD_LB_SPEC>;
#[doc = "AXI interconnect - TARG x long burst functionality modification"]
pub mod axi_targ1_fn_mod_lb;
#[doc = "AXI_TARG2_FN_MOD_LB (rw) register accessor: AXI interconnect - TARG x long burst functionality modification\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`axi_targ2_fn_mod_lb::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`axi_targ2_fn_mod_lb::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`axi_targ2_fn_mod_lb`]
module"]
pub type AXI_TARG2_FN_MOD_LB = crate::Reg<axi_targ2_fn_mod_lb::AXI_TARG2_FN_MOD_LB_SPEC>;
#[doc = "AXI interconnect - TARG x long burst functionality modification"]
pub mod axi_targ2_fn_mod_lb;
#[doc = "AXI_TARG1_FN_MOD (rw) register accessor: AXI interconnect - TARG x long burst functionality modification\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`axi_targ1_fn_mod::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`axi_targ1_fn_mod::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`axi_targ1_fn_mod`]
module"]
pub type AXI_TARG1_FN_MOD = crate::Reg<axi_targ1_fn_mod::AXI_TARG1_FN_MOD_SPEC>;
#[doc = "AXI interconnect - TARG x long burst functionality modification"]
pub mod axi_targ1_fn_mod;
#[doc = "AXI_TARG2_FN_MOD (rw) register accessor: AXI interconnect - TARG x long burst functionality modification\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`axi_targ2_fn_mod::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`axi_targ2_fn_mod::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`axi_targ2_fn_mod`]
module"]
pub type AXI_TARG2_FN_MOD = crate::Reg<axi_targ2_fn_mod::AXI_TARG2_FN_MOD_SPEC>;
#[doc = "AXI interconnect - TARG x long burst functionality modification"]
pub mod axi_targ2_fn_mod;
#[doc = "AXI_TARG7_FN_MOD (rw) register accessor: AXI interconnect - TARG x long burst functionality modification\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`axi_targ7_fn_mod::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`axi_targ7_fn_mod::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`axi_targ7_fn_mod`]
module"]
pub type AXI_TARG7_FN_MOD = crate::Reg<axi_targ7_fn_mod::AXI_TARG7_FN_MOD_SPEC>;
#[doc = "AXI interconnect - TARG x long burst functionality modification"]
pub mod axi_targ7_fn_mod;
#[doc = "AXI_INI1_FN_MOD2 (rw) register accessor: AXI interconnect - INI x functionality modification 2 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`axi_ini1_fn_mod2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`axi_ini1_fn_mod2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`axi_ini1_fn_mod2`]
module"]
pub type AXI_INI1_FN_MOD2 = crate::Reg<axi_ini1_fn_mod2::AXI_INI1_FN_MOD2_SPEC>;
#[doc = "AXI interconnect - INI x functionality modification 2 register"]
pub mod axi_ini1_fn_mod2;
#[doc = "AXI_INI3_FN_MOD2 (rw) register accessor: AXI interconnect - INI x functionality modification 2 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`axi_ini3_fn_mod2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`axi_ini3_fn_mod2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`axi_ini3_fn_mod2`]
module"]
pub type AXI_INI3_FN_MOD2 = crate::Reg<axi_ini3_fn_mod2::AXI_INI3_FN_MOD2_SPEC>;
#[doc = "AXI interconnect - INI x functionality modification 2 register"]
pub mod axi_ini3_fn_mod2;
#[doc = "AXI_INI1_FN_MOD_AHB (rw) register accessor: AXI interconnect - INI x AHB functionality modification register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`axi_ini1_fn_mod_ahb::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`axi_ini1_fn_mod_ahb::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`axi_ini1_fn_mod_ahb`]
module"]
pub type AXI_INI1_FN_MOD_AHB = crate::Reg<axi_ini1_fn_mod_ahb::AXI_INI1_FN_MOD_AHB_SPEC>;
#[doc = "AXI interconnect - INI x AHB functionality modification register"]
pub mod axi_ini1_fn_mod_ahb;
#[doc = "AXI_INI3_FN_MOD_AHB (rw) register accessor: AXI interconnect - INI x AHB functionality modification register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`axi_ini3_fn_mod_ahb::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`axi_ini3_fn_mod_ahb::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`axi_ini3_fn_mod_ahb`]
module"]
pub type AXI_INI3_FN_MOD_AHB = crate::Reg<axi_ini3_fn_mod_ahb::AXI_INI3_FN_MOD_AHB_SPEC>;
#[doc = "AXI interconnect - INI x AHB functionality modification register"]
pub mod axi_ini3_fn_mod_ahb;
#[doc = "AXI_INI1_READ_QOS (rw) register accessor: AXI interconnect - INI x read QoS register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`axi_ini1_read_qos::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`axi_ini1_read_qos::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`axi_ini1_read_qos`]
module"]
pub type AXI_INI1_READ_QOS = crate::Reg<axi_ini1_read_qos::AXI_INI1_READ_QOS_SPEC>;
#[doc = "AXI interconnect - INI x read QoS register"]
pub mod axi_ini1_read_qos;
#[doc = "AXI_INI2_READ_QOS (rw) register accessor: AXI interconnect - INI x read QoS register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`axi_ini2_read_qos::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`axi_ini2_read_qos::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`axi_ini2_read_qos`]
module"]
pub type AXI_INI2_READ_QOS = crate::Reg<axi_ini2_read_qos::AXI_INI2_READ_QOS_SPEC>;
#[doc = "AXI interconnect - INI x read QoS register"]
pub mod axi_ini2_read_qos;
#[doc = "AXI_INI3_READ_QOS (rw) register accessor: AXI interconnect - INI x read QoS register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`axi_ini3_read_qos::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`axi_ini3_read_qos::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`axi_ini3_read_qos`]
module"]
pub type AXI_INI3_READ_QOS = crate::Reg<axi_ini3_read_qos::AXI_INI3_READ_QOS_SPEC>;
#[doc = "AXI interconnect - INI x read QoS register"]
pub mod axi_ini3_read_qos;
#[doc = "AXI_INI4_READ_QOS (rw) register accessor: AXI interconnect - INI x read QoS register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`axi_ini4_read_qos::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`axi_ini4_read_qos::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`axi_ini4_read_qos`]
module"]
pub type AXI_INI4_READ_QOS = crate::Reg<axi_ini4_read_qos::AXI_INI4_READ_QOS_SPEC>;
#[doc = "AXI interconnect - INI x read QoS register"]
pub mod axi_ini4_read_qos;
#[doc = "AXI_INI5_READ_QOS (rw) register accessor: AXI interconnect - INI x read QoS register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`axi_ini5_read_qos::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`axi_ini5_read_qos::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`axi_ini5_read_qos`]
module"]
pub type AXI_INI5_READ_QOS = crate::Reg<axi_ini5_read_qos::AXI_INI5_READ_QOS_SPEC>;
#[doc = "AXI interconnect - INI x read QoS register"]
pub mod axi_ini5_read_qos;
#[doc = "AXI_INI6_READ_QOS (rw) register accessor: AXI interconnect - INI x read QoS register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`axi_ini6_read_qos::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`axi_ini6_read_qos::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`axi_ini6_read_qos`]
module"]
pub type AXI_INI6_READ_QOS = crate::Reg<axi_ini6_read_qos::AXI_INI6_READ_QOS_SPEC>;
#[doc = "AXI interconnect - INI x read QoS register"]
pub mod axi_ini6_read_qos;
#[doc = "AXI_INI1_WRITE_QOS (rw) register accessor: AXI interconnect - INI x write QoS register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`axi_ini1_write_qos::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`axi_ini1_write_qos::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`axi_ini1_write_qos`]
module"]
pub type AXI_INI1_WRITE_QOS = crate::Reg<axi_ini1_write_qos::AXI_INI1_WRITE_QOS_SPEC>;
#[doc = "AXI interconnect - INI x write QoS register"]
pub mod axi_ini1_write_qos;
#[doc = "AXI_INI2_WRITE_QOS (rw) register accessor: AXI interconnect - INI x write QoS register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`axi_ini2_write_qos::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`axi_ini2_write_qos::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`axi_ini2_write_qos`]
module"]
pub type AXI_INI2_WRITE_QOS = crate::Reg<axi_ini2_write_qos::AXI_INI2_WRITE_QOS_SPEC>;
#[doc = "AXI interconnect - INI x write QoS register"]
pub mod axi_ini2_write_qos;
#[doc = "AXI_INI3_WRITE_QOS (rw) register accessor: AXI interconnect - INI x write QoS register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`axi_ini3_write_qos::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`axi_ini3_write_qos::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`axi_ini3_write_qos`]
module"]
pub type AXI_INI3_WRITE_QOS = crate::Reg<axi_ini3_write_qos::AXI_INI3_WRITE_QOS_SPEC>;
#[doc = "AXI interconnect - INI x write QoS register"]
pub mod axi_ini3_write_qos;
#[doc = "AXI_INI4_WRITE_QOS (rw) register accessor: AXI interconnect - INI x write QoS register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`axi_ini4_write_qos::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`axi_ini4_write_qos::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`axi_ini4_write_qos`]
module"]
pub type AXI_INI4_WRITE_QOS = crate::Reg<axi_ini4_write_qos::AXI_INI4_WRITE_QOS_SPEC>;
#[doc = "AXI interconnect - INI x write QoS register"]
pub mod axi_ini4_write_qos;
#[doc = "AXI_INI5_WRITE_QOS (rw) register accessor: AXI interconnect - INI x write QoS register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`axi_ini5_write_qos::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`axi_ini5_write_qos::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`axi_ini5_write_qos`]
module"]
pub type AXI_INI5_WRITE_QOS = crate::Reg<axi_ini5_write_qos::AXI_INI5_WRITE_QOS_SPEC>;
#[doc = "AXI interconnect - INI x write QoS register"]
pub mod axi_ini5_write_qos;
#[doc = "AXI_INI6_WRITE_QOS (rw) register accessor: AXI interconnect - INI x write QoS register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`axi_ini6_write_qos::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`axi_ini6_write_qos::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`axi_ini6_write_qos`]
module"]
pub type AXI_INI6_WRITE_QOS = crate::Reg<axi_ini6_write_qos::AXI_INI6_WRITE_QOS_SPEC>;
#[doc = "AXI interconnect - INI x write QoS register"]
pub mod axi_ini6_write_qos;
#[doc = "AXI_INI1_FN_MOD (rw) register accessor: AXI interconnect - INI x issuing functionality modification register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`axi_ini1_fn_mod::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`axi_ini1_fn_mod::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`axi_ini1_fn_mod`]
module"]
pub type AXI_INI1_FN_MOD = crate::Reg<axi_ini1_fn_mod::AXI_INI1_FN_MOD_SPEC>;
#[doc = "AXI interconnect - INI x issuing functionality modification register"]
pub mod axi_ini1_fn_mod;
#[doc = "AXI_INI2_FN_MOD (rw) register accessor: AXI interconnect - INI x issuing functionality modification register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`axi_ini2_fn_mod::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`axi_ini2_fn_mod::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`axi_ini2_fn_mod`]
module"]
pub type AXI_INI2_FN_MOD = crate::Reg<axi_ini2_fn_mod::AXI_INI2_FN_MOD_SPEC>;
#[doc = "AXI interconnect - INI x issuing functionality modification register"]
pub mod axi_ini2_fn_mod;
#[doc = "AXI_INI3_FN_MOD (rw) register accessor: AXI interconnect - INI x issuing functionality modification register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`axi_ini3_fn_mod::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`axi_ini3_fn_mod::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`axi_ini3_fn_mod`]
module"]
pub type AXI_INI3_FN_MOD = crate::Reg<axi_ini3_fn_mod::AXI_INI3_FN_MOD_SPEC>;
#[doc = "AXI interconnect - INI x issuing functionality modification register"]
pub mod axi_ini3_fn_mod;
#[doc = "AXI_INI4_FN_MOD (rw) register accessor: AXI interconnect - INI x issuing functionality modification register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`axi_ini4_fn_mod::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`axi_ini4_fn_mod::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`axi_ini4_fn_mod`]
module"]
pub type AXI_INI4_FN_MOD = crate::Reg<axi_ini4_fn_mod::AXI_INI4_FN_MOD_SPEC>;
#[doc = "AXI interconnect - INI x issuing functionality modification register"]
pub mod axi_ini4_fn_mod;
#[doc = "AXI_INI5_FN_MOD (rw) register accessor: AXI interconnect - INI x issuing functionality modification register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`axi_ini5_fn_mod::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`axi_ini5_fn_mod::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`axi_ini5_fn_mod`]
module"]
pub type AXI_INI5_FN_MOD = crate::Reg<axi_ini5_fn_mod::AXI_INI5_FN_MOD_SPEC>;
#[doc = "AXI interconnect - INI x issuing functionality modification register"]
pub mod axi_ini5_fn_mod;
#[doc = "AXI_INI6_FN_MOD (rw) register accessor: AXI interconnect - INI x issuing functionality modification register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`axi_ini6_fn_mod::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`axi_ini6_fn_mod::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`axi_ini6_fn_mod`]
module"]
pub type AXI_INI6_FN_MOD = crate::Reg<axi_ini6_fn_mod::AXI_INI6_FN_MOD_SPEC>;
#[doc = "AXI interconnect - INI x issuing functionality modification register"]
pub mod axi_ini6_fn_mod;
