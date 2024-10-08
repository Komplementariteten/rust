#[doc = "Register `TX_PACKET_COUNT_GOOD` reader"]
pub type R = crate::R<TX_PACKET_COUNT_GOOD_SPEC>;
#[doc = "Field `TXPKTG` reader - TXPKTG"]
pub type TXPKTG_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - TXPKTG"]
    #[inline(always)]
    pub fn txpktg(&self) -> TXPKTG_R {
        TXPKTG_R::new(self.bits)
    }
}
#[doc = "Tx packet count good register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tx_packet_count_good::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TX_PACKET_COUNT_GOOD_SPEC;
impl crate::RegisterSpec for TX_PACKET_COUNT_GOOD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tx_packet_count_good::R`](R) reader structure"]
impl crate::Readable for TX_PACKET_COUNT_GOOD_SPEC {}
#[doc = "`reset()` method sets TX_PACKET_COUNT_GOOD to value 0"]
impl crate::Resettable for TX_PACKET_COUNT_GOOD_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
