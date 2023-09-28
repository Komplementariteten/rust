#[doc = "Register `CRR6` reader"]
pub type R = crate::R<CRR6_SPEC>;
#[doc = "Register `CRR6` writer"]
pub type W = crate::W<CRR6_SPEC>;
#[doc = "Field `CCR6` reader - Capture/Compare 6 value"]
pub type CCR6_R = crate::FieldReader<u16>;
#[doc = "Field `CCR6` writer - Capture/Compare 6 value"]
pub type CCR6_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 16, O, u16>;
impl R {
    #[doc = "Bits 0:15 - Capture/Compare 6 value"]
    #[inline(always)]
    pub fn ccr6(&self) -> CCR6_R {
        CCR6_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Capture/Compare 6 value"]
    #[inline(always)]
    #[must_use]
    pub fn ccr6(&mut self) -> CCR6_W<CRR6_SPEC, 0> {
        CCR6_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "capture/compare register 6\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`crr6::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`crr6::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CRR6_SPEC;
impl crate::RegisterSpec for CRR6_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`crr6::R`](R) reader structure"]
impl crate::Readable for CRR6_SPEC {}
#[doc = "`write(|w| ..)` method takes [`crr6::W`](W) writer structure"]
impl crate::Writable for CRR6_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CRR6 to value 0"]
impl crate::Resettable for CRR6_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
