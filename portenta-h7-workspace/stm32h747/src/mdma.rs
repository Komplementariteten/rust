#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - MDMA Global Interrupt/Status Register"]
    pub mdma_gisr0: MDMA_GISR0,
    _reserved1: [u8; 0x3c],
    #[doc = "0x40 - MDMA channel x interrupt/status register"]
    pub mdma_c0isr: MDMA_C0ISR,
    #[doc = "0x44 - MDMA channel x interrupt flag clear register"]
    pub mdma_c0ifcr: MDMA_C0IFCR,
    #[doc = "0x48 - MDMA Channel x error status register"]
    pub mdma_c0esr: MDMA_C0ESR,
    #[doc = "0x4c - This register is used to control the concerned channel."]
    pub mdma_c0cr: MDMA_C0CR,
    #[doc = "0x50 - This register is used to configure the concerned channel."]
    pub mdma_c0tcr: MDMA_C0TCR,
    #[doc = "0x54 - MDMA Channel x block number of data register"]
    pub mdma_c0bndtr: MDMA_C0BNDTR,
    #[doc = "0x58 - MDMA channel x source address register"]
    pub mdma_c0sar: MDMA_C0SAR,
    #[doc = "0x5c - MDMA channel x destination address register"]
    pub mdma_c0dar: MDMA_C0DAR,
    #[doc = "0x60 - MDMA channel x Block Repeat address Update register"]
    pub mdma_c0brur: MDMA_C0BRUR,
    #[doc = "0x64 - MDMA channel x Link Address register"]
    pub mdma_c0lar: MDMA_C0LAR,
    #[doc = "0x68 - MDMA channel x Trigger and Bus selection Register"]
    pub mdma_c0tbr: MDMA_C0TBR,
    _reserved12: [u8; 0x04],
    #[doc = "0x70 - MDMA channel x Mask address register"]
    pub mdma_c0mar: MDMA_C0MAR,
    #[doc = "0x74 - MDMA channel x Mask Data register"]
    pub mdma_c0mdr: MDMA_C0MDR,
    _reserved14: [u8; 0x08],
    #[doc = "0x80 - MDMA channel x interrupt/status register"]
    pub mdma_c1isr: MDMA_C1ISR,
    #[doc = "0x84 - MDMA channel x interrupt flag clear register"]
    pub mdma_c1ifcr: MDMA_C1IFCR,
    #[doc = "0x88 - MDMA Channel x error status register"]
    pub mdma_c1esr: MDMA_C1ESR,
    #[doc = "0x8c - This register is used to control the concerned channel."]
    pub mdma_c1cr: MDMA_C1CR,
    #[doc = "0x90 - This register is used to configure the concerned channel."]
    pub mdma_c1tcr: MDMA_C1TCR,
    #[doc = "0x94 - MDMA Channel x block number of data register"]
    pub mdma_c1bndtr: MDMA_C1BNDTR,
    #[doc = "0x98 - MDMA channel x source address register"]
    pub mdma_c1sar: MDMA_C1SAR,
    #[doc = "0x9c - MDMA channel x destination address register"]
    pub mdma_c1dar: MDMA_C1DAR,
    #[doc = "0xa0 - MDMA channel x Block Repeat address Update register"]
    pub mdma_c1brur: MDMA_C1BRUR,
    #[doc = "0xa4 - MDMA channel x Link Address register"]
    pub mdma_c1lar: MDMA_C1LAR,
    #[doc = "0xa8 - MDMA channel x Trigger and Bus selection Register"]
    pub mdma_c1tbr: MDMA_C1TBR,
    _reserved25: [u8; 0x04],
    #[doc = "0xb0 - MDMA channel x Mask address register"]
    pub mdma_c1mar: MDMA_C1MAR,
    #[doc = "0xb4 - MDMA channel x Mask Data register"]
    pub mdma_c1mdr: MDMA_C1MDR,
    _reserved27: [u8; 0x08],
    #[doc = "0xc0 - MDMA channel x interrupt/status register"]
    pub mdma_c2isr: MDMA_C2ISR,
    #[doc = "0xc4 - MDMA channel x interrupt flag clear register"]
    pub mdma_c2ifcr: MDMA_C2IFCR,
    #[doc = "0xc8 - MDMA Channel x error status register"]
    pub mdma_c2esr: MDMA_C2ESR,
    #[doc = "0xcc - This register is used to control the concerned channel."]
    pub mdma_c2cr: MDMA_C2CR,
    #[doc = "0xd0 - This register is used to configure the concerned channel."]
    pub mdma_c2tcr: MDMA_C2TCR,
    #[doc = "0xd4 - MDMA Channel x block number of data register"]
    pub mdma_c2bndtr: MDMA_C2BNDTR,
    #[doc = "0xd8 - MDMA channel x source address register"]
    pub mdma_c2sar: MDMA_C2SAR,
    #[doc = "0xdc - MDMA channel x destination address register"]
    pub mdma_c2dar: MDMA_C2DAR,
    #[doc = "0xe0 - MDMA channel x Block Repeat address Update register"]
    pub mdma_c2brur: MDMA_C2BRUR,
    #[doc = "0xe4 - MDMA channel x Link Address register"]
    pub mdma_c2lar: MDMA_C2LAR,
    #[doc = "0xe8 - MDMA channel x Trigger and Bus selection Register"]
    pub mdma_c2tbr: MDMA_C2TBR,
    _reserved38: [u8; 0x04],
    #[doc = "0xf0 - MDMA channel x Mask address register"]
    pub mdma_c2mar: MDMA_C2MAR,
    #[doc = "0xf4 - MDMA channel x Mask Data register"]
    pub mdma_c2mdr: MDMA_C2MDR,
    _reserved40: [u8; 0x08],
    #[doc = "0x100 - MDMA channel x interrupt/status register"]
    pub mdma_c3isr: MDMA_C3ISR,
    #[doc = "0x104 - MDMA channel x interrupt flag clear register"]
    pub mdma_c3ifcr: MDMA_C3IFCR,
    #[doc = "0x108 - MDMA Channel x error status register"]
    pub mdma_c3esr: MDMA_C3ESR,
    #[doc = "0x10c - This register is used to control the concerned channel."]
    pub mdma_c3cr: MDMA_C3CR,
    #[doc = "0x110 - This register is used to configure the concerned channel."]
    pub mdma_c3tcr: MDMA_C3TCR,
    #[doc = "0x114 - MDMA Channel x block number of data register"]
    pub mdma_c3bndtr: MDMA_C3BNDTR,
    #[doc = "0x118 - MDMA channel x source address register"]
    pub mdma_c3sar: MDMA_C3SAR,
    #[doc = "0x11c - MDMA channel x destination address register"]
    pub mdma_c3dar: MDMA_C3DAR,
    #[doc = "0x120 - MDMA channel x Block Repeat address Update register"]
    pub mdma_c3brur: MDMA_C3BRUR,
    #[doc = "0x124 - MDMA channel x Link Address register"]
    pub mdma_c3lar: MDMA_C3LAR,
    #[doc = "0x128 - MDMA channel x Trigger and Bus selection Register"]
    pub mdma_c3tbr: MDMA_C3TBR,
    _reserved51: [u8; 0x04],
    #[doc = "0x130 - MDMA channel x Mask address register"]
    pub mdma_c3mar: MDMA_C3MAR,
    #[doc = "0x134 - MDMA channel x Mask Data register"]
    pub mdma_c3mdr: MDMA_C3MDR,
    _reserved53: [u8; 0x08],
    #[doc = "0x140 - MDMA channel x interrupt/status register"]
    pub mdma_c4isr: MDMA_C4ISR,
    #[doc = "0x144 - MDMA channel x interrupt flag clear register"]
    pub mdma_c4ifcr: MDMA_C4IFCR,
    #[doc = "0x148 - MDMA Channel x error status register"]
    pub mdma_c4esr: MDMA_C4ESR,
    #[doc = "0x14c - This register is used to control the concerned channel."]
    pub mdma_c4cr: MDMA_C4CR,
    #[doc = "0x150 - This register is used to configure the concerned channel."]
    pub mdma_c4tcr: MDMA_C4TCR,
    #[doc = "0x154 - MDMA Channel x block number of data register"]
    pub mdma_c4bndtr: MDMA_C4BNDTR,
    #[doc = "0x158 - MDMA channel x source address register"]
    pub mdma_c4sar: MDMA_C4SAR,
    #[doc = "0x15c - MDMA channel x destination address register"]
    pub mdma_c4dar: MDMA_C4DAR,
    #[doc = "0x160 - MDMA channel x Block Repeat address Update register"]
    pub mdma_c4brur: MDMA_C4BRUR,
    #[doc = "0x164 - MDMA channel x Link Address register"]
    pub mdma_c4lar: MDMA_C4LAR,
    #[doc = "0x168 - MDMA channel x Trigger and Bus selection Register"]
    pub mdma_c4tbr: MDMA_C4TBR,
    _reserved64: [u8; 0x04],
    #[doc = "0x170 - MDMA channel x Mask address register"]
    pub mdma_c4mar: MDMA_C4MAR,
    #[doc = "0x174 - MDMA channel x Mask Data register"]
    pub mdma_c4mdr: MDMA_C4MDR,
    _reserved66: [u8; 0x08],
    #[doc = "0x180 - MDMA channel x interrupt/status register"]
    pub mdma_c5isr: MDMA_C5ISR,
    #[doc = "0x184 - MDMA channel x interrupt flag clear register"]
    pub mdma_c5ifcr: MDMA_C5IFCR,
    #[doc = "0x188 - MDMA Channel x error status register"]
    pub mdma_c5esr: MDMA_C5ESR,
    #[doc = "0x18c - This register is used to control the concerned channel."]
    pub mdma_c5cr: MDMA_C5CR,
    #[doc = "0x190 - This register is used to configure the concerned channel."]
    pub mdma_c5tcr: MDMA_C5TCR,
    #[doc = "0x194 - MDMA Channel x block number of data register"]
    pub mdma_c5bndtr: MDMA_C5BNDTR,
    #[doc = "0x198 - MDMA channel x source address register"]
    pub mdma_c5sar: MDMA_C5SAR,
    #[doc = "0x19c - MDMA channel x destination address register"]
    pub mdma_c5dar: MDMA_C5DAR,
    #[doc = "0x1a0 - MDMA channel x Block Repeat address Update register"]
    pub mdma_c5brur: MDMA_C5BRUR,
    #[doc = "0x1a4 - MDMA channel x Link Address register"]
    pub mdma_c5lar: MDMA_C5LAR,
    #[doc = "0x1a8 - MDMA channel x Trigger and Bus selection Register"]
    pub mdma_c5tbr: MDMA_C5TBR,
    _reserved77: [u8; 0x04],
    #[doc = "0x1b0 - MDMA channel x Mask address register"]
    pub mdma_c5mar: MDMA_C5MAR,
    #[doc = "0x1b4 - MDMA channel x Mask Data register"]
    pub mdma_c5mdr: MDMA_C5MDR,
    _reserved79: [u8; 0x08],
    #[doc = "0x1c0 - MDMA channel x interrupt/status register"]
    pub mdma_c6isr: MDMA_C6ISR,
    #[doc = "0x1c4 - MDMA channel x interrupt flag clear register"]
    pub mdma_c6ifcr: MDMA_C6IFCR,
    #[doc = "0x1c8 - MDMA Channel x error status register"]
    pub mdma_c6esr: MDMA_C6ESR,
    #[doc = "0x1cc - This register is used to control the concerned channel."]
    pub mdma_c6cr: MDMA_C6CR,
    #[doc = "0x1d0 - This register is used to configure the concerned channel."]
    pub mdma_c6tcr: MDMA_C6TCR,
    #[doc = "0x1d4 - MDMA Channel x block number of data register"]
    pub mdma_c6bndtr: MDMA_C6BNDTR,
    #[doc = "0x1d8 - MDMA channel x source address register"]
    pub mdma_c6sar: MDMA_C6SAR,
    #[doc = "0x1dc - MDMA channel x destination address register"]
    pub mdma_c6dar: MDMA_C6DAR,
    #[doc = "0x1e0 - MDMA channel x Block Repeat address Update register"]
    pub mdma_c6brur: MDMA_C6BRUR,
    #[doc = "0x1e4 - MDMA channel x Link Address register"]
    pub mdma_c6lar: MDMA_C6LAR,
    #[doc = "0x1e8 - MDMA channel x Trigger and Bus selection Register"]
    pub mdma_c6tbr: MDMA_C6TBR,
    _reserved90: [u8; 0x04],
    #[doc = "0x1f0 - MDMA channel x Mask address register"]
    pub mdma_c6mar: MDMA_C6MAR,
    #[doc = "0x1f4 - MDMA channel x Mask Data register"]
    pub mdma_c6mdr: MDMA_C6MDR,
    _reserved92: [u8; 0x08],
    #[doc = "0x200 - MDMA channel x interrupt/status register"]
    pub mdma_c7isr: MDMA_C7ISR,
    #[doc = "0x204 - MDMA channel x interrupt flag clear register"]
    pub mdma_c7ifcr: MDMA_C7IFCR,
    #[doc = "0x208 - MDMA Channel x error status register"]
    pub mdma_c7esr: MDMA_C7ESR,
    #[doc = "0x20c - This register is used to control the concerned channel."]
    pub mdma_c7cr: MDMA_C7CR,
    #[doc = "0x210 - This register is used to configure the concerned channel."]
    pub mdma_c7tcr: MDMA_C7TCR,
    #[doc = "0x214 - MDMA Channel x block number of data register"]
    pub mdma_c7bndtr: MDMA_C7BNDTR,
    #[doc = "0x218 - MDMA channel x source address register"]
    pub mdma_c7sar: MDMA_C7SAR,
    #[doc = "0x21c - MDMA channel x destination address register"]
    pub mdma_c7dar: MDMA_C7DAR,
    #[doc = "0x220 - MDMA channel x Block Repeat address Update register"]
    pub mdma_c7brur: MDMA_C7BRUR,
    #[doc = "0x224 - MDMA channel x Link Address register"]
    pub mdma_c7lar: MDMA_C7LAR,
    #[doc = "0x228 - MDMA channel x Trigger and Bus selection Register"]
    pub mdma_c7tbr: MDMA_C7TBR,
    _reserved103: [u8; 0x04],
    #[doc = "0x230 - MDMA channel x Mask address register"]
    pub mdma_c7mar: MDMA_C7MAR,
    #[doc = "0x234 - MDMA channel x Mask Data register"]
    pub mdma_c7mdr: MDMA_C7MDR,
    _reserved105: [u8; 0x08],
    #[doc = "0x240 - MDMA channel x interrupt/status register"]
    pub mdma_c8isr: MDMA_C8ISR,
    #[doc = "0x244 - MDMA channel x interrupt flag clear register"]
    pub mdma_c8ifcr: MDMA_C8IFCR,
    #[doc = "0x248 - MDMA Channel x error status register"]
    pub mdma_c8esr: MDMA_C8ESR,
    #[doc = "0x24c - This register is used to control the concerned channel."]
    pub mdma_c8cr: MDMA_C8CR,
    #[doc = "0x250 - This register is used to configure the concerned channel."]
    pub mdma_c8tcr: MDMA_C8TCR,
    #[doc = "0x254 - MDMA Channel x block number of data register"]
    pub mdma_c8bndtr: MDMA_C8BNDTR,
    #[doc = "0x258 - MDMA channel x source address register"]
    pub mdma_c8sar: MDMA_C8SAR,
    #[doc = "0x25c - MDMA channel x destination address register"]
    pub mdma_c8dar: MDMA_C8DAR,
    #[doc = "0x260 - MDMA channel x Block Repeat address Update register"]
    pub mdma_c8brur: MDMA_C8BRUR,
    #[doc = "0x264 - MDMA channel x Link Address register"]
    pub mdma_c8lar: MDMA_C8LAR,
    #[doc = "0x268 - MDMA channel x Trigger and Bus selection Register"]
    pub mdma_c8tbr: MDMA_C8TBR,
    _reserved116: [u8; 0x04],
    #[doc = "0x270 - MDMA channel x Mask address register"]
    pub mdma_c8mar: MDMA_C8MAR,
    #[doc = "0x274 - MDMA channel x Mask Data register"]
    pub mdma_c8mdr: MDMA_C8MDR,
    _reserved118: [u8; 0x08],
    #[doc = "0x280 - MDMA channel x interrupt/status register"]
    pub mdma_c9isr: MDMA_C9ISR,
    #[doc = "0x284 - MDMA channel x interrupt flag clear register"]
    pub mdma_c9ifcr: MDMA_C9IFCR,
    #[doc = "0x288 - MDMA Channel x error status register"]
    pub mdma_c9esr: MDMA_C9ESR,
    #[doc = "0x28c - This register is used to control the concerned channel."]
    pub mdma_c9cr: MDMA_C9CR,
    #[doc = "0x290 - This register is used to configure the concerned channel."]
    pub mdma_c9tcr: MDMA_C9TCR,
    #[doc = "0x294 - MDMA Channel x block number of data register"]
    pub mdma_c9bndtr: MDMA_C9BNDTR,
    #[doc = "0x298 - MDMA channel x source address register"]
    pub mdma_c9sar: MDMA_C9SAR,
    #[doc = "0x29c - MDMA channel x destination address register"]
    pub mdma_c9dar: MDMA_C9DAR,
    #[doc = "0x2a0 - MDMA channel x Block Repeat address Update register"]
    pub mdma_c9brur: MDMA_C9BRUR,
    #[doc = "0x2a4 - MDMA channel x Link Address register"]
    pub mdma_c9lar: MDMA_C9LAR,
    #[doc = "0x2a8 - MDMA channel x Trigger and Bus selection Register"]
    pub mdma_c9tbr: MDMA_C9TBR,
    _reserved129: [u8; 0x04],
    #[doc = "0x2b0 - MDMA channel x Mask address register"]
    pub mdma_c9mar: MDMA_C9MAR,
    #[doc = "0x2b4 - MDMA channel x Mask Data register"]
    pub mdma_c9mdr: MDMA_C9MDR,
    _reserved131: [u8; 0x08],
    #[doc = "0x2c0 - MDMA channel x interrupt/status register"]
    pub mdma_c10isr: MDMA_C10ISR,
    #[doc = "0x2c4 - MDMA channel x interrupt flag clear register"]
    pub mdma_c10ifcr: MDMA_C10IFCR,
    #[doc = "0x2c8 - MDMA Channel x error status register"]
    pub mdma_c10esr: MDMA_C10ESR,
    #[doc = "0x2cc - This register is used to control the concerned channel."]
    pub mdma_c10cr: MDMA_C10CR,
    #[doc = "0x2d0 - This register is used to configure the concerned channel."]
    pub mdma_c10tcr: MDMA_C10TCR,
    #[doc = "0x2d4 - MDMA Channel x block number of data register"]
    pub mdma_c10bndtr: MDMA_C10BNDTR,
    #[doc = "0x2d8 - MDMA channel x source address register"]
    pub mdma_c10sar: MDMA_C10SAR,
    #[doc = "0x2dc - MDMA channel x destination address register"]
    pub mdma_c10dar: MDMA_C10DAR,
    #[doc = "0x2e0 - MDMA channel x Block Repeat address Update register"]
    pub mdma_c10brur: MDMA_C10BRUR,
    #[doc = "0x2e4 - MDMA channel x Link Address register"]
    pub mdma_c10lar: MDMA_C10LAR,
    #[doc = "0x2e8 - MDMA channel x Trigger and Bus selection Register"]
    pub mdma_c10tbr: MDMA_C10TBR,
    _reserved142: [u8; 0x04],
    #[doc = "0x2f0 - MDMA channel x Mask address register"]
    pub mdma_c10mar: MDMA_C10MAR,
    #[doc = "0x2f4 - MDMA channel x Mask Data register"]
    pub mdma_c10mdr: MDMA_C10MDR,
    _reserved144: [u8; 0x08],
    #[doc = "0x300 - MDMA channel x interrupt/status register"]
    pub mdma_c11isr: MDMA_C11ISR,
    #[doc = "0x304 - MDMA channel x interrupt flag clear register"]
    pub mdma_c11ifcr: MDMA_C11IFCR,
    #[doc = "0x308 - MDMA Channel x error status register"]
    pub mdma_c11esr: MDMA_C11ESR,
    #[doc = "0x30c - This register is used to control the concerned channel."]
    pub mdma_c11cr: MDMA_C11CR,
    #[doc = "0x310 - This register is used to configure the concerned channel."]
    pub mdma_c11tcr: MDMA_C11TCR,
    #[doc = "0x314 - MDMA Channel x block number of data register"]
    pub mdma_c11bndtr: MDMA_C11BNDTR,
    #[doc = "0x318 - MDMA channel x source address register"]
    pub mdma_c11sar: MDMA_C11SAR,
    #[doc = "0x31c - MDMA channel x destination address register"]
    pub mdma_c11dar: MDMA_C11DAR,
    #[doc = "0x320 - MDMA channel x Block Repeat address Update register"]
    pub mdma_c11brur: MDMA_C11BRUR,
    #[doc = "0x324 - MDMA channel x Link Address register"]
    pub mdma_c11lar: MDMA_C11LAR,
    #[doc = "0x328 - MDMA channel x Trigger and Bus selection Register"]
    pub mdma_c11tbr: MDMA_C11TBR,
    _reserved155: [u8; 0x04],
    #[doc = "0x330 - MDMA channel x Mask address register"]
    pub mdma_c11mar: MDMA_C11MAR,
    #[doc = "0x334 - MDMA channel x Mask Data register"]
    pub mdma_c11mdr: MDMA_C11MDR,
    _reserved157: [u8; 0x08],
    #[doc = "0x340 - MDMA channel x interrupt/status register"]
    pub mdma_c12isr: MDMA_C12ISR,
    #[doc = "0x344 - MDMA channel x interrupt flag clear register"]
    pub mdma_c12ifcr: MDMA_C12IFCR,
    #[doc = "0x348 - MDMA Channel x error status register"]
    pub mdma_c12esr: MDMA_C12ESR,
    #[doc = "0x34c - This register is used to control the concerned channel."]
    pub mdma_c12cr: MDMA_C12CR,
    #[doc = "0x350 - This register is used to configure the concerned channel."]
    pub mdma_c12tcr: MDMA_C12TCR,
    #[doc = "0x354 - MDMA Channel x block number of data register"]
    pub mdma_c12bndtr: MDMA_C12BNDTR,
    #[doc = "0x358 - MDMA channel x source address register"]
    pub mdma_c12sar: MDMA_C12SAR,
    #[doc = "0x35c - MDMA channel x destination address register"]
    pub mdma_c12dar: MDMA_C12DAR,
    #[doc = "0x360 - MDMA channel x Block Repeat address Update register"]
    pub mdma_c12brur: MDMA_C12BRUR,
    #[doc = "0x364 - MDMA channel x Link Address register"]
    pub mdma_c12lar: MDMA_C12LAR,
    #[doc = "0x368 - MDMA channel x Trigger and Bus selection Register"]
    pub mdma_c12tbr: MDMA_C12TBR,
    _reserved168: [u8; 0x04],
    #[doc = "0x370 - MDMA channel x Mask address register"]
    pub mdma_c12mar: MDMA_C12MAR,
    #[doc = "0x374 - MDMA channel x Mask Data register"]
    pub mdma_c12mdr: MDMA_C12MDR,
    _reserved170: [u8; 0x08],
    #[doc = "0x380 - MDMA channel x interrupt/status register"]
    pub mdma_c13isr: MDMA_C13ISR,
    #[doc = "0x384 - MDMA channel x interrupt flag clear register"]
    pub mdma_c13ifcr: MDMA_C13IFCR,
    #[doc = "0x388 - MDMA Channel x error status register"]
    pub mdma_c13esr: MDMA_C13ESR,
    #[doc = "0x38c - This register is used to control the concerned channel."]
    pub mdma_c13cr: MDMA_C13CR,
    #[doc = "0x390 - This register is used to configure the concerned channel."]
    pub mdma_c13tcr: MDMA_C13TCR,
    #[doc = "0x394 - MDMA Channel x block number of data register"]
    pub mdma_c13bndtr: MDMA_C13BNDTR,
    #[doc = "0x398 - MDMA channel x source address register"]
    pub mdma_c13sar: MDMA_C13SAR,
    #[doc = "0x39c - MDMA channel x destination address register"]
    pub mdma_c13dar: MDMA_C13DAR,
    #[doc = "0x3a0 - MDMA channel x Block Repeat address Update register"]
    pub mdma_c13brur: MDMA_C13BRUR,
    #[doc = "0x3a4 - MDMA channel x Link Address register"]
    pub mdma_c13lar: MDMA_C13LAR,
    #[doc = "0x3a8 - MDMA channel x Trigger and Bus selection Register"]
    pub mdma_c13tbr: MDMA_C13TBR,
    _reserved181: [u8; 0x04],
    #[doc = "0x3b0 - MDMA channel x Mask address register"]
    pub mdma_c13mar: MDMA_C13MAR,
    #[doc = "0x3b4 - MDMA channel x Mask Data register"]
    pub mdma_c13mdr: MDMA_C13MDR,
    _reserved183: [u8; 0x08],
    #[doc = "0x3c0 - MDMA channel x interrupt/status register"]
    pub mdma_c14isr: MDMA_C14ISR,
    #[doc = "0x3c4 - MDMA channel x interrupt flag clear register"]
    pub mdma_c14ifcr: MDMA_C14IFCR,
    #[doc = "0x3c8 - MDMA Channel x error status register"]
    pub mdma_c14esr: MDMA_C14ESR,
    #[doc = "0x3cc - This register is used to control the concerned channel."]
    pub mdma_c14cr: MDMA_C14CR,
    #[doc = "0x3d0 - This register is used to configure the concerned channel."]
    pub mdma_c14tcr: MDMA_C14TCR,
    #[doc = "0x3d4 - MDMA Channel x block number of data register"]
    pub mdma_c14bndtr: MDMA_C14BNDTR,
    #[doc = "0x3d8 - MDMA channel x source address register"]
    pub mdma_c14sar: MDMA_C14SAR,
    #[doc = "0x3dc - MDMA channel x destination address register"]
    pub mdma_c14dar: MDMA_C14DAR,
    #[doc = "0x3e0 - MDMA channel x Block Repeat address Update register"]
    pub mdma_c14brur: MDMA_C14BRUR,
    #[doc = "0x3e4 - MDMA channel x Link Address register"]
    pub mdma_c14lar: MDMA_C14LAR,
    #[doc = "0x3e8 - MDMA channel x Trigger and Bus selection Register"]
    pub mdma_c14tbr: MDMA_C14TBR,
    _reserved194: [u8; 0x04],
    #[doc = "0x3f0 - MDMA channel x Mask address register"]
    pub mdma_c14mar: MDMA_C14MAR,
    #[doc = "0x3f4 - MDMA channel x Mask Data register"]
    pub mdma_c14mdr: MDMA_C14MDR,
    _reserved196: [u8; 0x08],
    #[doc = "0x400 - MDMA channel x interrupt/status register"]
    pub mdma_c15isr: MDMA_C15ISR,
    #[doc = "0x404 - MDMA channel x interrupt flag clear register"]
    pub mdma_c15ifcr: MDMA_C15IFCR,
    #[doc = "0x408 - MDMA Channel x error status register"]
    pub mdma_c15esr: MDMA_C15ESR,
    #[doc = "0x40c - This register is used to control the concerned channel."]
    pub mdma_c15cr: MDMA_C15CR,
    #[doc = "0x410 - This register is used to configure the concerned channel."]
    pub mdma_c15tcr: MDMA_C15TCR,
    #[doc = "0x414 - MDMA Channel x block number of data register"]
    pub mdma_c15bndtr: MDMA_C15BNDTR,
    #[doc = "0x418 - MDMA channel x source address register"]
    pub mdma_c15sar: MDMA_C15SAR,
    #[doc = "0x41c - MDMA channel x destination address register"]
    pub mdma_c15dar: MDMA_C15DAR,
    #[doc = "0x420 - MDMA channel x Block Repeat address Update register"]
    pub mdma_c15brur: MDMA_C15BRUR,
    #[doc = "0x424 - MDMA channel x Link Address register"]
    pub mdma_c15lar: MDMA_C15LAR,
    #[doc = "0x428 - MDMA channel x Trigger and Bus selection Register"]
    pub mdma_c15tbr: MDMA_C15TBR,
    _reserved207: [u8; 0x04],
    #[doc = "0x430 - MDMA channel x Mask address register"]
    pub mdma_c15mar: MDMA_C15MAR,
    #[doc = "0x434 - MDMA channel x Mask Data register"]
    pub mdma_c15mdr: MDMA_C15MDR,
}
#[doc = "MDMA_GISR0 (r) register accessor: MDMA Global Interrupt/Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mdma_gisr0::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mdma_gisr0`]
module"]
pub type MDMA_GISR0 = crate::Reg<mdma_gisr0::MDMA_GISR0_SPEC>;
#[doc = "MDMA Global Interrupt/Status Register"]
pub mod mdma_gisr0;
#[doc = "MDMA_C0ISR (r) register accessor: MDMA channel x interrupt/status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mdma_c0isr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mdma_c0isr`]
module"]
pub type MDMA_C0ISR = crate::Reg<mdma_c0isr::MDMA_C0ISR_SPEC>;
#[doc = "MDMA channel x interrupt/status register"]
pub mod mdma_c0isr;
#[doc = "MDMA_C0IFCR (w) register accessor: MDMA channel x interrupt flag clear register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mdma_c0ifcr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mdma_c0ifcr`]
module"]
pub type MDMA_C0IFCR = crate::Reg<mdma_c0ifcr::MDMA_C0IFCR_SPEC>;
#[doc = "MDMA channel x interrupt flag clear register"]
pub mod mdma_c0ifcr;
#[doc = "MDMA_C0ESR (r) register accessor: MDMA Channel x error status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mdma_c0esr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mdma_c0esr`]
module"]
pub type MDMA_C0ESR = crate::Reg<mdma_c0esr::MDMA_C0ESR_SPEC>;
#[doc = "MDMA Channel x error status register"]
pub mod mdma_c0esr;
#[doc = "MDMA_C0CR (rw) register accessor: This register is used to control the concerned channel.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mdma_c0cr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mdma_c0cr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mdma_c0cr`]
module"]
pub type MDMA_C0CR = crate::Reg<mdma_c0cr::MDMA_C0CR_SPEC>;
#[doc = "This register is used to control the concerned channel."]
pub mod mdma_c0cr;
#[doc = "MDMA_C0TCR (rw) register accessor: This register is used to configure the concerned channel.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mdma_c0tcr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mdma_c0tcr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mdma_c0tcr`]
module"]
pub type MDMA_C0TCR = crate::Reg<mdma_c0tcr::MDMA_C0TCR_SPEC>;
#[doc = "This register is used to configure the concerned channel."]
pub mod mdma_c0tcr;
#[doc = "MDMA_C0BNDTR (rw) register accessor: MDMA Channel x block number of data register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mdma_c0bndtr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mdma_c0bndtr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mdma_c0bndtr`]
module"]
pub type MDMA_C0BNDTR = crate::Reg<mdma_c0bndtr::MDMA_C0BNDTR_SPEC>;
#[doc = "MDMA Channel x block number of data register"]
pub mod mdma_c0bndtr;
#[doc = "MDMA_C0SAR (rw) register accessor: MDMA channel x source address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mdma_c0sar::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mdma_c0sar::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mdma_c0sar`]
module"]
pub type MDMA_C0SAR = crate::Reg<mdma_c0sar::MDMA_C0SAR_SPEC>;
#[doc = "MDMA channel x source address register"]
pub mod mdma_c0sar;
#[doc = "MDMA_C0DAR (rw) register accessor: MDMA channel x destination address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mdma_c0dar::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mdma_c0dar::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mdma_c0dar`]
module"]
pub type MDMA_C0DAR = crate::Reg<mdma_c0dar::MDMA_C0DAR_SPEC>;
#[doc = "MDMA channel x destination address register"]
pub mod mdma_c0dar;
#[doc = "MDMA_C0BRUR (rw) register accessor: MDMA channel x Block Repeat address Update register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mdma_c0brur::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mdma_c0brur::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mdma_c0brur`]
module"]
pub type MDMA_C0BRUR = crate::Reg<mdma_c0brur::MDMA_C0BRUR_SPEC>;
#[doc = "MDMA channel x Block Repeat address Update register"]
pub mod mdma_c0brur;
#[doc = "MDMA_C0LAR (rw) register accessor: MDMA channel x Link Address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mdma_c0lar::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mdma_c0lar::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mdma_c0lar`]
module"]
pub type MDMA_C0LAR = crate::Reg<mdma_c0lar::MDMA_C0LAR_SPEC>;
#[doc = "MDMA channel x Link Address register"]
pub mod mdma_c0lar;
#[doc = "MDMA_C0TBR (rw) register accessor: MDMA channel x Trigger and Bus selection Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mdma_c0tbr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mdma_c0tbr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mdma_c0tbr`]
module"]
pub type MDMA_C0TBR = crate::Reg<mdma_c0tbr::MDMA_C0TBR_SPEC>;
#[doc = "MDMA channel x Trigger and Bus selection Register"]
pub mod mdma_c0tbr;
#[doc = "MDMA_C0MAR (rw) register accessor: MDMA channel x Mask address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mdma_c0mar::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mdma_c0mar::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mdma_c0mar`]
module"]
pub type MDMA_C0MAR = crate::Reg<mdma_c0mar::MDMA_C0MAR_SPEC>;
#[doc = "MDMA channel x Mask address register"]
pub mod mdma_c0mar;
#[doc = "MDMA_C0MDR (rw) register accessor: MDMA channel x Mask Data register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mdma_c0mdr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mdma_c0mdr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mdma_c0mdr`]
module"]
pub type MDMA_C0MDR = crate::Reg<mdma_c0mdr::MDMA_C0MDR_SPEC>;
#[doc = "MDMA channel x Mask Data register"]
pub mod mdma_c0mdr;
#[doc = "MDMA_C1ISR (r) register accessor: MDMA channel x interrupt/status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mdma_c1isr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mdma_c1isr`]
module"]
pub type MDMA_C1ISR = crate::Reg<mdma_c1isr::MDMA_C1ISR_SPEC>;
#[doc = "MDMA channel x interrupt/status register"]
pub mod mdma_c1isr;
#[doc = "MDMA_C1IFCR (w) register accessor: MDMA channel x interrupt flag clear register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mdma_c1ifcr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mdma_c1ifcr`]
module"]
pub type MDMA_C1IFCR = crate::Reg<mdma_c1ifcr::MDMA_C1IFCR_SPEC>;
#[doc = "MDMA channel x interrupt flag clear register"]
pub mod mdma_c1ifcr;
#[doc = "MDMA_C1ESR (r) register accessor: MDMA Channel x error status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mdma_c1esr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mdma_c1esr`]
module"]
pub type MDMA_C1ESR = crate::Reg<mdma_c1esr::MDMA_C1ESR_SPEC>;
#[doc = "MDMA Channel x error status register"]
pub mod mdma_c1esr;
#[doc = "MDMA_C1CR (rw) register accessor: This register is used to control the concerned channel.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mdma_c1cr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mdma_c1cr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mdma_c1cr`]
module"]
pub type MDMA_C1CR = crate::Reg<mdma_c1cr::MDMA_C1CR_SPEC>;
#[doc = "This register is used to control the concerned channel."]
pub mod mdma_c1cr;
#[doc = "MDMA_C1TCR (rw) register accessor: This register is used to configure the concerned channel.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mdma_c1tcr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mdma_c1tcr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mdma_c1tcr`]
module"]
pub type MDMA_C1TCR = crate::Reg<mdma_c1tcr::MDMA_C1TCR_SPEC>;
#[doc = "This register is used to configure the concerned channel."]
pub mod mdma_c1tcr;
#[doc = "MDMA_C1BNDTR (rw) register accessor: MDMA Channel x block number of data register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mdma_c1bndtr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mdma_c1bndtr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mdma_c1bndtr`]
module"]
pub type MDMA_C1BNDTR = crate::Reg<mdma_c1bndtr::MDMA_C1BNDTR_SPEC>;
#[doc = "MDMA Channel x block number of data register"]
pub mod mdma_c1bndtr;
#[doc = "MDMA_C1SAR (rw) register accessor: MDMA channel x source address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mdma_c1sar::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mdma_c1sar::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mdma_c1sar`]
module"]
pub type MDMA_C1SAR = crate::Reg<mdma_c1sar::MDMA_C1SAR_SPEC>;
#[doc = "MDMA channel x source address register"]
pub mod mdma_c1sar;
#[doc = "MDMA_C1DAR (rw) register accessor: MDMA channel x destination address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mdma_c1dar::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mdma_c1dar::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mdma_c1dar`]
module"]
pub type MDMA_C1DAR = crate::Reg<mdma_c1dar::MDMA_C1DAR_SPEC>;
#[doc = "MDMA channel x destination address register"]
pub mod mdma_c1dar;
#[doc = "MDMA_C1BRUR (rw) register accessor: MDMA channel x Block Repeat address Update register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mdma_c1brur::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mdma_c1brur::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mdma_c1brur`]
module"]
pub type MDMA_C1BRUR = crate::Reg<mdma_c1brur::MDMA_C1BRUR_SPEC>;
#[doc = "MDMA channel x Block Repeat address Update register"]
pub mod mdma_c1brur;
#[doc = "MDMA_C1LAR (rw) register accessor: MDMA channel x Link Address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mdma_c1lar::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mdma_c1lar::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mdma_c1lar`]
module"]
pub type MDMA_C1LAR = crate::Reg<mdma_c1lar::MDMA_C1LAR_SPEC>;
#[doc = "MDMA channel x Link Address register"]
pub mod mdma_c1lar;
#[doc = "MDMA_C1TBR (rw) register accessor: MDMA channel x Trigger and Bus selection Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mdma_c1tbr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mdma_c1tbr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mdma_c1tbr`]
module"]
pub type MDMA_C1TBR = crate::Reg<mdma_c1tbr::MDMA_C1TBR_SPEC>;
#[doc = "MDMA channel x Trigger and Bus selection Register"]
pub mod mdma_c1tbr;
#[doc = "MDMA_C1MAR (rw) register accessor: MDMA channel x Mask address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mdma_c1mar::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mdma_c1mar::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mdma_c1mar`]
module"]
pub type MDMA_C1MAR = crate::Reg<mdma_c1mar::MDMA_C1MAR_SPEC>;
#[doc = "MDMA channel x Mask address register"]
pub mod mdma_c1mar;
#[doc = "MDMA_C1MDR (rw) register accessor: MDMA channel x Mask Data register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mdma_c1mdr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mdma_c1mdr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mdma_c1mdr`]
module"]
pub type MDMA_C1MDR = crate::Reg<mdma_c1mdr::MDMA_C1MDR_SPEC>;
#[doc = "MDMA channel x Mask Data register"]
pub mod mdma_c1mdr;
#[doc = "MDMA_C2ISR (r) register accessor: MDMA channel x interrupt/status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mdma_c2isr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mdma_c2isr`]
module"]
pub type MDMA_C2ISR = crate::Reg<mdma_c2isr::MDMA_C2ISR_SPEC>;
#[doc = "MDMA channel x interrupt/status register"]
pub mod mdma_c2isr;
#[doc = "MDMA_C2IFCR (w) register accessor: MDMA channel x interrupt flag clear register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mdma_c2ifcr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mdma_c2ifcr`]
module"]
pub type MDMA_C2IFCR = crate::Reg<mdma_c2ifcr::MDMA_C2IFCR_SPEC>;
#[doc = "MDMA channel x interrupt flag clear register"]
pub mod mdma_c2ifcr;
#[doc = "MDMA_C2ESR (r) register accessor: MDMA Channel x error status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mdma_c2esr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mdma_c2esr`]
module"]
pub type MDMA_C2ESR = crate::Reg<mdma_c2esr::MDMA_C2ESR_SPEC>;
#[doc = "MDMA Channel x error status register"]
pub mod mdma_c2esr;
#[doc = "MDMA_C2CR (rw) register accessor: This register is used to control the concerned channel.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mdma_c2cr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mdma_c2cr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mdma_c2cr`]
module"]
pub type MDMA_C2CR = crate::Reg<mdma_c2cr::MDMA_C2CR_SPEC>;
#[doc = "This register is used to control the concerned channel."]
pub mod mdma_c2cr;
#[doc = "MDMA_C2TCR (rw) register accessor: This register is used to configure the concerned channel.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mdma_c2tcr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mdma_c2tcr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mdma_c2tcr`]
module"]
pub type MDMA_C2TCR = crate::Reg<mdma_c2tcr::MDMA_C2TCR_SPEC>;
#[doc = "This register is used to configure the concerned channel."]
pub mod mdma_c2tcr;
#[doc = "MDMA_C2BNDTR (rw) register accessor: MDMA Channel x block number of data register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mdma_c2bndtr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mdma_c2bndtr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mdma_c2bndtr`]
module"]
pub type MDMA_C2BNDTR = crate::Reg<mdma_c2bndtr::MDMA_C2BNDTR_SPEC>;
#[doc = "MDMA Channel x block number of data register"]
pub mod mdma_c2bndtr;
#[doc = "MDMA_C2SAR (rw) register accessor: MDMA channel x source address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mdma_c2sar::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mdma_c2sar::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mdma_c2sar`]
module"]
pub type MDMA_C2SAR = crate::Reg<mdma_c2sar::MDMA_C2SAR_SPEC>;
#[doc = "MDMA channel x source address register"]
pub mod mdma_c2sar;
#[doc = "MDMA_C2DAR (rw) register accessor: MDMA channel x destination address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mdma_c2dar::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mdma_c2dar::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mdma_c2dar`]
module"]
pub type MDMA_C2DAR = crate::Reg<mdma_c2dar::MDMA_C2DAR_SPEC>;
#[doc = "MDMA channel x destination address register"]
pub mod mdma_c2dar;
#[doc = "MDMA_C2BRUR (rw) register accessor: MDMA channel x Block Repeat address Update register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mdma_c2brur::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mdma_c2brur::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mdma_c2brur`]
module"]
pub type MDMA_C2BRUR = crate::Reg<mdma_c2brur::MDMA_C2BRUR_SPEC>;
#[doc = "MDMA channel x Block Repeat address Update register"]
pub mod mdma_c2brur;
#[doc = "MDMA_C2LAR (rw) register accessor: MDMA channel x Link Address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mdma_c2lar::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mdma_c2lar::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mdma_c2lar`]
module"]
pub type MDMA_C2LAR = crate::Reg<mdma_c2lar::MDMA_C2LAR_SPEC>;
#[doc = "MDMA channel x Link Address register"]
pub mod mdma_c2lar;
#[doc = "MDMA_C2TBR (rw) register accessor: MDMA channel x Trigger and Bus selection Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mdma_c2tbr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mdma_c2tbr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mdma_c2tbr`]
module"]
pub type MDMA_C2TBR = crate::Reg<mdma_c2tbr::MDMA_C2TBR_SPEC>;
#[doc = "MDMA channel x Trigger and Bus selection Register"]
pub mod mdma_c2tbr;
#[doc = "MDMA_C2MAR (rw) register accessor: MDMA channel x Mask address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mdma_c2mar::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mdma_c2mar::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mdma_c2mar`]
module"]
pub type MDMA_C2MAR = crate::Reg<mdma_c2mar::MDMA_C2MAR_SPEC>;
#[doc = "MDMA channel x Mask address register"]
pub mod mdma_c2mar;
#[doc = "MDMA_C2MDR (rw) register accessor: MDMA channel x Mask Data register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mdma_c2mdr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mdma_c2mdr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mdma_c2mdr`]
module"]
pub type MDMA_C2MDR = crate::Reg<mdma_c2mdr::MDMA_C2MDR_SPEC>;
#[doc = "MDMA channel x Mask Data register"]
pub mod mdma_c2mdr;
#[doc = "MDMA_C3ISR (r) register accessor: MDMA channel x interrupt/status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mdma_c3isr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mdma_c3isr`]
module"]
pub type MDMA_C3ISR = crate::Reg<mdma_c3isr::MDMA_C3ISR_SPEC>;
#[doc = "MDMA channel x interrupt/status register"]
pub mod mdma_c3isr;
#[doc = "MDMA_C3IFCR (w) register accessor: MDMA channel x interrupt flag clear register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mdma_c3ifcr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mdma_c3ifcr`]
module"]
pub type MDMA_C3IFCR = crate::Reg<mdma_c3ifcr::MDMA_C3IFCR_SPEC>;
#[doc = "MDMA channel x interrupt flag clear register"]
pub mod mdma_c3ifcr;
#[doc = "MDMA_C3ESR (r) register accessor: MDMA Channel x error status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mdma_c3esr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mdma_c3esr`]
module"]
pub type MDMA_C3ESR = crate::Reg<mdma_c3esr::MDMA_C3ESR_SPEC>;
#[doc = "MDMA Channel x error status register"]
pub mod mdma_c3esr;
#[doc = "MDMA_C3CR (rw) register accessor: This register is used to control the concerned channel.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mdma_c3cr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mdma_c3cr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mdma_c3cr`]
module"]
pub type MDMA_C3CR = crate::Reg<mdma_c3cr::MDMA_C3CR_SPEC>;
#[doc = "This register is used to control the concerned channel."]
pub mod mdma_c3cr;
#[doc = "MDMA_C3TCR (rw) register accessor: This register is used to configure the concerned channel.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mdma_c3tcr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mdma_c3tcr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mdma_c3tcr`]
module"]
pub type MDMA_C3TCR = crate::Reg<mdma_c3tcr::MDMA_C3TCR_SPEC>;
#[doc = "This register is used to configure the concerned channel."]
pub mod mdma_c3tcr;
#[doc = "MDMA_C3BNDTR (rw) register accessor: MDMA Channel x block number of data register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mdma_c3bndtr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mdma_c3bndtr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mdma_c3bndtr`]
module"]
pub type MDMA_C3BNDTR = crate::Reg<mdma_c3bndtr::MDMA_C3BNDTR_SPEC>;
#[doc = "MDMA Channel x block number of data register"]
pub mod mdma_c3bndtr;
#[doc = "MDMA_C3SAR (rw) register accessor: MDMA channel x source address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mdma_c3sar::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mdma_c3sar::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mdma_c3sar`]
module"]
pub type MDMA_C3SAR = crate::Reg<mdma_c3sar::MDMA_C3SAR_SPEC>;
#[doc = "MDMA channel x source address register"]
pub mod mdma_c3sar;
#[doc = "MDMA_C3DAR (rw) register accessor: MDMA channel x destination address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mdma_c3dar::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mdma_c3dar::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mdma_c3dar`]
module"]
pub type MDMA_C3DAR = crate::Reg<mdma_c3dar::MDMA_C3DAR_SPEC>;
#[doc = "MDMA channel x destination address register"]
pub mod mdma_c3dar;
#[doc = "MDMA_C3BRUR (rw) register accessor: MDMA channel x Block Repeat address Update register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mdma_c3brur::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mdma_c3brur::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mdma_c3brur`]
module"]
pub type MDMA_C3BRUR = crate::Reg<mdma_c3brur::MDMA_C3BRUR_SPEC>;
#[doc = "MDMA channel x Block Repeat address Update register"]
pub mod mdma_c3brur;
#[doc = "MDMA_C3LAR (rw) register accessor: MDMA channel x Link Address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mdma_c3lar::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mdma_c3lar::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mdma_c3lar`]
module"]
pub type MDMA_C3LAR = crate::Reg<mdma_c3lar::MDMA_C3LAR_SPEC>;
#[doc = "MDMA channel x Link Address register"]
pub mod mdma_c3lar;
#[doc = "MDMA_C3TBR (rw) register accessor: MDMA channel x Trigger and Bus selection Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mdma_c3tbr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mdma_c3tbr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mdma_c3tbr`]
module"]
pub type MDMA_C3TBR = crate::Reg<mdma_c3tbr::MDMA_C3TBR_SPEC>;
#[doc = "MDMA channel x Trigger and Bus selection Register"]
pub mod mdma_c3tbr;
#[doc = "MDMA_C3MAR (rw) register accessor: MDMA channel x Mask address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mdma_c3mar::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mdma_c3mar::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mdma_c3mar`]
module"]
pub type MDMA_C3MAR = crate::Reg<mdma_c3mar::MDMA_C3MAR_SPEC>;
#[doc = "MDMA channel x Mask address register"]
pub mod mdma_c3mar;
#[doc = "MDMA_C3MDR (rw) register accessor: MDMA channel x Mask Data register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mdma_c3mdr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mdma_c3mdr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mdma_c3mdr`]
module"]
pub type MDMA_C3MDR = crate::Reg<mdma_c3mdr::MDMA_C3MDR_SPEC>;
#[doc = "MDMA channel x Mask Data register"]
pub mod mdma_c3mdr;
#[doc = "MDMA_C4ISR (r) register accessor: MDMA channel x interrupt/status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mdma_c4isr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mdma_c4isr`]
module"]
pub type MDMA_C4ISR = crate::Reg<mdma_c4isr::MDMA_C4ISR_SPEC>;
#[doc = "MDMA channel x interrupt/status register"]
pub mod mdma_c4isr;
#[doc = "MDMA_C4IFCR (w) register accessor: MDMA channel x interrupt flag clear register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mdma_c4ifcr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mdma_c4ifcr`]
module"]
pub type MDMA_C4IFCR = crate::Reg<mdma_c4ifcr::MDMA_C4IFCR_SPEC>;
#[doc = "MDMA channel x interrupt flag clear register"]
pub mod mdma_c4ifcr;
#[doc = "MDMA_C4ESR (r) register accessor: MDMA Channel x error status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mdma_c4esr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mdma_c4esr`]
module"]
pub type MDMA_C4ESR = crate::Reg<mdma_c4esr::MDMA_C4ESR_SPEC>;
#[doc = "MDMA Channel x error status register"]
pub mod mdma_c4esr;
#[doc = "MDMA_C4CR (rw) register accessor: This register is used to control the concerned channel.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mdma_c4cr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mdma_c4cr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mdma_c4cr`]
module"]
pub type MDMA_C4CR = crate::Reg<mdma_c4cr::MDMA_C4CR_SPEC>;
#[doc = "This register is used to control the concerned channel."]
pub mod mdma_c4cr;
#[doc = "MDMA_C4TCR (rw) register accessor: This register is used to configure the concerned channel.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mdma_c4tcr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mdma_c4tcr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mdma_c4tcr`]
module"]
pub type MDMA_C4TCR = crate::Reg<mdma_c4tcr::MDMA_C4TCR_SPEC>;
#[doc = "This register is used to configure the concerned channel."]
pub mod mdma_c4tcr;
#[doc = "MDMA_C4BNDTR (rw) register accessor: MDMA Channel x block number of data register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mdma_c4bndtr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mdma_c4bndtr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mdma_c4bndtr`]
module"]
pub type MDMA_C4BNDTR = crate::Reg<mdma_c4bndtr::MDMA_C4BNDTR_SPEC>;
#[doc = "MDMA Channel x block number of data register"]
pub mod mdma_c4bndtr;
#[doc = "MDMA_C4SAR (rw) register accessor: MDMA channel x source address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mdma_c4sar::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mdma_c4sar::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mdma_c4sar`]
module"]
pub type MDMA_C4SAR = crate::Reg<mdma_c4sar::MDMA_C4SAR_SPEC>;
#[doc = "MDMA channel x source address register"]
pub mod mdma_c4sar;
#[doc = "MDMA_C4DAR (rw) register accessor: MDMA channel x destination address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mdma_c4dar::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mdma_c4dar::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mdma_c4dar`]
module"]
pub type MDMA_C4DAR = crate::Reg<mdma_c4dar::MDMA_C4DAR_SPEC>;
#[doc = "MDMA channel x destination address register"]
pub mod mdma_c4dar;
#[doc = "MDMA_C4BRUR (rw) register accessor: MDMA channel x Block Repeat address Update register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mdma_c4brur::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mdma_c4brur::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mdma_c4brur`]
module"]
pub type MDMA_C4BRUR = crate::Reg<mdma_c4brur::MDMA_C4BRUR_SPEC>;
#[doc = "MDMA channel x Block Repeat address Update register"]
pub mod mdma_c4brur;
#[doc = "MDMA_C4LAR (rw) register accessor: MDMA channel x Link Address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mdma_c4lar::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mdma_c4lar::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mdma_c4lar`]
module"]
pub type MDMA_C4LAR = crate::Reg<mdma_c4lar::MDMA_C4LAR_SPEC>;
#[doc = "MDMA channel x Link Address register"]
pub mod mdma_c4lar;
#[doc = "MDMA_C4TBR (rw) register accessor: MDMA channel x Trigger and Bus selection Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mdma_c4tbr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mdma_c4tbr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mdma_c4tbr`]
module"]
pub type MDMA_C4TBR = crate::Reg<mdma_c4tbr::MDMA_C4TBR_SPEC>;
#[doc = "MDMA channel x Trigger and Bus selection Register"]
pub mod mdma_c4tbr;
#[doc = "MDMA_C4MAR (rw) register accessor: MDMA channel x Mask address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mdma_c4mar::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mdma_c4mar::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mdma_c4mar`]
module"]
pub type MDMA_C4MAR = crate::Reg<mdma_c4mar::MDMA_C4MAR_SPEC>;
#[doc = "MDMA channel x Mask address register"]
pub mod mdma_c4mar;
#[doc = "MDMA_C4MDR (rw) register accessor: MDMA channel x Mask Data register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mdma_c4mdr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mdma_c4mdr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mdma_c4mdr`]
module"]
pub type MDMA_C4MDR = crate::Reg<mdma_c4mdr::MDMA_C4MDR_SPEC>;
#[doc = "MDMA channel x Mask Data register"]
pub mod mdma_c4mdr;
#[doc = "MDMA_C5ISR (r) register accessor: MDMA channel x interrupt/status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mdma_c5isr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mdma_c5isr`]
module"]
pub type MDMA_C5ISR = crate::Reg<mdma_c5isr::MDMA_C5ISR_SPEC>;
#[doc = "MDMA channel x interrupt/status register"]
pub mod mdma_c5isr;
#[doc = "MDMA_C5IFCR (w) register accessor: MDMA channel x interrupt flag clear register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mdma_c5ifcr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mdma_c5ifcr`]
module"]
pub type MDMA_C5IFCR = crate::Reg<mdma_c5ifcr::MDMA_C5IFCR_SPEC>;
#[doc = "MDMA channel x interrupt flag clear register"]
pub mod mdma_c5ifcr;
#[doc = "MDMA_C5ESR (r) register accessor: MDMA Channel x error status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mdma_c5esr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mdma_c5esr`]
module"]
pub type MDMA_C5ESR = crate::Reg<mdma_c5esr::MDMA_C5ESR_SPEC>;
#[doc = "MDMA Channel x error status register"]
pub mod mdma_c5esr;
#[doc = "MDMA_C5CR (rw) register accessor: This register is used to control the concerned channel.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mdma_c5cr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mdma_c5cr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mdma_c5cr`]
module"]
pub type MDMA_C5CR = crate::Reg<mdma_c5cr::MDMA_C5CR_SPEC>;
#[doc = "This register is used to control the concerned channel."]
pub mod mdma_c5cr;
#[doc = "MDMA_C5TCR (rw) register accessor: This register is used to configure the concerned channel.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mdma_c5tcr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mdma_c5tcr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mdma_c5tcr`]
module"]
pub type MDMA_C5TCR = crate::Reg<mdma_c5tcr::MDMA_C5TCR_SPEC>;
#[doc = "This register is used to configure the concerned channel."]
pub mod mdma_c5tcr;
#[doc = "MDMA_C5BNDTR (rw) register accessor: MDMA Channel x block number of data register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mdma_c5bndtr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mdma_c5bndtr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mdma_c5bndtr`]
module"]
pub type MDMA_C5BNDTR = crate::Reg<mdma_c5bndtr::MDMA_C5BNDTR_SPEC>;
#[doc = "MDMA Channel x block number of data register"]
pub mod mdma_c5bndtr;
#[doc = "MDMA_C5SAR (rw) register accessor: MDMA channel x source address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mdma_c5sar::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mdma_c5sar::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mdma_c5sar`]
module"]
pub type MDMA_C5SAR = crate::Reg<mdma_c5sar::MDMA_C5SAR_SPEC>;
#[doc = "MDMA channel x source address register"]
pub mod mdma_c5sar;
#[doc = "MDMA_C5DAR (rw) register accessor: MDMA channel x destination address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mdma_c5dar::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mdma_c5dar::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mdma_c5dar`]
module"]
pub type MDMA_C5DAR = crate::Reg<mdma_c5dar::MDMA_C5DAR_SPEC>;
#[doc = "MDMA channel x destination address register"]
pub mod mdma_c5dar;
#[doc = "MDMA_C5BRUR (rw) register accessor: MDMA channel x Block Repeat address Update register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mdma_c5brur::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mdma_c5brur::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mdma_c5brur`]
module"]
pub type MDMA_C5BRUR = crate::Reg<mdma_c5brur::MDMA_C5BRUR_SPEC>;
#[doc = "MDMA channel x Block Repeat address Update register"]
pub mod mdma_c5brur;
#[doc = "MDMA_C5LAR (rw) register accessor: MDMA channel x Link Address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mdma_c5lar::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mdma_c5lar::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mdma_c5lar`]
module"]
pub type MDMA_C5LAR = crate::Reg<mdma_c5lar::MDMA_C5LAR_SPEC>;
#[doc = "MDMA channel x Link Address register"]
pub mod mdma_c5lar;
#[doc = "MDMA_C5TBR (rw) register accessor: MDMA channel x Trigger and Bus selection Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mdma_c5tbr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mdma_c5tbr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mdma_c5tbr`]
module"]
pub type MDMA_C5TBR = crate::Reg<mdma_c5tbr::MDMA_C5TBR_SPEC>;
#[doc = "MDMA channel x Trigger and Bus selection Register"]
pub mod mdma_c5tbr;
#[doc = "MDMA_C5MAR (rw) register accessor: MDMA channel x Mask address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mdma_c5mar::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mdma_c5mar::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mdma_c5mar`]
module"]
pub type MDMA_C5MAR = crate::Reg<mdma_c5mar::MDMA_C5MAR_SPEC>;
#[doc = "MDMA channel x Mask address register"]
pub mod mdma_c5mar;
#[doc = "MDMA_C5MDR (rw) register accessor: MDMA channel x Mask Data register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mdma_c5mdr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mdma_c5mdr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mdma_c5mdr`]
module"]
pub type MDMA_C5MDR = crate::Reg<mdma_c5mdr::MDMA_C5MDR_SPEC>;
#[doc = "MDMA channel x Mask Data register"]
pub mod mdma_c5mdr;
#[doc = "MDMA_C6ISR (r) register accessor: MDMA channel x interrupt/status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mdma_c6isr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mdma_c6isr`]
module"]
pub type MDMA_C6ISR = crate::Reg<mdma_c6isr::MDMA_C6ISR_SPEC>;
#[doc = "MDMA channel x interrupt/status register"]
pub mod mdma_c6isr;
#[doc = "MDMA_C6IFCR (w) register accessor: MDMA channel x interrupt flag clear register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mdma_c6ifcr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mdma_c6ifcr`]
module"]
pub type MDMA_C6IFCR = crate::Reg<mdma_c6ifcr::MDMA_C6IFCR_SPEC>;
#[doc = "MDMA channel x interrupt flag clear register"]
pub mod mdma_c6ifcr;
#[doc = "MDMA_C6ESR (r) register accessor: MDMA Channel x error status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mdma_c6esr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mdma_c6esr`]
module"]
pub type MDMA_C6ESR = crate::Reg<mdma_c6esr::MDMA_C6ESR_SPEC>;
#[doc = "MDMA Channel x error status register"]
pub mod mdma_c6esr;
#[doc = "MDMA_C6CR (rw) register accessor: This register is used to control the concerned channel.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mdma_c6cr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mdma_c6cr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mdma_c6cr`]
module"]
pub type MDMA_C6CR = crate::Reg<mdma_c6cr::MDMA_C6CR_SPEC>;
#[doc = "This register is used to control the concerned channel."]
pub mod mdma_c6cr;
#[doc = "MDMA_C6TCR (rw) register accessor: This register is used to configure the concerned channel.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mdma_c6tcr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mdma_c6tcr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mdma_c6tcr`]
module"]
pub type MDMA_C6TCR = crate::Reg<mdma_c6tcr::MDMA_C6TCR_SPEC>;
#[doc = "This register is used to configure the concerned channel."]
pub mod mdma_c6tcr;
#[doc = "MDMA_C6BNDTR (rw) register accessor: MDMA Channel x block number of data register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mdma_c6bndtr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mdma_c6bndtr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mdma_c6bndtr`]
module"]
pub type MDMA_C6BNDTR = crate::Reg<mdma_c6bndtr::MDMA_C6BNDTR_SPEC>;
#[doc = "MDMA Channel x block number of data register"]
pub mod mdma_c6bndtr;
#[doc = "MDMA_C6SAR (rw) register accessor: MDMA channel x source address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mdma_c6sar::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mdma_c6sar::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mdma_c6sar`]
module"]
pub type MDMA_C6SAR = crate::Reg<mdma_c6sar::MDMA_C6SAR_SPEC>;
#[doc = "MDMA channel x source address register"]
pub mod mdma_c6sar;
#[doc = "MDMA_C6DAR (rw) register accessor: MDMA channel x destination address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mdma_c6dar::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mdma_c6dar::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mdma_c6dar`]
module"]
pub type MDMA_C6DAR = crate::Reg<mdma_c6dar::MDMA_C6DAR_SPEC>;
#[doc = "MDMA channel x destination address register"]
pub mod mdma_c6dar;
#[doc = "MDMA_C6BRUR (rw) register accessor: MDMA channel x Block Repeat address Update register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mdma_c6brur::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mdma_c6brur::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mdma_c6brur`]
module"]
pub type MDMA_C6BRUR = crate::Reg<mdma_c6brur::MDMA_C6BRUR_SPEC>;
#[doc = "MDMA channel x Block Repeat address Update register"]
pub mod mdma_c6brur;
#[doc = "MDMA_C6LAR (rw) register accessor: MDMA channel x Link Address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mdma_c6lar::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mdma_c6lar::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mdma_c6lar`]
module"]
pub type MDMA_C6LAR = crate::Reg<mdma_c6lar::MDMA_C6LAR_SPEC>;
#[doc = "MDMA channel x Link Address register"]
pub mod mdma_c6lar;
#[doc = "MDMA_C6TBR (rw) register accessor: MDMA channel x Trigger and Bus selection Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mdma_c6tbr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mdma_c6tbr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mdma_c6tbr`]
module"]
pub type MDMA_C6TBR = crate::Reg<mdma_c6tbr::MDMA_C6TBR_SPEC>;
#[doc = "MDMA channel x Trigger and Bus selection Register"]
pub mod mdma_c6tbr;
#[doc = "MDMA_C6MAR (rw) register accessor: MDMA channel x Mask address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mdma_c6mar::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mdma_c6mar::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mdma_c6mar`]
module"]
pub type MDMA_C6MAR = crate::Reg<mdma_c6mar::MDMA_C6MAR_SPEC>;
#[doc = "MDMA channel x Mask address register"]
pub mod mdma_c6mar;
#[doc = "MDMA_C6MDR (rw) register accessor: MDMA channel x Mask Data register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mdma_c6mdr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mdma_c6mdr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mdma_c6mdr`]
module"]
pub type MDMA_C6MDR = crate::Reg<mdma_c6mdr::MDMA_C6MDR_SPEC>;
#[doc = "MDMA channel x Mask Data register"]
pub mod mdma_c6mdr;
#[doc = "MDMA_C7ISR (r) register accessor: MDMA channel x interrupt/status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mdma_c7isr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mdma_c7isr`]
module"]
pub type MDMA_C7ISR = crate::Reg<mdma_c7isr::MDMA_C7ISR_SPEC>;
#[doc = "MDMA channel x interrupt/status register"]
pub mod mdma_c7isr;
#[doc = "MDMA_C7IFCR (w) register accessor: MDMA channel x interrupt flag clear register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mdma_c7ifcr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mdma_c7ifcr`]
module"]
pub type MDMA_C7IFCR = crate::Reg<mdma_c7ifcr::MDMA_C7IFCR_SPEC>;
#[doc = "MDMA channel x interrupt flag clear register"]
pub mod mdma_c7ifcr;
#[doc = "MDMA_C7ESR (r) register accessor: MDMA Channel x error status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mdma_c7esr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mdma_c7esr`]
module"]
pub type MDMA_C7ESR = crate::Reg<mdma_c7esr::MDMA_C7ESR_SPEC>;
#[doc = "MDMA Channel x error status register"]
pub mod mdma_c7esr;
#[doc = "MDMA_C7CR (rw) register accessor: This register is used to control the concerned channel.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mdma_c7cr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mdma_c7cr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mdma_c7cr`]
module"]
pub type MDMA_C7CR = crate::Reg<mdma_c7cr::MDMA_C7CR_SPEC>;
#[doc = "This register is used to control the concerned channel."]
pub mod mdma_c7cr;
#[doc = "MDMA_C7TCR (rw) register accessor: This register is used to configure the concerned channel.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mdma_c7tcr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mdma_c7tcr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mdma_c7tcr`]
module"]
pub type MDMA_C7TCR = crate::Reg<mdma_c7tcr::MDMA_C7TCR_SPEC>;
#[doc = "This register is used to configure the concerned channel."]
pub mod mdma_c7tcr;
#[doc = "MDMA_C7BNDTR (rw) register accessor: MDMA Channel x block number of data register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mdma_c7bndtr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mdma_c7bndtr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mdma_c7bndtr`]
module"]
pub type MDMA_C7BNDTR = crate::Reg<mdma_c7bndtr::MDMA_C7BNDTR_SPEC>;
#[doc = "MDMA Channel x block number of data register"]
pub mod mdma_c7bndtr;
#[doc = "MDMA_C7SAR (rw) register accessor: MDMA channel x source address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mdma_c7sar::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mdma_c7sar::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mdma_c7sar`]
module"]
pub type MDMA_C7SAR = crate::Reg<mdma_c7sar::MDMA_C7SAR_SPEC>;
#[doc = "MDMA channel x source address register"]
pub mod mdma_c7sar;
#[doc = "MDMA_C7DAR (rw) register accessor: MDMA channel x destination address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mdma_c7dar::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mdma_c7dar::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mdma_c7dar`]
module"]
pub type MDMA_C7DAR = crate::Reg<mdma_c7dar::MDMA_C7DAR_SPEC>;
#[doc = "MDMA channel x destination address register"]
pub mod mdma_c7dar;
#[doc = "MDMA_C7BRUR (rw) register accessor: MDMA channel x Block Repeat address Update register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mdma_c7brur::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mdma_c7brur::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mdma_c7brur`]
module"]
pub type MDMA_C7BRUR = crate::Reg<mdma_c7brur::MDMA_C7BRUR_SPEC>;
#[doc = "MDMA channel x Block Repeat address Update register"]
pub mod mdma_c7brur;
#[doc = "MDMA_C7LAR (rw) register accessor: MDMA channel x Link Address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mdma_c7lar::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mdma_c7lar::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mdma_c7lar`]
module"]
pub type MDMA_C7LAR = crate::Reg<mdma_c7lar::MDMA_C7LAR_SPEC>;
#[doc = "MDMA channel x Link Address register"]
pub mod mdma_c7lar;
#[doc = "MDMA_C7TBR (rw) register accessor: MDMA channel x Trigger and Bus selection Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mdma_c7tbr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mdma_c7tbr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mdma_c7tbr`]
module"]
pub type MDMA_C7TBR = crate::Reg<mdma_c7tbr::MDMA_C7TBR_SPEC>;
#[doc = "MDMA channel x Trigger and Bus selection Register"]
pub mod mdma_c7tbr;
#[doc = "MDMA_C7MAR (rw) register accessor: MDMA channel x Mask address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mdma_c7mar::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mdma_c7mar::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mdma_c7mar`]
module"]
pub type MDMA_C7MAR = crate::Reg<mdma_c7mar::MDMA_C7MAR_SPEC>;
#[doc = "MDMA channel x Mask address register"]
pub mod mdma_c7mar;
#[doc = "MDMA_C7MDR (rw) register accessor: MDMA channel x Mask Data register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mdma_c7mdr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mdma_c7mdr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mdma_c7mdr`]
module"]
pub type MDMA_C7MDR = crate::Reg<mdma_c7mdr::MDMA_C7MDR_SPEC>;
#[doc = "MDMA channel x Mask Data register"]
pub mod mdma_c7mdr;
#[doc = "MDMA_C8ISR (r) register accessor: MDMA channel x interrupt/status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mdma_c8isr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mdma_c8isr`]
module"]
pub type MDMA_C8ISR = crate::Reg<mdma_c8isr::MDMA_C8ISR_SPEC>;
#[doc = "MDMA channel x interrupt/status register"]
pub mod mdma_c8isr;
#[doc = "MDMA_C8IFCR (w) register accessor: MDMA channel x interrupt flag clear register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mdma_c8ifcr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mdma_c8ifcr`]
module"]
pub type MDMA_C8IFCR = crate::Reg<mdma_c8ifcr::MDMA_C8IFCR_SPEC>;
#[doc = "MDMA channel x interrupt flag clear register"]
pub mod mdma_c8ifcr;
#[doc = "MDMA_C8ESR (r) register accessor: MDMA Channel x error status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mdma_c8esr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mdma_c8esr`]
module"]
pub type MDMA_C8ESR = crate::Reg<mdma_c8esr::MDMA_C8ESR_SPEC>;
#[doc = "MDMA Channel x error status register"]
pub mod mdma_c8esr;
#[doc = "MDMA_C8CR (rw) register accessor: This register is used to control the concerned channel.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mdma_c8cr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mdma_c8cr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mdma_c8cr`]
module"]
pub type MDMA_C8CR = crate::Reg<mdma_c8cr::MDMA_C8CR_SPEC>;
#[doc = "This register is used to control the concerned channel."]
pub mod mdma_c8cr;
#[doc = "MDMA_C8TCR (rw) register accessor: This register is used to configure the concerned channel.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mdma_c8tcr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mdma_c8tcr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mdma_c8tcr`]
module"]
pub type MDMA_C8TCR = crate::Reg<mdma_c8tcr::MDMA_C8TCR_SPEC>;
#[doc = "This register is used to configure the concerned channel."]
pub mod mdma_c8tcr;
#[doc = "MDMA_C8BNDTR (rw) register accessor: MDMA Channel x block number of data register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mdma_c8bndtr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mdma_c8bndtr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mdma_c8bndtr`]
module"]
pub type MDMA_C8BNDTR = crate::Reg<mdma_c8bndtr::MDMA_C8BNDTR_SPEC>;
#[doc = "MDMA Channel x block number of data register"]
pub mod mdma_c8bndtr;
#[doc = "MDMA_C8SAR (rw) register accessor: MDMA channel x source address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mdma_c8sar::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mdma_c8sar::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mdma_c8sar`]
module"]
pub type MDMA_C8SAR = crate::Reg<mdma_c8sar::MDMA_C8SAR_SPEC>;
#[doc = "MDMA channel x source address register"]
pub mod mdma_c8sar;
#[doc = "MDMA_C8DAR (rw) register accessor: MDMA channel x destination address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mdma_c8dar::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mdma_c8dar::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mdma_c8dar`]
module"]
pub type MDMA_C8DAR = crate::Reg<mdma_c8dar::MDMA_C8DAR_SPEC>;
#[doc = "MDMA channel x destination address register"]
pub mod mdma_c8dar;
#[doc = "MDMA_C8BRUR (rw) register accessor: MDMA channel x Block Repeat address Update register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mdma_c8brur::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mdma_c8brur::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mdma_c8brur`]
module"]
pub type MDMA_C8BRUR = crate::Reg<mdma_c8brur::MDMA_C8BRUR_SPEC>;
#[doc = "MDMA channel x Block Repeat address Update register"]
pub mod mdma_c8brur;
#[doc = "MDMA_C8LAR (rw) register accessor: MDMA channel x Link Address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mdma_c8lar::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mdma_c8lar::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mdma_c8lar`]
module"]
pub type MDMA_C8LAR = crate::Reg<mdma_c8lar::MDMA_C8LAR_SPEC>;
#[doc = "MDMA channel x Link Address register"]
pub mod mdma_c8lar;
#[doc = "MDMA_C8TBR (rw) register accessor: MDMA channel x Trigger and Bus selection Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mdma_c8tbr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mdma_c8tbr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mdma_c8tbr`]
module"]
pub type MDMA_C8TBR = crate::Reg<mdma_c8tbr::MDMA_C8TBR_SPEC>;
#[doc = "MDMA channel x Trigger and Bus selection Register"]
pub mod mdma_c8tbr;
#[doc = "MDMA_C8MAR (rw) register accessor: MDMA channel x Mask address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mdma_c8mar::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mdma_c8mar::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mdma_c8mar`]
module"]
pub type MDMA_C8MAR = crate::Reg<mdma_c8mar::MDMA_C8MAR_SPEC>;
#[doc = "MDMA channel x Mask address register"]
pub mod mdma_c8mar;
#[doc = "MDMA_C8MDR (rw) register accessor: MDMA channel x Mask Data register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mdma_c8mdr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mdma_c8mdr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mdma_c8mdr`]
module"]
pub type MDMA_C8MDR = crate::Reg<mdma_c8mdr::MDMA_C8MDR_SPEC>;
#[doc = "MDMA channel x Mask Data register"]
pub mod mdma_c8mdr;
#[doc = "MDMA_C9ISR (r) register accessor: MDMA channel x interrupt/status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mdma_c9isr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mdma_c9isr`]
module"]
pub type MDMA_C9ISR = crate::Reg<mdma_c9isr::MDMA_C9ISR_SPEC>;
#[doc = "MDMA channel x interrupt/status register"]
pub mod mdma_c9isr;
#[doc = "MDMA_C9IFCR (w) register accessor: MDMA channel x interrupt flag clear register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mdma_c9ifcr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mdma_c9ifcr`]
module"]
pub type MDMA_C9IFCR = crate::Reg<mdma_c9ifcr::MDMA_C9IFCR_SPEC>;
#[doc = "MDMA channel x interrupt flag clear register"]
pub mod mdma_c9ifcr;
#[doc = "MDMA_C9ESR (r) register accessor: MDMA Channel x error status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mdma_c9esr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mdma_c9esr`]
module"]
pub type MDMA_C9ESR = crate::Reg<mdma_c9esr::MDMA_C9ESR_SPEC>;
#[doc = "MDMA Channel x error status register"]
pub mod mdma_c9esr;
#[doc = "MDMA_C9CR (rw) register accessor: This register is used to control the concerned channel.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mdma_c9cr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mdma_c9cr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mdma_c9cr`]
module"]
pub type MDMA_C9CR = crate::Reg<mdma_c9cr::MDMA_C9CR_SPEC>;
#[doc = "This register is used to control the concerned channel."]
pub mod mdma_c9cr;
#[doc = "MDMA_C9TCR (rw) register accessor: This register is used to configure the concerned channel.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mdma_c9tcr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mdma_c9tcr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mdma_c9tcr`]
module"]
pub type MDMA_C9TCR = crate::Reg<mdma_c9tcr::MDMA_C9TCR_SPEC>;
#[doc = "This register is used to configure the concerned channel."]
pub mod mdma_c9tcr;
#[doc = "MDMA_C9BNDTR (rw) register accessor: MDMA Channel x block number of data register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mdma_c9bndtr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mdma_c9bndtr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mdma_c9bndtr`]
module"]
pub type MDMA_C9BNDTR = crate::Reg<mdma_c9bndtr::MDMA_C9BNDTR_SPEC>;
#[doc = "MDMA Channel x block number of data register"]
pub mod mdma_c9bndtr;
#[doc = "MDMA_C9SAR (rw) register accessor: MDMA channel x source address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mdma_c9sar::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mdma_c9sar::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mdma_c9sar`]
module"]
pub type MDMA_C9SAR = crate::Reg<mdma_c9sar::MDMA_C9SAR_SPEC>;
#[doc = "MDMA channel x source address register"]
pub mod mdma_c9sar;
#[doc = "MDMA_C9DAR (rw) register accessor: MDMA channel x destination address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mdma_c9dar::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mdma_c9dar::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mdma_c9dar`]
module"]
pub type MDMA_C9DAR = crate::Reg<mdma_c9dar::MDMA_C9DAR_SPEC>;
#[doc = "MDMA channel x destination address register"]
pub mod mdma_c9dar;
#[doc = "MDMA_C9BRUR (rw) register accessor: MDMA channel x Block Repeat address Update register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mdma_c9brur::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mdma_c9brur::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mdma_c9brur`]
module"]
pub type MDMA_C9BRUR = crate::Reg<mdma_c9brur::MDMA_C9BRUR_SPEC>;
#[doc = "MDMA channel x Block Repeat address Update register"]
pub mod mdma_c9brur;
#[doc = "MDMA_C9LAR (rw) register accessor: MDMA channel x Link Address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mdma_c9lar::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mdma_c9lar::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mdma_c9lar`]
module"]
pub type MDMA_C9LAR = crate::Reg<mdma_c9lar::MDMA_C9LAR_SPEC>;
#[doc = "MDMA channel x Link Address register"]
pub mod mdma_c9lar;
#[doc = "MDMA_C9TBR (rw) register accessor: MDMA channel x Trigger and Bus selection Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mdma_c9tbr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mdma_c9tbr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mdma_c9tbr`]
module"]
pub type MDMA_C9TBR = crate::Reg<mdma_c9tbr::MDMA_C9TBR_SPEC>;
#[doc = "MDMA channel x Trigger and Bus selection Register"]
pub mod mdma_c9tbr;
#[doc = "MDMA_C9MAR (rw) register accessor: MDMA channel x Mask address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mdma_c9mar::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mdma_c9mar::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mdma_c9mar`]
module"]
pub type MDMA_C9MAR = crate::Reg<mdma_c9mar::MDMA_C9MAR_SPEC>;
#[doc = "MDMA channel x Mask address register"]
pub mod mdma_c9mar;
#[doc = "MDMA_C9MDR (rw) register accessor: MDMA channel x Mask Data register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mdma_c9mdr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mdma_c9mdr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mdma_c9mdr`]
module"]
pub type MDMA_C9MDR = crate::Reg<mdma_c9mdr::MDMA_C9MDR_SPEC>;
#[doc = "MDMA channel x Mask Data register"]
pub mod mdma_c9mdr;
#[doc = "MDMA_C10ISR (r) register accessor: MDMA channel x interrupt/status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mdma_c10isr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mdma_c10isr`]
module"]
pub type MDMA_C10ISR = crate::Reg<mdma_c10isr::MDMA_C10ISR_SPEC>;
#[doc = "MDMA channel x interrupt/status register"]
pub mod mdma_c10isr;
#[doc = "MDMA_C10IFCR (w) register accessor: MDMA channel x interrupt flag clear register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mdma_c10ifcr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mdma_c10ifcr`]
module"]
pub type MDMA_C10IFCR = crate::Reg<mdma_c10ifcr::MDMA_C10IFCR_SPEC>;
#[doc = "MDMA channel x interrupt flag clear register"]
pub mod mdma_c10ifcr;
#[doc = "MDMA_C10ESR (r) register accessor: MDMA Channel x error status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mdma_c10esr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mdma_c10esr`]
module"]
pub type MDMA_C10ESR = crate::Reg<mdma_c10esr::MDMA_C10ESR_SPEC>;
#[doc = "MDMA Channel x error status register"]
pub mod mdma_c10esr;
#[doc = "MDMA_C10CR (rw) register accessor: This register is used to control the concerned channel.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mdma_c10cr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mdma_c10cr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mdma_c10cr`]
module"]
pub type MDMA_C10CR = crate::Reg<mdma_c10cr::MDMA_C10CR_SPEC>;
#[doc = "This register is used to control the concerned channel."]
pub mod mdma_c10cr;
#[doc = "MDMA_C10TCR (rw) register accessor: This register is used to configure the concerned channel.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mdma_c10tcr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mdma_c10tcr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mdma_c10tcr`]
module"]
pub type MDMA_C10TCR = crate::Reg<mdma_c10tcr::MDMA_C10TCR_SPEC>;
#[doc = "This register is used to configure the concerned channel."]
pub mod mdma_c10tcr;
#[doc = "MDMA_C10BNDTR (rw) register accessor: MDMA Channel x block number of data register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mdma_c10bndtr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mdma_c10bndtr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mdma_c10bndtr`]
module"]
pub type MDMA_C10BNDTR = crate::Reg<mdma_c10bndtr::MDMA_C10BNDTR_SPEC>;
#[doc = "MDMA Channel x block number of data register"]
pub mod mdma_c10bndtr;
#[doc = "MDMA_C10SAR (rw) register accessor: MDMA channel x source address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mdma_c10sar::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mdma_c10sar::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mdma_c10sar`]
module"]
pub type MDMA_C10SAR = crate::Reg<mdma_c10sar::MDMA_C10SAR_SPEC>;
#[doc = "MDMA channel x source address register"]
pub mod mdma_c10sar;
#[doc = "MDMA_C10DAR (rw) register accessor: MDMA channel x destination address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mdma_c10dar::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mdma_c10dar::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mdma_c10dar`]
module"]
pub type MDMA_C10DAR = crate::Reg<mdma_c10dar::MDMA_C10DAR_SPEC>;
#[doc = "MDMA channel x destination address register"]
pub mod mdma_c10dar;
#[doc = "MDMA_C10BRUR (rw) register accessor: MDMA channel x Block Repeat address Update register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mdma_c10brur::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mdma_c10brur::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mdma_c10brur`]
module"]
pub type MDMA_C10BRUR = crate::Reg<mdma_c10brur::MDMA_C10BRUR_SPEC>;
#[doc = "MDMA channel x Block Repeat address Update register"]
pub mod mdma_c10brur;
#[doc = "MDMA_C10LAR (rw) register accessor: MDMA channel x Link Address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mdma_c10lar::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mdma_c10lar::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mdma_c10lar`]
module"]
pub type MDMA_C10LAR = crate::Reg<mdma_c10lar::MDMA_C10LAR_SPEC>;
#[doc = "MDMA channel x Link Address register"]
pub mod mdma_c10lar;
#[doc = "MDMA_C10TBR (rw) register accessor: MDMA channel x Trigger and Bus selection Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mdma_c10tbr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mdma_c10tbr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mdma_c10tbr`]
module"]
pub type MDMA_C10TBR = crate::Reg<mdma_c10tbr::MDMA_C10TBR_SPEC>;
#[doc = "MDMA channel x Trigger and Bus selection Register"]
pub mod mdma_c10tbr;
#[doc = "MDMA_C10MAR (rw) register accessor: MDMA channel x Mask address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mdma_c10mar::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mdma_c10mar::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mdma_c10mar`]
module"]
pub type MDMA_C10MAR = crate::Reg<mdma_c10mar::MDMA_C10MAR_SPEC>;
#[doc = "MDMA channel x Mask address register"]
pub mod mdma_c10mar;
#[doc = "MDMA_C10MDR (rw) register accessor: MDMA channel x Mask Data register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mdma_c10mdr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mdma_c10mdr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mdma_c10mdr`]
module"]
pub type MDMA_C10MDR = crate::Reg<mdma_c10mdr::MDMA_C10MDR_SPEC>;
#[doc = "MDMA channel x Mask Data register"]
pub mod mdma_c10mdr;
#[doc = "MDMA_C11ISR (r) register accessor: MDMA channel x interrupt/status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mdma_c11isr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mdma_c11isr`]
module"]
pub type MDMA_C11ISR = crate::Reg<mdma_c11isr::MDMA_C11ISR_SPEC>;
#[doc = "MDMA channel x interrupt/status register"]
pub mod mdma_c11isr;
#[doc = "MDMA_C11IFCR (w) register accessor: MDMA channel x interrupt flag clear register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mdma_c11ifcr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mdma_c11ifcr`]
module"]
pub type MDMA_C11IFCR = crate::Reg<mdma_c11ifcr::MDMA_C11IFCR_SPEC>;
#[doc = "MDMA channel x interrupt flag clear register"]
pub mod mdma_c11ifcr;
#[doc = "MDMA_C11ESR (r) register accessor: MDMA Channel x error status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mdma_c11esr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mdma_c11esr`]
module"]
pub type MDMA_C11ESR = crate::Reg<mdma_c11esr::MDMA_C11ESR_SPEC>;
#[doc = "MDMA Channel x error status register"]
pub mod mdma_c11esr;
#[doc = "MDMA_C11CR (rw) register accessor: This register is used to control the concerned channel.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mdma_c11cr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mdma_c11cr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mdma_c11cr`]
module"]
pub type MDMA_C11CR = crate::Reg<mdma_c11cr::MDMA_C11CR_SPEC>;
#[doc = "This register is used to control the concerned channel."]
pub mod mdma_c11cr;
#[doc = "MDMA_C11TCR (rw) register accessor: This register is used to configure the concerned channel.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mdma_c11tcr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mdma_c11tcr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mdma_c11tcr`]
module"]
pub type MDMA_C11TCR = crate::Reg<mdma_c11tcr::MDMA_C11TCR_SPEC>;
#[doc = "This register is used to configure the concerned channel."]
pub mod mdma_c11tcr;
#[doc = "MDMA_C11BNDTR (rw) register accessor: MDMA Channel x block number of data register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mdma_c11bndtr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mdma_c11bndtr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mdma_c11bndtr`]
module"]
pub type MDMA_C11BNDTR = crate::Reg<mdma_c11bndtr::MDMA_C11BNDTR_SPEC>;
#[doc = "MDMA Channel x block number of data register"]
pub mod mdma_c11bndtr;
#[doc = "MDMA_C11SAR (rw) register accessor: MDMA channel x source address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mdma_c11sar::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mdma_c11sar::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mdma_c11sar`]
module"]
pub type MDMA_C11SAR = crate::Reg<mdma_c11sar::MDMA_C11SAR_SPEC>;
#[doc = "MDMA channel x source address register"]
pub mod mdma_c11sar;
#[doc = "MDMA_C11DAR (rw) register accessor: MDMA channel x destination address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mdma_c11dar::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mdma_c11dar::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mdma_c11dar`]
module"]
pub type MDMA_C11DAR = crate::Reg<mdma_c11dar::MDMA_C11DAR_SPEC>;
#[doc = "MDMA channel x destination address register"]
pub mod mdma_c11dar;
#[doc = "MDMA_C11BRUR (rw) register accessor: MDMA channel x Block Repeat address Update register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mdma_c11brur::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mdma_c11brur::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mdma_c11brur`]
module"]
pub type MDMA_C11BRUR = crate::Reg<mdma_c11brur::MDMA_C11BRUR_SPEC>;
#[doc = "MDMA channel x Block Repeat address Update register"]
pub mod mdma_c11brur;
#[doc = "MDMA_C11LAR (rw) register accessor: MDMA channel x Link Address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mdma_c11lar::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mdma_c11lar::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mdma_c11lar`]
module"]
pub type MDMA_C11LAR = crate::Reg<mdma_c11lar::MDMA_C11LAR_SPEC>;
#[doc = "MDMA channel x Link Address register"]
pub mod mdma_c11lar;
#[doc = "MDMA_C11TBR (rw) register accessor: MDMA channel x Trigger and Bus selection Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mdma_c11tbr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mdma_c11tbr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mdma_c11tbr`]
module"]
pub type MDMA_C11TBR = crate::Reg<mdma_c11tbr::MDMA_C11TBR_SPEC>;
#[doc = "MDMA channel x Trigger and Bus selection Register"]
pub mod mdma_c11tbr;
#[doc = "MDMA_C11MAR (rw) register accessor: MDMA channel x Mask address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mdma_c11mar::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mdma_c11mar::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mdma_c11mar`]
module"]
pub type MDMA_C11MAR = crate::Reg<mdma_c11mar::MDMA_C11MAR_SPEC>;
#[doc = "MDMA channel x Mask address register"]
pub mod mdma_c11mar;
#[doc = "MDMA_C11MDR (rw) register accessor: MDMA channel x Mask Data register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mdma_c11mdr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mdma_c11mdr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mdma_c11mdr`]
module"]
pub type MDMA_C11MDR = crate::Reg<mdma_c11mdr::MDMA_C11MDR_SPEC>;
#[doc = "MDMA channel x Mask Data register"]
pub mod mdma_c11mdr;
#[doc = "MDMA_C12ISR (r) register accessor: MDMA channel x interrupt/status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mdma_c12isr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mdma_c12isr`]
module"]
pub type MDMA_C12ISR = crate::Reg<mdma_c12isr::MDMA_C12ISR_SPEC>;
#[doc = "MDMA channel x interrupt/status register"]
pub mod mdma_c12isr;
#[doc = "MDMA_C12IFCR (w) register accessor: MDMA channel x interrupt flag clear register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mdma_c12ifcr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mdma_c12ifcr`]
module"]
pub type MDMA_C12IFCR = crate::Reg<mdma_c12ifcr::MDMA_C12IFCR_SPEC>;
#[doc = "MDMA channel x interrupt flag clear register"]
pub mod mdma_c12ifcr;
#[doc = "MDMA_C12ESR (r) register accessor: MDMA Channel x error status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mdma_c12esr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mdma_c12esr`]
module"]
pub type MDMA_C12ESR = crate::Reg<mdma_c12esr::MDMA_C12ESR_SPEC>;
#[doc = "MDMA Channel x error status register"]
pub mod mdma_c12esr;
#[doc = "MDMA_C12CR (rw) register accessor: This register is used to control the concerned channel.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mdma_c12cr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mdma_c12cr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mdma_c12cr`]
module"]
pub type MDMA_C12CR = crate::Reg<mdma_c12cr::MDMA_C12CR_SPEC>;
#[doc = "This register is used to control the concerned channel."]
pub mod mdma_c12cr;
#[doc = "MDMA_C12TCR (rw) register accessor: This register is used to configure the concerned channel.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mdma_c12tcr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mdma_c12tcr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mdma_c12tcr`]
module"]
pub type MDMA_C12TCR = crate::Reg<mdma_c12tcr::MDMA_C12TCR_SPEC>;
#[doc = "This register is used to configure the concerned channel."]
pub mod mdma_c12tcr;
#[doc = "MDMA_C12BNDTR (rw) register accessor: MDMA Channel x block number of data register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mdma_c12bndtr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mdma_c12bndtr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mdma_c12bndtr`]
module"]
pub type MDMA_C12BNDTR = crate::Reg<mdma_c12bndtr::MDMA_C12BNDTR_SPEC>;
#[doc = "MDMA Channel x block number of data register"]
pub mod mdma_c12bndtr;
#[doc = "MDMA_C12SAR (rw) register accessor: MDMA channel x source address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mdma_c12sar::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mdma_c12sar::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mdma_c12sar`]
module"]
pub type MDMA_C12SAR = crate::Reg<mdma_c12sar::MDMA_C12SAR_SPEC>;
#[doc = "MDMA channel x source address register"]
pub mod mdma_c12sar;
#[doc = "MDMA_C12DAR (rw) register accessor: MDMA channel x destination address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mdma_c12dar::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mdma_c12dar::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mdma_c12dar`]
module"]
pub type MDMA_C12DAR = crate::Reg<mdma_c12dar::MDMA_C12DAR_SPEC>;
#[doc = "MDMA channel x destination address register"]
pub mod mdma_c12dar;
#[doc = "MDMA_C12BRUR (rw) register accessor: MDMA channel x Block Repeat address Update register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mdma_c12brur::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mdma_c12brur::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mdma_c12brur`]
module"]
pub type MDMA_C12BRUR = crate::Reg<mdma_c12brur::MDMA_C12BRUR_SPEC>;
#[doc = "MDMA channel x Block Repeat address Update register"]
pub mod mdma_c12brur;
#[doc = "MDMA_C12LAR (rw) register accessor: MDMA channel x Link Address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mdma_c12lar::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mdma_c12lar::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mdma_c12lar`]
module"]
pub type MDMA_C12LAR = crate::Reg<mdma_c12lar::MDMA_C12LAR_SPEC>;
#[doc = "MDMA channel x Link Address register"]
pub mod mdma_c12lar;
#[doc = "MDMA_C12TBR (rw) register accessor: MDMA channel x Trigger and Bus selection Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mdma_c12tbr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mdma_c12tbr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mdma_c12tbr`]
module"]
pub type MDMA_C12TBR = crate::Reg<mdma_c12tbr::MDMA_C12TBR_SPEC>;
#[doc = "MDMA channel x Trigger and Bus selection Register"]
pub mod mdma_c12tbr;
#[doc = "MDMA_C12MAR (rw) register accessor: MDMA channel x Mask address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mdma_c12mar::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mdma_c12mar::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mdma_c12mar`]
module"]
pub type MDMA_C12MAR = crate::Reg<mdma_c12mar::MDMA_C12MAR_SPEC>;
#[doc = "MDMA channel x Mask address register"]
pub mod mdma_c12mar;
#[doc = "MDMA_C12MDR (rw) register accessor: MDMA channel x Mask Data register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mdma_c12mdr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mdma_c12mdr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mdma_c12mdr`]
module"]
pub type MDMA_C12MDR = crate::Reg<mdma_c12mdr::MDMA_C12MDR_SPEC>;
#[doc = "MDMA channel x Mask Data register"]
pub mod mdma_c12mdr;
#[doc = "MDMA_C13ISR (r) register accessor: MDMA channel x interrupt/status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mdma_c13isr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mdma_c13isr`]
module"]
pub type MDMA_C13ISR = crate::Reg<mdma_c13isr::MDMA_C13ISR_SPEC>;
#[doc = "MDMA channel x interrupt/status register"]
pub mod mdma_c13isr;
#[doc = "MDMA_C13IFCR (w) register accessor: MDMA channel x interrupt flag clear register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mdma_c13ifcr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mdma_c13ifcr`]
module"]
pub type MDMA_C13IFCR = crate::Reg<mdma_c13ifcr::MDMA_C13IFCR_SPEC>;
#[doc = "MDMA channel x interrupt flag clear register"]
pub mod mdma_c13ifcr;
#[doc = "MDMA_C13ESR (r) register accessor: MDMA Channel x error status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mdma_c13esr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mdma_c13esr`]
module"]
pub type MDMA_C13ESR = crate::Reg<mdma_c13esr::MDMA_C13ESR_SPEC>;
#[doc = "MDMA Channel x error status register"]
pub mod mdma_c13esr;
#[doc = "MDMA_C13CR (rw) register accessor: This register is used to control the concerned channel.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mdma_c13cr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mdma_c13cr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mdma_c13cr`]
module"]
pub type MDMA_C13CR = crate::Reg<mdma_c13cr::MDMA_C13CR_SPEC>;
#[doc = "This register is used to control the concerned channel."]
pub mod mdma_c13cr;
#[doc = "MDMA_C13TCR (rw) register accessor: This register is used to configure the concerned channel.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mdma_c13tcr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mdma_c13tcr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mdma_c13tcr`]
module"]
pub type MDMA_C13TCR = crate::Reg<mdma_c13tcr::MDMA_C13TCR_SPEC>;
#[doc = "This register is used to configure the concerned channel."]
pub mod mdma_c13tcr;
#[doc = "MDMA_C13BNDTR (rw) register accessor: MDMA Channel x block number of data register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mdma_c13bndtr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mdma_c13bndtr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mdma_c13bndtr`]
module"]
pub type MDMA_C13BNDTR = crate::Reg<mdma_c13bndtr::MDMA_C13BNDTR_SPEC>;
#[doc = "MDMA Channel x block number of data register"]
pub mod mdma_c13bndtr;
#[doc = "MDMA_C13SAR (rw) register accessor: MDMA channel x source address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mdma_c13sar::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mdma_c13sar::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mdma_c13sar`]
module"]
pub type MDMA_C13SAR = crate::Reg<mdma_c13sar::MDMA_C13SAR_SPEC>;
#[doc = "MDMA channel x source address register"]
pub mod mdma_c13sar;
#[doc = "MDMA_C13DAR (rw) register accessor: MDMA channel x destination address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mdma_c13dar::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mdma_c13dar::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mdma_c13dar`]
module"]
pub type MDMA_C13DAR = crate::Reg<mdma_c13dar::MDMA_C13DAR_SPEC>;
#[doc = "MDMA channel x destination address register"]
pub mod mdma_c13dar;
#[doc = "MDMA_C13BRUR (rw) register accessor: MDMA channel x Block Repeat address Update register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mdma_c13brur::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mdma_c13brur::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mdma_c13brur`]
module"]
pub type MDMA_C13BRUR = crate::Reg<mdma_c13brur::MDMA_C13BRUR_SPEC>;
#[doc = "MDMA channel x Block Repeat address Update register"]
pub mod mdma_c13brur;
#[doc = "MDMA_C13LAR (rw) register accessor: MDMA channel x Link Address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mdma_c13lar::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mdma_c13lar::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mdma_c13lar`]
module"]
pub type MDMA_C13LAR = crate::Reg<mdma_c13lar::MDMA_C13LAR_SPEC>;
#[doc = "MDMA channel x Link Address register"]
pub mod mdma_c13lar;
#[doc = "MDMA_C13TBR (rw) register accessor: MDMA channel x Trigger and Bus selection Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mdma_c13tbr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mdma_c13tbr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mdma_c13tbr`]
module"]
pub type MDMA_C13TBR = crate::Reg<mdma_c13tbr::MDMA_C13TBR_SPEC>;
#[doc = "MDMA channel x Trigger and Bus selection Register"]
pub mod mdma_c13tbr;
#[doc = "MDMA_C13MAR (rw) register accessor: MDMA channel x Mask address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mdma_c13mar::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mdma_c13mar::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mdma_c13mar`]
module"]
pub type MDMA_C13MAR = crate::Reg<mdma_c13mar::MDMA_C13MAR_SPEC>;
#[doc = "MDMA channel x Mask address register"]
pub mod mdma_c13mar;
#[doc = "MDMA_C13MDR (rw) register accessor: MDMA channel x Mask Data register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mdma_c13mdr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mdma_c13mdr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mdma_c13mdr`]
module"]
pub type MDMA_C13MDR = crate::Reg<mdma_c13mdr::MDMA_C13MDR_SPEC>;
#[doc = "MDMA channel x Mask Data register"]
pub mod mdma_c13mdr;
#[doc = "MDMA_C14ISR (r) register accessor: MDMA channel x interrupt/status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mdma_c14isr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mdma_c14isr`]
module"]
pub type MDMA_C14ISR = crate::Reg<mdma_c14isr::MDMA_C14ISR_SPEC>;
#[doc = "MDMA channel x interrupt/status register"]
pub mod mdma_c14isr;
#[doc = "MDMA_C14IFCR (w) register accessor: MDMA channel x interrupt flag clear register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mdma_c14ifcr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mdma_c14ifcr`]
module"]
pub type MDMA_C14IFCR = crate::Reg<mdma_c14ifcr::MDMA_C14IFCR_SPEC>;
#[doc = "MDMA channel x interrupt flag clear register"]
pub mod mdma_c14ifcr;
#[doc = "MDMA_C14ESR (r) register accessor: MDMA Channel x error status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mdma_c14esr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mdma_c14esr`]
module"]
pub type MDMA_C14ESR = crate::Reg<mdma_c14esr::MDMA_C14ESR_SPEC>;
#[doc = "MDMA Channel x error status register"]
pub mod mdma_c14esr;
#[doc = "MDMA_C14CR (rw) register accessor: This register is used to control the concerned channel.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mdma_c14cr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mdma_c14cr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mdma_c14cr`]
module"]
pub type MDMA_C14CR = crate::Reg<mdma_c14cr::MDMA_C14CR_SPEC>;
#[doc = "This register is used to control the concerned channel."]
pub mod mdma_c14cr;
#[doc = "MDMA_C14TCR (rw) register accessor: This register is used to configure the concerned channel.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mdma_c14tcr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mdma_c14tcr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mdma_c14tcr`]
module"]
pub type MDMA_C14TCR = crate::Reg<mdma_c14tcr::MDMA_C14TCR_SPEC>;
#[doc = "This register is used to configure the concerned channel."]
pub mod mdma_c14tcr;
#[doc = "MDMA_C14BNDTR (rw) register accessor: MDMA Channel x block number of data register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mdma_c14bndtr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mdma_c14bndtr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mdma_c14bndtr`]
module"]
pub type MDMA_C14BNDTR = crate::Reg<mdma_c14bndtr::MDMA_C14BNDTR_SPEC>;
#[doc = "MDMA Channel x block number of data register"]
pub mod mdma_c14bndtr;
#[doc = "MDMA_C14SAR (rw) register accessor: MDMA channel x source address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mdma_c14sar::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mdma_c14sar::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mdma_c14sar`]
module"]
pub type MDMA_C14SAR = crate::Reg<mdma_c14sar::MDMA_C14SAR_SPEC>;
#[doc = "MDMA channel x source address register"]
pub mod mdma_c14sar;
#[doc = "MDMA_C14DAR (rw) register accessor: MDMA channel x destination address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mdma_c14dar::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mdma_c14dar::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mdma_c14dar`]
module"]
pub type MDMA_C14DAR = crate::Reg<mdma_c14dar::MDMA_C14DAR_SPEC>;
#[doc = "MDMA channel x destination address register"]
pub mod mdma_c14dar;
#[doc = "MDMA_C14BRUR (rw) register accessor: MDMA channel x Block Repeat address Update register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mdma_c14brur::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mdma_c14brur::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mdma_c14brur`]
module"]
pub type MDMA_C14BRUR = crate::Reg<mdma_c14brur::MDMA_C14BRUR_SPEC>;
#[doc = "MDMA channel x Block Repeat address Update register"]
pub mod mdma_c14brur;
#[doc = "MDMA_C14LAR (rw) register accessor: MDMA channel x Link Address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mdma_c14lar::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mdma_c14lar::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mdma_c14lar`]
module"]
pub type MDMA_C14LAR = crate::Reg<mdma_c14lar::MDMA_C14LAR_SPEC>;
#[doc = "MDMA channel x Link Address register"]
pub mod mdma_c14lar;
#[doc = "MDMA_C14TBR (rw) register accessor: MDMA channel x Trigger and Bus selection Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mdma_c14tbr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mdma_c14tbr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mdma_c14tbr`]
module"]
pub type MDMA_C14TBR = crate::Reg<mdma_c14tbr::MDMA_C14TBR_SPEC>;
#[doc = "MDMA channel x Trigger and Bus selection Register"]
pub mod mdma_c14tbr;
#[doc = "MDMA_C14MAR (rw) register accessor: MDMA channel x Mask address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mdma_c14mar::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mdma_c14mar::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mdma_c14mar`]
module"]
pub type MDMA_C14MAR = crate::Reg<mdma_c14mar::MDMA_C14MAR_SPEC>;
#[doc = "MDMA channel x Mask address register"]
pub mod mdma_c14mar;
#[doc = "MDMA_C14MDR (rw) register accessor: MDMA channel x Mask Data register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mdma_c14mdr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mdma_c14mdr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mdma_c14mdr`]
module"]
pub type MDMA_C14MDR = crate::Reg<mdma_c14mdr::MDMA_C14MDR_SPEC>;
#[doc = "MDMA channel x Mask Data register"]
pub mod mdma_c14mdr;
#[doc = "MDMA_C15ISR (r) register accessor: MDMA channel x interrupt/status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mdma_c15isr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mdma_c15isr`]
module"]
pub type MDMA_C15ISR = crate::Reg<mdma_c15isr::MDMA_C15ISR_SPEC>;
#[doc = "MDMA channel x interrupt/status register"]
pub mod mdma_c15isr;
#[doc = "MDMA_C15IFCR (w) register accessor: MDMA channel x interrupt flag clear register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mdma_c15ifcr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mdma_c15ifcr`]
module"]
pub type MDMA_C15IFCR = crate::Reg<mdma_c15ifcr::MDMA_C15IFCR_SPEC>;
#[doc = "MDMA channel x interrupt flag clear register"]
pub mod mdma_c15ifcr;
#[doc = "MDMA_C15ESR (r) register accessor: MDMA Channel x error status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mdma_c15esr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mdma_c15esr`]
module"]
pub type MDMA_C15ESR = crate::Reg<mdma_c15esr::MDMA_C15ESR_SPEC>;
#[doc = "MDMA Channel x error status register"]
pub mod mdma_c15esr;
#[doc = "MDMA_C15CR (rw) register accessor: This register is used to control the concerned channel.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mdma_c15cr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mdma_c15cr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mdma_c15cr`]
module"]
pub type MDMA_C15CR = crate::Reg<mdma_c15cr::MDMA_C15CR_SPEC>;
#[doc = "This register is used to control the concerned channel."]
pub mod mdma_c15cr;
#[doc = "MDMA_C15TCR (rw) register accessor: This register is used to configure the concerned channel.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mdma_c15tcr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mdma_c15tcr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mdma_c15tcr`]
module"]
pub type MDMA_C15TCR = crate::Reg<mdma_c15tcr::MDMA_C15TCR_SPEC>;
#[doc = "This register is used to configure the concerned channel."]
pub mod mdma_c15tcr;
#[doc = "MDMA_C15BNDTR (rw) register accessor: MDMA Channel x block number of data register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mdma_c15bndtr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mdma_c15bndtr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mdma_c15bndtr`]
module"]
pub type MDMA_C15BNDTR = crate::Reg<mdma_c15bndtr::MDMA_C15BNDTR_SPEC>;
#[doc = "MDMA Channel x block number of data register"]
pub mod mdma_c15bndtr;
#[doc = "MDMA_C15SAR (rw) register accessor: MDMA channel x source address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mdma_c15sar::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mdma_c15sar::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mdma_c15sar`]
module"]
pub type MDMA_C15SAR = crate::Reg<mdma_c15sar::MDMA_C15SAR_SPEC>;
#[doc = "MDMA channel x source address register"]
pub mod mdma_c15sar;
#[doc = "MDMA_C15DAR (rw) register accessor: MDMA channel x destination address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mdma_c15dar::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mdma_c15dar::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mdma_c15dar`]
module"]
pub type MDMA_C15DAR = crate::Reg<mdma_c15dar::MDMA_C15DAR_SPEC>;
#[doc = "MDMA channel x destination address register"]
pub mod mdma_c15dar;
#[doc = "MDMA_C15BRUR (rw) register accessor: MDMA channel x Block Repeat address Update register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mdma_c15brur::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mdma_c15brur::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mdma_c15brur`]
module"]
pub type MDMA_C15BRUR = crate::Reg<mdma_c15brur::MDMA_C15BRUR_SPEC>;
#[doc = "MDMA channel x Block Repeat address Update register"]
pub mod mdma_c15brur;
#[doc = "MDMA_C15LAR (rw) register accessor: MDMA channel x Link Address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mdma_c15lar::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mdma_c15lar::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mdma_c15lar`]
module"]
pub type MDMA_C15LAR = crate::Reg<mdma_c15lar::MDMA_C15LAR_SPEC>;
#[doc = "MDMA channel x Link Address register"]
pub mod mdma_c15lar;
#[doc = "MDMA_C15TBR (rw) register accessor: MDMA channel x Trigger and Bus selection Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mdma_c15tbr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mdma_c15tbr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mdma_c15tbr`]
module"]
pub type MDMA_C15TBR = crate::Reg<mdma_c15tbr::MDMA_C15TBR_SPEC>;
#[doc = "MDMA channel x Trigger and Bus selection Register"]
pub mod mdma_c15tbr;
#[doc = "MDMA_C15MAR (rw) register accessor: MDMA channel x Mask address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mdma_c15mar::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mdma_c15mar::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mdma_c15mar`]
module"]
pub type MDMA_C15MAR = crate::Reg<mdma_c15mar::MDMA_C15MAR_SPEC>;
#[doc = "MDMA channel x Mask address register"]
pub mod mdma_c15mar;
#[doc = "MDMA_C15MDR (rw) register accessor: MDMA channel x Mask Data register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mdma_c15mdr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mdma_c15mdr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mdma_c15mdr`]
module"]
pub type MDMA_C15MDR = crate::Reg<mdma_c15mdr::MDMA_C15MDR_SPEC>;
#[doc = "MDMA channel x Mask Data register"]
pub mod mdma_c15mdr;
