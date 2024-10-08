#[doc = "Register `OTG_HS_HPTXFSIZ` reader"]
pub type R = crate::R<OTG_HS_HPTXFSIZ_SPEC>;
#[doc = "Register `OTG_HS_HPTXFSIZ` writer"]
pub type W = crate::W<OTG_HS_HPTXFSIZ_SPEC>;
#[doc = "Field `PTXSA` reader - Host periodic TxFIFO start address"]
pub type PTXSA_R = crate::FieldReader<u16>;
#[doc = "Field `PTXSA` writer - Host periodic TxFIFO start address"]
pub type PTXSA_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 16, O, u16>;
#[doc = "Field `PTXFD` reader - Host periodic TxFIFO depth"]
pub type PTXFD_R = crate::FieldReader<u16>;
#[doc = "Field `PTXFD` writer - Host periodic TxFIFO depth"]
pub type PTXFD_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 16, O, u16>;
impl R {
    #[doc = "Bits 0:15 - Host periodic TxFIFO start address"]
    #[inline(always)]
    pub fn ptxsa(&self) -> PTXSA_R {
        PTXSA_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - Host periodic TxFIFO depth"]
    #[inline(always)]
    pub fn ptxfd(&self) -> PTXFD_R {
        PTXFD_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Host periodic TxFIFO start address"]
    #[inline(always)]
    #[must_use]
    pub fn ptxsa(&mut self) -> PTXSA_W<OTG_HS_HPTXFSIZ_SPEC, 0> {
        PTXSA_W::new(self)
    }
    #[doc = "Bits 16:31 - Host periodic TxFIFO depth"]
    #[inline(always)]
    #[must_use]
    pub fn ptxfd(&mut self) -> PTXFD_W<OTG_HS_HPTXFSIZ_SPEC, 16> {
        PTXFD_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "OTG_HS Host periodic transmit FIFO size register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`otg_hs_hptxfsiz::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`otg_hs_hptxfsiz::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OTG_HS_HPTXFSIZ_SPEC;
impl crate::RegisterSpec for OTG_HS_HPTXFSIZ_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`otg_hs_hptxfsiz::R`](R) reader structure"]
impl crate::Readable for OTG_HS_HPTXFSIZ_SPEC {}
#[doc = "`write(|w| ..)` method takes [`otg_hs_hptxfsiz::W`](W) writer structure"]
impl crate::Writable for OTG_HS_HPTXFSIZ_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets OTG_HS_HPTXFSIZ to value 0x0200_0600"]
impl crate::Resettable for OTG_HS_HPTXFSIZ_SPEC {
    const RESET_VALUE: Self::Ux = 0x0200_0600;
}
