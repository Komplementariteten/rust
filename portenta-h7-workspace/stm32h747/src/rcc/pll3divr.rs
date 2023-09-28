#[doc = "Register `PLL3DIVR` reader"]
pub type R = crate::R<PLL3DIVR_SPEC>;
#[doc = "Register `PLL3DIVR` writer"]
pub type W = crate::W<PLL3DIVR_SPEC>;
#[doc = "Field `DIVN3` reader - Multiplication factor for PLL1 VCO"]
pub type DIVN3_R = crate::FieldReader<u16>;
#[doc = "Field `DIVN3` writer - Multiplication factor for PLL1 VCO"]
pub type DIVN3_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 9, O, u16>;
#[doc = "Field `DIVP3` reader - PLL DIVP division factor"]
pub type DIVP3_R = crate::FieldReader;
#[doc = "Field `DIVP3` writer - PLL DIVP division factor"]
pub type DIVP3_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 7, O>;
#[doc = "Field `DIVQ3` reader - PLL DIVQ division factor"]
pub type DIVQ3_R = crate::FieldReader;
#[doc = "Field `DIVQ3` writer - PLL DIVQ division factor"]
pub type DIVQ3_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 7, O>;
#[doc = "Field `DIVR3` reader - PLL DIVR division factor"]
pub type DIVR3_R = crate::FieldReader;
#[doc = "Field `DIVR3` writer - PLL DIVR division factor"]
pub type DIVR3_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 7, O>;
impl R {
    #[doc = "Bits 0:8 - Multiplication factor for PLL1 VCO"]
    #[inline(always)]
    pub fn divn3(&self) -> DIVN3_R {
        DIVN3_R::new((self.bits & 0x01ff) as u16)
    }
    #[doc = "Bits 9:15 - PLL DIVP division factor"]
    #[inline(always)]
    pub fn divp3(&self) -> DIVP3_R {
        DIVP3_R::new(((self.bits >> 9) & 0x7f) as u8)
    }
    #[doc = "Bits 16:22 - PLL DIVQ division factor"]
    #[inline(always)]
    pub fn divq3(&self) -> DIVQ3_R {
        DIVQ3_R::new(((self.bits >> 16) & 0x7f) as u8)
    }
    #[doc = "Bits 24:30 - PLL DIVR division factor"]
    #[inline(always)]
    pub fn divr3(&self) -> DIVR3_R {
        DIVR3_R::new(((self.bits >> 24) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:8 - Multiplication factor for PLL1 VCO"]
    #[inline(always)]
    #[must_use]
    pub fn divn3(&mut self) -> DIVN3_W<PLL3DIVR_SPEC, 0> {
        DIVN3_W::new(self)
    }
    #[doc = "Bits 9:15 - PLL DIVP division factor"]
    #[inline(always)]
    #[must_use]
    pub fn divp3(&mut self) -> DIVP3_W<PLL3DIVR_SPEC, 9> {
        DIVP3_W::new(self)
    }
    #[doc = "Bits 16:22 - PLL DIVQ division factor"]
    #[inline(always)]
    #[must_use]
    pub fn divq3(&mut self) -> DIVQ3_W<PLL3DIVR_SPEC, 16> {
        DIVQ3_W::new(self)
    }
    #[doc = "Bits 24:30 - PLL DIVR division factor"]
    #[inline(always)]
    #[must_use]
    pub fn divr3(&mut self) -> DIVR3_W<PLL3DIVR_SPEC, 24> {
        DIVR3_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "RCC PLL3 Dividers Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pll3divr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pll3divr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PLL3DIVR_SPEC;
impl crate::RegisterSpec for PLL3DIVR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pll3divr::R`](R) reader structure"]
impl crate::Readable for PLL3DIVR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`pll3divr::W`](W) writer structure"]
impl crate::Writable for PLL3DIVR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PLL3DIVR to value 0x0101_0280"]
impl crate::Resettable for PLL3DIVR_SPEC {
    const RESET_VALUE: Self::Ux = 0x0101_0280;
}
