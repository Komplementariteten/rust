#[doc = "Register `DSI_WRPCR` reader"]
pub type R = crate::R<DSI_WRPCR_SPEC>;
#[doc = "Register `DSI_WRPCR` writer"]
pub type W = crate::W<DSI_WRPCR_SPEC>;
#[doc = "Field `PLLEN` reader - PLLEN"]
pub type PLLEN_R = crate::BitReader;
#[doc = "Field `PLLEN` writer - PLLEN"]
pub type PLLEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `NDIV` reader - NDIV"]
pub type NDIV_R = crate::FieldReader;
#[doc = "Field `NDIV` writer - NDIV"]
pub type NDIV_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 7, O>;
#[doc = "Field `IDF` reader - IDF"]
pub type IDF_R = crate::FieldReader;
#[doc = "Field `IDF` writer - IDF"]
pub type IDF_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O>;
#[doc = "Field `ODF` reader - ODF"]
pub type ODF_R = crate::FieldReader;
#[doc = "Field `ODF` writer - ODF"]
pub type ODF_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
#[doc = "Field `REGEN` reader - REGEN"]
pub type REGEN_R = crate::BitReader;
#[doc = "Field `REGEN` writer - REGEN"]
pub type REGEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - PLLEN"]
    #[inline(always)]
    pub fn pllen(&self) -> PLLEN_R {
        PLLEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 2:8 - NDIV"]
    #[inline(always)]
    pub fn ndiv(&self) -> NDIV_R {
        NDIV_R::new(((self.bits >> 2) & 0x7f) as u8)
    }
    #[doc = "Bits 11:14 - IDF"]
    #[inline(always)]
    pub fn idf(&self) -> IDF_R {
        IDF_R::new(((self.bits >> 11) & 0x0f) as u8)
    }
    #[doc = "Bits 16:17 - ODF"]
    #[inline(always)]
    pub fn odf(&self) -> ODF_R {
        ODF_R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bit 24 - REGEN"]
    #[inline(always)]
    pub fn regen(&self) -> REGEN_R {
        REGEN_R::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - PLLEN"]
    #[inline(always)]
    #[must_use]
    pub fn pllen(&mut self) -> PLLEN_W<DSI_WRPCR_SPEC, 0> {
        PLLEN_W::new(self)
    }
    #[doc = "Bits 2:8 - NDIV"]
    #[inline(always)]
    #[must_use]
    pub fn ndiv(&mut self) -> NDIV_W<DSI_WRPCR_SPEC, 2> {
        NDIV_W::new(self)
    }
    #[doc = "Bits 11:14 - IDF"]
    #[inline(always)]
    #[must_use]
    pub fn idf(&mut self) -> IDF_W<DSI_WRPCR_SPEC, 11> {
        IDF_W::new(self)
    }
    #[doc = "Bits 16:17 - ODF"]
    #[inline(always)]
    #[must_use]
    pub fn odf(&mut self) -> ODF_W<DSI_WRPCR_SPEC, 16> {
        ODF_W::new(self)
    }
    #[doc = "Bit 24 - REGEN"]
    #[inline(always)]
    #[must_use]
    pub fn regen(&mut self) -> REGEN_W<DSI_WRPCR_SPEC, 24> {
        REGEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "DSI wrapper regulator and PLL control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dsi_wrpcr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dsi_wrpcr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DSI_WRPCR_SPEC;
impl crate::RegisterSpec for DSI_WRPCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dsi_wrpcr::R`](R) reader structure"]
impl crate::Readable for DSI_WRPCR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dsi_wrpcr::W`](W) writer structure"]
impl crate::Writable for DSI_WRPCR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DSI_WRPCR to value 0"]
impl crate::Resettable for DSI_WRPCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
