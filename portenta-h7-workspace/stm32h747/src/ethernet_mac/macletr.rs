#[doc = "Register `MACLETR` reader"]
pub type R = crate::R<MACLETR_SPEC>;
#[doc = "Register `MACLETR` writer"]
pub type W = crate::W<MACLETR_SPEC>;
#[doc = "Field `LPIET` reader - LPIET"]
pub type LPIET_R = crate::FieldReader<u32>;
#[doc = "Field `LPIET` writer - LPIET"]
pub type LPIET_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 17, O, u32>;
impl R {
    #[doc = "Bits 0:16 - LPIET"]
    #[inline(always)]
    pub fn lpiet(&self) -> LPIET_R {
        LPIET_R::new(self.bits & 0x0001_ffff)
    }
}
impl W {
    #[doc = "Bits 0:16 - LPIET"]
    #[inline(always)]
    #[must_use]
    pub fn lpiet(&mut self) -> LPIET_W<MACLETR_SPEC, 0> {
        LPIET_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "LPI entry timer register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`macletr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`macletr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MACLETR_SPEC;
impl crate::RegisterSpec for MACLETR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`macletr::R`](R) reader structure"]
impl crate::Readable for MACLETR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`macletr::W`](W) writer structure"]
impl crate::Writable for MACLETR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MACLETR to value 0"]
impl crate::Resettable for MACLETR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
