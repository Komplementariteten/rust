#[doc = "Register `CMP2CR` reader"]
pub type R = crate::R<CMP2CR_SPEC>;
#[doc = "Register `CMP2CR` writer"]
pub type W = crate::W<CMP2CR_SPEC>;
#[doc = "Field `CMP2x` reader - Timerx Compare 2 value"]
pub type CMP2X_R = crate::FieldReader<u16>;
#[doc = "Field `CMP2x` writer - Timerx Compare 2 value"]
pub type CMP2X_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 16, O, u16>;
impl R {
    #[doc = "Bits 0:15 - Timerx Compare 2 value"]
    #[inline(always)]
    pub fn cmp2x(&self) -> CMP2X_R {
        CMP2X_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Timerx Compare 2 value"]
    #[inline(always)]
    #[must_use]
    pub fn cmp2x(&mut self) -> CMP2X_W<CMP2CR_SPEC, 0> {
        CMP2X_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Timerx Compare 2 Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cmp2cr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cmp2cr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CMP2CR_SPEC;
impl crate::RegisterSpec for CMP2CR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cmp2cr::R`](R) reader structure"]
impl crate::Readable for CMP2CR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`cmp2cr::W`](W) writer structure"]
impl crate::Writable for CMP2CR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CMP2CR to value 0"]
impl crate::Resettable for CMP2CR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
