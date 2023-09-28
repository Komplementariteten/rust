#[doc = "Register `ACR_` reader"]
pub type R = crate::R<ACR__SPEC>;
#[doc = "Register `ACR_` writer"]
pub type W = crate::W<ACR__SPEC>;
#[doc = "Field `LATENCY` reader - Read latency"]
pub type LATENCY_R = crate::FieldReader;
#[doc = "Field `LATENCY` writer - Read latency"]
pub type LATENCY_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 3, O>;
#[doc = "Field `WRHIGHFREQ` reader - Flash signal delay"]
pub type WRHIGHFREQ_R = crate::FieldReader;
#[doc = "Field `WRHIGHFREQ` writer - Flash signal delay"]
pub type WRHIGHFREQ_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
impl R {
    #[doc = "Bits 0:2 - Read latency"]
    #[inline(always)]
    pub fn latency(&self) -> LATENCY_R {
        LATENCY_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 4:5 - Flash signal delay"]
    #[inline(always)]
    pub fn wrhighfreq(&self) -> WRHIGHFREQ_R {
        WRHIGHFREQ_R::new(((self.bits >> 4) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Read latency"]
    #[inline(always)]
    #[must_use]
    pub fn latency(&mut self) -> LATENCY_W<ACR__SPEC, 0> {
        LATENCY_W::new(self)
    }
    #[doc = "Bits 4:5 - Flash signal delay"]
    #[inline(always)]
    #[must_use]
    pub fn wrhighfreq(&mut self) -> WRHIGHFREQ_W<ACR__SPEC, 4> {
        WRHIGHFREQ_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Access control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`acr_::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`acr_::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ACR__SPEC;
impl crate::RegisterSpec for ACR__SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`acr_::R`](R) reader structure"]
impl crate::Readable for ACR__SPEC {}
#[doc = "`write(|w| ..)` method takes [`acr_::W`](W) writer structure"]
impl crate::Writable for ACR__SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ACR_ to value 0"]
impl crate::Resettable for ACR__SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
