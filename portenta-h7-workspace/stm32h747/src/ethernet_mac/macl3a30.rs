#[doc = "Register `MACL3A30` reader"]
pub type R = crate::R<MACL3A30_SPEC>;
#[doc = "Register `MACL3A30` writer"]
pub type W = crate::W<MACL3A30_SPEC>;
#[doc = "Field `L3A30` reader - L3A30"]
pub type L3A30_R = crate::FieldReader<u32>;
#[doc = "Field `L3A30` writer - L3A30"]
pub type L3A30_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 32, O, u32>;
impl R {
    #[doc = "Bits 0:31 - L3A30"]
    #[inline(always)]
    pub fn l3a30(&self) -> L3A30_R {
        L3A30_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - L3A30"]
    #[inline(always)]
    #[must_use]
    pub fn l3a30(&mut self) -> L3A30_W<MACL3A30_SPEC, 0> {
        L3A30_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Layer3 Address 3 filter 0 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`macl3a30::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`macl3a30::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MACL3A30_SPEC;
impl crate::RegisterSpec for MACL3A30_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`macl3a30::R`](R) reader structure"]
impl crate::Readable for MACL3A30_SPEC {}
#[doc = "`write(|w| ..)` method takes [`macl3a30::W`](W) writer structure"]
impl crate::Writable for MACL3A30_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MACL3A30 to value 0"]
impl crate::Resettable for MACL3A30_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
