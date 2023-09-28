#[doc = "Register `PLLCFGR` reader"]
pub type R = crate::R<PLLCFGR_SPEC>;
#[doc = "Register `PLLCFGR` writer"]
pub type W = crate::W<PLLCFGR_SPEC>;
#[doc = "Field `PLL1FRACEN` reader - PLL1 fractional latch enable"]
pub type PLL1FRACEN_R = crate::BitReader;
#[doc = "Field `PLL1FRACEN` writer - PLL1 fractional latch enable"]
pub type PLL1FRACEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PLL1VCOSEL` reader - PLL1 VCO selection"]
pub type PLL1VCOSEL_R = crate::BitReader;
#[doc = "Field `PLL1VCOSEL` writer - PLL1 VCO selection"]
pub type PLL1VCOSEL_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PLL1RGE` reader - PLL1 input frequency range"]
pub type PLL1RGE_R = crate::FieldReader;
#[doc = "Field `PLL1RGE` writer - PLL1 input frequency range"]
pub type PLL1RGE_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
#[doc = "Field `PLL2FRACEN` reader - PLL2 fractional latch enable"]
pub type PLL2FRACEN_R = crate::BitReader;
#[doc = "Field `PLL2FRACEN` writer - PLL2 fractional latch enable"]
pub type PLL2FRACEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PLL2VCOSEL` reader - PLL2 VCO selection"]
pub type PLL2VCOSEL_R = crate::BitReader;
#[doc = "Field `PLL2VCOSEL` writer - PLL2 VCO selection"]
pub type PLL2VCOSEL_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PLL2RGE` reader - PLL2 input frequency range"]
pub type PLL2RGE_R = crate::FieldReader;
#[doc = "Field `PLL2RGE` writer - PLL2 input frequency range"]
pub type PLL2RGE_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
#[doc = "Field `PLL3FRACEN` reader - PLL3 fractional latch enable"]
pub type PLL3FRACEN_R = crate::BitReader;
#[doc = "Field `PLL3FRACEN` writer - PLL3 fractional latch enable"]
pub type PLL3FRACEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PLL3VCOSEL` reader - PLL3 VCO selection"]
pub type PLL3VCOSEL_R = crate::BitReader;
#[doc = "Field `PLL3VCOSEL` writer - PLL3 VCO selection"]
pub type PLL3VCOSEL_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PLL3RGE` reader - PLL3 input frequency range"]
pub type PLL3RGE_R = crate::FieldReader;
#[doc = "Field `PLL3RGE` writer - PLL3 input frequency range"]
pub type PLL3RGE_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
#[doc = "Field `DIVP1EN` reader - PLL1 DIVP divider output enable"]
pub type DIVP1EN_R = crate::BitReader;
#[doc = "Field `DIVP1EN` writer - PLL1 DIVP divider output enable"]
pub type DIVP1EN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `DIVQ1EN` reader - PLL1 DIVQ divider output enable"]
pub type DIVQ1EN_R = crate::BitReader;
#[doc = "Field `DIVQ1EN` writer - PLL1 DIVQ divider output enable"]
pub type DIVQ1EN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `DIVR1EN` reader - PLL1 DIVR divider output enable"]
pub type DIVR1EN_R = crate::BitReader;
#[doc = "Field `DIVR1EN` writer - PLL1 DIVR divider output enable"]
pub type DIVR1EN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `DIVP2EN` reader - PLL2 DIVP divider output enable"]
pub type DIVP2EN_R = crate::BitReader;
#[doc = "Field `DIVP2EN` writer - PLL2 DIVP divider output enable"]
pub type DIVP2EN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `DIVQ2EN` reader - PLL2 DIVQ divider output enable"]
pub type DIVQ2EN_R = crate::BitReader;
#[doc = "Field `DIVQ2EN` writer - PLL2 DIVQ divider output enable"]
pub type DIVQ2EN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `DIVR2EN` reader - PLL2 DIVR divider output enable"]
pub type DIVR2EN_R = crate::BitReader;
#[doc = "Field `DIVR2EN` writer - PLL2 DIVR divider output enable"]
pub type DIVR2EN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `DIVP3EN` reader - PLL3 DIVP divider output enable"]
pub type DIVP3EN_R = crate::BitReader;
#[doc = "Field `DIVP3EN` writer - PLL3 DIVP divider output enable"]
pub type DIVP3EN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `DIVQ3EN` reader - PLL3 DIVQ divider output enable"]
pub type DIVQ3EN_R = crate::BitReader;
#[doc = "Field `DIVQ3EN` writer - PLL3 DIVQ divider output enable"]
pub type DIVQ3EN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `DIVR3EN` reader - PLL3 DIVR divider output enable"]
pub type DIVR3EN_R = crate::BitReader;
#[doc = "Field `DIVR3EN` writer - PLL3 DIVR divider output enable"]
pub type DIVR3EN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - PLL1 fractional latch enable"]
    #[inline(always)]
    pub fn pll1fracen(&self) -> PLL1FRACEN_R {
        PLL1FRACEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - PLL1 VCO selection"]
    #[inline(always)]
    pub fn pll1vcosel(&self) -> PLL1VCOSEL_R {
        PLL1VCOSEL_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:3 - PLL1 input frequency range"]
    #[inline(always)]
    pub fn pll1rge(&self) -> PLL1RGE_R {
        PLL1RGE_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bit 4 - PLL2 fractional latch enable"]
    #[inline(always)]
    pub fn pll2fracen(&self) -> PLL2FRACEN_R {
        PLL2FRACEN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - PLL2 VCO selection"]
    #[inline(always)]
    pub fn pll2vcosel(&self) -> PLL2VCOSEL_R {
        PLL2VCOSEL_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 6:7 - PLL2 input frequency range"]
    #[inline(always)]
    pub fn pll2rge(&self) -> PLL2RGE_R {
        PLL2RGE_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bit 8 - PLL3 fractional latch enable"]
    #[inline(always)]
    pub fn pll3fracen(&self) -> PLL3FRACEN_R {
        PLL3FRACEN_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - PLL3 VCO selection"]
    #[inline(always)]
    pub fn pll3vcosel(&self) -> PLL3VCOSEL_R {
        PLL3VCOSEL_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bits 10:11 - PLL3 input frequency range"]
    #[inline(always)]
    pub fn pll3rge(&self) -> PLL3RGE_R {
        PLL3RGE_R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bit 16 - PLL1 DIVP divider output enable"]
    #[inline(always)]
    pub fn divp1en(&self) -> DIVP1EN_R {
        DIVP1EN_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - PLL1 DIVQ divider output enable"]
    #[inline(always)]
    pub fn divq1en(&self) -> DIVQ1EN_R {
        DIVQ1EN_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - PLL1 DIVR divider output enable"]
    #[inline(always)]
    pub fn divr1en(&self) -> DIVR1EN_R {
        DIVR1EN_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - PLL2 DIVP divider output enable"]
    #[inline(always)]
    pub fn divp2en(&self) -> DIVP2EN_R {
        DIVP2EN_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - PLL2 DIVQ divider output enable"]
    #[inline(always)]
    pub fn divq2en(&self) -> DIVQ2EN_R {
        DIVQ2EN_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - PLL2 DIVR divider output enable"]
    #[inline(always)]
    pub fn divr2en(&self) -> DIVR2EN_R {
        DIVR2EN_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - PLL3 DIVP divider output enable"]
    #[inline(always)]
    pub fn divp3en(&self) -> DIVP3EN_R {
        DIVP3EN_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - PLL3 DIVQ divider output enable"]
    #[inline(always)]
    pub fn divq3en(&self) -> DIVQ3EN_R {
        DIVQ3EN_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - PLL3 DIVR divider output enable"]
    #[inline(always)]
    pub fn divr3en(&self) -> DIVR3EN_R {
        DIVR3EN_R::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - PLL1 fractional latch enable"]
    #[inline(always)]
    #[must_use]
    pub fn pll1fracen(&mut self) -> PLL1FRACEN_W<PLLCFGR_SPEC, 0> {
        PLL1FRACEN_W::new(self)
    }
    #[doc = "Bit 1 - PLL1 VCO selection"]
    #[inline(always)]
    #[must_use]
    pub fn pll1vcosel(&mut self) -> PLL1VCOSEL_W<PLLCFGR_SPEC, 1> {
        PLL1VCOSEL_W::new(self)
    }
    #[doc = "Bits 2:3 - PLL1 input frequency range"]
    #[inline(always)]
    #[must_use]
    pub fn pll1rge(&mut self) -> PLL1RGE_W<PLLCFGR_SPEC, 2> {
        PLL1RGE_W::new(self)
    }
    #[doc = "Bit 4 - PLL2 fractional latch enable"]
    #[inline(always)]
    #[must_use]
    pub fn pll2fracen(&mut self) -> PLL2FRACEN_W<PLLCFGR_SPEC, 4> {
        PLL2FRACEN_W::new(self)
    }
    #[doc = "Bit 5 - PLL2 VCO selection"]
    #[inline(always)]
    #[must_use]
    pub fn pll2vcosel(&mut self) -> PLL2VCOSEL_W<PLLCFGR_SPEC, 5> {
        PLL2VCOSEL_W::new(self)
    }
    #[doc = "Bits 6:7 - PLL2 input frequency range"]
    #[inline(always)]
    #[must_use]
    pub fn pll2rge(&mut self) -> PLL2RGE_W<PLLCFGR_SPEC, 6> {
        PLL2RGE_W::new(self)
    }
    #[doc = "Bit 8 - PLL3 fractional latch enable"]
    #[inline(always)]
    #[must_use]
    pub fn pll3fracen(&mut self) -> PLL3FRACEN_W<PLLCFGR_SPEC, 8> {
        PLL3FRACEN_W::new(self)
    }
    #[doc = "Bit 9 - PLL3 VCO selection"]
    #[inline(always)]
    #[must_use]
    pub fn pll3vcosel(&mut self) -> PLL3VCOSEL_W<PLLCFGR_SPEC, 9> {
        PLL3VCOSEL_W::new(self)
    }
    #[doc = "Bits 10:11 - PLL3 input frequency range"]
    #[inline(always)]
    #[must_use]
    pub fn pll3rge(&mut self) -> PLL3RGE_W<PLLCFGR_SPEC, 10> {
        PLL3RGE_W::new(self)
    }
    #[doc = "Bit 16 - PLL1 DIVP divider output enable"]
    #[inline(always)]
    #[must_use]
    pub fn divp1en(&mut self) -> DIVP1EN_W<PLLCFGR_SPEC, 16> {
        DIVP1EN_W::new(self)
    }
    #[doc = "Bit 17 - PLL1 DIVQ divider output enable"]
    #[inline(always)]
    #[must_use]
    pub fn divq1en(&mut self) -> DIVQ1EN_W<PLLCFGR_SPEC, 17> {
        DIVQ1EN_W::new(self)
    }
    #[doc = "Bit 18 - PLL1 DIVR divider output enable"]
    #[inline(always)]
    #[must_use]
    pub fn divr1en(&mut self) -> DIVR1EN_W<PLLCFGR_SPEC, 18> {
        DIVR1EN_W::new(self)
    }
    #[doc = "Bit 19 - PLL2 DIVP divider output enable"]
    #[inline(always)]
    #[must_use]
    pub fn divp2en(&mut self) -> DIVP2EN_W<PLLCFGR_SPEC, 19> {
        DIVP2EN_W::new(self)
    }
    #[doc = "Bit 20 - PLL2 DIVQ divider output enable"]
    #[inline(always)]
    #[must_use]
    pub fn divq2en(&mut self) -> DIVQ2EN_W<PLLCFGR_SPEC, 20> {
        DIVQ2EN_W::new(self)
    }
    #[doc = "Bit 21 - PLL2 DIVR divider output enable"]
    #[inline(always)]
    #[must_use]
    pub fn divr2en(&mut self) -> DIVR2EN_W<PLLCFGR_SPEC, 21> {
        DIVR2EN_W::new(self)
    }
    #[doc = "Bit 22 - PLL3 DIVP divider output enable"]
    #[inline(always)]
    #[must_use]
    pub fn divp3en(&mut self) -> DIVP3EN_W<PLLCFGR_SPEC, 22> {
        DIVP3EN_W::new(self)
    }
    #[doc = "Bit 23 - PLL3 DIVQ divider output enable"]
    #[inline(always)]
    #[must_use]
    pub fn divq3en(&mut self) -> DIVQ3EN_W<PLLCFGR_SPEC, 23> {
        DIVQ3EN_W::new(self)
    }
    #[doc = "Bit 24 - PLL3 DIVR divider output enable"]
    #[inline(always)]
    #[must_use]
    pub fn divr3en(&mut self) -> DIVR3EN_W<PLLCFGR_SPEC, 24> {
        DIVR3EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "RCC PLLs Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pllcfgr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pllcfgr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PLLCFGR_SPEC;
impl crate::RegisterSpec for PLLCFGR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pllcfgr::R`](R) reader structure"]
impl crate::Readable for PLLCFGR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`pllcfgr::W`](W) writer structure"]
impl crate::Writable for PLLCFGR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PLLCFGR to value 0x01ff_0000"]
impl crate::Resettable for PLLCFGR_SPEC {
    const RESET_VALUE: Self::Ux = 0x01ff_0000;
}
