#[doc = "Register `RESP2R` reader"]
pub type R = crate::R<RESP2R_SPEC>;
#[doc = "Field `CARDSTATUS2` reader - see Table404."]
pub type CARDSTATUS2_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - see Table404."]
    #[inline(always)]
    pub fn cardstatus2(&self) -> CARDSTATUS2_R {
        CARDSTATUS2_R::new(self.bits)
    }
}
#[doc = "The SDMMC_RESP1/2/3/4R registers contain the status of a card, which is part of the received response.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`resp2r::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RESP2R_SPEC;
impl crate::RegisterSpec for RESP2R_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`resp2r::R`](R) reader structure"]
impl crate::Readable for RESP2R_SPEC {}
#[doc = "`reset()` method sets RESP2R to value 0"]
impl crate::Resettable for RESP2R_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
