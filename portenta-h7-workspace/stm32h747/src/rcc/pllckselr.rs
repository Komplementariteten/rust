#[doc = "Register `PLLCKSELR` reader"]
pub type R = crate::R<PLLCKSELR_SPEC>;
#[doc = "Register `PLLCKSELR` writer"]
pub type W = crate::W<PLLCKSELR_SPEC>;
#[doc = "Field `PLLSRC` reader - DIVMx and PLLs clock source selection"]
pub type PLLSRC_R = crate::FieldReader;
#[doc = "Field `PLLSRC` writer - DIVMx and PLLs clock source selection"]
pub type PLLSRC_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
#[doc = "Field `DIVM1` reader - Prescaler for PLL1"]
pub type DIVM1_R = crate::FieldReader;
#[doc = "Field `DIVM1` writer - Prescaler for PLL1"]
pub type DIVM1_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 6, O>;
#[doc = "Field `DIVM2` reader - Prescaler for PLL2"]
pub type DIVM2_R = crate::FieldReader;
#[doc = "Field `DIVM2` writer - Prescaler for PLL2"]
pub type DIVM2_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 6, O>;
#[doc = "Field `DIVM3` reader - Prescaler for PLL3"]
pub type DIVM3_R = crate::FieldReader;
#[doc = "Field `DIVM3` writer - Prescaler for PLL3"]
pub type DIVM3_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 6, O>;
impl R {
    #[doc = "Bits 0:1 - DIVMx and PLLs clock source selection"]
    #[inline(always)]
    pub fn pllsrc(&self) -> PLLSRC_R {
        PLLSRC_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 4:9 - Prescaler for PLL1"]
    #[inline(always)]
    pub fn divm1(&self) -> DIVM1_R {
        DIVM1_R::new(((self.bits >> 4) & 0x3f) as u8)
    }
    #[doc = "Bits 12:17 - Prescaler for PLL2"]
    #[inline(always)]
    pub fn divm2(&self) -> DIVM2_R {
        DIVM2_R::new(((self.bits >> 12) & 0x3f) as u8)
    }
    #[doc = "Bits 20:25 - Prescaler for PLL3"]
    #[inline(always)]
    pub fn divm3(&self) -> DIVM3_R {
        DIVM3_R::new(((self.bits >> 20) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - DIVMx and PLLs clock source selection"]
    #[inline(always)]
    #[must_use]
    pub fn pllsrc(&mut self) -> PLLSRC_W<PLLCKSELR_SPEC, 0> {
        PLLSRC_W::new(self)
    }
    #[doc = "Bits 4:9 - Prescaler for PLL1"]
    #[inline(always)]
    #[must_use]
    pub fn divm1(&mut self) -> DIVM1_W<PLLCKSELR_SPEC, 4> {
        DIVM1_W::new(self)
    }
    #[doc = "Bits 12:17 - Prescaler for PLL2"]
    #[inline(always)]
    #[must_use]
    pub fn divm2(&mut self) -> DIVM2_W<PLLCKSELR_SPEC, 12> {
        DIVM2_W::new(self)
    }
    #[doc = "Bits 20:25 - Prescaler for PLL3"]
    #[inline(always)]
    #[must_use]
    pub fn divm3(&mut self) -> DIVM3_W<PLLCKSELR_SPEC, 20> {
        DIVM3_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "RCC PLLs Clock Source Selection Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pllckselr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pllckselr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PLLCKSELR_SPEC;
impl crate::RegisterSpec for PLLCKSELR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pllckselr::R`](R) reader structure"]
impl crate::Readable for PLLCKSELR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`pllckselr::W`](W) writer structure"]
impl crate::Writable for PLLCKSELR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PLLCKSELR to value 0x0202_0200"]
impl crate::Resettable for PLLCKSELR_SPEC {
    const RESET_VALUE: Self::Ux = 0x0202_0200;
}
