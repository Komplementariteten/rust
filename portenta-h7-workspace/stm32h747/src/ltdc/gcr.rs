#[doc = "Register `GCR` reader"]
pub type R = crate::R<GCR_SPEC>;
#[doc = "Register `GCR` writer"]
pub type W = crate::W<GCR_SPEC>;
#[doc = "Field `LTDCEN` reader - LCD-TFT controller enable bit"]
pub type LTDCEN_R = crate::BitReader;
#[doc = "Field `LTDCEN` writer - LCD-TFT controller enable bit"]
pub type LTDCEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `DBW` reader - Dither Blue Width"]
pub type DBW_R = crate::FieldReader;
#[doc = "Field `DGW` reader - Dither Green Width"]
pub type DGW_R = crate::FieldReader;
#[doc = "Field `DRW` reader - Dither Red Width"]
pub type DRW_R = crate::FieldReader;
#[doc = "Field `DEN` reader - Dither Enable"]
pub type DEN_R = crate::BitReader;
#[doc = "Field `DEN` writer - Dither Enable"]
pub type DEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PCPOL` reader - Pixel Clock Polarity"]
pub type PCPOL_R = crate::BitReader;
#[doc = "Field `PCPOL` writer - Pixel Clock Polarity"]
pub type PCPOL_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `DEPOL` reader - Data Enable Polarity"]
pub type DEPOL_R = crate::BitReader;
#[doc = "Field `DEPOL` writer - Data Enable Polarity"]
pub type DEPOL_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `VSPOL` reader - Vertical Synchronization Polarity"]
pub type VSPOL_R = crate::BitReader;
#[doc = "Field `VSPOL` writer - Vertical Synchronization Polarity"]
pub type VSPOL_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `HSPOL` reader - Horizontal Synchronization Polarity"]
pub type HSPOL_R = crate::BitReader;
#[doc = "Field `HSPOL` writer - Horizontal Synchronization Polarity"]
pub type HSPOL_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - LCD-TFT controller enable bit"]
    #[inline(always)]
    pub fn ltdcen(&self) -> LTDCEN_R {
        LTDCEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 4:6 - Dither Blue Width"]
    #[inline(always)]
    pub fn dbw(&self) -> DBW_R {
        DBW_R::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bits 8:10 - Dither Green Width"]
    #[inline(always)]
    pub fn dgw(&self) -> DGW_R {
        DGW_R::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bits 12:14 - Dither Red Width"]
    #[inline(always)]
    pub fn drw(&self) -> DRW_R {
        DRW_R::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bit 16 - Dither Enable"]
    #[inline(always)]
    pub fn den(&self) -> DEN_R {
        DEN_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 28 - Pixel Clock Polarity"]
    #[inline(always)]
    pub fn pcpol(&self) -> PCPOL_R {
        PCPOL_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Data Enable Polarity"]
    #[inline(always)]
    pub fn depol(&self) -> DEPOL_R {
        DEPOL_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Vertical Synchronization Polarity"]
    #[inline(always)]
    pub fn vspol(&self) -> VSPOL_R {
        VSPOL_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Horizontal Synchronization Polarity"]
    #[inline(always)]
    pub fn hspol(&self) -> HSPOL_R {
        HSPOL_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - LCD-TFT controller enable bit"]
    #[inline(always)]
    #[must_use]
    pub fn ltdcen(&mut self) -> LTDCEN_W<GCR_SPEC, 0> {
        LTDCEN_W::new(self)
    }
    #[doc = "Bit 16 - Dither Enable"]
    #[inline(always)]
    #[must_use]
    pub fn den(&mut self) -> DEN_W<GCR_SPEC, 16> {
        DEN_W::new(self)
    }
    #[doc = "Bit 28 - Pixel Clock Polarity"]
    #[inline(always)]
    #[must_use]
    pub fn pcpol(&mut self) -> PCPOL_W<GCR_SPEC, 28> {
        PCPOL_W::new(self)
    }
    #[doc = "Bit 29 - Data Enable Polarity"]
    #[inline(always)]
    #[must_use]
    pub fn depol(&mut self) -> DEPOL_W<GCR_SPEC, 29> {
        DEPOL_W::new(self)
    }
    #[doc = "Bit 30 - Vertical Synchronization Polarity"]
    #[inline(always)]
    #[must_use]
    pub fn vspol(&mut self) -> VSPOL_W<GCR_SPEC, 30> {
        VSPOL_W::new(self)
    }
    #[doc = "Bit 31 - Horizontal Synchronization Polarity"]
    #[inline(always)]
    #[must_use]
    pub fn hspol(&mut self) -> HSPOL_W<GCR_SPEC, 31> {
        HSPOL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Global Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gcr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gcr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GCR_SPEC;
impl crate::RegisterSpec for GCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gcr::R`](R) reader structure"]
impl crate::Readable for GCR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`gcr::W`](W) writer structure"]
impl crate::Writable for GCR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets GCR to value 0x2220"]
impl crate::Resettable for GCR_SPEC {
    const RESET_VALUE: Self::Ux = 0x2220;
}
