#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - DSI Host version register"]
    pub dsi_vr: DSI_VR,
    #[doc = "0x04 - DSI Host control register"]
    pub dsi_cr: DSI_CR,
    #[doc = "0x08 - DSI Host clock control register"]
    pub dsi_ccr: DSI_CCR,
    #[doc = "0x0c - DSI Host LTDC VCID register"]
    pub dsi_lvcidr: DSI_LVCIDR,
    #[doc = "0x10 - DSI Host LTDC color coding register"]
    pub dsi_lcolcr: DSI_LCOLCR,
    #[doc = "0x14 - DSI Host LTDC polarity configuration register"]
    pub dsi_lpcr: DSI_LPCR,
    #[doc = "0x18 - DSI Host low-power mode configuration register"]
    pub dsi_lpmcr: DSI_LPMCR,
    _reserved7: [u8; 0x10],
    #[doc = "0x2c - DSI Host protocol configuration register"]
    pub dsi_pcr: DSI_PCR,
    #[doc = "0x30 - DSI Host generic VCID register"]
    pub dsi_gvcidr: DSI_GVCIDR,
    #[doc = "0x34 - DSI Host mode configuration register"]
    pub dsi_mcr: DSI_MCR,
    #[doc = "0x38 - DSI Host video mode configuration register"]
    pub dsi_vmcr: DSI_VMCR,
    #[doc = "0x3c - DSI Host video packet configuration register"]
    pub dsi_vpcr: DSI_VPCR,
    #[doc = "0x40 - DSI Host video chunks configuration register"]
    pub dsi_vccr: DSI_VCCR,
    #[doc = "0x44 - DSI Host video null packet configuration register"]
    pub dsi_vnpcr: DSI_VNPCR,
    #[doc = "0x48 - DSI Host video HSA configuration register"]
    pub dsi_vhsacr: DSI_VHSACR,
    #[doc = "0x4c - DSI Host video HBP configuration register"]
    pub dsi_vhbpcr: DSI_VHBPCR,
    #[doc = "0x50 - DSI Host video line configuration register"]
    pub dsi_vlcr: DSI_VLCR,
    #[doc = "0x54 - DSI Host video VSA configuration register"]
    pub dsi_vvsacr: DSI_VVSACR,
    #[doc = "0x58 - DSI Host video VBP configuration register"]
    pub dsi_vvbpcr: DSI_VVBPCR,
    #[doc = "0x5c - DSI Host video VFP configuration register"]
    pub dsi_vvfpcr: DSI_VVFPCR,
    #[doc = "0x60 - DSI Host video VA configuration register"]
    pub dsi_vvacr: DSI_VVACR,
    #[doc = "0x64 - DSI Host LTDC command configuration register"]
    pub dsi_lccr: DSI_LCCR,
    #[doc = "0x68 - DSI Host command mode configuration register"]
    pub dsi_cmcr: DSI_CMCR,
    #[doc = "0x6c - DSI Host generic header configuration register"]
    pub dsi_ghcr: DSI_GHCR,
    #[doc = "0x70 - DSI Host generic payload data register"]
    pub dsi_gpdr: DSI_GPDR,
    #[doc = "0x74 - DSI Host generic packet status register"]
    pub dsi_gpsr: DSI_GPSR,
    #[doc = "0x78 - DSI Host timeout counter configuration register 0"]
    pub dsi_tccr0: DSI_TCCR0,
    #[doc = "0x7c - DSI Host timeout counter configuration register 1"]
    pub dsi_tccr1: DSI_TCCR1,
    #[doc = "0x80 - DSI Host timeout counter configuration register 2"]
    pub dsi_tccr2: DSI_TCCR2,
    #[doc = "0x84 - DSI Host timeout counter configuration register 3"]
    pub dsi_tccr3: DSI_TCCR3,
    #[doc = "0x88 - DSI Host timeout counter configuration register 4"]
    pub dsi_tccr4: DSI_TCCR4,
    #[doc = "0x8c - DSI Host timeout counter configuration register 5"]
    pub dsi_tccr5: DSI_TCCR5,
    _reserved32: [u8; 0x04],
    #[doc = "0x94 - DSI Host clock lane configuration register"]
    pub dsi_clcr: DSI_CLCR,
    #[doc = "0x98 - DSI Host clock lane timer configuration register"]
    pub dsi_cltcr: DSI_CLTCR,
    #[doc = "0x9c - DSI Host data lane timer configuration register"]
    pub dsi_dltcr: DSI_DLTCR,
    #[doc = "0xa0 - DSI Host PHY control register"]
    pub dsi_pctlr: DSI_PCTLR,
    #[doc = "0xa4 - DSI Host PHY configuration register"]
    pub dsi_pconfr: DSI_PCONFR,
    #[doc = "0xa8 - DSI Host PHY ULPS control register"]
    pub dsi_pucr: DSI_PUCR,
    #[doc = "0xac - DSI Host PHY TX triggers configuration register"]
    pub dsi_pttcr: DSI_PTTCR,
    #[doc = "0xb0 - DSI Host PHY status register"]
    pub dsi_psr: DSI_PSR,
    _reserved40: [u8; 0x08],
    #[doc = "0xbc - DSI Host interrupt and status register 0"]
    pub dsi_isr0: DSI_ISR0,
    #[doc = "0xc0 - DSI Host interrupt and status register 1"]
    pub dsi_isr1: DSI_ISR1,
    #[doc = "0xc4 - DSI Host interrupt enable register 0"]
    pub dsi_ier0: DSI_IER0,
    #[doc = "0xc8 - DSI Host interrupt enable register 1"]
    pub dsi_ier1: DSI_IER1,
    _reserved44: [u8; 0x0c],
    #[doc = "0xd8 - DSI Host force interrupt register 0"]
    pub dsi_fir0: DSI_FIR0,
    #[doc = "0xdc - DSI Host force interrupt register 1"]
    pub dsi_fir1: DSI_FIR1,
    _reserved46: [u8; 0x20],
    #[doc = "0x100 - DSI Host video shadow control register"]
    pub dsi_vscr: DSI_VSCR,
    _reserved47: [u8; 0x08],
    #[doc = "0x10c - DSI Host LTDC current VCID register"]
    pub dsi_lcvcidr: DSI_LCVCIDR,
    #[doc = "0x110 - DSI Host LTDC current color coding register"]
    pub dsi_lcccr: DSI_LCCCR,
    _reserved49: [u8; 0x04],
    #[doc = "0x118 - DSI Host low-power mode current configuration register"]
    pub dsi_lpmccr: DSI_LPMCCR,
    _reserved50: [u8; 0x1c],
    #[doc = "0x138 - DSI Host video mode current configuration register"]
    pub dsi_vmccr: DSI_VMCCR,
    #[doc = "0x13c - DSI Host video packet current configuration register"]
    pub dsi_vpccr: DSI_VPCCR,
    #[doc = "0x140 - DSI Host video chunks current configuration register"]
    pub dsi_vcccr: DSI_VCCCR,
    #[doc = "0x144 - DSI Host video null packet current configuration register"]
    pub dsi_vnpccr: DSI_VNPCCR,
    #[doc = "0x148 - DSI Host video HSA current configuration register"]
    pub dsi_vhsaccr: DSI_VHSACCR,
    #[doc = "0x14c - DSI Host video HBP current configuration register"]
    pub dsi_vhbpccr: DSI_VHBPCCR,
    #[doc = "0x150 - DSI Host video line current configuration register"]
    pub dsi_vlccr: DSI_VLCCR,
    #[doc = "0x154 - DSI Host video VSA current configuration register"]
    pub dsi_vvsaccr: DSI_VVSACCR,
    #[doc = "0x158 - DSI Host video VBP current configuration register"]
    pub dsi_vvbpccr: DSI_VVBPCCR,
    #[doc = "0x15c - DSI Host video VFP current configuration register"]
    pub dsi_vvfpccr: DSI_VVFPCCR,
    #[doc = "0x160 - DSI Host video VA current configuration register"]
    pub dsi_vvaccr: DSI_VVACCR,
    _reserved61: [u8; 0x029c],
    #[doc = "0x400 - DSI wrapper configuration register"]
    pub dsi_wcfgr: DSI_WCFGR,
    #[doc = "0x404 - DSI wrapper control register"]
    pub dsi_wcr: DSI_WCR,
    #[doc = "0x408 - DSI wrapper interrupt enable register"]
    pub dsi_wier: DSI_WIER,
    #[doc = "0x40c - DSI wrapper interrupt and status register"]
    pub dsi_wisr: DSI_WISR,
    #[doc = "0x410 - DSI wrapper interrupt flag clear register"]
    pub dsi_wifcr: DSI_WIFCR,
    _reserved66: [u8; 0x04],
    #[doc = "0x418 - DSI wrapper PHY configuration register 0"]
    pub dsi_wpcr0: DSI_WPCR0,
    #[doc = "0x41c - This register shall be programmed only when DSI is stopped (CR. DSIEN=0 and CR.EN = 0)."]
    pub dsi_wpcr1: DSI_WPCR1,
    #[doc = "0x420 - DSI wrapper PHY configuration register 2"]
    pub dsi_wpcr2: DSI_WPCR2,
    #[doc = "0x424 - DSI wrapper PHY configuration register 3"]
    pub dsi_wpcr3: DSI_WPCR3,
    #[doc = "0x428 - DSI wrapper PHY configuration register 4"]
    pub dsi_wpcr4: DSI_WPCR4,
    _reserved71: [u8; 0x04],
    #[doc = "0x430 - DSI wrapper regulator and PLL control register"]
    pub dsi_wrpcr: DSI_WRPCR,
}
#[doc = "DSI_VR (r) register accessor: DSI Host version register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dsi_vr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`dsi_vr`]
module"]
pub type DSI_VR = crate::Reg<dsi_vr::DSI_VR_SPEC>;
#[doc = "DSI Host version register"]
pub mod dsi_vr;
#[doc = "DSI_CR (rw) register accessor: DSI Host control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dsi_cr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dsi_cr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`dsi_cr`]
module"]
pub type DSI_CR = crate::Reg<dsi_cr::DSI_CR_SPEC>;
#[doc = "DSI Host control register"]
pub mod dsi_cr;
#[doc = "DSI_CCR (rw) register accessor: DSI Host clock control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dsi_ccr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dsi_ccr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`dsi_ccr`]
module"]
pub type DSI_CCR = crate::Reg<dsi_ccr::DSI_CCR_SPEC>;
#[doc = "DSI Host clock control register"]
pub mod dsi_ccr;
#[doc = "DSI_LVCIDR (rw) register accessor: DSI Host LTDC VCID register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dsi_lvcidr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dsi_lvcidr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`dsi_lvcidr`]
module"]
pub type DSI_LVCIDR = crate::Reg<dsi_lvcidr::DSI_LVCIDR_SPEC>;
#[doc = "DSI Host LTDC VCID register"]
pub mod dsi_lvcidr;
#[doc = "DSI_LCOLCR (rw) register accessor: DSI Host LTDC color coding register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dsi_lcolcr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dsi_lcolcr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`dsi_lcolcr`]
module"]
pub type DSI_LCOLCR = crate::Reg<dsi_lcolcr::DSI_LCOLCR_SPEC>;
#[doc = "DSI Host LTDC color coding register"]
pub mod dsi_lcolcr;
#[doc = "DSI_LPCR (rw) register accessor: DSI Host LTDC polarity configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dsi_lpcr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dsi_lpcr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`dsi_lpcr`]
module"]
pub type DSI_LPCR = crate::Reg<dsi_lpcr::DSI_LPCR_SPEC>;
#[doc = "DSI Host LTDC polarity configuration register"]
pub mod dsi_lpcr;
#[doc = "DSI_LPMCR (rw) register accessor: DSI Host low-power mode configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dsi_lpmcr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dsi_lpmcr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`dsi_lpmcr`]
module"]
pub type DSI_LPMCR = crate::Reg<dsi_lpmcr::DSI_LPMCR_SPEC>;
#[doc = "DSI Host low-power mode configuration register"]
pub mod dsi_lpmcr;
#[doc = "DSI_PCR (rw) register accessor: DSI Host protocol configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dsi_pcr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dsi_pcr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`dsi_pcr`]
module"]
pub type DSI_PCR = crate::Reg<dsi_pcr::DSI_PCR_SPEC>;
#[doc = "DSI Host protocol configuration register"]
pub mod dsi_pcr;
#[doc = "DSI_GVCIDR (r) register accessor: DSI Host generic VCID register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dsi_gvcidr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`dsi_gvcidr`]
module"]
pub type DSI_GVCIDR = crate::Reg<dsi_gvcidr::DSI_GVCIDR_SPEC>;
#[doc = "DSI Host generic VCID register"]
pub mod dsi_gvcidr;
#[doc = "DSI_MCR (rw) register accessor: DSI Host mode configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dsi_mcr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dsi_mcr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`dsi_mcr`]
module"]
pub type DSI_MCR = crate::Reg<dsi_mcr::DSI_MCR_SPEC>;
#[doc = "DSI Host mode configuration register"]
pub mod dsi_mcr;
#[doc = "DSI_VMCR (rw) register accessor: DSI Host video mode configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dsi_vmcr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dsi_vmcr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`dsi_vmcr`]
module"]
pub type DSI_VMCR = crate::Reg<dsi_vmcr::DSI_VMCR_SPEC>;
#[doc = "DSI Host video mode configuration register"]
pub mod dsi_vmcr;
#[doc = "DSI_VPCR (rw) register accessor: DSI Host video packet configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dsi_vpcr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dsi_vpcr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`dsi_vpcr`]
module"]
pub type DSI_VPCR = crate::Reg<dsi_vpcr::DSI_VPCR_SPEC>;
#[doc = "DSI Host video packet configuration register"]
pub mod dsi_vpcr;
#[doc = "DSI_VCCR (rw) register accessor: DSI Host video chunks configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dsi_vccr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dsi_vccr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`dsi_vccr`]
module"]
pub type DSI_VCCR = crate::Reg<dsi_vccr::DSI_VCCR_SPEC>;
#[doc = "DSI Host video chunks configuration register"]
pub mod dsi_vccr;
#[doc = "DSI_VNPCR (rw) register accessor: DSI Host video null packet configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dsi_vnpcr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dsi_vnpcr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`dsi_vnpcr`]
module"]
pub type DSI_VNPCR = crate::Reg<dsi_vnpcr::DSI_VNPCR_SPEC>;
#[doc = "DSI Host video null packet configuration register"]
pub mod dsi_vnpcr;
#[doc = "DSI_VHSACR (rw) register accessor: DSI Host video HSA configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dsi_vhsacr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dsi_vhsacr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`dsi_vhsacr`]
module"]
pub type DSI_VHSACR = crate::Reg<dsi_vhsacr::DSI_VHSACR_SPEC>;
#[doc = "DSI Host video HSA configuration register"]
pub mod dsi_vhsacr;
#[doc = "DSI_VHBPCR (rw) register accessor: DSI Host video HBP configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dsi_vhbpcr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dsi_vhbpcr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`dsi_vhbpcr`]
module"]
pub type DSI_VHBPCR = crate::Reg<dsi_vhbpcr::DSI_VHBPCR_SPEC>;
#[doc = "DSI Host video HBP configuration register"]
pub mod dsi_vhbpcr;
#[doc = "DSI_VLCR (rw) register accessor: DSI Host video line configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dsi_vlcr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dsi_vlcr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`dsi_vlcr`]
module"]
pub type DSI_VLCR = crate::Reg<dsi_vlcr::DSI_VLCR_SPEC>;
#[doc = "DSI Host video line configuration register"]
pub mod dsi_vlcr;
#[doc = "DSI_VVSACR (rw) register accessor: DSI Host video VSA configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dsi_vvsacr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dsi_vvsacr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`dsi_vvsacr`]
module"]
pub type DSI_VVSACR = crate::Reg<dsi_vvsacr::DSI_VVSACR_SPEC>;
#[doc = "DSI Host video VSA configuration register"]
pub mod dsi_vvsacr;
#[doc = "DSI_VVBPCR (rw) register accessor: DSI Host video VBP configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dsi_vvbpcr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dsi_vvbpcr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`dsi_vvbpcr`]
module"]
pub type DSI_VVBPCR = crate::Reg<dsi_vvbpcr::DSI_VVBPCR_SPEC>;
#[doc = "DSI Host video VBP configuration register"]
pub mod dsi_vvbpcr;
#[doc = "DSI_VVFPCR (rw) register accessor: DSI Host video VFP configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dsi_vvfpcr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dsi_vvfpcr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`dsi_vvfpcr`]
module"]
pub type DSI_VVFPCR = crate::Reg<dsi_vvfpcr::DSI_VVFPCR_SPEC>;
#[doc = "DSI Host video VFP configuration register"]
pub mod dsi_vvfpcr;
#[doc = "DSI_VVACR (rw) register accessor: DSI Host video VA configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dsi_vvacr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dsi_vvacr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`dsi_vvacr`]
module"]
pub type DSI_VVACR = crate::Reg<dsi_vvacr::DSI_VVACR_SPEC>;
#[doc = "DSI Host video VA configuration register"]
pub mod dsi_vvacr;
#[doc = "DSI_LCCR (rw) register accessor: DSI Host LTDC command configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dsi_lccr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dsi_lccr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`dsi_lccr`]
module"]
pub type DSI_LCCR = crate::Reg<dsi_lccr::DSI_LCCR_SPEC>;
#[doc = "DSI Host LTDC command configuration register"]
pub mod dsi_lccr;
#[doc = "DSI_CMCR (rw) register accessor: DSI Host command mode configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dsi_cmcr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dsi_cmcr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`dsi_cmcr`]
module"]
pub type DSI_CMCR = crate::Reg<dsi_cmcr::DSI_CMCR_SPEC>;
#[doc = "DSI Host command mode configuration register"]
pub mod dsi_cmcr;
#[doc = "DSI_GHCR (rw) register accessor: DSI Host generic header configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dsi_ghcr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dsi_ghcr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`dsi_ghcr`]
module"]
pub type DSI_GHCR = crate::Reg<dsi_ghcr::DSI_GHCR_SPEC>;
#[doc = "DSI Host generic header configuration register"]
pub mod dsi_ghcr;
#[doc = "DSI_GPDR (rw) register accessor: DSI Host generic payload data register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dsi_gpdr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dsi_gpdr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`dsi_gpdr`]
module"]
pub type DSI_GPDR = crate::Reg<dsi_gpdr::DSI_GPDR_SPEC>;
#[doc = "DSI Host generic payload data register"]
pub mod dsi_gpdr;
#[doc = "DSI_GPSR (r) register accessor: DSI Host generic packet status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dsi_gpsr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`dsi_gpsr`]
module"]
pub type DSI_GPSR = crate::Reg<dsi_gpsr::DSI_GPSR_SPEC>;
#[doc = "DSI Host generic packet status register"]
pub mod dsi_gpsr;
#[doc = "DSI_TCCR0 (rw) register accessor: DSI Host timeout counter configuration register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dsi_tccr0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dsi_tccr0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`dsi_tccr0`]
module"]
pub type DSI_TCCR0 = crate::Reg<dsi_tccr0::DSI_TCCR0_SPEC>;
#[doc = "DSI Host timeout counter configuration register 0"]
pub mod dsi_tccr0;
#[doc = "DSI_TCCR1 (rw) register accessor: DSI Host timeout counter configuration register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dsi_tccr1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dsi_tccr1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`dsi_tccr1`]
module"]
pub type DSI_TCCR1 = crate::Reg<dsi_tccr1::DSI_TCCR1_SPEC>;
#[doc = "DSI Host timeout counter configuration register 1"]
pub mod dsi_tccr1;
#[doc = "DSI_TCCR2 (rw) register accessor: DSI Host timeout counter configuration register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dsi_tccr2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dsi_tccr2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`dsi_tccr2`]
module"]
pub type DSI_TCCR2 = crate::Reg<dsi_tccr2::DSI_TCCR2_SPEC>;
#[doc = "DSI Host timeout counter configuration register 2"]
pub mod dsi_tccr2;
#[doc = "DSI_TCCR3 (rw) register accessor: DSI Host timeout counter configuration register 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dsi_tccr3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dsi_tccr3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`dsi_tccr3`]
module"]
pub type DSI_TCCR3 = crate::Reg<dsi_tccr3::DSI_TCCR3_SPEC>;
#[doc = "DSI Host timeout counter configuration register 3"]
pub mod dsi_tccr3;
#[doc = "DSI_TCCR4 (rw) register accessor: DSI Host timeout counter configuration register 4\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dsi_tccr4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dsi_tccr4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`dsi_tccr4`]
module"]
pub type DSI_TCCR4 = crate::Reg<dsi_tccr4::DSI_TCCR4_SPEC>;
#[doc = "DSI Host timeout counter configuration register 4"]
pub mod dsi_tccr4;
#[doc = "DSI_TCCR5 (rw) register accessor: DSI Host timeout counter configuration register 5\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dsi_tccr5::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dsi_tccr5::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`dsi_tccr5`]
module"]
pub type DSI_TCCR5 = crate::Reg<dsi_tccr5::DSI_TCCR5_SPEC>;
#[doc = "DSI Host timeout counter configuration register 5"]
pub mod dsi_tccr5;
#[doc = "DSI_CLCR (rw) register accessor: DSI Host clock lane configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dsi_clcr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dsi_clcr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`dsi_clcr`]
module"]
pub type DSI_CLCR = crate::Reg<dsi_clcr::DSI_CLCR_SPEC>;
#[doc = "DSI Host clock lane configuration register"]
pub mod dsi_clcr;
#[doc = "DSI_CLTCR (rw) register accessor: DSI Host clock lane timer configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dsi_cltcr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dsi_cltcr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`dsi_cltcr`]
module"]
pub type DSI_CLTCR = crate::Reg<dsi_cltcr::DSI_CLTCR_SPEC>;
#[doc = "DSI Host clock lane timer configuration register"]
pub mod dsi_cltcr;
#[doc = "DSI_DLTCR (rw) register accessor: DSI Host data lane timer configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dsi_dltcr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dsi_dltcr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`dsi_dltcr`]
module"]
pub type DSI_DLTCR = crate::Reg<dsi_dltcr::DSI_DLTCR_SPEC>;
#[doc = "DSI Host data lane timer configuration register"]
pub mod dsi_dltcr;
#[doc = "DSI_PCTLR (rw) register accessor: DSI Host PHY control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dsi_pctlr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dsi_pctlr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`dsi_pctlr`]
module"]
pub type DSI_PCTLR = crate::Reg<dsi_pctlr::DSI_PCTLR_SPEC>;
#[doc = "DSI Host PHY control register"]
pub mod dsi_pctlr;
#[doc = "DSI_PCONFR (rw) register accessor: DSI Host PHY configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dsi_pconfr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dsi_pconfr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`dsi_pconfr`]
module"]
pub type DSI_PCONFR = crate::Reg<dsi_pconfr::DSI_PCONFR_SPEC>;
#[doc = "DSI Host PHY configuration register"]
pub mod dsi_pconfr;
#[doc = "DSI_PUCR (rw) register accessor: DSI Host PHY ULPS control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dsi_pucr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dsi_pucr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`dsi_pucr`]
module"]
pub type DSI_PUCR = crate::Reg<dsi_pucr::DSI_PUCR_SPEC>;
#[doc = "DSI Host PHY ULPS control register"]
pub mod dsi_pucr;
#[doc = "DSI_PTTCR (rw) register accessor: DSI Host PHY TX triggers configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dsi_pttcr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dsi_pttcr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`dsi_pttcr`]
module"]
pub type DSI_PTTCR = crate::Reg<dsi_pttcr::DSI_PTTCR_SPEC>;
#[doc = "DSI Host PHY TX triggers configuration register"]
pub mod dsi_pttcr;
#[doc = "DSI_PSR (r) register accessor: DSI Host PHY status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dsi_psr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`dsi_psr`]
module"]
pub type DSI_PSR = crate::Reg<dsi_psr::DSI_PSR_SPEC>;
#[doc = "DSI Host PHY status register"]
pub mod dsi_psr;
#[doc = "DSI_ISR0 (r) register accessor: DSI Host interrupt and status register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dsi_isr0::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`dsi_isr0`]
module"]
pub type DSI_ISR0 = crate::Reg<dsi_isr0::DSI_ISR0_SPEC>;
#[doc = "DSI Host interrupt and status register 0"]
pub mod dsi_isr0;
#[doc = "DSI_ISR1 (r) register accessor: DSI Host interrupt and status register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dsi_isr1::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`dsi_isr1`]
module"]
pub type DSI_ISR1 = crate::Reg<dsi_isr1::DSI_ISR1_SPEC>;
#[doc = "DSI Host interrupt and status register 1"]
pub mod dsi_isr1;
#[doc = "DSI_IER0 (rw) register accessor: DSI Host interrupt enable register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dsi_ier0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dsi_ier0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`dsi_ier0`]
module"]
pub type DSI_IER0 = crate::Reg<dsi_ier0::DSI_IER0_SPEC>;
#[doc = "DSI Host interrupt enable register 0"]
pub mod dsi_ier0;
#[doc = "DSI_IER1 (rw) register accessor: DSI Host interrupt enable register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dsi_ier1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dsi_ier1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`dsi_ier1`]
module"]
pub type DSI_IER1 = crate::Reg<dsi_ier1::DSI_IER1_SPEC>;
#[doc = "DSI Host interrupt enable register 1"]
pub mod dsi_ier1;
#[doc = "DSI_FIR0 (w) register accessor: DSI Host force interrupt register 0\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dsi_fir0::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`dsi_fir0`]
module"]
pub type DSI_FIR0 = crate::Reg<dsi_fir0::DSI_FIR0_SPEC>;
#[doc = "DSI Host force interrupt register 0"]
pub mod dsi_fir0;
#[doc = "DSI_FIR1 (w) register accessor: DSI Host force interrupt register 1\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dsi_fir1::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`dsi_fir1`]
module"]
pub type DSI_FIR1 = crate::Reg<dsi_fir1::DSI_FIR1_SPEC>;
#[doc = "DSI Host force interrupt register 1"]
pub mod dsi_fir1;
#[doc = "DSI_VSCR (rw) register accessor: DSI Host video shadow control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dsi_vscr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dsi_vscr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`dsi_vscr`]
module"]
pub type DSI_VSCR = crate::Reg<dsi_vscr::DSI_VSCR_SPEC>;
#[doc = "DSI Host video shadow control register"]
pub mod dsi_vscr;
#[doc = "DSI_LCVCIDR (rw) register accessor: DSI Host LTDC current VCID register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dsi_lcvcidr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dsi_lcvcidr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`dsi_lcvcidr`]
module"]
pub type DSI_LCVCIDR = crate::Reg<dsi_lcvcidr::DSI_LCVCIDR_SPEC>;
#[doc = "DSI Host LTDC current VCID register"]
pub mod dsi_lcvcidr;
#[doc = "DSI_LCCCR (r) register accessor: DSI Host LTDC current color coding register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dsi_lcccr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`dsi_lcccr`]
module"]
pub type DSI_LCCCR = crate::Reg<dsi_lcccr::DSI_LCCCR_SPEC>;
#[doc = "DSI Host LTDC current color coding register"]
pub mod dsi_lcccr;
#[doc = "DSI_LPMCCR (r) register accessor: DSI Host low-power mode current configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dsi_lpmccr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`dsi_lpmccr`]
module"]
pub type DSI_LPMCCR = crate::Reg<dsi_lpmccr::DSI_LPMCCR_SPEC>;
#[doc = "DSI Host low-power mode current configuration register"]
pub mod dsi_lpmccr;
#[doc = "DSI_VMCCR (r) register accessor: DSI Host video mode current configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dsi_vmccr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`dsi_vmccr`]
module"]
pub type DSI_VMCCR = crate::Reg<dsi_vmccr::DSI_VMCCR_SPEC>;
#[doc = "DSI Host video mode current configuration register"]
pub mod dsi_vmccr;
#[doc = "DSI_VPCCR (r) register accessor: DSI Host video packet current configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dsi_vpccr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`dsi_vpccr`]
module"]
pub type DSI_VPCCR = crate::Reg<dsi_vpccr::DSI_VPCCR_SPEC>;
#[doc = "DSI Host video packet current configuration register"]
pub mod dsi_vpccr;
#[doc = "DSI_VCCCR (r) register accessor: DSI Host video chunks current configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dsi_vcccr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`dsi_vcccr`]
module"]
pub type DSI_VCCCR = crate::Reg<dsi_vcccr::DSI_VCCCR_SPEC>;
#[doc = "DSI Host video chunks current configuration register"]
pub mod dsi_vcccr;
#[doc = "DSI_VNPCCR (r) register accessor: DSI Host video null packet current configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dsi_vnpccr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`dsi_vnpccr`]
module"]
pub type DSI_VNPCCR = crate::Reg<dsi_vnpccr::DSI_VNPCCR_SPEC>;
#[doc = "DSI Host video null packet current configuration register"]
pub mod dsi_vnpccr;
#[doc = "DSI_VHSACCR (r) register accessor: DSI Host video HSA current configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dsi_vhsaccr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`dsi_vhsaccr`]
module"]
pub type DSI_VHSACCR = crate::Reg<dsi_vhsaccr::DSI_VHSACCR_SPEC>;
#[doc = "DSI Host video HSA current configuration register"]
pub mod dsi_vhsaccr;
#[doc = "DSI_VHBPCCR (r) register accessor: DSI Host video HBP current configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dsi_vhbpccr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`dsi_vhbpccr`]
module"]
pub type DSI_VHBPCCR = crate::Reg<dsi_vhbpccr::DSI_VHBPCCR_SPEC>;
#[doc = "DSI Host video HBP current configuration register"]
pub mod dsi_vhbpccr;
#[doc = "DSI_VLCCR (r) register accessor: DSI Host video line current configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dsi_vlccr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`dsi_vlccr`]
module"]
pub type DSI_VLCCR = crate::Reg<dsi_vlccr::DSI_VLCCR_SPEC>;
#[doc = "DSI Host video line current configuration register"]
pub mod dsi_vlccr;
#[doc = "DSI_VVSACCR (r) register accessor: DSI Host video VSA current configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dsi_vvsaccr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`dsi_vvsaccr`]
module"]
pub type DSI_VVSACCR = crate::Reg<dsi_vvsaccr::DSI_VVSACCR_SPEC>;
#[doc = "DSI Host video VSA current configuration register"]
pub mod dsi_vvsaccr;
#[doc = "DSI_VVBPCCR (r) register accessor: DSI Host video VBP current configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dsi_vvbpccr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`dsi_vvbpccr`]
module"]
pub type DSI_VVBPCCR = crate::Reg<dsi_vvbpccr::DSI_VVBPCCR_SPEC>;
#[doc = "DSI Host video VBP current configuration register"]
pub mod dsi_vvbpccr;
#[doc = "DSI_VVFPCCR (r) register accessor: DSI Host video VFP current configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dsi_vvfpccr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`dsi_vvfpccr`]
module"]
pub type DSI_VVFPCCR = crate::Reg<dsi_vvfpccr::DSI_VVFPCCR_SPEC>;
#[doc = "DSI Host video VFP current configuration register"]
pub mod dsi_vvfpccr;
#[doc = "DSI_VVACCR (r) register accessor: DSI Host video VA current configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dsi_vvaccr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`dsi_vvaccr`]
module"]
pub type DSI_VVACCR = crate::Reg<dsi_vvaccr::DSI_VVACCR_SPEC>;
#[doc = "DSI Host video VA current configuration register"]
pub mod dsi_vvaccr;
#[doc = "DSI_WCFGR (rw) register accessor: DSI wrapper configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dsi_wcfgr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dsi_wcfgr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`dsi_wcfgr`]
module"]
pub type DSI_WCFGR = crate::Reg<dsi_wcfgr::DSI_WCFGR_SPEC>;
#[doc = "DSI wrapper configuration register"]
pub mod dsi_wcfgr;
#[doc = "DSI_WCR (rw) register accessor: DSI wrapper control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dsi_wcr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dsi_wcr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`dsi_wcr`]
module"]
pub type DSI_WCR = crate::Reg<dsi_wcr::DSI_WCR_SPEC>;
#[doc = "DSI wrapper control register"]
pub mod dsi_wcr;
#[doc = "DSI_WIER (rw) register accessor: DSI wrapper interrupt enable register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dsi_wier::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dsi_wier::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`dsi_wier`]
module"]
pub type DSI_WIER = crate::Reg<dsi_wier::DSI_WIER_SPEC>;
#[doc = "DSI wrapper interrupt enable register"]
pub mod dsi_wier;
#[doc = "DSI_WISR (r) register accessor: DSI wrapper interrupt and status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dsi_wisr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`dsi_wisr`]
module"]
pub type DSI_WISR = crate::Reg<dsi_wisr::DSI_WISR_SPEC>;
#[doc = "DSI wrapper interrupt and status register"]
pub mod dsi_wisr;
#[doc = "DSI_WIFCR (w) register accessor: DSI wrapper interrupt flag clear register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dsi_wifcr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`dsi_wifcr`]
module"]
pub type DSI_WIFCR = crate::Reg<dsi_wifcr::DSI_WIFCR_SPEC>;
#[doc = "DSI wrapper interrupt flag clear register"]
pub mod dsi_wifcr;
#[doc = "DSI_WPCR0 (rw) register accessor: DSI wrapper PHY configuration register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dsi_wpcr0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dsi_wpcr0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`dsi_wpcr0`]
module"]
pub type DSI_WPCR0 = crate::Reg<dsi_wpcr0::DSI_WPCR0_SPEC>;
#[doc = "DSI wrapper PHY configuration register 0"]
pub mod dsi_wpcr0;
#[doc = "DSI_WPCR1 (rw) register accessor: This register shall be programmed only when DSI is stopped (CR. DSIEN=0 and CR.EN = 0).\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dsi_wpcr1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dsi_wpcr1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`dsi_wpcr1`]
module"]
pub type DSI_WPCR1 = crate::Reg<dsi_wpcr1::DSI_WPCR1_SPEC>;
#[doc = "This register shall be programmed only when DSI is stopped (CR. DSIEN=0 and CR.EN = 0)."]
pub mod dsi_wpcr1;
#[doc = "DSI_WPCR2 (rw) register accessor: DSI wrapper PHY configuration register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dsi_wpcr2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dsi_wpcr2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`dsi_wpcr2`]
module"]
pub type DSI_WPCR2 = crate::Reg<dsi_wpcr2::DSI_WPCR2_SPEC>;
#[doc = "DSI wrapper PHY configuration register 2"]
pub mod dsi_wpcr2;
#[doc = "DSI_WPCR3 (rw) register accessor: DSI wrapper PHY configuration register 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dsi_wpcr3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dsi_wpcr3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`dsi_wpcr3`]
module"]
pub type DSI_WPCR3 = crate::Reg<dsi_wpcr3::DSI_WPCR3_SPEC>;
#[doc = "DSI wrapper PHY configuration register 3"]
pub mod dsi_wpcr3;
#[doc = "DSI_WPCR4 (rw) register accessor: DSI wrapper PHY configuration register 4\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dsi_wpcr4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dsi_wpcr4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`dsi_wpcr4`]
module"]
pub type DSI_WPCR4 = crate::Reg<dsi_wpcr4::DSI_WPCR4_SPEC>;
#[doc = "DSI wrapper PHY configuration register 4"]
pub mod dsi_wpcr4;
#[doc = "DSI_WRPCR (rw) register accessor: DSI wrapper regulator and PLL control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dsi_wrpcr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dsi_wrpcr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`dsi_wrpcr`]
module"]
pub type DSI_WRPCR = crate::Reg<dsi_wrpcr::DSI_WRPCR_SPEC>;
#[doc = "DSI wrapper regulator and PLL control register"]
pub mod dsi_wrpcr;
