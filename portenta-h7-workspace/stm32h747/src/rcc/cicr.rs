#[doc = "Register `CICR` reader"]
pub type R = crate::R<CICR_SPEC>;
#[doc = "Register `CICR` writer"]
pub type W = crate::W<CICR_SPEC>;
#[doc = "Field `LSIRDYC` reader - LSI ready Interrupt Clear"]
pub type LSIRDYC_R = crate::BitReader;
#[doc = "Field `LSIRDYC` writer - LSI ready Interrupt Clear"]
pub type LSIRDYC_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `LSERDYC` reader - LSE ready Interrupt Clear"]
pub type LSERDYC_R = crate::BitReader;
#[doc = "Field `LSERDYC` writer - LSE ready Interrupt Clear"]
pub type LSERDYC_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `HSIRDYC` reader - HSI ready Interrupt Clear"]
pub type HSIRDYC_R = crate::BitReader;
#[doc = "Field `HSIRDYC` writer - HSI ready Interrupt Clear"]
pub type HSIRDYC_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `HSERDYC` reader - HSE ready Interrupt Clear"]
pub type HSERDYC_R = crate::BitReader;
#[doc = "Field `HSERDYC` writer - HSE ready Interrupt Clear"]
pub type HSERDYC_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `HSE_ready_Interrupt_Clear` reader - CSI ready Interrupt Clear"]
pub type HSE_READY_INTERRUPT_CLEAR_R = crate::BitReader;
#[doc = "Field `HSE_ready_Interrupt_Clear` writer - CSI ready Interrupt Clear"]
pub type HSE_READY_INTERRUPT_CLEAR_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `RC48RDYC` reader - RC48 ready Interrupt Clear"]
pub type RC48RDYC_R = crate::BitReader;
#[doc = "Field `RC48RDYC` writer - RC48 ready Interrupt Clear"]
pub type RC48RDYC_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PLL1RDYC` reader - PLL1 ready Interrupt Clear"]
pub type PLL1RDYC_R = crate::BitReader;
#[doc = "Field `PLL1RDYC` writer - PLL1 ready Interrupt Clear"]
pub type PLL1RDYC_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PLL2RDYC` reader - PLL2 ready Interrupt Clear"]
pub type PLL2RDYC_R = crate::BitReader;
#[doc = "Field `PLL2RDYC` writer - PLL2 ready Interrupt Clear"]
pub type PLL2RDYC_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PLL3RDYC` reader - PLL3 ready Interrupt Clear"]
pub type PLL3RDYC_R = crate::BitReader;
#[doc = "Field `PLL3RDYC` writer - PLL3 ready Interrupt Clear"]
pub type PLL3RDYC_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `LSECSSC` reader - LSE clock security system Interrupt Clear"]
pub type LSECSSC_R = crate::BitReader;
#[doc = "Field `LSECSSC` writer - LSE clock security system Interrupt Clear"]
pub type LSECSSC_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `HSECSSC` reader - HSE clock security system Interrupt Clear"]
pub type HSECSSC_R = crate::BitReader;
#[doc = "Field `HSECSSC` writer - HSE clock security system Interrupt Clear"]
pub type HSECSSC_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - LSI ready Interrupt Clear"]
    #[inline(always)]
    pub fn lsirdyc(&self) -> LSIRDYC_R {
        LSIRDYC_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - LSE ready Interrupt Clear"]
    #[inline(always)]
    pub fn lserdyc(&self) -> LSERDYC_R {
        LSERDYC_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - HSI ready Interrupt Clear"]
    #[inline(always)]
    pub fn hsirdyc(&self) -> HSIRDYC_R {
        HSIRDYC_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - HSE ready Interrupt Clear"]
    #[inline(always)]
    pub fn hserdyc(&self) -> HSERDYC_R {
        HSERDYC_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - CSI ready Interrupt Clear"]
    #[inline(always)]
    pub fn hse_ready_interrupt_clear(&self) -> HSE_READY_INTERRUPT_CLEAR_R {
        HSE_READY_INTERRUPT_CLEAR_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - RC48 ready Interrupt Clear"]
    #[inline(always)]
    pub fn rc48rdyc(&self) -> RC48RDYC_R {
        RC48RDYC_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - PLL1 ready Interrupt Clear"]
    #[inline(always)]
    pub fn pll1rdyc(&self) -> PLL1RDYC_R {
        PLL1RDYC_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - PLL2 ready Interrupt Clear"]
    #[inline(always)]
    pub fn pll2rdyc(&self) -> PLL2RDYC_R {
        PLL2RDYC_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - PLL3 ready Interrupt Clear"]
    #[inline(always)]
    pub fn pll3rdyc(&self) -> PLL3RDYC_R {
        PLL3RDYC_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - LSE clock security system Interrupt Clear"]
    #[inline(always)]
    pub fn lsecssc(&self) -> LSECSSC_R {
        LSECSSC_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - HSE clock security system Interrupt Clear"]
    #[inline(always)]
    pub fn hsecssc(&self) -> HSECSSC_R {
        HSECSSC_R::new(((self.bits >> 10) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - LSI ready Interrupt Clear"]
    #[inline(always)]
    #[must_use]
    pub fn lsirdyc(&mut self) -> LSIRDYC_W<CICR_SPEC, 0> {
        LSIRDYC_W::new(self)
    }
    #[doc = "Bit 1 - LSE ready Interrupt Clear"]
    #[inline(always)]
    #[must_use]
    pub fn lserdyc(&mut self) -> LSERDYC_W<CICR_SPEC, 1> {
        LSERDYC_W::new(self)
    }
    #[doc = "Bit 2 - HSI ready Interrupt Clear"]
    #[inline(always)]
    #[must_use]
    pub fn hsirdyc(&mut self) -> HSIRDYC_W<CICR_SPEC, 2> {
        HSIRDYC_W::new(self)
    }
    #[doc = "Bit 3 - HSE ready Interrupt Clear"]
    #[inline(always)]
    #[must_use]
    pub fn hserdyc(&mut self) -> HSERDYC_W<CICR_SPEC, 3> {
        HSERDYC_W::new(self)
    }
    #[doc = "Bit 4 - CSI ready Interrupt Clear"]
    #[inline(always)]
    #[must_use]
    pub fn hse_ready_interrupt_clear(&mut self) -> HSE_READY_INTERRUPT_CLEAR_W<CICR_SPEC, 4> {
        HSE_READY_INTERRUPT_CLEAR_W::new(self)
    }
    #[doc = "Bit 5 - RC48 ready Interrupt Clear"]
    #[inline(always)]
    #[must_use]
    pub fn rc48rdyc(&mut self) -> RC48RDYC_W<CICR_SPEC, 5> {
        RC48RDYC_W::new(self)
    }
    #[doc = "Bit 6 - PLL1 ready Interrupt Clear"]
    #[inline(always)]
    #[must_use]
    pub fn pll1rdyc(&mut self) -> PLL1RDYC_W<CICR_SPEC, 6> {
        PLL1RDYC_W::new(self)
    }
    #[doc = "Bit 7 - PLL2 ready Interrupt Clear"]
    #[inline(always)]
    #[must_use]
    pub fn pll2rdyc(&mut self) -> PLL2RDYC_W<CICR_SPEC, 7> {
        PLL2RDYC_W::new(self)
    }
    #[doc = "Bit 8 - PLL3 ready Interrupt Clear"]
    #[inline(always)]
    #[must_use]
    pub fn pll3rdyc(&mut self) -> PLL3RDYC_W<CICR_SPEC, 8> {
        PLL3RDYC_W::new(self)
    }
    #[doc = "Bit 9 - LSE clock security system Interrupt Clear"]
    #[inline(always)]
    #[must_use]
    pub fn lsecssc(&mut self) -> LSECSSC_W<CICR_SPEC, 9> {
        LSECSSC_W::new(self)
    }
    #[doc = "Bit 10 - HSE clock security system Interrupt Clear"]
    #[inline(always)]
    #[must_use]
    pub fn hsecssc(&mut self) -> HSECSSC_W<CICR_SPEC, 10> {
        HSECSSC_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "RCC Clock Source Interrupt Clear Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cicr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cicr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CICR_SPEC;
impl crate::RegisterSpec for CICR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cicr::R`](R) reader structure"]
impl crate::Readable for CICR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`cicr::W`](W) writer structure"]
impl crate::Writable for CICR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CICR to value 0"]
impl crate::Resettable for CICR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
