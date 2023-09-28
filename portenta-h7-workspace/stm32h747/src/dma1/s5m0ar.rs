#[doc = "Register `S5M0AR` reader"]
pub type R = crate::R<S5M0AR_SPEC>;
#[doc = "Register `S5M0AR` writer"]
pub type W = crate::W<S5M0AR_SPEC>;
#[doc = "Field `M0A` reader - Memory 0 address"]
pub type M0A_R = crate::FieldReader<u32>;
#[doc = "Field `M0A` writer - Memory 0 address"]
pub type M0A_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 32, O, u32>;
impl R {
    #[doc = "Bits 0:31 - Memory 0 address"]
    #[inline(always)]
    pub fn m0a(&self) -> M0A_R {
        M0A_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Memory 0 address"]
    #[inline(always)]
    #[must_use]
    pub fn m0a(&mut self) -> M0A_W<S5M0AR_SPEC, 0> {
        M0A_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "stream x memory 0 address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`s5m0ar::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`s5m0ar::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct S5M0AR_SPEC;
impl crate::RegisterSpec for S5M0AR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`s5m0ar::R`](R) reader structure"]
impl crate::Readable for S5M0AR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`s5m0ar::W`](W) writer structure"]
impl crate::Writable for S5M0AR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets S5M0AR to value 0"]
impl crate::Resettable for S5M0AR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
