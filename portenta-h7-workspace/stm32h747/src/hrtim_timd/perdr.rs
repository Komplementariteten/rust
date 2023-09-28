#[doc = "Register `PERDR` reader"]
pub type R = crate::R<PERDR_SPEC>;
#[doc = "Register `PERDR` writer"]
pub type W = crate::W<PERDR_SPEC>;
#[doc = "Field `PERx` reader - Timerx Period value"]
pub type PERX_R = crate::FieldReader<u16>;
#[doc = "Field `PERx` writer - Timerx Period value"]
pub type PERX_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 16, O, u16>;
impl R {
    #[doc = "Bits 0:15 - Timerx Period value"]
    #[inline(always)]
    pub fn perx(&self) -> PERX_R {
        PERX_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Timerx Period value"]
    #[inline(always)]
    #[must_use]
    pub fn perx(&mut self) -> PERX_W<PERDR_SPEC, 0> {
        PERX_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Timerx Period Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`perdr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`perdr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PERDR_SPEC;
impl crate::RegisterSpec for PERDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`perdr::R`](R) reader structure"]
impl crate::Readable for PERDR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`perdr::W`](W) writer structure"]
impl crate::Writable for PERDR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PERDR to value 0xffff"]
impl crate::Resettable for PERDR_SPEC {
    const RESET_VALUE: Self::Ux = 0xffff;
}
