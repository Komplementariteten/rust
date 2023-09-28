#[doc = "Register `CMP1AR` reader"]
pub type R = crate::R<CMP1AR_SPEC>;
#[doc = "Register `CMP1AR` writer"]
pub type W = crate::W<CMP1AR_SPEC>;
#[doc = "Field `CMP1x` reader - Timerx Compare 1 value"]
pub type CMP1X_R = crate::FieldReader<u16>;
#[doc = "Field `CMP1x` writer - Timerx Compare 1 value"]
pub type CMP1X_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 16, O, u16>;
impl R {
    #[doc = "Bits 0:15 - Timerx Compare 1 value"]
    #[inline(always)]
    pub fn cmp1x(&self) -> CMP1X_R {
        CMP1X_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Timerx Compare 1 value"]
    #[inline(always)]
    #[must_use]
    pub fn cmp1x(&mut self) -> CMP1X_W<CMP1AR_SPEC, 0> {
        CMP1X_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Timerx Compare 1 Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cmp1ar::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cmp1ar::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CMP1AR_SPEC;
impl crate::RegisterSpec for CMP1AR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cmp1ar::R`](R) reader structure"]
impl crate::Readable for CMP1AR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`cmp1ar::W`](W) writer structure"]
impl crate::Writable for CMP1AR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CMP1AR to value 0"]
impl crate::Resettable for CMP1AR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
