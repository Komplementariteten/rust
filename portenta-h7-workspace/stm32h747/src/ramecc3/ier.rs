#[doc = "Register `IER` reader"]
pub type R = crate::R<IER_SPEC>;
#[doc = "Register `IER` writer"]
pub type W = crate::W<IER_SPEC>;
#[doc = "Field `GIE` reader - Global interrupt enable"]
pub type GIE_R = crate::BitReader;
#[doc = "Field `GIE` writer - Global interrupt enable"]
pub type GIE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `GECCSEIE_` reader - Global ECC single error interrupt enable"]
pub type GECCSEIE__R = crate::BitReader;
#[doc = "Field `GECCSEIE_` writer - Global ECC single error interrupt enable"]
pub type GECCSEIE__W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `GECCDEIE` reader - Global ECC double error interrupt enable"]
pub type GECCDEIE_R = crate::BitReader;
#[doc = "Field `GECCDEIE` writer - Global ECC double error interrupt enable"]
pub type GECCDEIE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `GECCDEBWIE` reader - Global ECC double error on byte write (BW) interrupt enable"]
pub type GECCDEBWIE_R = crate::BitReader;
#[doc = "Field `GECCDEBWIE` writer - Global ECC double error on byte write (BW) interrupt enable"]
pub type GECCDEBWIE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - Global interrupt enable"]
    #[inline(always)]
    pub fn gie(&self) -> GIE_R {
        GIE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Global ECC single error interrupt enable"]
    #[inline(always)]
    pub fn geccseie_(&self) -> GECCSEIE__R {
        GECCSEIE__R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Global ECC double error interrupt enable"]
    #[inline(always)]
    pub fn geccdeie(&self) -> GECCDEIE_R {
        GECCDEIE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Global ECC double error on byte write (BW) interrupt enable"]
    #[inline(always)]
    pub fn geccdebwie(&self) -> GECCDEBWIE_R {
        GECCDEBWIE_R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Global interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn gie(&mut self) -> GIE_W<IER_SPEC, 0> {
        GIE_W::new(self)
    }
    #[doc = "Bit 1 - Global ECC single error interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn geccseie_(&mut self) -> GECCSEIE__W<IER_SPEC, 1> {
        GECCSEIE__W::new(self)
    }
    #[doc = "Bit 2 - Global ECC double error interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn geccdeie(&mut self) -> GECCDEIE_W<IER_SPEC, 2> {
        GECCDEIE_W::new(self)
    }
    #[doc = "Bit 3 - Global ECC double error on byte write (BW) interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn geccdebwie(&mut self) -> GECCDEBWIE_W<IER_SPEC, 3> {
        GECCDEBWIE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "RAMECC interrupt enable register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ier::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ier::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IER_SPEC;
impl crate::RegisterSpec for IER_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ier::R`](R) reader structure"]
impl crate::Readable for IER_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ier::W`](W) writer structure"]
impl crate::Writable for IER_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets IER to value 0"]
impl crate::Resettable for IER_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
