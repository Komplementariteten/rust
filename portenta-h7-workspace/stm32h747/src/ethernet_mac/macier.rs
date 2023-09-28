#[doc = "Register `MACIER` reader"]
pub type R = crate::R<MACIER_SPEC>;
#[doc = "Register `MACIER` writer"]
pub type W = crate::W<MACIER_SPEC>;
#[doc = "Field `PHYIE` reader - PHYIE"]
pub type PHYIE_R = crate::BitReader;
#[doc = "Field `PHYIE` writer - PHYIE"]
pub type PHYIE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PMTIE` reader - PMTIE"]
pub type PMTIE_R = crate::BitReader;
#[doc = "Field `PMTIE` writer - PMTIE"]
pub type PMTIE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `LPIIE` reader - LPIIE"]
pub type LPIIE_R = crate::BitReader;
#[doc = "Field `LPIIE` writer - LPIIE"]
pub type LPIIE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TSIE` reader - TSIE"]
pub type TSIE_R = crate::BitReader;
#[doc = "Field `TSIE` writer - TSIE"]
pub type TSIE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TXSTSIE` reader - TXSTSIE"]
pub type TXSTSIE_R = crate::BitReader;
#[doc = "Field `TXSTSIE` writer - TXSTSIE"]
pub type TXSTSIE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `RXSTSIE` reader - RXSTSIE"]
pub type RXSTSIE_R = crate::BitReader;
#[doc = "Field `RXSTSIE` writer - RXSTSIE"]
pub type RXSTSIE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 3 - PHYIE"]
    #[inline(always)]
    pub fn phyie(&self) -> PHYIE_R {
        PHYIE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - PMTIE"]
    #[inline(always)]
    pub fn pmtie(&self) -> PMTIE_R {
        PMTIE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - LPIIE"]
    #[inline(always)]
    pub fn lpiie(&self) -> LPIIE_R {
        LPIIE_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 12 - TSIE"]
    #[inline(always)]
    pub fn tsie(&self) -> TSIE_R {
        TSIE_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - TXSTSIE"]
    #[inline(always)]
    pub fn txstsie(&self) -> TXSTSIE_R {
        TXSTSIE_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - RXSTSIE"]
    #[inline(always)]
    pub fn rxstsie(&self) -> RXSTSIE_R {
        RXSTSIE_R::new(((self.bits >> 14) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 3 - PHYIE"]
    #[inline(always)]
    #[must_use]
    pub fn phyie(&mut self) -> PHYIE_W<MACIER_SPEC, 3> {
        PHYIE_W::new(self)
    }
    #[doc = "Bit 4 - PMTIE"]
    #[inline(always)]
    #[must_use]
    pub fn pmtie(&mut self) -> PMTIE_W<MACIER_SPEC, 4> {
        PMTIE_W::new(self)
    }
    #[doc = "Bit 5 - LPIIE"]
    #[inline(always)]
    #[must_use]
    pub fn lpiie(&mut self) -> LPIIE_W<MACIER_SPEC, 5> {
        LPIIE_W::new(self)
    }
    #[doc = "Bit 12 - TSIE"]
    #[inline(always)]
    #[must_use]
    pub fn tsie(&mut self) -> TSIE_W<MACIER_SPEC, 12> {
        TSIE_W::new(self)
    }
    #[doc = "Bit 13 - TXSTSIE"]
    #[inline(always)]
    #[must_use]
    pub fn txstsie(&mut self) -> TXSTSIE_W<MACIER_SPEC, 13> {
        TXSTSIE_W::new(self)
    }
    #[doc = "Bit 14 - RXSTSIE"]
    #[inline(always)]
    #[must_use]
    pub fn rxstsie(&mut self) -> RXSTSIE_W<MACIER_SPEC, 14> {
        RXSTSIE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Interrupt enable register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`macier::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`macier::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MACIER_SPEC;
impl crate::RegisterSpec for MACIER_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`macier::R`](R) reader structure"]
impl crate::Readable for MACIER_SPEC {}
#[doc = "`write(|w| ..)` method takes [`macier::W`](W) writer structure"]
impl crate::Writable for MACIER_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MACIER to value 0"]
impl crate::Resettable for MACIER_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
