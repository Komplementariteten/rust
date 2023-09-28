#[doc = "Register `S6NDTR` reader"]
pub type R = crate::R<S6NDTR_SPEC>;
#[doc = "Register `S6NDTR` writer"]
pub type W = crate::W<S6NDTR_SPEC>;
#[doc = "Field `NDT` reader - Number of data items to transfer"]
pub type NDT_R = crate::FieldReader<u16>;
#[doc = "Field `NDT` writer - Number of data items to transfer"]
pub type NDT_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 16, O, u16>;
impl R {
    #[doc = "Bits 0:15 - Number of data items to transfer"]
    #[inline(always)]
    pub fn ndt(&self) -> NDT_R {
        NDT_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Number of data items to transfer"]
    #[inline(always)]
    #[must_use]
    pub fn ndt(&mut self) -> NDT_W<S6NDTR_SPEC, 0> {
        NDT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "stream x number of data register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`s6ndtr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`s6ndtr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct S6NDTR_SPEC;
impl crate::RegisterSpec for S6NDTR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`s6ndtr::R`](R) reader structure"]
impl crate::Readable for S6NDTR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`s6ndtr::W`](W) writer structure"]
impl crate::Writable for S6NDTR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets S6NDTR to value 0"]
impl crate::Resettable for S6NDTR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
