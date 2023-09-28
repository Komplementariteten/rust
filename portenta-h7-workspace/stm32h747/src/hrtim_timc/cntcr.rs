#[doc = "Register `CNTCR` reader"]
pub type R = crate::R<CNTCR_SPEC>;
#[doc = "Register `CNTCR` writer"]
pub type W = crate::W<CNTCR_SPEC>;
#[doc = "Field `CNTx` reader - Timerx Counter value"]
pub type CNTX_R = crate::FieldReader<u16>;
#[doc = "Field `CNTx` writer - Timerx Counter value"]
pub type CNTX_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 16, O, u16>;
impl R {
    #[doc = "Bits 0:15 - Timerx Counter value"]
    #[inline(always)]
    pub fn cntx(&self) -> CNTX_R {
        CNTX_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Timerx Counter value"]
    #[inline(always)]
    #[must_use]
    pub fn cntx(&mut self) -> CNTX_W<CNTCR_SPEC, 0> {
        CNTX_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Timerx Counter Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cntcr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cntcr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CNTCR_SPEC;
impl crate::RegisterSpec for CNTCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cntcr::R`](R) reader structure"]
impl crate::Readable for CNTCR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`cntcr::W`](W) writer structure"]
impl crate::Writable for CNTCR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CNTCR to value 0"]
impl crate::Resettable for CNTCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
