#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Operating mode configuration register"]
    pub maccr: MACCR,
    #[doc = "0x04 - Extended operating mode configuration register"]
    pub macecr: MACECR,
    #[doc = "0x08 - Packet filtering control register"]
    pub macpfr: MACPFR,
    #[doc = "0x0c - Watchdog timeout register"]
    pub macwtr: MACWTR,
    #[doc = "0x10 - Hash Table 0 register"]
    pub macht0r: MACHT0R,
    #[doc = "0x14 - Hash Table 1 register"]
    pub macht1r: MACHT1R,
    _reserved6: [u8; 0x38],
    #[doc = "0x50 - VLAN tag register"]
    pub macvtr: MACVTR,
    _reserved7: [u8; 0x04],
    #[doc = "0x58 - VLAN Hash table register"]
    pub macvhtr: MACVHTR,
    _reserved8: [u8; 0x04],
    #[doc = "0x60 - VLAN inclusion register"]
    pub macvir: MACVIR,
    #[doc = "0x64 - Inner VLAN inclusion register"]
    pub macivir: MACIVIR,
    _reserved10: [u8; 0x08],
    #[doc = "0x70 - Tx Queue flow control register"]
    pub macqtx_fcr: MACQTX_FCR,
    _reserved11: [u8; 0x1c],
    #[doc = "0x90 - Rx flow control register"]
    pub macrx_fcr: MACRX_FCR,
    _reserved12: [u8; 0x1c],
    #[doc = "0xb0 - Interrupt status register"]
    pub macisr: MACISR,
    #[doc = "0xb4 - Interrupt enable register"]
    pub macier: MACIER,
    #[doc = "0xb8 - Rx Tx status register"]
    pub macrx_tx_sr: MACRX_TX_SR,
    _reserved15: [u8; 0x04],
    #[doc = "0xc0 - PMT control status register"]
    pub macpcsr: MACPCSR,
    #[doc = "0xc4 - Remove wakeup packet filter register"]
    pub macrwkpfr: MACRWKPFR,
    _reserved17: [u8; 0x08],
    #[doc = "0xd0 - LPI control status register"]
    pub maclcsr: MACLCSR,
    #[doc = "0xd4 - LPI timers control register"]
    pub macltcr: MACLTCR,
    #[doc = "0xd8 - LPI entry timer register"]
    pub macletr: MACLETR,
    #[doc = "0xdc - 1-microsecond-tick counter register"]
    pub mac1ustcr: MAC1USTCR,
    _reserved21: [u8; 0x30],
    #[doc = "0x110 - Version register"]
    pub macvr: MACVR,
    #[doc = "0x114 - Debug register"]
    pub macdr: MACDR,
    _reserved23: [u8; 0x08],
    #[doc = "0x120 - HW feature 1 register"]
    pub machwf1r: MACHWF1R,
    #[doc = "0x124 - HW feature 2 register"]
    pub machwf2r: MACHWF2R,
    _reserved25: [u8; 0xd8],
    #[doc = "0x200 - MDIO address register"]
    pub macmdioar: MACMDIOAR,
    #[doc = "0x204 - MDIO data register"]
    pub macmdiodr: MACMDIODR,
    _reserved27: [u8; 0xf8],
    #[doc = "0x300 - Address 0 high register"]
    pub maca0hr: MACA0HR,
    #[doc = "0x304 - Address 0 low register"]
    pub maca0lr: MACA0LR,
    #[doc = "0x308 - Address 1 high register"]
    pub maca1hr: MACA1HR,
    #[doc = "0x30c - Address 1 low register"]
    pub maca1lr: MACA1LR,
    #[doc = "0x310 - Address 2 high register"]
    pub maca2hr: MACA2HR,
    #[doc = "0x314 - Address 2 low register"]
    pub maca2lr: MACA2LR,
    #[doc = "0x318 - Address 3 high register"]
    pub maca3hr: MACA3HR,
    #[doc = "0x31c - Address 3 low register"]
    pub maca3lr: MACA3LR,
    _reserved35: [u8; 0x03e0],
    #[doc = "0x700 - MMC control register"]
    pub mmc_control: MMC_CONTROL,
    #[doc = "0x704 - MMC Rx interrupt register"]
    pub mmc_rx_interrupt: MMC_RX_INTERRUPT,
    #[doc = "0x708 - MMC Tx interrupt register"]
    pub mmc_tx_interrupt: MMC_TX_INTERRUPT,
    #[doc = "0x70c - MMC Rx interrupt mask register"]
    pub mmc_rx_interrupt_mask: MMC_RX_INTERRUPT_MASK,
    #[doc = "0x710 - MMC Tx interrupt mask register"]
    pub mmc_tx_interrupt_mask: MMC_TX_INTERRUPT_MASK,
    _reserved40: [u8; 0x38],
    #[doc = "0x74c - Tx single collision good packets register"]
    pub tx_single_collision_good_packets: TX_SINGLE_COLLISION_GOOD_PACKETS,
    #[doc = "0x750 - Tx multiple collision good packets register"]
    pub tx_multiple_collision_good_packets: TX_MULTIPLE_COLLISION_GOOD_PACKETS,
    _reserved42: [u8; 0x14],
    #[doc = "0x768 - Tx packet count good register"]
    pub tx_packet_count_good: TX_PACKET_COUNT_GOOD,
    _reserved43: [u8; 0x28],
    #[doc = "0x794 - Rx CRC error packets register"]
    pub rx_crc_error_packets: RX_CRC_ERROR_PACKETS,
    #[doc = "0x798 - Rx alignment error packets register"]
    pub rx_alignment_error_packets: RX_ALIGNMENT_ERROR_PACKETS,
    _reserved45: [u8; 0x28],
    #[doc = "0x7c4 - Rx unicast packets good register"]
    pub rx_unicast_packets_good: RX_UNICAST_PACKETS_GOOD,
    _reserved46: [u8; 0x24],
    #[doc = "0x7ec - Tx LPI microsecond timer register"]
    pub tx_lpi_usec_cntr: TX_LPI_USEC_CNTR,
    #[doc = "0x7f0 - Tx LPI transition counter register"]
    pub tx_lpi_tran_cntr: TX_LPI_TRAN_CNTR,
    #[doc = "0x7f4 - Rx LPI microsecond counter register"]
    pub rx_lpi_usec_cntr: RX_LPI_USEC_CNTR,
    #[doc = "0x7f8 - Rx LPI transition counter register"]
    pub rx_lpi_tran_cntr: RX_LPI_TRAN_CNTR,
    _reserved50: [u8; 0x0104],
    #[doc = "0x900 - L3 and L4 control 0 register"]
    pub macl3l4c0r: MACL3L4C0R,
    #[doc = "0x904 - Layer4 address filter 0 register"]
    pub macl4a0r: MACL4A0R,
    _reserved52: [u8; 0x08],
    #[doc = "0x910 - MACL3A00R"]
    pub macl3a00r: MACL3A00R,
    #[doc = "0x914 - Layer3 address 1 filter 0 register"]
    pub macl3a10r: MACL3A10R,
    #[doc = "0x918 - Layer3 Address 2 filter 0 register"]
    pub macl3a20: MACL3A20,
    #[doc = "0x91c - Layer3 Address 3 filter 0 register"]
    pub macl3a30: MACL3A30,
    _reserved56: [u8; 0x10],
    #[doc = "0x930 - L3 and L4 control 1 register"]
    pub macl3l4c1r: MACL3L4C1R,
    #[doc = "0x934 - Layer 4 address filter 1 register"]
    pub macl4a1r: MACL4A1R,
    _reserved58: [u8; 0x08],
    #[doc = "0x940 - Layer3 address 0 filter 1 Register"]
    pub macl3a01r: MACL3A01R,
    #[doc = "0x944 - Layer3 address 1 filter 1 register"]
    pub macl3a11r: MACL3A11R,
    #[doc = "0x948 - Layer3 address 2 filter 1 Register"]
    pub macl3a21r: MACL3A21R,
    #[doc = "0x94c - Layer3 address 3 filter 1 register"]
    pub macl3a31r: MACL3A31R,
    _reserved62: [u8; 0x0190],
    #[doc = "0xae0 - ARP address register"]
    pub macarpar: MACARPAR,
    _reserved63: [u8; 0x1c],
    #[doc = "0xb00 - Timestamp control Register"]
    pub mactscr: MACTSCR,
    #[doc = "0xb04 - Sub-second increment register"]
    pub macssir: MACSSIR,
    #[doc = "0xb08 - System time seconds register"]
    pub macstsr: MACSTSR,
    #[doc = "0xb0c - System time nanoseconds register"]
    pub macstnr: MACSTNR,
    #[doc = "0xb10 - System time seconds update register"]
    pub macstsur: MACSTSUR,
    #[doc = "0xb14 - System time nanoseconds update register"]
    pub macstnur: MACSTNUR,
    #[doc = "0xb18 - Timestamp addend register"]
    pub mactsar: MACTSAR,
    _reserved70: [u8; 0x04],
    #[doc = "0xb20 - Timestamp status register"]
    pub mactssr: MACTSSR,
    _reserved71: [u8; 0x0c],
    #[doc = "0xb30 - Tx timestamp status nanoseconds register"]
    pub mactx_tssnr: MACTX_TSSNR,
    #[doc = "0xb34 - Tx timestamp status seconds register"]
    pub mactx_tsssr: MACTX_TSSSR,
    _reserved73: [u8; 0x08],
    #[doc = "0xb40 - Auxiliary control register"]
    pub macacr: MACACR,
    _reserved74: [u8; 0x04],
    #[doc = "0xb48 - Auxiliary timestamp nanoseconds register"]
    pub macatsnr: MACATSNR,
    #[doc = "0xb4c - Auxiliary timestamp seconds register"]
    pub macatssr: MACATSSR,
    #[doc = "0xb50 - Timestamp Ingress asymmetric correction register"]
    pub mactsiacr: MACTSIACR,
    #[doc = "0xb54 - Timestamp Egress asymmetric correction register"]
    pub mactseacr: MACTSEACR,
    #[doc = "0xb58 - Timestamp Ingress correction nanosecond register"]
    pub mactsicnr: MACTSICNR,
    #[doc = "0xb5c - Timestamp Egress correction nanosecond register"]
    pub mactsecnr: MACTSECNR,
    _reserved80: [u8; 0x10],
    #[doc = "0xb70 - PPS control register"]
    pub macppscr: MACPPSCR,
    _reserved81: [u8; 0x0c],
    #[doc = "0xb80 - PPS target time seconds register"]
    pub macppsttsr: MACPPSTTSR,
    #[doc = "0xb84 - PPS target time nanoseconds register"]
    pub macppsttnr: MACPPSTTNR,
    #[doc = "0xb88 - PPS interval register"]
    pub macppsir: MACPPSIR,
    #[doc = "0xb8c - PPS width register"]
    pub macppswr: MACPPSWR,
    _reserved85: [u8; 0x30],
    #[doc = "0xbc0 - PTP Offload control register"]
    pub macpocr: MACPOCR,
    #[doc = "0xbc4 - PTP Source Port Identity 0 Register"]
    pub macspi0r: MACSPI0R,
    #[doc = "0xbc8 - PTP Source port identity 1 register"]
    pub macspi1r: MACSPI1R,
    #[doc = "0xbcc - PTP Source port identity 2 register"]
    pub macspi2r: MACSPI2R,
    #[doc = "0xbd0 - Log message interval register"]
    pub maclmir: MACLMIR,
    _reserved90: [u8; 0x2c],
    #[doc = "0xc00 - Operating mode Register"]
    pub mtlomr: MTLOMR,
    _reserved91: [u8; 0x1c],
    #[doc = "0xc20 - Interrupt status Register"]
    pub mtlisr: MTLISR,
    _reserved92: [u8; 0xdc],
    #[doc = "0xd00 - Tx queue operating mode Register"]
    pub mtltx_qomr: MTLTX_QOMR,
    #[doc = "0xd04 - Tx queue underflow register"]
    pub mtltx_qur: MTLTX_QUR,
    #[doc = "0xd08 - Tx queue debug Register"]
    pub mtltx_qdr: MTLTX_QDR,
    _reserved95: [u8; 0x20],
    #[doc = "0xd2c - Queue interrupt control status Register"]
    pub mtlqicsr: MTLQICSR,
    #[doc = "0xd30 - Rx queue operating mode register"]
    pub mtlrx_qomr: MTLRX_QOMR,
    #[doc = "0xd34 - Rx queue missed packet and overflow counter register"]
    pub mtlrx_qmpocr: MTLRX_QMPOCR,
    #[doc = "0xd38 - Rx queue debug register"]
    pub mtlrx_qdr: MTLRX_QDR,
    _reserved99: [u8; 0x02c4],
    #[doc = "0x1000 - DMA mode register"]
    pub dmamr: DMAMR,
    #[doc = "0x1004 - System bus mode register"]
    pub dmasbmr: DMASBMR,
    #[doc = "0x1008 - Interrupt status register"]
    pub dmaisr: DMAISR,
    #[doc = "0x100c - Debug status register"]
    pub dmadsr: DMADSR,
    _reserved103: [u8; 0xf0],
    #[doc = "0x1100 - Channel control register"]
    pub dmaccr: DMACCR,
    #[doc = "0x1104 - Channel transmit control register"]
    pub dmactx_cr: DMACTX_CR,
    #[doc = "0x1108 - Channel receive control register"]
    pub dmacrx_cr: DMACRX_CR,
    _reserved106: [u8; 0x08],
    #[doc = "0x1114 - Channel Tx descriptor list address register"]
    pub dmactx_dlar: DMACTX_DLAR,
    _reserved107: [u8; 0x04],
    #[doc = "0x111c - Channel Rx descriptor list address register"]
    pub dmacrx_dlar: DMACRX_DLAR,
    #[doc = "0x1120 - Channel Tx descriptor tail pointer register"]
    pub dmactx_dtpr: DMACTX_DTPR,
    _reserved109: [u8; 0x04],
    #[doc = "0x1128 - Channel Rx descriptor tail pointer register"]
    pub dmacrx_dtpr: DMACRX_DTPR,
    #[doc = "0x112c - Channel Tx descriptor ring length register"]
    pub dmactx_rlr: DMACTX_RLR,
    #[doc = "0x1130 - Channel Rx descriptor ring length register"]
    pub dmacrx_rlr: DMACRX_RLR,
    #[doc = "0x1134 - Channel interrupt enable register"]
    pub dmacier: DMACIER,
    #[doc = "0x1138 - Channel Rx interrupt watchdog timer register"]
    pub dmacrx_iwtr: DMACRX_IWTR,
    _reserved114: [u8; 0x08],
    #[doc = "0x1144 - Channel current application transmit descriptor register"]
    pub dmaccatx_dr: DMACCATX_DR,
    _reserved115: [u8; 0x04],
    #[doc = "0x114c - Channel current application receive descriptor register"]
    pub dmaccarx_dr: DMACCARX_DR,
    _reserved116: [u8; 0x04],
    #[doc = "0x1154 - Channel current application transmit buffer register"]
    pub dmaccatx_br: DMACCATX_BR,
    _reserved117: [u8; 0x04],
    #[doc = "0x115c - Channel current application receive buffer register"]
    pub dmaccarx_br: DMACCARX_BR,
    #[doc = "0x1160 - Channel status register"]
    pub dmacsr: DMACSR,
    _reserved119: [u8; 0x08],
    #[doc = "0x116c - Channel missed frame count register"]
    pub dmacmfcr: DMACMFCR,
}
#[doc = "DMAMR (rw) register accessor: DMA mode register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dmamr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dmamr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`dmamr`]
module"]
pub type DMAMR = crate::Reg<dmamr::DMAMR_SPEC>;
#[doc = "DMA mode register"]
pub mod dmamr;
#[doc = "DMASBMR (rw) register accessor: System bus mode register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dmasbmr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dmasbmr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`dmasbmr`]
module"]
pub type DMASBMR = crate::Reg<dmasbmr::DMASBMR_SPEC>;
#[doc = "System bus mode register"]
pub mod dmasbmr;
#[doc = "DMAISR (r) register accessor: Interrupt status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dmaisr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`dmaisr`]
module"]
pub type DMAISR = crate::Reg<dmaisr::DMAISR_SPEC>;
#[doc = "Interrupt status register"]
pub mod dmaisr;
#[doc = "DMADSR (r) register accessor: Debug status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dmadsr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`dmadsr`]
module"]
pub type DMADSR = crate::Reg<dmadsr::DMADSR_SPEC>;
#[doc = "Debug status register"]
pub mod dmadsr;
#[doc = "DMACCR (rw) register accessor: Channel control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dmaccr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dmaccr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`dmaccr`]
module"]
pub type DMACCR = crate::Reg<dmaccr::DMACCR_SPEC>;
#[doc = "Channel control register"]
pub mod dmaccr;
#[doc = "DMACTxCR (rw) register accessor: Channel transmit control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dmactx_cr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dmactx_cr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`dmactx_cr`]
module"]
pub type DMACTX_CR = crate::Reg<dmactx_cr::DMACTX_CR_SPEC>;
#[doc = "Channel transmit control register"]
pub mod dmactx_cr;
#[doc = "DMACRxCR (rw) register accessor: Channel receive control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dmacrx_cr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dmacrx_cr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`dmacrx_cr`]
module"]
pub type DMACRX_CR = crate::Reg<dmacrx_cr::DMACRX_CR_SPEC>;
#[doc = "Channel receive control register"]
pub mod dmacrx_cr;
#[doc = "DMACTxDLAR (rw) register accessor: Channel Tx descriptor list address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dmactx_dlar::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dmactx_dlar::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`dmactx_dlar`]
module"]
pub type DMACTX_DLAR = crate::Reg<dmactx_dlar::DMACTX_DLAR_SPEC>;
#[doc = "Channel Tx descriptor list address register"]
pub mod dmactx_dlar;
#[doc = "DMACRxDLAR (rw) register accessor: Channel Rx descriptor list address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dmacrx_dlar::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dmacrx_dlar::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`dmacrx_dlar`]
module"]
pub type DMACRX_DLAR = crate::Reg<dmacrx_dlar::DMACRX_DLAR_SPEC>;
#[doc = "Channel Rx descriptor list address register"]
pub mod dmacrx_dlar;
#[doc = "DMACTxDTPR (rw) register accessor: Channel Tx descriptor tail pointer register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dmactx_dtpr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dmactx_dtpr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`dmactx_dtpr`]
module"]
pub type DMACTX_DTPR = crate::Reg<dmactx_dtpr::DMACTX_DTPR_SPEC>;
#[doc = "Channel Tx descriptor tail pointer register"]
pub mod dmactx_dtpr;
#[doc = "DMACRxDTPR (rw) register accessor: Channel Rx descriptor tail pointer register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dmacrx_dtpr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dmacrx_dtpr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`dmacrx_dtpr`]
module"]
pub type DMACRX_DTPR = crate::Reg<dmacrx_dtpr::DMACRX_DTPR_SPEC>;
#[doc = "Channel Rx descriptor tail pointer register"]
pub mod dmacrx_dtpr;
#[doc = "DMACTxRLR (rw) register accessor: Channel Tx descriptor ring length register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dmactx_rlr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dmactx_rlr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`dmactx_rlr`]
module"]
pub type DMACTX_RLR = crate::Reg<dmactx_rlr::DMACTX_RLR_SPEC>;
#[doc = "Channel Tx descriptor ring length register"]
pub mod dmactx_rlr;
#[doc = "DMACRxRLR (rw) register accessor: Channel Rx descriptor ring length register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dmacrx_rlr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dmacrx_rlr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`dmacrx_rlr`]
module"]
pub type DMACRX_RLR = crate::Reg<dmacrx_rlr::DMACRX_RLR_SPEC>;
#[doc = "Channel Rx descriptor ring length register"]
pub mod dmacrx_rlr;
#[doc = "DMACIER (rw) register accessor: Channel interrupt enable register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dmacier::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dmacier::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`dmacier`]
module"]
pub type DMACIER = crate::Reg<dmacier::DMACIER_SPEC>;
#[doc = "Channel interrupt enable register"]
pub mod dmacier;
#[doc = "DMACRxIWTR (rw) register accessor: Channel Rx interrupt watchdog timer register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dmacrx_iwtr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dmacrx_iwtr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`dmacrx_iwtr`]
module"]
pub type DMACRX_IWTR = crate::Reg<dmacrx_iwtr::DMACRX_IWTR_SPEC>;
#[doc = "Channel Rx interrupt watchdog timer register"]
pub mod dmacrx_iwtr;
#[doc = "DMACCATxDR (r) register accessor: Channel current application transmit descriptor register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dmaccatx_dr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`dmaccatx_dr`]
module"]
pub type DMACCATX_DR = crate::Reg<dmaccatx_dr::DMACCATX_DR_SPEC>;
#[doc = "Channel current application transmit descriptor register"]
pub mod dmaccatx_dr;
#[doc = "DMACCARxDR (r) register accessor: Channel current application receive descriptor register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dmaccarx_dr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`dmaccarx_dr`]
module"]
pub type DMACCARX_DR = crate::Reg<dmaccarx_dr::DMACCARX_DR_SPEC>;
#[doc = "Channel current application receive descriptor register"]
pub mod dmaccarx_dr;
#[doc = "DMACCATxBR (r) register accessor: Channel current application transmit buffer register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dmaccatx_br::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`dmaccatx_br`]
module"]
pub type DMACCATX_BR = crate::Reg<dmaccatx_br::DMACCATX_BR_SPEC>;
#[doc = "Channel current application transmit buffer register"]
pub mod dmaccatx_br;
#[doc = "DMACCARxBR (r) register accessor: Channel current application receive buffer register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dmaccarx_br::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`dmaccarx_br`]
module"]
pub type DMACCARX_BR = crate::Reg<dmaccarx_br::DMACCARX_BR_SPEC>;
#[doc = "Channel current application receive buffer register"]
pub mod dmaccarx_br;
#[doc = "DMACSR (rw) register accessor: Channel status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dmacsr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dmacsr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`dmacsr`]
module"]
pub type DMACSR = crate::Reg<dmacsr::DMACSR_SPEC>;
#[doc = "Channel status register"]
pub mod dmacsr;
#[doc = "DMACMFCR (r) register accessor: Channel missed frame count register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dmacmfcr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`dmacmfcr`]
module"]
pub type DMACMFCR = crate::Reg<dmacmfcr::DMACMFCR_SPEC>;
#[doc = "Channel missed frame count register"]
pub mod dmacmfcr;
#[doc = "MTLOMR (rw) register accessor: Operating mode Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mtlomr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mtlomr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mtlomr`]
module"]
pub type MTLOMR = crate::Reg<mtlomr::MTLOMR_SPEC>;
#[doc = "Operating mode Register"]
pub mod mtlomr;
#[doc = "MTLISR (r) register accessor: Interrupt status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mtlisr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mtlisr`]
module"]
pub type MTLISR = crate::Reg<mtlisr::MTLISR_SPEC>;
#[doc = "Interrupt status Register"]
pub mod mtlisr;
#[doc = "MTLTxQOMR (rw) register accessor: Tx queue operating mode Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mtltx_qomr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mtltx_qomr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mtltx_qomr`]
module"]
pub type MTLTX_QOMR = crate::Reg<mtltx_qomr::MTLTX_QOMR_SPEC>;
#[doc = "Tx queue operating mode Register"]
pub mod mtltx_qomr;
#[doc = "MTLTxQUR (r) register accessor: Tx queue underflow register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mtltx_qur::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mtltx_qur`]
module"]
pub type MTLTX_QUR = crate::Reg<mtltx_qur::MTLTX_QUR_SPEC>;
#[doc = "Tx queue underflow register"]
pub mod mtltx_qur;
#[doc = "MTLTxQDR (r) register accessor: Tx queue debug Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mtltx_qdr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mtltx_qdr`]
module"]
pub type MTLTX_QDR = crate::Reg<mtltx_qdr::MTLTX_QDR_SPEC>;
#[doc = "Tx queue debug Register"]
pub mod mtltx_qdr;
#[doc = "MTLQICSR (rw) register accessor: Queue interrupt control status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mtlqicsr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mtlqicsr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mtlqicsr`]
module"]
pub type MTLQICSR = crate::Reg<mtlqicsr::MTLQICSR_SPEC>;
#[doc = "Queue interrupt control status Register"]
pub mod mtlqicsr;
#[doc = "MTLRxQOMR (rw) register accessor: Rx queue operating mode register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mtlrx_qomr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mtlrx_qomr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mtlrx_qomr`]
module"]
pub type MTLRX_QOMR = crate::Reg<mtlrx_qomr::MTLRX_QOMR_SPEC>;
#[doc = "Rx queue operating mode register"]
pub mod mtlrx_qomr;
#[doc = "MTLRxQMPOCR (r) register accessor: Rx queue missed packet and overflow counter register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mtlrx_qmpocr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mtlrx_qmpocr`]
module"]
pub type MTLRX_QMPOCR = crate::Reg<mtlrx_qmpocr::MTLRX_QMPOCR_SPEC>;
#[doc = "Rx queue missed packet and overflow counter register"]
pub mod mtlrx_qmpocr;
#[doc = "MTLRxQDR (r) register accessor: Rx queue debug register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mtlrx_qdr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mtlrx_qdr`]
module"]
pub type MTLRX_QDR = crate::Reg<mtlrx_qdr::MTLRX_QDR_SPEC>;
#[doc = "Rx queue debug register"]
pub mod mtlrx_qdr;
#[doc = "MACCR (rw) register accessor: Operating mode configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`maccr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`maccr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`maccr`]
module"]
pub type MACCR = crate::Reg<maccr::MACCR_SPEC>;
#[doc = "Operating mode configuration register"]
pub mod maccr;
#[doc = "MACECR (rw) register accessor: Extended operating mode configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`macecr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`macecr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`macecr`]
module"]
pub type MACECR = crate::Reg<macecr::MACECR_SPEC>;
#[doc = "Extended operating mode configuration register"]
pub mod macecr;
#[doc = "MACPFR (rw) register accessor: Packet filtering control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`macpfr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`macpfr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`macpfr`]
module"]
pub type MACPFR = crate::Reg<macpfr::MACPFR_SPEC>;
#[doc = "Packet filtering control register"]
pub mod macpfr;
#[doc = "MACWTR (rw) register accessor: Watchdog timeout register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`macwtr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`macwtr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`macwtr`]
module"]
pub type MACWTR = crate::Reg<macwtr::MACWTR_SPEC>;
#[doc = "Watchdog timeout register"]
pub mod macwtr;
#[doc = "MACHT0R (rw) register accessor: Hash Table 0 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`macht0r::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`macht0r::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`macht0r`]
module"]
pub type MACHT0R = crate::Reg<macht0r::MACHT0R_SPEC>;
#[doc = "Hash Table 0 register"]
pub mod macht0r;
#[doc = "MACHT1R (rw) register accessor: Hash Table 1 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`macht1r::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`macht1r::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`macht1r`]
module"]
pub type MACHT1R = crate::Reg<macht1r::MACHT1R_SPEC>;
#[doc = "Hash Table 1 register"]
pub mod macht1r;
#[doc = "MACVTR (rw) register accessor: VLAN tag register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`macvtr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`macvtr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`macvtr`]
module"]
pub type MACVTR = crate::Reg<macvtr::MACVTR_SPEC>;
#[doc = "VLAN tag register"]
pub mod macvtr;
#[doc = "MACVHTR (rw) register accessor: VLAN Hash table register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`macvhtr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`macvhtr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`macvhtr`]
module"]
pub type MACVHTR = crate::Reg<macvhtr::MACVHTR_SPEC>;
#[doc = "VLAN Hash table register"]
pub mod macvhtr;
#[doc = "MACVIR (rw) register accessor: VLAN inclusion register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`macvir::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`macvir::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`macvir`]
module"]
pub type MACVIR = crate::Reg<macvir::MACVIR_SPEC>;
#[doc = "VLAN inclusion register"]
pub mod macvir;
#[doc = "MACIVIR (rw) register accessor: Inner VLAN inclusion register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`macivir::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`macivir::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`macivir`]
module"]
pub type MACIVIR = crate::Reg<macivir::MACIVIR_SPEC>;
#[doc = "Inner VLAN inclusion register"]
pub mod macivir;
#[doc = "MACQTxFCR (rw) register accessor: Tx Queue flow control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`macqtx_fcr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`macqtx_fcr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`macqtx_fcr`]
module"]
pub type MACQTX_FCR = crate::Reg<macqtx_fcr::MACQTX_FCR_SPEC>;
#[doc = "Tx Queue flow control register"]
pub mod macqtx_fcr;
#[doc = "MACRxFCR (rw) register accessor: Rx flow control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`macrx_fcr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`macrx_fcr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`macrx_fcr`]
module"]
pub type MACRX_FCR = crate::Reg<macrx_fcr::MACRX_FCR_SPEC>;
#[doc = "Rx flow control register"]
pub mod macrx_fcr;
#[doc = "MACISR (r) register accessor: Interrupt status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`macisr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`macisr`]
module"]
pub type MACISR = crate::Reg<macisr::MACISR_SPEC>;
#[doc = "Interrupt status register"]
pub mod macisr;
#[doc = "MACIER (rw) register accessor: Interrupt enable register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`macier::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`macier::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`macier`]
module"]
pub type MACIER = crate::Reg<macier::MACIER_SPEC>;
#[doc = "Interrupt enable register"]
pub mod macier;
#[doc = "MACRxTxSR (r) register accessor: Rx Tx status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`macrx_tx_sr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`macrx_tx_sr`]
module"]
pub type MACRX_TX_SR = crate::Reg<macrx_tx_sr::MACRX_TX_SR_SPEC>;
#[doc = "Rx Tx status register"]
pub mod macrx_tx_sr;
#[doc = "MACPCSR (rw) register accessor: PMT control status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`macpcsr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`macpcsr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`macpcsr`]
module"]
pub type MACPCSR = crate::Reg<macpcsr::MACPCSR_SPEC>;
#[doc = "PMT control status register"]
pub mod macpcsr;
#[doc = "MACRWKPFR (rw) register accessor: Remove wakeup packet filter register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`macrwkpfr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`macrwkpfr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`macrwkpfr`]
module"]
pub type MACRWKPFR = crate::Reg<macrwkpfr::MACRWKPFR_SPEC>;
#[doc = "Remove wakeup packet filter register"]
pub mod macrwkpfr;
#[doc = "MACLCSR (rw) register accessor: LPI control status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`maclcsr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`maclcsr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`maclcsr`]
module"]
pub type MACLCSR = crate::Reg<maclcsr::MACLCSR_SPEC>;
#[doc = "LPI control status register"]
pub mod maclcsr;
#[doc = "MACLTCR (rw) register accessor: LPI timers control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`macltcr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`macltcr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`macltcr`]
module"]
pub type MACLTCR = crate::Reg<macltcr::MACLTCR_SPEC>;
#[doc = "LPI timers control register"]
pub mod macltcr;
#[doc = "MACLETR (rw) register accessor: LPI entry timer register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`macletr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`macletr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`macletr`]
module"]
pub type MACLETR = crate::Reg<macletr::MACLETR_SPEC>;
#[doc = "LPI entry timer register"]
pub mod macletr;
#[doc = "MAC1USTCR (rw) register accessor: 1-microsecond-tick counter register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mac1ustcr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mac1ustcr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mac1ustcr`]
module"]
pub type MAC1USTCR = crate::Reg<mac1ustcr::MAC1USTCR_SPEC>;
#[doc = "1-microsecond-tick counter register"]
pub mod mac1ustcr;
#[doc = "MACVR (r) register accessor: Version register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`macvr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`macvr`]
module"]
pub type MACVR = crate::Reg<macvr::MACVR_SPEC>;
#[doc = "Version register"]
pub mod macvr;
#[doc = "MACHWF1R (r) register accessor: HW feature 1 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`machwf1r::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`machwf1r`]
module"]
pub type MACHWF1R = crate::Reg<machwf1r::MACHWF1R_SPEC>;
#[doc = "HW feature 1 register"]
pub mod machwf1r;
#[doc = "MACHWF2R (r) register accessor: HW feature 2 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`machwf2r::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`machwf2r`]
module"]
pub type MACHWF2R = crate::Reg<machwf2r::MACHWF2R_SPEC>;
#[doc = "HW feature 2 register"]
pub mod machwf2r;
#[doc = "MACMDIOAR (rw) register accessor: MDIO address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`macmdioar::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`macmdioar::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`macmdioar`]
module"]
pub type MACMDIOAR = crate::Reg<macmdioar::MACMDIOAR_SPEC>;
#[doc = "MDIO address register"]
pub mod macmdioar;
#[doc = "MACMDIODR (rw) register accessor: MDIO data register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`macmdiodr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`macmdiodr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`macmdiodr`]
module"]
pub type MACMDIODR = crate::Reg<macmdiodr::MACMDIODR_SPEC>;
#[doc = "MDIO data register"]
pub mod macmdiodr;
#[doc = "MACARPAR (rw) register accessor: ARP address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`macarpar::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`macarpar::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`macarpar`]
module"]
pub type MACARPAR = crate::Reg<macarpar::MACARPAR_SPEC>;
#[doc = "ARP address register"]
pub mod macarpar;
#[doc = "MACA0HR (rw) register accessor: Address 0 high register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`maca0hr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`maca0hr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`maca0hr`]
module"]
pub type MACA0HR = crate::Reg<maca0hr::MACA0HR_SPEC>;
#[doc = "Address 0 high register"]
pub mod maca0hr;
#[doc = "MACA0LR (rw) register accessor: Address 0 low register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`maca0lr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`maca0lr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`maca0lr`]
module"]
pub type MACA0LR = crate::Reg<maca0lr::MACA0LR_SPEC>;
#[doc = "Address 0 low register"]
pub mod maca0lr;
#[doc = "MACA1LR (rw) register accessor: Address 1 low register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`maca1lr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`maca1lr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`maca1lr`]
module"]
pub type MACA1LR = crate::Reg<maca1lr::MACA1LR_SPEC>;
#[doc = "Address 1 low register"]
pub mod maca1lr;
#[doc = "MACA2LR (rw) register accessor: Address 2 low register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`maca2lr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`maca2lr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`maca2lr`]
module"]
pub type MACA2LR = crate::Reg<maca2lr::MACA2LR_SPEC>;
#[doc = "Address 2 low register"]
pub mod maca2lr;
#[doc = "MACA3LR (rw) register accessor: Address 3 low register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`maca3lr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`maca3lr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`maca3lr`]
module"]
pub type MACA3LR = crate::Reg<maca3lr::MACA3LR_SPEC>;
#[doc = "Address 3 low register"]
pub mod maca3lr;
#[doc = "MACA1HR (rw) register accessor: Address 1 high register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`maca1hr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`maca1hr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`maca1hr`]
module"]
pub type MACA1HR = crate::Reg<maca1hr::MACA1HR_SPEC>;
#[doc = "Address 1 high register"]
pub mod maca1hr;
#[doc = "MACA2HR (rw) register accessor: Address 2 high register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`maca2hr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`maca2hr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`maca2hr`]
module"]
pub type MACA2HR = crate::Reg<maca2hr::MACA2HR_SPEC>;
#[doc = "Address 2 high register"]
pub mod maca2hr;
#[doc = "MACA3HR (rw) register accessor: Address 3 high register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`maca3hr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`maca3hr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`maca3hr`]
module"]
pub type MACA3HR = crate::Reg<maca3hr::MACA3HR_SPEC>;
#[doc = "Address 3 high register"]
pub mod maca3hr;
#[doc = "MMC_CONTROL (rw) register accessor: MMC control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mmc_control::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mmc_control::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mmc_control`]
module"]
pub type MMC_CONTROL = crate::Reg<mmc_control::MMC_CONTROL_SPEC>;
#[doc = "MMC control register"]
pub mod mmc_control;
#[doc = "MMC_RX_INTERRUPT (r) register accessor: MMC Rx interrupt register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mmc_rx_interrupt::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mmc_rx_interrupt`]
module"]
pub type MMC_RX_INTERRUPT = crate::Reg<mmc_rx_interrupt::MMC_RX_INTERRUPT_SPEC>;
#[doc = "MMC Rx interrupt register"]
pub mod mmc_rx_interrupt;
#[doc = "MMC_TX_INTERRUPT (r) register accessor: MMC Tx interrupt register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mmc_tx_interrupt::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mmc_tx_interrupt`]
module"]
pub type MMC_TX_INTERRUPT = crate::Reg<mmc_tx_interrupt::MMC_TX_INTERRUPT_SPEC>;
#[doc = "MMC Tx interrupt register"]
pub mod mmc_tx_interrupt;
#[doc = "MMC_RX_INTERRUPT_MASK (rw) register accessor: MMC Rx interrupt mask register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mmc_rx_interrupt_mask::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mmc_rx_interrupt_mask::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mmc_rx_interrupt_mask`]
module"]
pub type MMC_RX_INTERRUPT_MASK = crate::Reg<mmc_rx_interrupt_mask::MMC_RX_INTERRUPT_MASK_SPEC>;
#[doc = "MMC Rx interrupt mask register"]
pub mod mmc_rx_interrupt_mask;
#[doc = "MMC_TX_INTERRUPT_MASK (rw) register accessor: MMC Tx interrupt mask register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mmc_tx_interrupt_mask::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mmc_tx_interrupt_mask::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mmc_tx_interrupt_mask`]
module"]
pub type MMC_TX_INTERRUPT_MASK = crate::Reg<mmc_tx_interrupt_mask::MMC_TX_INTERRUPT_MASK_SPEC>;
#[doc = "MMC Tx interrupt mask register"]
pub mod mmc_tx_interrupt_mask;
#[doc = "TX_SINGLE_COLLISION_GOOD_PACKETS (r) register accessor: Tx single collision good packets register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tx_single_collision_good_packets::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`tx_single_collision_good_packets`]
module"]
pub type TX_SINGLE_COLLISION_GOOD_PACKETS =
    crate::Reg<tx_single_collision_good_packets::TX_SINGLE_COLLISION_GOOD_PACKETS_SPEC>;
#[doc = "Tx single collision good packets register"]
pub mod tx_single_collision_good_packets;
#[doc = "TX_MULTIPLE_COLLISION_GOOD_PACKETS (r) register accessor: Tx multiple collision good packets register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tx_multiple_collision_good_packets::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`tx_multiple_collision_good_packets`]
module"]
pub type TX_MULTIPLE_COLLISION_GOOD_PACKETS =
    crate::Reg<tx_multiple_collision_good_packets::TX_MULTIPLE_COLLISION_GOOD_PACKETS_SPEC>;
#[doc = "Tx multiple collision good packets register"]
pub mod tx_multiple_collision_good_packets;
#[doc = "TX_PACKET_COUNT_GOOD (r) register accessor: Tx packet count good register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tx_packet_count_good::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`tx_packet_count_good`]
module"]
pub type TX_PACKET_COUNT_GOOD = crate::Reg<tx_packet_count_good::TX_PACKET_COUNT_GOOD_SPEC>;
#[doc = "Tx packet count good register"]
pub mod tx_packet_count_good;
#[doc = "RX_CRC_ERROR_PACKETS (r) register accessor: Rx CRC error packets register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rx_crc_error_packets::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`rx_crc_error_packets`]
module"]
pub type RX_CRC_ERROR_PACKETS = crate::Reg<rx_crc_error_packets::RX_CRC_ERROR_PACKETS_SPEC>;
#[doc = "Rx CRC error packets register"]
pub mod rx_crc_error_packets;
#[doc = "RX_ALIGNMENT_ERROR_PACKETS (r) register accessor: Rx alignment error packets register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rx_alignment_error_packets::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`rx_alignment_error_packets`]
module"]
pub type RX_ALIGNMENT_ERROR_PACKETS =
    crate::Reg<rx_alignment_error_packets::RX_ALIGNMENT_ERROR_PACKETS_SPEC>;
#[doc = "Rx alignment error packets register"]
pub mod rx_alignment_error_packets;
#[doc = "RX_UNICAST_PACKETS_GOOD (r) register accessor: Rx unicast packets good register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rx_unicast_packets_good::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`rx_unicast_packets_good`]
module"]
pub type RX_UNICAST_PACKETS_GOOD =
    crate::Reg<rx_unicast_packets_good::RX_UNICAST_PACKETS_GOOD_SPEC>;
#[doc = "Rx unicast packets good register"]
pub mod rx_unicast_packets_good;
#[doc = "TX_LPI_USEC_CNTR (r) register accessor: Tx LPI microsecond timer register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tx_lpi_usec_cntr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`tx_lpi_usec_cntr`]
module"]
pub type TX_LPI_USEC_CNTR = crate::Reg<tx_lpi_usec_cntr::TX_LPI_USEC_CNTR_SPEC>;
#[doc = "Tx LPI microsecond timer register"]
pub mod tx_lpi_usec_cntr;
#[doc = "TX_LPI_TRAN_CNTR (r) register accessor: Tx LPI transition counter register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tx_lpi_tran_cntr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`tx_lpi_tran_cntr`]
module"]
pub type TX_LPI_TRAN_CNTR = crate::Reg<tx_lpi_tran_cntr::TX_LPI_TRAN_CNTR_SPEC>;
#[doc = "Tx LPI transition counter register"]
pub mod tx_lpi_tran_cntr;
#[doc = "RX_LPI_USEC_CNTR (r) register accessor: Rx LPI microsecond counter register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rx_lpi_usec_cntr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`rx_lpi_usec_cntr`]
module"]
pub type RX_LPI_USEC_CNTR = crate::Reg<rx_lpi_usec_cntr::RX_LPI_USEC_CNTR_SPEC>;
#[doc = "Rx LPI microsecond counter register"]
pub mod rx_lpi_usec_cntr;
#[doc = "RX_LPI_TRAN_CNTR (r) register accessor: Rx LPI transition counter register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rx_lpi_tran_cntr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`rx_lpi_tran_cntr`]
module"]
pub type RX_LPI_TRAN_CNTR = crate::Reg<rx_lpi_tran_cntr::RX_LPI_TRAN_CNTR_SPEC>;
#[doc = "Rx LPI transition counter register"]
pub mod rx_lpi_tran_cntr;
#[doc = "MACL3L4C0R (rw) register accessor: L3 and L4 control 0 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`macl3l4c0r::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`macl3l4c0r::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`macl3l4c0r`]
module"]
pub type MACL3L4C0R = crate::Reg<macl3l4c0r::MACL3L4C0R_SPEC>;
#[doc = "L3 and L4 control 0 register"]
pub mod macl3l4c0r;
#[doc = "MACL4A0R (rw) register accessor: Layer4 address filter 0 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`macl4a0r::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`macl4a0r::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`macl4a0r`]
module"]
pub type MACL4A0R = crate::Reg<macl4a0r::MACL4A0R_SPEC>;
#[doc = "Layer4 address filter 0 register"]
pub mod macl4a0r;
#[doc = "MACDR (r) register accessor: Debug register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`macdr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`macdr`]
module"]
pub type MACDR = crate::Reg<macdr::MACDR_SPEC>;
#[doc = "Debug register"]
pub mod macdr;
#[doc = "MACL3A00R (rw) register accessor: MACL3A00R\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`macl3a00r::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`macl3a00r::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`macl3a00r`]
module"]
pub type MACL3A00R = crate::Reg<macl3a00r::MACL3A00R_SPEC>;
#[doc = "MACL3A00R"]
pub mod macl3a00r;
#[doc = "MACL3A10R (rw) register accessor: Layer3 address 1 filter 0 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`macl3a10r::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`macl3a10r::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`macl3a10r`]
module"]
pub type MACL3A10R = crate::Reg<macl3a10r::MACL3A10R_SPEC>;
#[doc = "Layer3 address 1 filter 0 register"]
pub mod macl3a10r;
#[doc = "MACL3A20 (rw) register accessor: Layer3 Address 2 filter 0 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`macl3a20::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`macl3a20::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`macl3a20`]
module"]
pub type MACL3A20 = crate::Reg<macl3a20::MACL3A20_SPEC>;
#[doc = "Layer3 Address 2 filter 0 register"]
pub mod macl3a20;
#[doc = "MACL3A30 (rw) register accessor: Layer3 Address 3 filter 0 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`macl3a30::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`macl3a30::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`macl3a30`]
module"]
pub type MACL3A30 = crate::Reg<macl3a30::MACL3A30_SPEC>;
#[doc = "Layer3 Address 3 filter 0 register"]
pub mod macl3a30;
#[doc = "MACL3L4C1R (rw) register accessor: L3 and L4 control 1 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`macl3l4c1r::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`macl3l4c1r::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`macl3l4c1r`]
module"]
pub type MACL3L4C1R = crate::Reg<macl3l4c1r::MACL3L4C1R_SPEC>;
#[doc = "L3 and L4 control 1 register"]
pub mod macl3l4c1r;
#[doc = "MACL4A1R (rw) register accessor: Layer 4 address filter 1 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`macl4a1r::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`macl4a1r::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`macl4a1r`]
module"]
pub type MACL4A1R = crate::Reg<macl4a1r::MACL4A1R_SPEC>;
#[doc = "Layer 4 address filter 1 register"]
pub mod macl4a1r;
#[doc = "MACL3A01R (rw) register accessor: Layer3 address 0 filter 1 Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`macl3a01r::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`macl3a01r::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`macl3a01r`]
module"]
pub type MACL3A01R = crate::Reg<macl3a01r::MACL3A01R_SPEC>;
#[doc = "Layer3 address 0 filter 1 Register"]
pub mod macl3a01r;
#[doc = "MACL3A11R (rw) register accessor: Layer3 address 1 filter 1 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`macl3a11r::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`macl3a11r::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`macl3a11r`]
module"]
pub type MACL3A11R = crate::Reg<macl3a11r::MACL3A11R_SPEC>;
#[doc = "Layer3 address 1 filter 1 register"]
pub mod macl3a11r;
#[doc = "MACL3A21R (rw) register accessor: Layer3 address 2 filter 1 Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`macl3a21r::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`macl3a21r::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`macl3a21r`]
module"]
pub type MACL3A21R = crate::Reg<macl3a21r::MACL3A21R_SPEC>;
#[doc = "Layer3 address 2 filter 1 Register"]
pub mod macl3a21r;
#[doc = "MACL3A31R (rw) register accessor: Layer3 address 3 filter 1 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`macl3a31r::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`macl3a31r::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`macl3a31r`]
module"]
pub type MACL3A31R = crate::Reg<macl3a31r::MACL3A31R_SPEC>;
#[doc = "Layer3 address 3 filter 1 register"]
pub mod macl3a31r;
#[doc = "MACTSCR (rw) register accessor: Timestamp control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mactscr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mactscr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mactscr`]
module"]
pub type MACTSCR = crate::Reg<mactscr::MACTSCR_SPEC>;
#[doc = "Timestamp control Register"]
pub mod mactscr;
#[doc = "MACSSIR (rw) register accessor: Sub-second increment register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`macssir::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`macssir::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`macssir`]
module"]
pub type MACSSIR = crate::Reg<macssir::MACSSIR_SPEC>;
#[doc = "Sub-second increment register"]
pub mod macssir;
#[doc = "MACSTSR (r) register accessor: System time seconds register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`macstsr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`macstsr`]
module"]
pub type MACSTSR = crate::Reg<macstsr::MACSTSR_SPEC>;
#[doc = "System time seconds register"]
pub mod macstsr;
#[doc = "MACSTNR (r) register accessor: System time nanoseconds register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`macstnr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`macstnr`]
module"]
pub type MACSTNR = crate::Reg<macstnr::MACSTNR_SPEC>;
#[doc = "System time nanoseconds register"]
pub mod macstnr;
#[doc = "MACSTSUR (rw) register accessor: System time seconds update register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`macstsur::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`macstsur::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`macstsur`]
module"]
pub type MACSTSUR = crate::Reg<macstsur::MACSTSUR_SPEC>;
#[doc = "System time seconds update register"]
pub mod macstsur;
#[doc = "MACSTNUR (rw) register accessor: System time nanoseconds update register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`macstnur::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`macstnur::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`macstnur`]
module"]
pub type MACSTNUR = crate::Reg<macstnur::MACSTNUR_SPEC>;
#[doc = "System time nanoseconds update register"]
pub mod macstnur;
#[doc = "MACTSAR (rw) register accessor: Timestamp addend register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mactsar::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mactsar::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mactsar`]
module"]
pub type MACTSAR = crate::Reg<mactsar::MACTSAR_SPEC>;
#[doc = "Timestamp addend register"]
pub mod mactsar;
#[doc = "MACTSSR (r) register accessor: Timestamp status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mactssr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mactssr`]
module"]
pub type MACTSSR = crate::Reg<mactssr::MACTSSR_SPEC>;
#[doc = "Timestamp status register"]
pub mod mactssr;
#[doc = "MACTxTSSNR (r) register accessor: Tx timestamp status nanoseconds register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mactx_tssnr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mactx_tssnr`]
module"]
pub type MACTX_TSSNR = crate::Reg<mactx_tssnr::MACTX_TSSNR_SPEC>;
#[doc = "Tx timestamp status nanoseconds register"]
pub mod mactx_tssnr;
#[doc = "MACTxTSSSR (r) register accessor: Tx timestamp status seconds register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mactx_tsssr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mactx_tsssr`]
module"]
pub type MACTX_TSSSR = crate::Reg<mactx_tsssr::MACTX_TSSSR_SPEC>;
#[doc = "Tx timestamp status seconds register"]
pub mod mactx_tsssr;
#[doc = "MACACR (rw) register accessor: Auxiliary control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`macacr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`macacr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`macacr`]
module"]
pub type MACACR = crate::Reg<macacr::MACACR_SPEC>;
#[doc = "Auxiliary control register"]
pub mod macacr;
#[doc = "MACATSNR (r) register accessor: Auxiliary timestamp nanoseconds register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`macatsnr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`macatsnr`]
module"]
pub type MACATSNR = crate::Reg<macatsnr::MACATSNR_SPEC>;
#[doc = "Auxiliary timestamp nanoseconds register"]
pub mod macatsnr;
#[doc = "MACATSSR (r) register accessor: Auxiliary timestamp seconds register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`macatssr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`macatssr`]
module"]
pub type MACATSSR = crate::Reg<macatssr::MACATSSR_SPEC>;
#[doc = "Auxiliary timestamp seconds register"]
pub mod macatssr;
#[doc = "MACTSIACR (rw) register accessor: Timestamp Ingress asymmetric correction register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mactsiacr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mactsiacr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mactsiacr`]
module"]
pub type MACTSIACR = crate::Reg<mactsiacr::MACTSIACR_SPEC>;
#[doc = "Timestamp Ingress asymmetric correction register"]
pub mod mactsiacr;
#[doc = "MACTSEACR (rw) register accessor: Timestamp Egress asymmetric correction register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mactseacr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mactseacr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mactseacr`]
module"]
pub type MACTSEACR = crate::Reg<mactseacr::MACTSEACR_SPEC>;
#[doc = "Timestamp Egress asymmetric correction register"]
pub mod mactseacr;
#[doc = "MACTSICNR (rw) register accessor: Timestamp Ingress correction nanosecond register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mactsicnr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mactsicnr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mactsicnr`]
module"]
pub type MACTSICNR = crate::Reg<mactsicnr::MACTSICNR_SPEC>;
#[doc = "Timestamp Ingress correction nanosecond register"]
pub mod mactsicnr;
#[doc = "MACTSECNR (rw) register accessor: Timestamp Egress correction nanosecond register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mactsecnr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mactsecnr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mactsecnr`]
module"]
pub type MACTSECNR = crate::Reg<mactsecnr::MACTSECNR_SPEC>;
#[doc = "Timestamp Egress correction nanosecond register"]
pub mod mactsecnr;
#[doc = "MACPPSCR (rw) register accessor: PPS control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`macppscr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`macppscr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`macppscr`]
module"]
pub type MACPPSCR = crate::Reg<macppscr::MACPPSCR_SPEC>;
#[doc = "PPS control register"]
pub mod macppscr;
#[doc = "MACPPSTTSR (rw) register accessor: PPS target time seconds register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`macppsttsr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`macppsttsr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`macppsttsr`]
module"]
pub type MACPPSTTSR = crate::Reg<macppsttsr::MACPPSTTSR_SPEC>;
#[doc = "PPS target time seconds register"]
pub mod macppsttsr;
#[doc = "MACPPSTTNR (rw) register accessor: PPS target time nanoseconds register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`macppsttnr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`macppsttnr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`macppsttnr`]
module"]
pub type MACPPSTTNR = crate::Reg<macppsttnr::MACPPSTTNR_SPEC>;
#[doc = "PPS target time nanoseconds register"]
pub mod macppsttnr;
#[doc = "MACPPSIR (rw) register accessor: PPS interval register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`macppsir::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`macppsir::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`macppsir`]
module"]
pub type MACPPSIR = crate::Reg<macppsir::MACPPSIR_SPEC>;
#[doc = "PPS interval register"]
pub mod macppsir;
#[doc = "MACPPSWR (rw) register accessor: PPS width register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`macppswr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`macppswr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`macppswr`]
module"]
pub type MACPPSWR = crate::Reg<macppswr::MACPPSWR_SPEC>;
#[doc = "PPS width register"]
pub mod macppswr;
#[doc = "MACPOCR (rw) register accessor: PTP Offload control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`macpocr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`macpocr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`macpocr`]
module"]
pub type MACPOCR = crate::Reg<macpocr::MACPOCR_SPEC>;
#[doc = "PTP Offload control register"]
pub mod macpocr;
#[doc = "MACSPI0R (rw) register accessor: PTP Source Port Identity 0 Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`macspi0r::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`macspi0r::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`macspi0r`]
module"]
pub type MACSPI0R = crate::Reg<macspi0r::MACSPI0R_SPEC>;
#[doc = "PTP Source Port Identity 0 Register"]
pub mod macspi0r;
#[doc = "MACSPI1R (rw) register accessor: PTP Source port identity 1 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`macspi1r::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`macspi1r::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`macspi1r`]
module"]
pub type MACSPI1R = crate::Reg<macspi1r::MACSPI1R_SPEC>;
#[doc = "PTP Source port identity 1 register"]
pub mod macspi1r;
#[doc = "MACSPI2R (rw) register accessor: PTP Source port identity 2 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`macspi2r::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`macspi2r::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`macspi2r`]
module"]
pub type MACSPI2R = crate::Reg<macspi2r::MACSPI2R_SPEC>;
#[doc = "PTP Source port identity 2 register"]
pub mod macspi2r;
#[doc = "MACLMIR (rw) register accessor: Log message interval register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`maclmir::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`maclmir::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`maclmir`]
module"]
pub type MACLMIR = crate::Reg<maclmir::MACLMIR_SPEC>;
#[doc = "Log message interval register"]
pub mod maclmir;
