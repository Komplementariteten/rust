#[doc = "Register `C1_APB1HENR` reader"]
pub type R = crate::R<C1_APB1HENR_SPEC>;
#[doc = "Register `C1_APB1HENR` writer"]
pub type W = crate::W<C1_APB1HENR_SPEC>;
#[doc = "Field `CRSEN` reader - Clock Recovery System peripheral clock enable"]
pub type CRSEN_R = crate::BitReader;
#[doc = "Field `CRSEN` writer - Clock Recovery System peripheral clock enable"]
pub type CRSEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SWPEN` reader - SWPMI Peripheral Clocks Enable"]
pub type SWPEN_R = crate::BitReader;
#[doc = "Field `SWPEN` writer - SWPMI Peripheral Clocks Enable"]
pub type SWPEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `OPAMPEN` reader - OPAMP peripheral clock enable"]
pub type OPAMPEN_R = crate::BitReader;
#[doc = "Field `OPAMPEN` writer - OPAMP peripheral clock enable"]
pub type OPAMPEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `MDIOSEN` reader - MDIOS peripheral clock enable"]
pub type MDIOSEN_R = crate::BitReader;
#[doc = "Field `MDIOSEN` writer - MDIOS peripheral clock enable"]
pub type MDIOSEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `FDCANEN` reader - FDCAN Peripheral Clocks Enable"]
pub type FDCANEN_R = crate::BitReader;
#[doc = "Field `FDCANEN` writer - FDCAN Peripheral Clocks Enable"]
pub type FDCANEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 1 - Clock Recovery System peripheral clock enable"]
    #[inline(always)]
    pub fn crsen(&self) -> CRSEN_R {
        CRSEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - SWPMI Peripheral Clocks Enable"]
    #[inline(always)]
    pub fn swpen(&self) -> SWPEN_R {
        SWPEN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 4 - OPAMP peripheral clock enable"]
    #[inline(always)]
    pub fn opampen(&self) -> OPAMPEN_R {
        OPAMPEN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - MDIOS peripheral clock enable"]
    #[inline(always)]
    pub fn mdiosen(&self) -> MDIOSEN_R {
        MDIOSEN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 8 - FDCAN Peripheral Clocks Enable"]
    #[inline(always)]
    pub fn fdcanen(&self) -> FDCANEN_R {
        FDCANEN_R::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - Clock Recovery System peripheral clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn crsen(&mut self) -> CRSEN_W<C1_APB1HENR_SPEC, 1> {
        CRSEN_W::new(self)
    }
    #[doc = "Bit 2 - SWPMI Peripheral Clocks Enable"]
    #[inline(always)]
    #[must_use]
    pub fn swpen(&mut self) -> SWPEN_W<C1_APB1HENR_SPEC, 2> {
        SWPEN_W::new(self)
    }
    #[doc = "Bit 4 - OPAMP peripheral clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn opampen(&mut self) -> OPAMPEN_W<C1_APB1HENR_SPEC, 4> {
        OPAMPEN_W::new(self)
    }
    #[doc = "Bit 5 - MDIOS peripheral clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn mdiosen(&mut self) -> MDIOSEN_W<C1_APB1HENR_SPEC, 5> {
        MDIOSEN_W::new(self)
    }
    #[doc = "Bit 8 - FDCAN Peripheral Clocks Enable"]
    #[inline(always)]
    #[must_use]
    pub fn fdcanen(&mut self) -> FDCANEN_W<C1_APB1HENR_SPEC, 8> {
        FDCANEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "RCC APB1 Clock Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`c1_apb1henr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`c1_apb1henr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct C1_APB1HENR_SPEC;
impl crate::RegisterSpec for C1_APB1HENR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`c1_apb1henr::R`](R) reader structure"]
impl crate::Readable for C1_APB1HENR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`c1_apb1henr::W`](W) writer structure"]
impl crate::Writable for C1_APB1HENR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets C1_APB1HENR to value 0"]
impl crate::Resettable for C1_APB1HENR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
