#[doc = "Register `RESP4R` reader"]
pub type R = crate::R<RESP4R_SPEC>;
#[doc = "Field `CARDSTATUS4` reader - see Table404."]
pub type CARDSTATUS4_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - see Table404."]
    #[inline(always)]
    pub fn cardstatus4(&self) -> CARDSTATUS4_R {
        CARDSTATUS4_R::new(self.bits)
    }
}
#[doc = "The SDMMC_RESP1/2/3/4R registers contain the status of a card, which is part of the received response.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`resp4r::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RESP4R_SPEC;
impl crate::RegisterSpec for RESP4R_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`resp4r::R`](R) reader structure"]
impl crate::Readable for RESP4R_SPEC {}
#[doc = "`reset()` method sets RESP4R to value 0"]
impl crate::Resettable for RESP4R_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
