#[doc = "Register `APB1HRSTR` reader"]
pub type R = crate::R<APB1HRSTR_SPEC>;
#[doc = "Register `APB1HRSTR` writer"]
pub type W = crate::W<APB1HRSTR_SPEC>;
#[doc = "Field `CRSRST` reader - Clock Recovery System reset"]
pub type CRSRST_R = crate::BitReader;
#[doc = "Field `CRSRST` writer - Clock Recovery System reset"]
pub type CRSRST_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SWPRST` reader - SWPMI block reset"]
pub type SWPRST_R = crate::BitReader;
#[doc = "Field `SWPRST` writer - SWPMI block reset"]
pub type SWPRST_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `OPAMPRST` reader - OPAMP block reset"]
pub type OPAMPRST_R = crate::BitReader;
#[doc = "Field `OPAMPRST` writer - OPAMP block reset"]
pub type OPAMPRST_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `MDIOSRST` reader - MDIOS block reset"]
pub type MDIOSRST_R = crate::BitReader;
#[doc = "Field `MDIOSRST` writer - MDIOS block reset"]
pub type MDIOSRST_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `FDCANRST` reader - FDCAN block reset"]
pub type FDCANRST_R = crate::BitReader;
#[doc = "Field `FDCANRST` writer - FDCAN block reset"]
pub type FDCANRST_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 1 - Clock Recovery System reset"]
    #[inline(always)]
    pub fn crsrst(&self) -> CRSRST_R {
        CRSRST_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - SWPMI block reset"]
    #[inline(always)]
    pub fn swprst(&self) -> SWPRST_R {
        SWPRST_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 4 - OPAMP block reset"]
    #[inline(always)]
    pub fn opamprst(&self) -> OPAMPRST_R {
        OPAMPRST_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - MDIOS block reset"]
    #[inline(always)]
    pub fn mdiosrst(&self) -> MDIOSRST_R {
        MDIOSRST_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 8 - FDCAN block reset"]
    #[inline(always)]
    pub fn fdcanrst(&self) -> FDCANRST_R {
        FDCANRST_R::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - Clock Recovery System reset"]
    #[inline(always)]
    #[must_use]
    pub fn crsrst(&mut self) -> CRSRST_W<APB1HRSTR_SPEC, 1> {
        CRSRST_W::new(self)
    }
    #[doc = "Bit 2 - SWPMI block reset"]
    #[inline(always)]
    #[must_use]
    pub fn swprst(&mut self) -> SWPRST_W<APB1HRSTR_SPEC, 2> {
        SWPRST_W::new(self)
    }
    #[doc = "Bit 4 - OPAMP block reset"]
    #[inline(always)]
    #[must_use]
    pub fn opamprst(&mut self) -> OPAMPRST_W<APB1HRSTR_SPEC, 4> {
        OPAMPRST_W::new(self)
    }
    #[doc = "Bit 5 - MDIOS block reset"]
    #[inline(always)]
    #[must_use]
    pub fn mdiosrst(&mut self) -> MDIOSRST_W<APB1HRSTR_SPEC, 5> {
        MDIOSRST_W::new(self)
    }
    #[doc = "Bit 8 - FDCAN block reset"]
    #[inline(always)]
    #[must_use]
    pub fn fdcanrst(&mut self) -> FDCANRST_W<APB1HRSTR_SPEC, 8> {
        FDCANRST_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "RCC APB1 Peripheral Reset Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apb1hrstr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apb1hrstr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct APB1HRSTR_SPEC;
impl crate::RegisterSpec for APB1HRSTR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`apb1hrstr::R`](R) reader structure"]
impl crate::Readable for APB1HRSTR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`apb1hrstr::W`](W) writer structure"]
impl crate::Writable for APB1HRSTR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets APB1HRSTR to value 0"]
impl crate::Resettable for APB1HRSTR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
