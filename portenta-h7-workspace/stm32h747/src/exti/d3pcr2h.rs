#[doc = "Register `D3PCR2H` reader"]
pub type R = crate::R<D3PCR2H_SPEC>;
#[doc = "Register `D3PCR2H` writer"]
pub type W = crate::W<D3PCR2H_SPEC>;
#[doc = "Field `PCS48` reader - Pending request clear input signal selection on Event input x= truncate ((n+96)/2)"]
pub type PCS48_R = crate::FieldReader;
#[doc = "Field `PCS48` writer - Pending request clear input signal selection on Event input x= truncate ((n+96)/2)"]
pub type PCS48_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
#[doc = "Field `PCS49` reader - Pending request clear input signal selection on Event input x= truncate ((n+96)/2)"]
pub type PCS49_R = crate::FieldReader;
#[doc = "Field `PCS49` writer - Pending request clear input signal selection on Event input x= truncate ((n+96)/2)"]
pub type PCS49_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
#[doc = "Field `PCS50` reader - Pending request clear input signal selection on Event input x= truncate ((n+96)/2)"]
pub type PCS50_R = crate::FieldReader;
#[doc = "Field `PCS50` writer - Pending request clear input signal selection on Event input x= truncate ((n+96)/2)"]
pub type PCS50_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
#[doc = "Field `PCS51` reader - Pending request clear input signal selection on Event input x= truncate ((n+96)/2)"]
pub type PCS51_R = crate::FieldReader;
#[doc = "Field `PCS51` writer - Pending request clear input signal selection on Event input x= truncate ((n+96)/2)"]
pub type PCS51_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
#[doc = "Field `PCS52` reader - Pending request clear input signal selection on Event input x= truncate ((n+96)/2)"]
pub type PCS52_R = crate::FieldReader;
#[doc = "Field `PCS52` writer - Pending request clear input signal selection on Event input x= truncate ((n+96)/2)"]
pub type PCS52_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
#[doc = "Field `PCS53` reader - Pending request clear input signal selection on Event input x= truncate ((n+96)/2)"]
pub type PCS53_R = crate::FieldReader;
#[doc = "Field `PCS53` writer - Pending request clear input signal selection on Event input x= truncate ((n+96)/2)"]
pub type PCS53_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
impl R {
    #[doc = "Bits 0:1 - Pending request clear input signal selection on Event input x= truncate ((n+96)/2)"]
    #[inline(always)]
    pub fn pcs48(&self) -> PCS48_R {
        PCS48_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - Pending request clear input signal selection on Event input x= truncate ((n+96)/2)"]
    #[inline(always)]
    pub fn pcs49(&self) -> PCS49_R {
        PCS49_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - Pending request clear input signal selection on Event input x= truncate ((n+96)/2)"]
    #[inline(always)]
    pub fn pcs50(&self) -> PCS50_R {
        PCS50_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - Pending request clear input signal selection on Event input x= truncate ((n+96)/2)"]
    #[inline(always)]
    pub fn pcs51(&self) -> PCS51_R {
        PCS51_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:9 - Pending request clear input signal selection on Event input x= truncate ((n+96)/2)"]
    #[inline(always)]
    pub fn pcs52(&self) -> PCS52_R {
        PCS52_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - Pending request clear input signal selection on Event input x= truncate ((n+96)/2)"]
    #[inline(always)]
    pub fn pcs53(&self) -> PCS53_R {
        PCS53_R::new(((self.bits >> 10) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Pending request clear input signal selection on Event input x= truncate ((n+96)/2)"]
    #[inline(always)]
    #[must_use]
    pub fn pcs48(&mut self) -> PCS48_W<D3PCR2H_SPEC, 0> {
        PCS48_W::new(self)
    }
    #[doc = "Bits 2:3 - Pending request clear input signal selection on Event input x= truncate ((n+96)/2)"]
    #[inline(always)]
    #[must_use]
    pub fn pcs49(&mut self) -> PCS49_W<D3PCR2H_SPEC, 2> {
        PCS49_W::new(self)
    }
    #[doc = "Bits 4:5 - Pending request clear input signal selection on Event input x= truncate ((n+96)/2)"]
    #[inline(always)]
    #[must_use]
    pub fn pcs50(&mut self) -> PCS50_W<D3PCR2H_SPEC, 4> {
        PCS50_W::new(self)
    }
    #[doc = "Bits 6:7 - Pending request clear input signal selection on Event input x= truncate ((n+96)/2)"]
    #[inline(always)]
    #[must_use]
    pub fn pcs51(&mut self) -> PCS51_W<D3PCR2H_SPEC, 6> {
        PCS51_W::new(self)
    }
    #[doc = "Bits 8:9 - Pending request clear input signal selection on Event input x= truncate ((n+96)/2)"]
    #[inline(always)]
    #[must_use]
    pub fn pcs52(&mut self) -> PCS52_W<D3PCR2H_SPEC, 8> {
        PCS52_W::new(self)
    }
    #[doc = "Bits 10:11 - Pending request clear input signal selection on Event input x= truncate ((n+96)/2)"]
    #[inline(always)]
    #[must_use]
    pub fn pcs53(&mut self) -> PCS53_W<D3PCR2H_SPEC, 10> {
        PCS53_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "EXTI D3 pending clear selection register high\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`d3pcr2h::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`d3pcr2h::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct D3PCR2H_SPEC;
impl crate::RegisterSpec for D3PCR2H_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`d3pcr2h::R`](R) reader structure"]
impl crate::Readable for D3PCR2H_SPEC {}
#[doc = "`write(|w| ..)` method takes [`d3pcr2h::W`](W) writer structure"]
impl crate::Writable for D3PCR2H_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets D3PCR2H to value 0"]
impl crate::Resettable for D3PCR2H_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
