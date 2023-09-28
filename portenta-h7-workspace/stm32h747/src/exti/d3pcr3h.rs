#[doc = "Register `D3PCR3H` reader"]
pub type R = crate::R<D3PCR3H_SPEC>;
#[doc = "Register `D3PCR3H` writer"]
pub type W = crate::W<D3PCR3H_SPEC>;
#[doc = "Field `PCS88` reader - D3 Pending request clear input signal selection on Event input x= truncate N+160/2"]
pub type PCS88_R = crate::FieldReader;
#[doc = "Field `PCS88` writer - D3 Pending request clear input signal selection on Event input x= truncate N+160/2"]
pub type PCS88_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
impl R {
    #[doc = "Bits 18:19 - D3 Pending request clear input signal selection on Event input x= truncate N+160/2"]
    #[inline(always)]
    pub fn pcs88(&self) -> PCS88_R {
        PCS88_R::new(((self.bits >> 18) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 18:19 - D3 Pending request clear input signal selection on Event input x= truncate N+160/2"]
    #[inline(always)]
    #[must_use]
    pub fn pcs88(&mut self) -> PCS88_W<D3PCR3H_SPEC, 18> {
        PCS88_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "EXTI D3 pending clear selection register high\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`d3pcr3h::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`d3pcr3h::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct D3PCR3H_SPEC;
impl crate::RegisterSpec for D3PCR3H_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`d3pcr3h::R`](R) reader structure"]
impl crate::Readable for D3PCR3H_SPEC {}
#[doc = "`write(|w| ..)` method takes [`d3pcr3h::W`](W) writer structure"]
impl crate::Writable for D3PCR3H_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets D3PCR3H to value 0"]
impl crate::Resettable for D3PCR3H_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
