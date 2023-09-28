#[doc = "Register `CIFR` reader"]
pub type R = crate::R<CIFR_SPEC>;
#[doc = "Register `CIFR` writer"]
pub type W = crate::W<CIFR_SPEC>;
#[doc = "Field `LSIRDYF` reader - LSI ready Interrupt Flag"]
pub type LSIRDYF_R = crate::BitReader;
#[doc = "Field `LSIRDYF` writer - LSI ready Interrupt Flag"]
pub type LSIRDYF_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `LSERDYF` reader - LSE ready Interrupt Flag"]
pub type LSERDYF_R = crate::BitReader;
#[doc = "Field `LSERDYF` writer - LSE ready Interrupt Flag"]
pub type LSERDYF_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `HSIRDYF` reader - HSI ready Interrupt Flag"]
pub type HSIRDYF_R = crate::BitReader;
#[doc = "Field `HSIRDYF` writer - HSI ready Interrupt Flag"]
pub type HSIRDYF_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `HSERDYF` reader - HSE ready Interrupt Flag"]
pub type HSERDYF_R = crate::BitReader;
#[doc = "Field `HSERDYF` writer - HSE ready Interrupt Flag"]
pub type HSERDYF_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CSIRDY` reader - CSI ready Interrupt Flag"]
pub type CSIRDY_R = crate::BitReader;
#[doc = "Field `CSIRDY` writer - CSI ready Interrupt Flag"]
pub type CSIRDY_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `RC48RDYF` reader - RC48 ready Interrupt Flag"]
pub type RC48RDYF_R = crate::BitReader;
#[doc = "Field `RC48RDYF` writer - RC48 ready Interrupt Flag"]
pub type RC48RDYF_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PLL1RDYF` reader - PLL1 ready Interrupt Flag"]
pub type PLL1RDYF_R = crate::BitReader;
#[doc = "Field `PLL1RDYF` writer - PLL1 ready Interrupt Flag"]
pub type PLL1RDYF_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PLL2RDYF` reader - PLL2 ready Interrupt Flag"]
pub type PLL2RDYF_R = crate::BitReader;
#[doc = "Field `PLL2RDYF` writer - PLL2 ready Interrupt Flag"]
pub type PLL2RDYF_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PLL3RDYF` reader - PLL3 ready Interrupt Flag"]
pub type PLL3RDYF_R = crate::BitReader;
#[doc = "Field `PLL3RDYF` writer - PLL3 ready Interrupt Flag"]
pub type PLL3RDYF_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `LSECSSF` reader - LSE clock security system Interrupt Flag"]
pub type LSECSSF_R = crate::BitReader;
#[doc = "Field `LSECSSF` writer - LSE clock security system Interrupt Flag"]
pub type LSECSSF_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `HSECSSF` reader - HSE clock security system Interrupt Flag"]
pub type HSECSSF_R = crate::BitReader;
#[doc = "Field `HSECSSF` writer - HSE clock security system Interrupt Flag"]
pub type HSECSSF_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - LSI ready Interrupt Flag"]
    #[inline(always)]
    pub fn lsirdyf(&self) -> LSIRDYF_R {
        LSIRDYF_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - LSE ready Interrupt Flag"]
    #[inline(always)]
    pub fn lserdyf(&self) -> LSERDYF_R {
        LSERDYF_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - HSI ready Interrupt Flag"]
    #[inline(always)]
    pub fn hsirdyf(&self) -> HSIRDYF_R {
        HSIRDYF_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - HSE ready Interrupt Flag"]
    #[inline(always)]
    pub fn hserdyf(&self) -> HSERDYF_R {
        HSERDYF_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - CSI ready Interrupt Flag"]
    #[inline(always)]
    pub fn csirdy(&self) -> CSIRDY_R {
        CSIRDY_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - RC48 ready Interrupt Flag"]
    #[inline(always)]
    pub fn rc48rdyf(&self) -> RC48RDYF_R {
        RC48RDYF_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - PLL1 ready Interrupt Flag"]
    #[inline(always)]
    pub fn pll1rdyf(&self) -> PLL1RDYF_R {
        PLL1RDYF_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - PLL2 ready Interrupt Flag"]
    #[inline(always)]
    pub fn pll2rdyf(&self) -> PLL2RDYF_R {
        PLL2RDYF_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - PLL3 ready Interrupt Flag"]
    #[inline(always)]
    pub fn pll3rdyf(&self) -> PLL3RDYF_R {
        PLL3RDYF_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - LSE clock security system Interrupt Flag"]
    #[inline(always)]
    pub fn lsecssf(&self) -> LSECSSF_R {
        LSECSSF_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - HSE clock security system Interrupt Flag"]
    #[inline(always)]
    pub fn hsecssf(&self) -> HSECSSF_R {
        HSECSSF_R::new(((self.bits >> 10) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - LSI ready Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn lsirdyf(&mut self) -> LSIRDYF_W<CIFR_SPEC, 0> {
        LSIRDYF_W::new(self)
    }
    #[doc = "Bit 1 - LSE ready Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn lserdyf(&mut self) -> LSERDYF_W<CIFR_SPEC, 1> {
        LSERDYF_W::new(self)
    }
    #[doc = "Bit 2 - HSI ready Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn hsirdyf(&mut self) -> HSIRDYF_W<CIFR_SPEC, 2> {
        HSIRDYF_W::new(self)
    }
    #[doc = "Bit 3 - HSE ready Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn hserdyf(&mut self) -> HSERDYF_W<CIFR_SPEC, 3> {
        HSERDYF_W::new(self)
    }
    #[doc = "Bit 4 - CSI ready Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn csirdy(&mut self) -> CSIRDY_W<CIFR_SPEC, 4> {
        CSIRDY_W::new(self)
    }
    #[doc = "Bit 5 - RC48 ready Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn rc48rdyf(&mut self) -> RC48RDYF_W<CIFR_SPEC, 5> {
        RC48RDYF_W::new(self)
    }
    #[doc = "Bit 6 - PLL1 ready Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn pll1rdyf(&mut self) -> PLL1RDYF_W<CIFR_SPEC, 6> {
        PLL1RDYF_W::new(self)
    }
    #[doc = "Bit 7 - PLL2 ready Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn pll2rdyf(&mut self) -> PLL2RDYF_W<CIFR_SPEC, 7> {
        PLL2RDYF_W::new(self)
    }
    #[doc = "Bit 8 - PLL3 ready Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn pll3rdyf(&mut self) -> PLL3RDYF_W<CIFR_SPEC, 8> {
        PLL3RDYF_W::new(self)
    }
    #[doc = "Bit 9 - LSE clock security system Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn lsecssf(&mut self) -> LSECSSF_W<CIFR_SPEC, 9> {
        LSECSSF_W::new(self)
    }
    #[doc = "Bit 10 - HSE clock security system Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn hsecssf(&mut self) -> HSECSSF_W<CIFR_SPEC, 10> {
        HSECSSF_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "RCC Clock Source Interrupt Flag Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cifr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cifr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CIFR_SPEC;
impl crate::RegisterSpec for CIFR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cifr::R`](R) reader structure"]
impl crate::Readable for CIFR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`cifr::W`](W) writer structure"]
impl crate::Writable for CIFR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CIFR to value 0"]
impl crate::Resettable for CIFR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
