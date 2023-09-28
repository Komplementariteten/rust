#[doc = "Register `PLL2DIVR` reader"]
pub type R = crate::R<PLL2DIVR_SPEC>;
#[doc = "Register `PLL2DIVR` writer"]
pub type W = crate::W<PLL2DIVR_SPEC>;
#[doc = "Field `DIVN1` reader - Multiplication factor for PLL1 VCO"]
pub type DIVN1_R = crate::FieldReader<u16>;
#[doc = "Field `DIVN1` writer - Multiplication factor for PLL1 VCO"]
pub type DIVN1_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 9, O, u16>;
#[doc = "Field `DIVP1` reader - PLL1 DIVP division factor"]
pub type DIVP1_R = crate::FieldReader;
#[doc = "Field `DIVP1` writer - PLL1 DIVP division factor"]
pub type DIVP1_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 7, O>;
#[doc = "Field `DIVQ1` reader - PLL1 DIVQ division factor"]
pub type DIVQ1_R = crate::FieldReader;
#[doc = "Field `DIVQ1` writer - PLL1 DIVQ division factor"]
pub type DIVQ1_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 7, O>;
#[doc = "Field `DIVR1` reader - PLL1 DIVR division factor"]
pub type DIVR1_R = crate::FieldReader;
#[doc = "Field `DIVR1` writer - PLL1 DIVR division factor"]
pub type DIVR1_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 7, O>;
impl R {
    #[doc = "Bits 0:8 - Multiplication factor for PLL1 VCO"]
    #[inline(always)]
    pub fn divn1(&self) -> DIVN1_R {
        DIVN1_R::new((self.bits & 0x01ff) as u16)
    }
    #[doc = "Bits 9:15 - PLL1 DIVP division factor"]
    #[inline(always)]
    pub fn divp1(&self) -> DIVP1_R {
        DIVP1_R::new(((self.bits >> 9) & 0x7f) as u8)
    }
    #[doc = "Bits 16:22 - PLL1 DIVQ division factor"]
    #[inline(always)]
    pub fn divq1(&self) -> DIVQ1_R {
        DIVQ1_R::new(((self.bits >> 16) & 0x7f) as u8)
    }
    #[doc = "Bits 24:30 - PLL1 DIVR division factor"]
    #[inline(always)]
    pub fn divr1(&self) -> DIVR1_R {
        DIVR1_R::new(((self.bits >> 24) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:8 - Multiplication factor for PLL1 VCO"]
    #[inline(always)]
    #[must_use]
    pub fn divn1(&mut self) -> DIVN1_W<PLL2DIVR_SPEC, 0> {
        DIVN1_W::new(self)
    }
    #[doc = "Bits 9:15 - PLL1 DIVP division factor"]
    #[inline(always)]
    #[must_use]
    pub fn divp1(&mut self) -> DIVP1_W<PLL2DIVR_SPEC, 9> {
        DIVP1_W::new(self)
    }
    #[doc = "Bits 16:22 - PLL1 DIVQ division factor"]
    #[inline(always)]
    #[must_use]
    pub fn divq1(&mut self) -> DIVQ1_W<PLL2DIVR_SPEC, 16> {
        DIVQ1_W::new(self)
    }
    #[doc = "Bits 24:30 - PLL1 DIVR division factor"]
    #[inline(always)]
    #[must_use]
    pub fn divr1(&mut self) -> DIVR1_W<PLL2DIVR_SPEC, 24> {
        DIVR1_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "RCC PLL2 Dividers Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pll2divr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pll2divr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PLL2DIVR_SPEC;
impl crate::RegisterSpec for PLL2DIVR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pll2divr::R`](R) reader structure"]
impl crate::Readable for PLL2DIVR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`pll2divr::W`](W) writer structure"]
impl crate::Writable for PLL2DIVR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PLL2DIVR to value 0x0101_0280"]
impl crate::Resettable for PLL2DIVR_SPEC {
    const RESET_VALUE: Self::Ux = 0x0101_0280;
}
