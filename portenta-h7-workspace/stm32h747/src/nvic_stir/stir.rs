#[doc = "Register `STIR` reader"]
pub type R = crate::R<STIR_SPEC>;
#[doc = "Register `STIR` writer"]
pub type W = crate::W<STIR_SPEC>;
#[doc = "Field `INTID` reader - Software generated interrupt ID"]
pub type INTID_R = crate::FieldReader<u16>;
#[doc = "Field `INTID` writer - Software generated interrupt ID"]
pub type INTID_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 9, O, u16>;
impl R {
    #[doc = "Bits 0:8 - Software generated interrupt ID"]
    #[inline(always)]
    pub fn intid(&self) -> INTID_R {
        INTID_R::new((self.bits & 0x01ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:8 - Software generated interrupt ID"]
    #[inline(always)]
    #[must_use]
    pub fn intid(&mut self) -> INTID_W<STIR_SPEC, 0> {
        INTID_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Software trigger interrupt register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`stir::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`stir::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct STIR_SPEC;
impl crate::RegisterSpec for STIR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`stir::R`](R) reader structure"]
impl crate::Readable for STIR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`stir::W`](W) writer structure"]
impl crate::Writable for STIR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets STIR to value 0"]
impl crate::Resettable for STIR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
