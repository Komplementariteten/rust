#[doc = "Register `M5CR` reader"]
pub type R = crate::R<M5CR_SPEC>;
#[doc = "Register `M5CR` writer"]
pub type W = crate::W<M5CR_SPEC>;
#[doc = "Field `ECCSEIE` reader - ECC single error interrupt enable"]
pub type ECCSEIE_R = crate::BitReader;
#[doc = "Field `ECCSEIE` writer - ECC single error interrupt enable"]
pub type ECCSEIE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `ECCDEIE` reader - ECC double error interrupt enable"]
pub type ECCDEIE_R = crate::BitReader;
#[doc = "Field `ECCDEIE` writer - ECC double error interrupt enable"]
pub type ECCDEIE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `ECCDEBWIE` reader - ECC double error on byte write (BW) interrupt enable"]
pub type ECCDEBWIE_R = crate::BitReader;
#[doc = "Field `ECCDEBWIE` writer - ECC double error on byte write (BW) interrupt enable"]
pub type ECCDEBWIE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `ECCELEN` reader - ECC error latching enable"]
pub type ECCELEN_R = crate::BitReader;
#[doc = "Field `ECCELEN` writer - ECC error latching enable"]
pub type ECCELEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 2 - ECC single error interrupt enable"]
    #[inline(always)]
    pub fn eccseie(&self) -> ECCSEIE_R {
        ECCSEIE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - ECC double error interrupt enable"]
    #[inline(always)]
    pub fn eccdeie(&self) -> ECCDEIE_R {
        ECCDEIE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - ECC double error on byte write (BW) interrupt enable"]
    #[inline(always)]
    pub fn eccdebwie(&self) -> ECCDEBWIE_R {
        ECCDEBWIE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - ECC error latching enable"]
    #[inline(always)]
    pub fn eccelen(&self) -> ECCELEN_R {
        ECCELEN_R::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 2 - ECC single error interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn eccseie(&mut self) -> ECCSEIE_W<M5CR_SPEC, 2> {
        ECCSEIE_W::new(self)
    }
    #[doc = "Bit 3 - ECC double error interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn eccdeie(&mut self) -> ECCDEIE_W<M5CR_SPEC, 3> {
        ECCDEIE_W::new(self)
    }
    #[doc = "Bit 4 - ECC double error on byte write (BW) interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn eccdebwie(&mut self) -> ECCDEBWIE_W<M5CR_SPEC, 4> {
        ECCDEBWIE_W::new(self)
    }
    #[doc = "Bit 5 - ECC error latching enable"]
    #[inline(always)]
    #[must_use]
    pub fn eccelen(&mut self) -> ECCELEN_W<M5CR_SPEC, 5> {
        ECCELEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "RAMECC monitor x configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`m5cr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`m5cr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct M5CR_SPEC;
impl crate::RegisterSpec for M5CR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`m5cr::R`](R) reader structure"]
impl crate::Readable for M5CR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`m5cr::W`](W) writer structure"]
impl crate::Writable for M5CR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets M5CR to value 0"]
impl crate::Resettable for M5CR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
