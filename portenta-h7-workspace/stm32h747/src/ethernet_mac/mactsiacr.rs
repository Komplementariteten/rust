#[doc = "Register `MACTSIACR` reader"]
pub type R = crate::R<MACTSIACR_SPEC>;
#[doc = "Register `MACTSIACR` writer"]
pub type W = crate::W<MACTSIACR_SPEC>;
#[doc = "Field `OSTIAC` reader - OSTIAC"]
pub type OSTIAC_R = crate::FieldReader<u32>;
#[doc = "Field `OSTIAC` writer - OSTIAC"]
pub type OSTIAC_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 32, O, u32>;
impl R {
    #[doc = "Bits 0:31 - OSTIAC"]
    #[inline(always)]
    pub fn ostiac(&self) -> OSTIAC_R {
        OSTIAC_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - OSTIAC"]
    #[inline(always)]
    #[must_use]
    pub fn ostiac(&mut self) -> OSTIAC_W<MACTSIACR_SPEC, 0> {
        OSTIAC_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Timestamp Ingress asymmetric correction register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mactsiacr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mactsiacr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MACTSIACR_SPEC;
impl crate::RegisterSpec for MACTSIACR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mactsiacr::R`](R) reader structure"]
impl crate::Readable for MACTSIACR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`mactsiacr::W`](W) writer structure"]
impl crate::Writable for MACTSIACR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MACTSIACR to value 0"]
impl crate::Resettable for MACTSIACR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
