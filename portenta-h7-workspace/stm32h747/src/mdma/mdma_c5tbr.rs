#[doc = "Register `MDMA_C5TBR` reader"]
pub type R = crate::R<MDMA_C5TBR_SPEC>;
#[doc = "Register `MDMA_C5TBR` writer"]
pub type W = crate::W<MDMA_C5TBR_SPEC>;
#[doc = "Field `TSEL` reader - Trigger selection"]
pub type TSEL_R = crate::FieldReader;
#[doc = "Field `TSEL` writer - Trigger selection"]
pub type TSEL_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 6, O>;
#[doc = "Field `SBUS` reader - Source BUS select This bit is protected and can be written only if EN is 0."]
pub type SBUS_R = crate::BitReader;
#[doc = "Field `SBUS` writer - Source BUS select This bit is protected and can be written only if EN is 0."]
pub type SBUS_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `DBUS` reader - Destination BUS slect This bit is protected and can be written only if EN is 0."]
pub type DBUS_R = crate::BitReader;
#[doc = "Field `DBUS` writer - Destination BUS slect This bit is protected and can be written only if EN is 0."]
pub type DBUS_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bits 0:5 - Trigger selection"]
    #[inline(always)]
    pub fn tsel(&self) -> TSEL_R {
        TSEL_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bit 16 - Source BUS select This bit is protected and can be written only if EN is 0."]
    #[inline(always)]
    pub fn sbus(&self) -> SBUS_R {
        SBUS_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Destination BUS slect This bit is protected and can be written only if EN is 0."]
    #[inline(always)]
    pub fn dbus(&self) -> DBUS_R {
        DBUS_R::new(((self.bits >> 17) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:5 - Trigger selection"]
    #[inline(always)]
    #[must_use]
    pub fn tsel(&mut self) -> TSEL_W<MDMA_C5TBR_SPEC, 0> {
        TSEL_W::new(self)
    }
    #[doc = "Bit 16 - Source BUS select This bit is protected and can be written only if EN is 0."]
    #[inline(always)]
    #[must_use]
    pub fn sbus(&mut self) -> SBUS_W<MDMA_C5TBR_SPEC, 16> {
        SBUS_W::new(self)
    }
    #[doc = "Bit 17 - Destination BUS slect This bit is protected and can be written only if EN is 0."]
    #[inline(always)]
    #[must_use]
    pub fn dbus(&mut self) -> DBUS_W<MDMA_C5TBR_SPEC, 17> {
        DBUS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "MDMA channel x Trigger and Bus selection Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mdma_c5tbr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mdma_c5tbr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MDMA_C5TBR_SPEC;
impl crate::RegisterSpec for MDMA_C5TBR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mdma_c5tbr::R`](R) reader structure"]
impl crate::Readable for MDMA_C5TBR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`mdma_c5tbr::W`](W) writer structure"]
impl crate::Writable for MDMA_C5TBR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MDMA_C5TBR to value 0"]
impl crate::Resettable for MDMA_C5TBR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
