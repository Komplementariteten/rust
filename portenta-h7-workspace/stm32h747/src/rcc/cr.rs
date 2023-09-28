#[doc = "Register `CR` reader"]
pub type R = crate::R<CR_SPEC>;
#[doc = "Register `CR` writer"]
pub type W = crate::W<CR_SPEC>;
#[doc = "Field `HSION` reader - Internal high-speed clock enable"]
pub type HSION_R = crate::BitReader;
#[doc = "Field `HSION` writer - Internal high-speed clock enable"]
pub type HSION_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `HSIKERON` reader - High Speed Internal clock enable in Stop mode"]
pub type HSIKERON_R = crate::BitReader;
#[doc = "Field `HSIKERON` writer - High Speed Internal clock enable in Stop mode"]
pub type HSIKERON_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `HSIRDY` reader - HSI clock ready flag"]
pub type HSIRDY_R = crate::BitReader;
#[doc = "Field `HSIRDY` writer - HSI clock ready flag"]
pub type HSIRDY_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `HSIDIV` reader - HSI clock divider"]
pub type HSIDIV_R = crate::FieldReader;
#[doc = "Field `HSIDIV` writer - HSI clock divider"]
pub type HSIDIV_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
#[doc = "Field `HSIDIVF` reader - HSI divider flag"]
pub type HSIDIVF_R = crate::BitReader;
#[doc = "Field `HSIDIVF` writer - HSI divider flag"]
pub type HSIDIVF_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CSION` reader - CSI clock enable"]
pub type CSION_R = crate::BitReader;
#[doc = "Field `CSION` writer - CSI clock enable"]
pub type CSION_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CSIRDY` reader - CSI clock ready flag"]
pub type CSIRDY_R = crate::BitReader;
#[doc = "Field `CSIRDY` writer - CSI clock ready flag"]
pub type CSIRDY_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CSIKERON` reader - CSI clock enable in Stop mode"]
pub type CSIKERON_R = crate::BitReader;
#[doc = "Field `CSIKERON` writer - CSI clock enable in Stop mode"]
pub type CSIKERON_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `RC48ON` reader - RC48 clock enable"]
pub type RC48ON_R = crate::BitReader;
#[doc = "Field `RC48ON` writer - RC48 clock enable"]
pub type RC48ON_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `RC48RDY` reader - RC48 clock ready flag"]
pub type RC48RDY_R = crate::BitReader;
#[doc = "Field `RC48RDY` writer - RC48 clock ready flag"]
pub type RC48RDY_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `D1CKRDY` reader - D1 domain clocks ready flag"]
pub type D1CKRDY_R = crate::BitReader;
#[doc = "Field `D1CKRDY` writer - D1 domain clocks ready flag"]
pub type D1CKRDY_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `D2CKRDY` reader - D2 domain clocks ready flag"]
pub type D2CKRDY_R = crate::BitReader;
#[doc = "Field `D2CKRDY` writer - D2 domain clocks ready flag"]
pub type D2CKRDY_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `HSEON` reader - HSE clock enable"]
pub type HSEON_R = crate::BitReader;
#[doc = "Field `HSEON` writer - HSE clock enable"]
pub type HSEON_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `HSERDY` reader - HSE clock ready flag"]
pub type HSERDY_R = crate::BitReader;
#[doc = "Field `HSERDY` writer - HSE clock ready flag"]
pub type HSERDY_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `HSEBYP` reader - HSE clock bypass"]
pub type HSEBYP_R = crate::BitReader;
#[doc = "Field `HSEBYP` writer - HSE clock bypass"]
pub type HSEBYP_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `HSECSSON` reader - HSE Clock Security System enable"]
pub type HSECSSON_R = crate::BitReader;
#[doc = "Field `HSECSSON` writer - HSE Clock Security System enable"]
pub type HSECSSON_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PLL1ON` reader - PLL1 enable"]
pub type PLL1ON_R = crate::BitReader;
#[doc = "Field `PLL1ON` writer - PLL1 enable"]
pub type PLL1ON_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PLL1RDY` reader - PLL1 clock ready flag"]
pub type PLL1RDY_R = crate::BitReader;
#[doc = "Field `PLL1RDY` writer - PLL1 clock ready flag"]
pub type PLL1RDY_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PLL2ON` reader - PLL2 enable"]
pub type PLL2ON_R = crate::BitReader;
#[doc = "Field `PLL2ON` writer - PLL2 enable"]
pub type PLL2ON_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PLL2RDY` reader - PLL2 clock ready flag"]
pub type PLL2RDY_R = crate::BitReader;
#[doc = "Field `PLL2RDY` writer - PLL2 clock ready flag"]
pub type PLL2RDY_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PLL3ON` reader - PLL3 enable"]
pub type PLL3ON_R = crate::BitReader;
#[doc = "Field `PLL3ON` writer - PLL3 enable"]
pub type PLL3ON_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PLL3RDY` reader - PLL3 clock ready flag"]
pub type PLL3RDY_R = crate::BitReader;
#[doc = "Field `PLL3RDY` writer - PLL3 clock ready flag"]
pub type PLL3RDY_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - Internal high-speed clock enable"]
    #[inline(always)]
    pub fn hsion(&self) -> HSION_R {
        HSION_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - High Speed Internal clock enable in Stop mode"]
    #[inline(always)]
    pub fn hsikeron(&self) -> HSIKERON_R {
        HSIKERON_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - HSI clock ready flag"]
    #[inline(always)]
    pub fn hsirdy(&self) -> HSIRDY_R {
        HSIRDY_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 3:4 - HSI clock divider"]
    #[inline(always)]
    pub fn hsidiv(&self) -> HSIDIV_R {
        HSIDIV_R::new(((self.bits >> 3) & 3) as u8)
    }
    #[doc = "Bit 5 - HSI divider flag"]
    #[inline(always)]
    pub fn hsidivf(&self) -> HSIDIVF_R {
        HSIDIVF_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 7 - CSI clock enable"]
    #[inline(always)]
    pub fn csion(&self) -> CSION_R {
        CSION_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - CSI clock ready flag"]
    #[inline(always)]
    pub fn csirdy(&self) -> CSIRDY_R {
        CSIRDY_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - CSI clock enable in Stop mode"]
    #[inline(always)]
    pub fn csikeron(&self) -> CSIKERON_R {
        CSIKERON_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 12 - RC48 clock enable"]
    #[inline(always)]
    pub fn rc48on(&self) -> RC48ON_R {
        RC48ON_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - RC48 clock ready flag"]
    #[inline(always)]
    pub fn rc48rdy(&self) -> RC48RDY_R {
        RC48RDY_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - D1 domain clocks ready flag"]
    #[inline(always)]
    pub fn d1ckrdy(&self) -> D1CKRDY_R {
        D1CKRDY_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - D2 domain clocks ready flag"]
    #[inline(always)]
    pub fn d2ckrdy(&self) -> D2CKRDY_R {
        D2CKRDY_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - HSE clock enable"]
    #[inline(always)]
    pub fn hseon(&self) -> HSEON_R {
        HSEON_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - HSE clock ready flag"]
    #[inline(always)]
    pub fn hserdy(&self) -> HSERDY_R {
        HSERDY_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - HSE clock bypass"]
    #[inline(always)]
    pub fn hsebyp(&self) -> HSEBYP_R {
        HSEBYP_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - HSE Clock Security System enable"]
    #[inline(always)]
    pub fn hsecsson(&self) -> HSECSSON_R {
        HSECSSON_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 24 - PLL1 enable"]
    #[inline(always)]
    pub fn pll1on(&self) -> PLL1ON_R {
        PLL1ON_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - PLL1 clock ready flag"]
    #[inline(always)]
    pub fn pll1rdy(&self) -> PLL1RDY_R {
        PLL1RDY_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - PLL2 enable"]
    #[inline(always)]
    pub fn pll2on(&self) -> PLL2ON_R {
        PLL2ON_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - PLL2 clock ready flag"]
    #[inline(always)]
    pub fn pll2rdy(&self) -> PLL2RDY_R {
        PLL2RDY_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - PLL3 enable"]
    #[inline(always)]
    pub fn pll3on(&self) -> PLL3ON_R {
        PLL3ON_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - PLL3 clock ready flag"]
    #[inline(always)]
    pub fn pll3rdy(&self) -> PLL3RDY_R {
        PLL3RDY_R::new(((self.bits >> 29) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Internal high-speed clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn hsion(&mut self) -> HSION_W<CR_SPEC, 0> {
        HSION_W::new(self)
    }
    #[doc = "Bit 1 - High Speed Internal clock enable in Stop mode"]
    #[inline(always)]
    #[must_use]
    pub fn hsikeron(&mut self) -> HSIKERON_W<CR_SPEC, 1> {
        HSIKERON_W::new(self)
    }
    #[doc = "Bit 2 - HSI clock ready flag"]
    #[inline(always)]
    #[must_use]
    pub fn hsirdy(&mut self) -> HSIRDY_W<CR_SPEC, 2> {
        HSIRDY_W::new(self)
    }
    #[doc = "Bits 3:4 - HSI clock divider"]
    #[inline(always)]
    #[must_use]
    pub fn hsidiv(&mut self) -> HSIDIV_W<CR_SPEC, 3> {
        HSIDIV_W::new(self)
    }
    #[doc = "Bit 5 - HSI divider flag"]
    #[inline(always)]
    #[must_use]
    pub fn hsidivf(&mut self) -> HSIDIVF_W<CR_SPEC, 5> {
        HSIDIVF_W::new(self)
    }
    #[doc = "Bit 7 - CSI clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn csion(&mut self) -> CSION_W<CR_SPEC, 7> {
        CSION_W::new(self)
    }
    #[doc = "Bit 8 - CSI clock ready flag"]
    #[inline(always)]
    #[must_use]
    pub fn csirdy(&mut self) -> CSIRDY_W<CR_SPEC, 8> {
        CSIRDY_W::new(self)
    }
    #[doc = "Bit 9 - CSI clock enable in Stop mode"]
    #[inline(always)]
    #[must_use]
    pub fn csikeron(&mut self) -> CSIKERON_W<CR_SPEC, 9> {
        CSIKERON_W::new(self)
    }
    #[doc = "Bit 12 - RC48 clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn rc48on(&mut self) -> RC48ON_W<CR_SPEC, 12> {
        RC48ON_W::new(self)
    }
    #[doc = "Bit 13 - RC48 clock ready flag"]
    #[inline(always)]
    #[must_use]
    pub fn rc48rdy(&mut self) -> RC48RDY_W<CR_SPEC, 13> {
        RC48RDY_W::new(self)
    }
    #[doc = "Bit 14 - D1 domain clocks ready flag"]
    #[inline(always)]
    #[must_use]
    pub fn d1ckrdy(&mut self) -> D1CKRDY_W<CR_SPEC, 14> {
        D1CKRDY_W::new(self)
    }
    #[doc = "Bit 15 - D2 domain clocks ready flag"]
    #[inline(always)]
    #[must_use]
    pub fn d2ckrdy(&mut self) -> D2CKRDY_W<CR_SPEC, 15> {
        D2CKRDY_W::new(self)
    }
    #[doc = "Bit 16 - HSE clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn hseon(&mut self) -> HSEON_W<CR_SPEC, 16> {
        HSEON_W::new(self)
    }
    #[doc = "Bit 17 - HSE clock ready flag"]
    #[inline(always)]
    #[must_use]
    pub fn hserdy(&mut self) -> HSERDY_W<CR_SPEC, 17> {
        HSERDY_W::new(self)
    }
    #[doc = "Bit 18 - HSE clock bypass"]
    #[inline(always)]
    #[must_use]
    pub fn hsebyp(&mut self) -> HSEBYP_W<CR_SPEC, 18> {
        HSEBYP_W::new(self)
    }
    #[doc = "Bit 19 - HSE Clock Security System enable"]
    #[inline(always)]
    #[must_use]
    pub fn hsecsson(&mut self) -> HSECSSON_W<CR_SPEC, 19> {
        HSECSSON_W::new(self)
    }
    #[doc = "Bit 24 - PLL1 enable"]
    #[inline(always)]
    #[must_use]
    pub fn pll1on(&mut self) -> PLL1ON_W<CR_SPEC, 24> {
        PLL1ON_W::new(self)
    }
    #[doc = "Bit 25 - PLL1 clock ready flag"]
    #[inline(always)]
    #[must_use]
    pub fn pll1rdy(&mut self) -> PLL1RDY_W<CR_SPEC, 25> {
        PLL1RDY_W::new(self)
    }
    #[doc = "Bit 26 - PLL2 enable"]
    #[inline(always)]
    #[must_use]
    pub fn pll2on(&mut self) -> PLL2ON_W<CR_SPEC, 26> {
        PLL2ON_W::new(self)
    }
    #[doc = "Bit 27 - PLL2 clock ready flag"]
    #[inline(always)]
    #[must_use]
    pub fn pll2rdy(&mut self) -> PLL2RDY_W<CR_SPEC, 27> {
        PLL2RDY_W::new(self)
    }
    #[doc = "Bit 28 - PLL3 enable"]
    #[inline(always)]
    #[must_use]
    pub fn pll3on(&mut self) -> PLL3ON_W<CR_SPEC, 28> {
        PLL3ON_W::new(self)
    }
    #[doc = "Bit 29 - PLL3 clock ready flag"]
    #[inline(always)]
    #[must_use]
    pub fn pll3rdy(&mut self) -> PLL3RDY_W<CR_SPEC, 29> {
        PLL3RDY_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "clock control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CR_SPEC;
impl crate::RegisterSpec for CR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cr::R`](R) reader structure"]
impl crate::Readable for CR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`cr::W`](W) writer structure"]
impl crate::Writable for CR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CR to value 0x83"]
impl crate::Resettable for CR_SPEC {
    const RESET_VALUE: Self::Ux = 0x83;
}
