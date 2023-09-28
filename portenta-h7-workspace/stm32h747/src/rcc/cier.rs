#[doc = "Register `CIER` reader"]
pub type R = crate::R<CIER_SPEC>;
#[doc = "Register `CIER` writer"]
pub type W = crate::W<CIER_SPEC>;
#[doc = "Field `LSIRDYIE` reader - LSI ready Interrupt Enable"]
pub type LSIRDYIE_R = crate::BitReader;
#[doc = "Field `LSIRDYIE` writer - LSI ready Interrupt Enable"]
pub type LSIRDYIE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `LSERDYIE` reader - LSE ready Interrupt Enable"]
pub type LSERDYIE_R = crate::BitReader;
#[doc = "Field `LSERDYIE` writer - LSE ready Interrupt Enable"]
pub type LSERDYIE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `HSIRDYIE` reader - HSI ready Interrupt Enable"]
pub type HSIRDYIE_R = crate::BitReader;
#[doc = "Field `HSIRDYIE` writer - HSI ready Interrupt Enable"]
pub type HSIRDYIE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `HSERDYIE` reader - HSE ready Interrupt Enable"]
pub type HSERDYIE_R = crate::BitReader;
#[doc = "Field `HSERDYIE` writer - HSE ready Interrupt Enable"]
pub type HSERDYIE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CSIRDYIE` reader - CSI ready Interrupt Enable"]
pub type CSIRDYIE_R = crate::BitReader;
#[doc = "Field `CSIRDYIE` writer - CSI ready Interrupt Enable"]
pub type CSIRDYIE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `RC48RDYIE` reader - RC48 ready Interrupt Enable"]
pub type RC48RDYIE_R = crate::BitReader;
#[doc = "Field `RC48RDYIE` writer - RC48 ready Interrupt Enable"]
pub type RC48RDYIE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PLL1RDYIE` reader - PLL1 ready Interrupt Enable"]
pub type PLL1RDYIE_R = crate::BitReader;
#[doc = "Field `PLL1RDYIE` writer - PLL1 ready Interrupt Enable"]
pub type PLL1RDYIE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PLL2RDYIE` reader - PLL2 ready Interrupt Enable"]
pub type PLL2RDYIE_R = crate::BitReader;
#[doc = "Field `PLL2RDYIE` writer - PLL2 ready Interrupt Enable"]
pub type PLL2RDYIE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PLL3RDYIE` reader - PLL3 ready Interrupt Enable"]
pub type PLL3RDYIE_R = crate::BitReader;
#[doc = "Field `PLL3RDYIE` writer - PLL3 ready Interrupt Enable"]
pub type PLL3RDYIE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `LSECSSIE` reader - LSE clock security system Interrupt Enable"]
pub type LSECSSIE_R = crate::BitReader;
#[doc = "Field `LSECSSIE` writer - LSE clock security system Interrupt Enable"]
pub type LSECSSIE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - LSI ready Interrupt Enable"]
    #[inline(always)]
    pub fn lsirdyie(&self) -> LSIRDYIE_R {
        LSIRDYIE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - LSE ready Interrupt Enable"]
    #[inline(always)]
    pub fn lserdyie(&self) -> LSERDYIE_R {
        LSERDYIE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - HSI ready Interrupt Enable"]
    #[inline(always)]
    pub fn hsirdyie(&self) -> HSIRDYIE_R {
        HSIRDYIE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - HSE ready Interrupt Enable"]
    #[inline(always)]
    pub fn hserdyie(&self) -> HSERDYIE_R {
        HSERDYIE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - CSI ready Interrupt Enable"]
    #[inline(always)]
    pub fn csirdyie(&self) -> CSIRDYIE_R {
        CSIRDYIE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - RC48 ready Interrupt Enable"]
    #[inline(always)]
    pub fn rc48rdyie(&self) -> RC48RDYIE_R {
        RC48RDYIE_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - PLL1 ready Interrupt Enable"]
    #[inline(always)]
    pub fn pll1rdyie(&self) -> PLL1RDYIE_R {
        PLL1RDYIE_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - PLL2 ready Interrupt Enable"]
    #[inline(always)]
    pub fn pll2rdyie(&self) -> PLL2RDYIE_R {
        PLL2RDYIE_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - PLL3 ready Interrupt Enable"]
    #[inline(always)]
    pub fn pll3rdyie(&self) -> PLL3RDYIE_R {
        PLL3RDYIE_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - LSE clock security system Interrupt Enable"]
    #[inline(always)]
    pub fn lsecssie(&self) -> LSECSSIE_R {
        LSECSSIE_R::new(((self.bits >> 9) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - LSI ready Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn lsirdyie(&mut self) -> LSIRDYIE_W<CIER_SPEC, 0> {
        LSIRDYIE_W::new(self)
    }
    #[doc = "Bit 1 - LSE ready Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn lserdyie(&mut self) -> LSERDYIE_W<CIER_SPEC, 1> {
        LSERDYIE_W::new(self)
    }
    #[doc = "Bit 2 - HSI ready Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn hsirdyie(&mut self) -> HSIRDYIE_W<CIER_SPEC, 2> {
        HSIRDYIE_W::new(self)
    }
    #[doc = "Bit 3 - HSE ready Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn hserdyie(&mut self) -> HSERDYIE_W<CIER_SPEC, 3> {
        HSERDYIE_W::new(self)
    }
    #[doc = "Bit 4 - CSI ready Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn csirdyie(&mut self) -> CSIRDYIE_W<CIER_SPEC, 4> {
        CSIRDYIE_W::new(self)
    }
    #[doc = "Bit 5 - RC48 ready Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn rc48rdyie(&mut self) -> RC48RDYIE_W<CIER_SPEC, 5> {
        RC48RDYIE_W::new(self)
    }
    #[doc = "Bit 6 - PLL1 ready Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn pll1rdyie(&mut self) -> PLL1RDYIE_W<CIER_SPEC, 6> {
        PLL1RDYIE_W::new(self)
    }
    #[doc = "Bit 7 - PLL2 ready Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn pll2rdyie(&mut self) -> PLL2RDYIE_W<CIER_SPEC, 7> {
        PLL2RDYIE_W::new(self)
    }
    #[doc = "Bit 8 - PLL3 ready Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn pll3rdyie(&mut self) -> PLL3RDYIE_W<CIER_SPEC, 8> {
        PLL3RDYIE_W::new(self)
    }
    #[doc = "Bit 9 - LSE clock security system Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn lsecssie(&mut self) -> LSECSSIE_W<CIER_SPEC, 9> {
        LSECSSIE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "RCC Clock Source Interrupt Enable Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cier::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cier::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CIER_SPEC;
impl crate::RegisterSpec for CIER_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cier::R`](R) reader structure"]
impl crate::Readable for CIER_SPEC {}
#[doc = "`write(|w| ..)` method takes [`cier::W`](W) writer structure"]
impl crate::Writable for CIER_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CIER to value 0"]
impl crate::Resettable for CIER_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
