#[doc = "Register `D3PCR1H` reader"]
pub type R = crate::R<D3PCR1H_SPEC>;
#[doc = "Register `D3PCR1H` writer"]
pub type W = crate::W<D3PCR1H_SPEC>;
#[doc = "Field `PCS19` reader - D3 Pending request clear input signal selection on Event input x = truncate ((n+32)/2)"]
pub type PCS19_R = crate::FieldReader;
#[doc = "Field `PCS19` writer - D3 Pending request clear input signal selection on Event input x = truncate ((n+32)/2)"]
pub type PCS19_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
#[doc = "Field `PCS20` reader - D3 Pending request clear input signal selection on Event input x = truncate ((n+32)/2)"]
pub type PCS20_R = crate::FieldReader;
#[doc = "Field `PCS20` writer - D3 Pending request clear input signal selection on Event input x = truncate ((n+32)/2)"]
pub type PCS20_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
#[doc = "Field `PCS21` reader - D3 Pending request clear input signal selection on Event input x = truncate ((n+32)/2)"]
pub type PCS21_R = crate::FieldReader;
#[doc = "Field `PCS21` writer - D3 Pending request clear input signal selection on Event input x = truncate ((n+32)/2)"]
pub type PCS21_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
#[doc = "Field `PCS25` reader - D3 Pending request clear input signal selection on Event input x = truncate ((n+32)/2)"]
pub type PCS25_R = crate::FieldReader;
#[doc = "Field `PCS25` writer - D3 Pending request clear input signal selection on Event input x = truncate ((n+32)/2)"]
pub type PCS25_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
impl R {
    #[doc = "Bits 6:7 - D3 Pending request clear input signal selection on Event input x = truncate ((n+32)/2)"]
    #[inline(always)]
    pub fn pcs19(&self) -> PCS19_R {
        PCS19_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:9 - D3 Pending request clear input signal selection on Event input x = truncate ((n+32)/2)"]
    #[inline(always)]
    pub fn pcs20(&self) -> PCS20_R {
        PCS20_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - D3 Pending request clear input signal selection on Event input x = truncate ((n+32)/2)"]
    #[inline(always)]
    pub fn pcs21(&self) -> PCS21_R {
        PCS21_R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 18:19 - D3 Pending request clear input signal selection on Event input x = truncate ((n+32)/2)"]
    #[inline(always)]
    pub fn pcs25(&self) -> PCS25_R {
        PCS25_R::new(((self.bits >> 18) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 6:7 - D3 Pending request clear input signal selection on Event input x = truncate ((n+32)/2)"]
    #[inline(always)]
    #[must_use]
    pub fn pcs19(&mut self) -> PCS19_W<D3PCR1H_SPEC, 6> {
        PCS19_W::new(self)
    }
    #[doc = "Bits 8:9 - D3 Pending request clear input signal selection on Event input x = truncate ((n+32)/2)"]
    #[inline(always)]
    #[must_use]
    pub fn pcs20(&mut self) -> PCS20_W<D3PCR1H_SPEC, 8> {
        PCS20_W::new(self)
    }
    #[doc = "Bits 10:11 - D3 Pending request clear input signal selection on Event input x = truncate ((n+32)/2)"]
    #[inline(always)]
    #[must_use]
    pub fn pcs21(&mut self) -> PCS21_W<D3PCR1H_SPEC, 10> {
        PCS21_W::new(self)
    }
    #[doc = "Bits 18:19 - D3 Pending request clear input signal selection on Event input x = truncate ((n+32)/2)"]
    #[inline(always)]
    #[must_use]
    pub fn pcs25(&mut self) -> PCS25_W<D3PCR1H_SPEC, 18> {
        PCS25_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "EXTI D3 pending clear selection register high\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`d3pcr1h::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`d3pcr1h::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct D3PCR1H_SPEC;
impl crate::RegisterSpec for D3PCR1H_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`d3pcr1h::R`](R) reader structure"]
impl crate::Readable for D3PCR1H_SPEC {}
#[doc = "`write(|w| ..)` method takes [`d3pcr1h::W`](W) writer structure"]
impl crate::Writable for D3PCR1H_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets D3PCR1H to value 0"]
impl crate::Resettable for D3PCR1H_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
