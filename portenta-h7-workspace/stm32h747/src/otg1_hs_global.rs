#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - OTG_HS control and status register"]
    pub otg_hs_gotgctl: OTG_HS_GOTGCTL,
    #[doc = "0x04 - OTG_HS interrupt register"]
    pub otg_hs_gotgint: OTG_HS_GOTGINT,
    #[doc = "0x08 - OTG_HS AHB configuration register"]
    pub otg_hs_gahbcfg: OTG_HS_GAHBCFG,
    #[doc = "0x0c - OTG_HS USB configuration register"]
    pub otg_hs_gusbcfg: OTG_HS_GUSBCFG,
    #[doc = "0x10 - OTG_HS reset register"]
    pub otg_hs_grstctl: OTG_HS_GRSTCTL,
    #[doc = "0x14 - OTG_HS core interrupt register"]
    pub otg_hs_gintsts: OTG_HS_GINTSTS,
    #[doc = "0x18 - OTG_HS interrupt mask register"]
    pub otg_hs_gintmsk: OTG_HS_GINTMSK,
    _reserved_7_otg_hs_grxstsr: [u8; 0x04],
    _reserved_8_otg_hs_grxstsp: [u8; 0x04],
    #[doc = "0x24 - OTG_HS Receive FIFO size register"]
    pub otg_hs_grxfsiz: OTG_HS_GRXFSIZ,
    _reserved_10_otg_hs: [u8; 0x04],
    #[doc = "0x2c - OTG_HS nonperiodic transmit FIFO/queue status register"]
    pub otg_hs_gnptxsts: OTG_HS_GNPTXSTS,
    _reserved12: [u8; 0x08],
    #[doc = "0x38 - OTG_HS general core configuration register"]
    pub otg_hs_gccfg: OTG_HS_GCCFG,
    #[doc = "0x3c - OTG_HS core ID register"]
    pub otg_hs_cid: OTG_HS_CID,
    _reserved14: [u8; 0x14],
    #[doc = "0x54 - OTG core LPM configuration register"]
    pub otg_hs_glpmcfg: OTG_HS_GLPMCFG,
    _reserved15: [u8; 0xa8],
    #[doc = "0x100 - OTG_HS Host periodic transmit FIFO size register"]
    pub otg_hs_hptxfsiz: OTG_HS_HPTXFSIZ,
    #[doc = "0x104 - OTG_HS device IN endpoint transmit FIFO size register"]
    pub otg_hs_dieptxf1: OTG_HS_DIEPTXF1,
    #[doc = "0x108 - OTG_HS device IN endpoint transmit FIFO size register"]
    pub otg_hs_dieptxf2: OTG_HS_DIEPTXF2,
    _reserved18: [u8; 0x10],
    #[doc = "0x11c - OTG_HS device IN endpoint transmit FIFO size register"]
    pub otg_hs_dieptxf3: OTG_HS_DIEPTXF3,
    #[doc = "0x120 - OTG_HS device IN endpoint transmit FIFO size register"]
    pub otg_hs_dieptxf4: OTG_HS_DIEPTXF4,
    #[doc = "0x124 - OTG_HS device IN endpoint transmit FIFO size register"]
    pub otg_hs_dieptxf5: OTG_HS_DIEPTXF5,
    #[doc = "0x128 - OTG_HS device IN endpoint transmit FIFO size register"]
    pub otg_hs_dieptxf6: OTG_HS_DIEPTXF6,
    #[doc = "0x12c - OTG_HS device IN endpoint transmit FIFO size register"]
    pub otg_hs_dieptxf7: OTG_HS_DIEPTXF7,
}
impl RegisterBlock {
    #[doc = "0x1c - OTG_HS Receive status debug read register (peripheral mode mode)"]
    #[inline(always)]
    pub const fn otg_hs_grxstsr_device(&self) -> &OTG_HS_GRXSTSR_DEVICE {
        unsafe { &*(self as *const Self).cast::<u8>().add(28usize).cast() }
    }
    #[doc = "0x1c - OTG_HS Receive status debug read register (host mode)"]
    #[inline(always)]
    pub const fn otg_hs_grxstsr_host(&self) -> &OTG_HS_GRXSTSR_HOST {
        unsafe { &*(self as *const Self).cast::<u8>().add(28usize).cast() }
    }
    #[doc = "0x20 - OTG_HS status read and pop register (peripheral mode)"]
    #[inline(always)]
    pub const fn otg_hs_grxstsp_device(&self) -> &OTG_HS_GRXSTSP_DEVICE {
        unsafe { &*(self as *const Self).cast::<u8>().add(32usize).cast() }
    }
    #[doc = "0x20 - OTG_HS status read and pop register (host mode)"]
    #[inline(always)]
    pub const fn otg_hs_grxstsp_host(&self) -> &OTG_HS_GRXSTSP_HOST {
        unsafe { &*(self as *const Self).cast::<u8>().add(32usize).cast() }
    }
    #[doc = "0x28 - Endpoint 0 transmit FIFO size (peripheral mode)"]
    #[inline(always)]
    pub const fn otg_hs_dieptxf0_device(&self) -> &OTG_HS_DIEPTXF0_DEVICE {
        unsafe { &*(self as *const Self).cast::<u8>().add(40usize).cast() }
    }
    #[doc = "0x28 - OTG_HS nonperiodic transmit FIFO size register (host mode)"]
    #[inline(always)]
    pub const fn otg_hs_hnptxfsiz_host(&self) -> &OTG_HS_HNPTXFSIZ_HOST {
        unsafe { &*(self as *const Self).cast::<u8>().add(40usize).cast() }
    }
}
#[doc = "OTG_HS_GOTGCTL (rw) register accessor: OTG_HS control and status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`otg_hs_gotgctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`otg_hs_gotgctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`otg_hs_gotgctl`]
module"]
pub type OTG_HS_GOTGCTL = crate::Reg<otg_hs_gotgctl::OTG_HS_GOTGCTL_SPEC>;
#[doc = "OTG_HS control and status register"]
pub mod otg_hs_gotgctl;
#[doc = "OTG_HS_GOTGINT (rw) register accessor: OTG_HS interrupt register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`otg_hs_gotgint::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`otg_hs_gotgint::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`otg_hs_gotgint`]
module"]
pub type OTG_HS_GOTGINT = crate::Reg<otg_hs_gotgint::OTG_HS_GOTGINT_SPEC>;
#[doc = "OTG_HS interrupt register"]
pub mod otg_hs_gotgint;
#[doc = "OTG_HS_GAHBCFG (rw) register accessor: OTG_HS AHB configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`otg_hs_gahbcfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`otg_hs_gahbcfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`otg_hs_gahbcfg`]
module"]
pub type OTG_HS_GAHBCFG = crate::Reg<otg_hs_gahbcfg::OTG_HS_GAHBCFG_SPEC>;
#[doc = "OTG_HS AHB configuration register"]
pub mod otg_hs_gahbcfg;
#[doc = "OTG_HS_GUSBCFG (rw) register accessor: OTG_HS USB configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`otg_hs_gusbcfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`otg_hs_gusbcfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`otg_hs_gusbcfg`]
module"]
pub type OTG_HS_GUSBCFG = crate::Reg<otg_hs_gusbcfg::OTG_HS_GUSBCFG_SPEC>;
#[doc = "OTG_HS USB configuration register"]
pub mod otg_hs_gusbcfg;
#[doc = "OTG_HS_GRSTCTL (rw) register accessor: OTG_HS reset register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`otg_hs_grstctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`otg_hs_grstctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`otg_hs_grstctl`]
module"]
pub type OTG_HS_GRSTCTL = crate::Reg<otg_hs_grstctl::OTG_HS_GRSTCTL_SPEC>;
#[doc = "OTG_HS reset register"]
pub mod otg_hs_grstctl;
#[doc = "OTG_HS_GINTSTS (rw) register accessor: OTG_HS core interrupt register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`otg_hs_gintsts::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`otg_hs_gintsts::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`otg_hs_gintsts`]
module"]
pub type OTG_HS_GINTSTS = crate::Reg<otg_hs_gintsts::OTG_HS_GINTSTS_SPEC>;
#[doc = "OTG_HS core interrupt register"]
pub mod otg_hs_gintsts;
#[doc = "OTG_HS_GINTMSK (rw) register accessor: OTG_HS interrupt mask register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`otg_hs_gintmsk::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`otg_hs_gintmsk::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`otg_hs_gintmsk`]
module"]
pub type OTG_HS_GINTMSK = crate::Reg<otg_hs_gintmsk::OTG_HS_GINTMSK_SPEC>;
#[doc = "OTG_HS interrupt mask register"]
pub mod otg_hs_gintmsk;
#[doc = "OTG_HS_GRXSTSR_Host (r) register accessor: OTG_HS Receive status debug read register (host mode)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`otg_hs_grxstsr_host::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`otg_hs_grxstsr_host`]
module"]
pub type OTG_HS_GRXSTSR_HOST = crate::Reg<otg_hs_grxstsr_host::OTG_HS_GRXSTSR_HOST_SPEC>;
#[doc = "OTG_HS Receive status debug read register (host mode)"]
pub mod otg_hs_grxstsr_host;
#[doc = "OTG_HS_GRXSTSP_Host (r) register accessor: OTG_HS status read and pop register (host mode)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`otg_hs_grxstsp_host::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`otg_hs_grxstsp_host`]
module"]
pub type OTG_HS_GRXSTSP_HOST = crate::Reg<otg_hs_grxstsp_host::OTG_HS_GRXSTSP_HOST_SPEC>;
#[doc = "OTG_HS status read and pop register (host mode)"]
pub mod otg_hs_grxstsp_host;
#[doc = "OTG_HS_GRXFSIZ (rw) register accessor: OTG_HS Receive FIFO size register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`otg_hs_grxfsiz::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`otg_hs_grxfsiz::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`otg_hs_grxfsiz`]
module"]
pub type OTG_HS_GRXFSIZ = crate::Reg<otg_hs_grxfsiz::OTG_HS_GRXFSIZ_SPEC>;
#[doc = "OTG_HS Receive FIFO size register"]
pub mod otg_hs_grxfsiz;
#[doc = "OTG_HS_HNPTXFSIZ_Host (rw) register accessor: OTG_HS nonperiodic transmit FIFO size register (host mode)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`otg_hs_hnptxfsiz_host::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`otg_hs_hnptxfsiz_host::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`otg_hs_hnptxfsiz_host`]
module"]
pub type OTG_HS_HNPTXFSIZ_HOST = crate::Reg<otg_hs_hnptxfsiz_host::OTG_HS_HNPTXFSIZ_HOST_SPEC>;
#[doc = "OTG_HS nonperiodic transmit FIFO size register (host mode)"]
pub mod otg_hs_hnptxfsiz_host;
#[doc = "OTG_HS_DIEPTXF0_Device (rw) register accessor: Endpoint 0 transmit FIFO size (peripheral mode)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`otg_hs_dieptxf0_device::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`otg_hs_dieptxf0_device::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`otg_hs_dieptxf0_device`]
module"]
pub type OTG_HS_DIEPTXF0_DEVICE = crate::Reg<otg_hs_dieptxf0_device::OTG_HS_DIEPTXF0_DEVICE_SPEC>;
#[doc = "Endpoint 0 transmit FIFO size (peripheral mode)"]
pub mod otg_hs_dieptxf0_device;
#[doc = "OTG_HS_GNPTXSTS (r) register accessor: OTG_HS nonperiodic transmit FIFO/queue status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`otg_hs_gnptxsts::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`otg_hs_gnptxsts`]
module"]
pub type OTG_HS_GNPTXSTS = crate::Reg<otg_hs_gnptxsts::OTG_HS_GNPTXSTS_SPEC>;
#[doc = "OTG_HS nonperiodic transmit FIFO/queue status register"]
pub mod otg_hs_gnptxsts;
#[doc = "OTG_HS_GCCFG (rw) register accessor: OTG_HS general core configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`otg_hs_gccfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`otg_hs_gccfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`otg_hs_gccfg`]
module"]
pub type OTG_HS_GCCFG = crate::Reg<otg_hs_gccfg::OTG_HS_GCCFG_SPEC>;
#[doc = "OTG_HS general core configuration register"]
pub mod otg_hs_gccfg;
#[doc = "OTG_HS_CID (rw) register accessor: OTG_HS core ID register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`otg_hs_cid::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`otg_hs_cid::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`otg_hs_cid`]
module"]
pub type OTG_HS_CID = crate::Reg<otg_hs_cid::OTG_HS_CID_SPEC>;
#[doc = "OTG_HS core ID register"]
pub mod otg_hs_cid;
#[doc = "OTG_HS_HPTXFSIZ (rw) register accessor: OTG_HS Host periodic transmit FIFO size register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`otg_hs_hptxfsiz::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`otg_hs_hptxfsiz::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`otg_hs_hptxfsiz`]
module"]
pub type OTG_HS_HPTXFSIZ = crate::Reg<otg_hs_hptxfsiz::OTG_HS_HPTXFSIZ_SPEC>;
#[doc = "OTG_HS Host periodic transmit FIFO size register"]
pub mod otg_hs_hptxfsiz;
#[doc = "OTG_HS_DIEPTXF1 (rw) register accessor: OTG_HS device IN endpoint transmit FIFO size register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`otg_hs_dieptxf1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`otg_hs_dieptxf1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`otg_hs_dieptxf1`]
module"]
pub type OTG_HS_DIEPTXF1 = crate::Reg<otg_hs_dieptxf1::OTG_HS_DIEPTXF1_SPEC>;
#[doc = "OTG_HS device IN endpoint transmit FIFO size register"]
pub mod otg_hs_dieptxf1;
#[doc = "OTG_HS_DIEPTXF2 (rw) register accessor: OTG_HS device IN endpoint transmit FIFO size register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`otg_hs_dieptxf2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`otg_hs_dieptxf2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`otg_hs_dieptxf2`]
module"]
pub type OTG_HS_DIEPTXF2 = crate::Reg<otg_hs_dieptxf2::OTG_HS_DIEPTXF2_SPEC>;
#[doc = "OTG_HS device IN endpoint transmit FIFO size register"]
pub mod otg_hs_dieptxf2;
#[doc = "OTG_HS_DIEPTXF3 (rw) register accessor: OTG_HS device IN endpoint transmit FIFO size register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`otg_hs_dieptxf3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`otg_hs_dieptxf3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`otg_hs_dieptxf3`]
module"]
pub type OTG_HS_DIEPTXF3 = crate::Reg<otg_hs_dieptxf3::OTG_HS_DIEPTXF3_SPEC>;
#[doc = "OTG_HS device IN endpoint transmit FIFO size register"]
pub mod otg_hs_dieptxf3;
#[doc = "OTG_HS_DIEPTXF4 (rw) register accessor: OTG_HS device IN endpoint transmit FIFO size register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`otg_hs_dieptxf4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`otg_hs_dieptxf4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`otg_hs_dieptxf4`]
module"]
pub type OTG_HS_DIEPTXF4 = crate::Reg<otg_hs_dieptxf4::OTG_HS_DIEPTXF4_SPEC>;
#[doc = "OTG_HS device IN endpoint transmit FIFO size register"]
pub mod otg_hs_dieptxf4;
#[doc = "OTG_HS_DIEPTXF5 (rw) register accessor: OTG_HS device IN endpoint transmit FIFO size register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`otg_hs_dieptxf5::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`otg_hs_dieptxf5::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`otg_hs_dieptxf5`]
module"]
pub type OTG_HS_DIEPTXF5 = crate::Reg<otg_hs_dieptxf5::OTG_HS_DIEPTXF5_SPEC>;
#[doc = "OTG_HS device IN endpoint transmit FIFO size register"]
pub mod otg_hs_dieptxf5;
#[doc = "OTG_HS_DIEPTXF6 (rw) register accessor: OTG_HS device IN endpoint transmit FIFO size register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`otg_hs_dieptxf6::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`otg_hs_dieptxf6::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`otg_hs_dieptxf6`]
module"]
pub type OTG_HS_DIEPTXF6 = crate::Reg<otg_hs_dieptxf6::OTG_HS_DIEPTXF6_SPEC>;
#[doc = "OTG_HS device IN endpoint transmit FIFO size register"]
pub mod otg_hs_dieptxf6;
#[doc = "OTG_HS_DIEPTXF7 (rw) register accessor: OTG_HS device IN endpoint transmit FIFO size register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`otg_hs_dieptxf7::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`otg_hs_dieptxf7::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`otg_hs_dieptxf7`]
module"]
pub type OTG_HS_DIEPTXF7 = crate::Reg<otg_hs_dieptxf7::OTG_HS_DIEPTXF7_SPEC>;
#[doc = "OTG_HS device IN endpoint transmit FIFO size register"]
pub mod otg_hs_dieptxf7;
#[doc = "OTG_HS_GRXSTSR_Device (r) register accessor: OTG_HS Receive status debug read register (peripheral mode mode)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`otg_hs_grxstsr_device::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`otg_hs_grxstsr_device`]
module"]
pub type OTG_HS_GRXSTSR_DEVICE = crate::Reg<otg_hs_grxstsr_device::OTG_HS_GRXSTSR_DEVICE_SPEC>;
#[doc = "OTG_HS Receive status debug read register (peripheral mode mode)"]
pub mod otg_hs_grxstsr_device;
#[doc = "OTG_HS_GRXSTSP_Device (r) register accessor: OTG_HS status read and pop register (peripheral mode)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`otg_hs_grxstsp_device::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`otg_hs_grxstsp_device`]
module"]
pub type OTG_HS_GRXSTSP_DEVICE = crate::Reg<otg_hs_grxstsp_device::OTG_HS_GRXSTSP_DEVICE_SPEC>;
#[doc = "OTG_HS status read and pop register (peripheral mode)"]
pub mod otg_hs_grxstsp_device;
#[doc = "OTG_HS_GLPMCFG (rw) register accessor: OTG core LPM configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`otg_hs_glpmcfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`otg_hs_glpmcfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`otg_hs_glpmcfg`]
module"]
pub type OTG_HS_GLPMCFG = crate::Reg<otg_hs_glpmcfg::OTG_HS_GLPMCFG_SPEC>;
#[doc = "OTG core LPM configuration register"]
pub mod otg_hs_glpmcfg;
