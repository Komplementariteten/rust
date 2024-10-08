#[doc = "Register `OTG_HS_HCDMA10` reader"]
pub type R = crate::R<OTG_HS_HCDMA10_SPEC>;
#[doc = "Register `OTG_HS_HCDMA10` writer"]
pub type W = crate::W<OTG_HS_HCDMA10_SPEC>;
#[doc = "Field `DMAADDR` reader - DMA address"]
pub type DMAADDR_R = crate::FieldReader<u32>;
#[doc = "Field `DMAADDR` writer - DMA address"]
pub type DMAADDR_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 32, O, u32>;
impl R {
    #[doc = "Bits 0:31 - DMA address"]
    #[inline(always)]
    pub fn dmaaddr(&self) -> DMAADDR_R {
        DMAADDR_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - DMA address"]
    #[inline(always)]
    #[must_use]
    pub fn dmaaddr(&mut self) -> DMAADDR_W<OTG_HS_HCDMA10_SPEC, 0> {
        DMAADDR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "OTG_HS host channel-10 DMA address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`otg_hs_hcdma10::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`otg_hs_hcdma10::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OTG_HS_HCDMA10_SPEC;
impl crate::RegisterSpec for OTG_HS_HCDMA10_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`otg_hs_hcdma10::R`](R) reader structure"]
impl crate::Readable for OTG_HS_HCDMA10_SPEC {}
#[doc = "`write(|w| ..)` method takes [`otg_hs_hcdma10::W`](W) writer structure"]
impl crate::Writable for OTG_HS_HCDMA10_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets OTG_HS_HCDMA10 to value 0"]
impl crate::Resettable for OTG_HS_HCDMA10_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
