#[doc = "Register `TX_SINGLE_COLLISION_GOOD_PACKETS` reader"]
pub type R = crate::R<TX_SINGLE_COLLISION_GOOD_PACKETS_SPEC>;
#[doc = "Field `TXSNGLCOLG` reader - TXSNGLCOLG"]
pub type TXSNGLCOLG_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - TXSNGLCOLG"]
    #[inline(always)]
    pub fn txsnglcolg(&self) -> TXSNGLCOLG_R {
        TXSNGLCOLG_R::new(self.bits)
    }
}
#[doc = "Tx single collision good packets register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tx_single_collision_good_packets::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TX_SINGLE_COLLISION_GOOD_PACKETS_SPEC;
impl crate::RegisterSpec for TX_SINGLE_COLLISION_GOOD_PACKETS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tx_single_collision_good_packets::R`](R) reader structure"]
impl crate::Readable for TX_SINGLE_COLLISION_GOOD_PACKETS_SPEC {}
#[doc = "`reset()` method sets TX_SINGLE_COLLISION_GOOD_PACKETS to value 0"]
impl crate::Resettable for TX_SINGLE_COLLISION_GOOD_PACKETS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
