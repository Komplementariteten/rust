#[doc = "Register `MDIOS_CLRFR` reader"]
pub type R = crate::R<MDIOS_CLRFR_SPEC>;
#[doc = "Register `MDIOS_CLRFR` writer"]
pub type W = crate::W<MDIOS_CLRFR_SPEC>;
#[doc = "Field `CPERF` reader - Clear the preamble error flag"]
pub type CPERF_R = crate::BitReader;
#[doc = "Field `CPERF` writer - Clear the preamble error flag"]
pub type CPERF_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CSERF` reader - Clear the start error flag"]
pub type CSERF_R = crate::BitReader;
#[doc = "Field `CSERF` writer - Clear the start error flag"]
pub type CSERF_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CTERF` reader - Clear the turnaround error flag"]
pub type CTERF_R = crate::BitReader;
#[doc = "Field `CTERF` writer - Clear the turnaround error flag"]
pub type CTERF_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - Clear the preamble error flag"]
    #[inline(always)]
    pub fn cperf(&self) -> CPERF_R {
        CPERF_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Clear the start error flag"]
    #[inline(always)]
    pub fn cserf(&self) -> CSERF_R {
        CSERF_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Clear the turnaround error flag"]
    #[inline(always)]
    pub fn cterf(&self) -> CTERF_R {
        CTERF_R::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Clear the preamble error flag"]
    #[inline(always)]
    #[must_use]
    pub fn cperf(&mut self) -> CPERF_W<MDIOS_CLRFR_SPEC, 0> {
        CPERF_W::new(self)
    }
    #[doc = "Bit 1 - Clear the start error flag"]
    #[inline(always)]
    #[must_use]
    pub fn cserf(&mut self) -> CSERF_W<MDIOS_CLRFR_SPEC, 1> {
        CSERF_W::new(self)
    }
    #[doc = "Bit 2 - Clear the turnaround error flag"]
    #[inline(always)]
    #[must_use]
    pub fn cterf(&mut self) -> CTERF_W<MDIOS_CLRFR_SPEC, 2> {
        CTERF_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "MDIOS clear flag register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mdios_clrfr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mdios_clrfr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MDIOS_CLRFR_SPEC;
impl crate::RegisterSpec for MDIOS_CLRFR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mdios_clrfr::R`](R) reader structure"]
impl crate::Readable for MDIOS_CLRFR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`mdios_clrfr::W`](W) writer structure"]
impl crate::Writable for MDIOS_CLRFR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MDIOS_CLRFR to value 0"]
impl crate::Resettable for MDIOS_CLRFR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
