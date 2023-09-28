#[doc = "Register `MACARPAR` reader"]
pub type R = crate::R<MACARPAR_SPEC>;
#[doc = "Register `MACARPAR` writer"]
pub type W = crate::W<MACARPAR_SPEC>;
#[doc = "Field `ARPPA` reader - ARPPA"]
pub type ARPPA_R = crate::FieldReader<u32>;
#[doc = "Field `ARPPA` writer - ARPPA"]
pub type ARPPA_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 32, O, u32>;
impl R {
    #[doc = "Bits 0:31 - ARPPA"]
    #[inline(always)]
    pub fn arppa(&self) -> ARPPA_R {
        ARPPA_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - ARPPA"]
    #[inline(always)]
    #[must_use]
    pub fn arppa(&mut self) -> ARPPA_W<MACARPAR_SPEC, 0> {
        ARPPA_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "ARP address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`macarpar::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`macarpar::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MACARPAR_SPEC;
impl crate::RegisterSpec for MACARPAR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`macarpar::R`](R) reader structure"]
impl crate::Readable for MACARPAR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`macarpar::W`](W) writer structure"]
impl crate::Writable for MACARPAR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MACARPAR to value 0"]
impl crate::Resettable for MACARPAR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
