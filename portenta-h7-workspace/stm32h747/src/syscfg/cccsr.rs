#[doc = "Register `CCCSR` reader"]
pub type R = crate::R<CCCSR_SPEC>;
#[doc = "Register `CCCSR` writer"]
pub type W = crate::W<CCCSR_SPEC>;
#[doc = "Field `EN` reader - enable"]
pub type EN_R = crate::BitReader;
#[doc = "Field `EN` writer - enable"]
pub type EN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CS` reader - Code selection"]
pub type CS_R = crate::BitReader;
#[doc = "Field `CS` writer - Code selection"]
pub type CS_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `READY` reader - Compensation cell ready flag"]
pub type READY_R = crate::BitReader;
#[doc = "Field `READY` writer - Compensation cell ready flag"]
pub type READY_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `HSLV` reader - High-speed at low-voltage"]
pub type HSLV_R = crate::BitReader;
#[doc = "Field `HSLV` writer - High-speed at low-voltage"]
pub type HSLV_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - enable"]
    #[inline(always)]
    pub fn en(&self) -> EN_R {
        EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Code selection"]
    #[inline(always)]
    pub fn cs(&self) -> CS_R {
        CS_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 8 - Compensation cell ready flag"]
    #[inline(always)]
    pub fn ready(&self) -> READY_R {
        READY_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 16 - High-speed at low-voltage"]
    #[inline(always)]
    pub fn hslv(&self) -> HSLV_R {
        HSLV_R::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - enable"]
    #[inline(always)]
    #[must_use]
    pub fn en(&mut self) -> EN_W<CCCSR_SPEC, 0> {
        EN_W::new(self)
    }
    #[doc = "Bit 1 - Code selection"]
    #[inline(always)]
    #[must_use]
    pub fn cs(&mut self) -> CS_W<CCCSR_SPEC, 1> {
        CS_W::new(self)
    }
    #[doc = "Bit 8 - Compensation cell ready flag"]
    #[inline(always)]
    #[must_use]
    pub fn ready(&mut self) -> READY_W<CCCSR_SPEC, 8> {
        READY_W::new(self)
    }
    #[doc = "Bit 16 - High-speed at low-voltage"]
    #[inline(always)]
    #[must_use]
    pub fn hslv(&mut self) -> HSLV_W<CCCSR_SPEC, 16> {
        HSLV_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "compensation cell control/status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cccsr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cccsr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CCCSR_SPEC;
impl crate::RegisterSpec for CCCSR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cccsr::R`](R) reader structure"]
impl crate::Readable for CCCSR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`cccsr::W`](W) writer structure"]
impl crate::Writable for CCCSR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CCCSR to value 0"]
impl crate::Resettable for CCCSR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
