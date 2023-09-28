#[doc = "Register `MACPPSIR` reader"]
pub type R = crate::R<MACPPSIR_SPEC>;
#[doc = "Register `MACPPSIR` writer"]
pub type W = crate::W<MACPPSIR_SPEC>;
#[doc = "Field `PPSINT0` reader - PPSINT0"]
pub type PPSINT0_R = crate::FieldReader<u32>;
#[doc = "Field `PPSINT0` writer - PPSINT0"]
pub type PPSINT0_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 32, O, u32>;
impl R {
    #[doc = "Bits 0:31 - PPSINT0"]
    #[inline(always)]
    pub fn ppsint0(&self) -> PPSINT0_R {
        PPSINT0_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - PPSINT0"]
    #[inline(always)]
    #[must_use]
    pub fn ppsint0(&mut self) -> PPSINT0_W<MACPPSIR_SPEC, 0> {
        PPSINT0_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "PPS interval register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`macppsir::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`macppsir::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MACPPSIR_SPEC;
impl crate::RegisterSpec for MACPPSIR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`macppsir::R`](R) reader structure"]
impl crate::Readable for MACPPSIR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`macppsir::W`](W) writer structure"]
impl crate::Writable for MACPPSIR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MACPPSIR to value 0"]
impl crate::Resettable for MACPPSIR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
