#[doc = "Register `D3PCR2L` reader"]
pub type R = crate::R<D3PCR2L_SPEC>;
#[doc = "Register `D3PCR2L` writer"]
pub type W = crate::W<D3PCR2L_SPEC>;
#[doc = "Field `PCS34` reader - D3 Pending request clear input signal selection on Event input x = truncate ((n+64)/2)"]
pub type PCS34_R = crate::FieldReader;
#[doc = "Field `PCS34` writer - D3 Pending request clear input signal selection on Event input x = truncate ((n+64)/2)"]
pub type PCS34_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
#[doc = "Field `PCS35` reader - D3 Pending request clear input signal selection on Event input x = truncate ((n+64)/2)"]
pub type PCS35_R = crate::FieldReader;
#[doc = "Field `PCS35` writer - D3 Pending request clear input signal selection on Event input x = truncate ((n+64)/2)"]
pub type PCS35_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
#[doc = "Field `PCS41` reader - D3 Pending request clear input signal selection on Event input x = truncate ((n+64)/2)"]
pub type PCS41_R = crate::FieldReader;
#[doc = "Field `PCS41` writer - D3 Pending request clear input signal selection on Event input x = truncate ((n+64)/2)"]
pub type PCS41_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
impl R {
    #[doc = "Bits 4:5 - D3 Pending request clear input signal selection on Event input x = truncate ((n+64)/2)"]
    #[inline(always)]
    pub fn pcs34(&self) -> PCS34_R {
        PCS34_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - D3 Pending request clear input signal selection on Event input x = truncate ((n+64)/2)"]
    #[inline(always)]
    pub fn pcs35(&self) -> PCS35_R {
        PCS35_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 18:19 - D3 Pending request clear input signal selection on Event input x = truncate ((n+64)/2)"]
    #[inline(always)]
    pub fn pcs41(&self) -> PCS41_R {
        PCS41_R::new(((self.bits >> 18) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 4:5 - D3 Pending request clear input signal selection on Event input x = truncate ((n+64)/2)"]
    #[inline(always)]
    #[must_use]
    pub fn pcs34(&mut self) -> PCS34_W<D3PCR2L_SPEC, 4> {
        PCS34_W::new(self)
    }
    #[doc = "Bits 6:7 - D3 Pending request clear input signal selection on Event input x = truncate ((n+64)/2)"]
    #[inline(always)]
    #[must_use]
    pub fn pcs35(&mut self) -> PCS35_W<D3PCR2L_SPEC, 6> {
        PCS35_W::new(self)
    }
    #[doc = "Bits 18:19 - D3 Pending request clear input signal selection on Event input x = truncate ((n+64)/2)"]
    #[inline(always)]
    #[must_use]
    pub fn pcs41(&mut self) -> PCS41_W<D3PCR2L_SPEC, 18> {
        PCS41_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "EXTI D3 pending clear selection register low\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`d3pcr2l::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`d3pcr2l::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct D3PCR2L_SPEC;
impl crate::RegisterSpec for D3PCR2L_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`d3pcr2l::R`](R) reader structure"]
impl crate::Readable for D3PCR2L_SPEC {}
#[doc = "`write(|w| ..)` method takes [`d3pcr2l::W`](W) writer structure"]
impl crate::Writable for D3PCR2L_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets D3PCR2L to value 0"]
impl crate::Resettable for D3PCR2L_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
