#[doc = "Register `PSMAR` reader"]
pub type R = crate::R<PSMAR_SPEC>;
#[doc = "Register `PSMAR` writer"]
pub type W = crate::W<PSMAR_SPEC>;
#[doc = "Field `MATCH` reader - Status match Value to be compared with the masked status register to get a match. This field can be written only when BUSY = 0."]
pub type MATCH_R = crate::FieldReader<u32>;
#[doc = "Field `MATCH` writer - Status match Value to be compared with the masked status register to get a match. This field can be written only when BUSY = 0."]
pub type MATCH_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 32, O, u32>;
impl R {
    #[doc = "Bits 0:31 - Status match Value to be compared with the masked status register to get a match. This field can be written only when BUSY = 0."]
    #[inline(always)]
    pub fn match_(&self) -> MATCH_R {
        MATCH_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Status match Value to be compared with the masked status register to get a match. This field can be written only when BUSY = 0."]
    #[inline(always)]
    #[must_use]
    pub fn match_(&mut self) -> MATCH_W<PSMAR_SPEC, 0> {
        MATCH_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "QUADSPI polling status match register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`psmar::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`psmar::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PSMAR_SPEC;
impl crate::RegisterSpec for PSMAR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`psmar::R`](R) reader structure"]
impl crate::Readable for PSMAR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`psmar::W`](W) writer structure"]
impl crate::Writable for PSMAR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PSMAR to value 0"]
impl crate::Resettable for PSMAR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
