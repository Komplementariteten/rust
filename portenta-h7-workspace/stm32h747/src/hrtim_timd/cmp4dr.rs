#[doc = "Register `CMP4DR` reader"]
pub type R = crate::R<CMP4DR_SPEC>;
#[doc = "Register `CMP4DR` writer"]
pub type W = crate::W<CMP4DR_SPEC>;
#[doc = "Field `CMP4x` reader - Timerx Compare 4 value"]
pub type CMP4X_R = crate::FieldReader<u16>;
#[doc = "Field `CMP4x` writer - Timerx Compare 4 value"]
pub type CMP4X_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 16, O, u16>;
impl R {
    #[doc = "Bits 0:15 - Timerx Compare 4 value"]
    #[inline(always)]
    pub fn cmp4x(&self) -> CMP4X_R {
        CMP4X_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Timerx Compare 4 value"]
    #[inline(always)]
    #[must_use]
    pub fn cmp4x(&mut self) -> CMP4X_W<CMP4DR_SPEC, 0> {
        CMP4X_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Timerx Compare 4 Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cmp4dr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cmp4dr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CMP4DR_SPEC;
impl crate::RegisterSpec for CMP4DR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cmp4dr::R`](R) reader structure"]
impl crate::Readable for CMP4DR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`cmp4dr::W`](W) writer structure"]
impl crate::Writable for CMP4DR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CMP4DR to value 0"]
impl crate::Resettable for CMP4DR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
