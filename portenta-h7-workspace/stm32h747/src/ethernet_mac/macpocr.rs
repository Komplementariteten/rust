#[doc = "Register `MACPOCR` reader"]
pub type R = crate::R<MACPOCR_SPEC>;
#[doc = "Register `MACPOCR` writer"]
pub type W = crate::W<MACPOCR_SPEC>;
#[doc = "Field `PTOEN` reader - PTOEN"]
pub type PTOEN_R = crate::BitReader;
#[doc = "Field `PTOEN` writer - PTOEN"]
pub type PTOEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `ASYNCEN` reader - ASYNCEN"]
pub type ASYNCEN_R = crate::BitReader;
#[doc = "Field `ASYNCEN` writer - ASYNCEN"]
pub type ASYNCEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `APDREQEN` reader - APDREQEN"]
pub type APDREQEN_R = crate::BitReader;
#[doc = "Field `APDREQEN` writer - APDREQEN"]
pub type APDREQEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `ASYNCTRIG` reader - ASYNCTRIG"]
pub type ASYNCTRIG_R = crate::BitReader;
#[doc = "Field `ASYNCTRIG` writer - ASYNCTRIG"]
pub type ASYNCTRIG_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `APDREQTRIG` reader - APDREQTRIG"]
pub type APDREQTRIG_R = crate::BitReader;
#[doc = "Field `APDREQTRIG` writer - APDREQTRIG"]
pub type APDREQTRIG_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `DRRDIS` reader - DRRDIS"]
pub type DRRDIS_R = crate::BitReader;
#[doc = "Field `DRRDIS` writer - DRRDIS"]
pub type DRRDIS_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `DN` reader - DN"]
pub type DN_R = crate::FieldReader;
#[doc = "Field `DN` writer - DN"]
pub type DN_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
impl R {
    #[doc = "Bit 0 - PTOEN"]
    #[inline(always)]
    pub fn ptoen(&self) -> PTOEN_R {
        PTOEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - ASYNCEN"]
    #[inline(always)]
    pub fn asyncen(&self) -> ASYNCEN_R {
        ASYNCEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - APDREQEN"]
    #[inline(always)]
    pub fn apdreqen(&self) -> APDREQEN_R {
        APDREQEN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 4 - ASYNCTRIG"]
    #[inline(always)]
    pub fn asynctrig(&self) -> ASYNCTRIG_R {
        ASYNCTRIG_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - APDREQTRIG"]
    #[inline(always)]
    pub fn apdreqtrig(&self) -> APDREQTRIG_R {
        APDREQTRIG_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - DRRDIS"]
    #[inline(always)]
    pub fn drrdis(&self) -> DRRDIS_R {
        DRRDIS_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bits 8:15 - DN"]
    #[inline(always)]
    pub fn dn(&self) -> DN_R {
        DN_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - PTOEN"]
    #[inline(always)]
    #[must_use]
    pub fn ptoen(&mut self) -> PTOEN_W<MACPOCR_SPEC, 0> {
        PTOEN_W::new(self)
    }
    #[doc = "Bit 1 - ASYNCEN"]
    #[inline(always)]
    #[must_use]
    pub fn asyncen(&mut self) -> ASYNCEN_W<MACPOCR_SPEC, 1> {
        ASYNCEN_W::new(self)
    }
    #[doc = "Bit 2 - APDREQEN"]
    #[inline(always)]
    #[must_use]
    pub fn apdreqen(&mut self) -> APDREQEN_W<MACPOCR_SPEC, 2> {
        APDREQEN_W::new(self)
    }
    #[doc = "Bit 4 - ASYNCTRIG"]
    #[inline(always)]
    #[must_use]
    pub fn asynctrig(&mut self) -> ASYNCTRIG_W<MACPOCR_SPEC, 4> {
        ASYNCTRIG_W::new(self)
    }
    #[doc = "Bit 5 - APDREQTRIG"]
    #[inline(always)]
    #[must_use]
    pub fn apdreqtrig(&mut self) -> APDREQTRIG_W<MACPOCR_SPEC, 5> {
        APDREQTRIG_W::new(self)
    }
    #[doc = "Bit 6 - DRRDIS"]
    #[inline(always)]
    #[must_use]
    pub fn drrdis(&mut self) -> DRRDIS_W<MACPOCR_SPEC, 6> {
        DRRDIS_W::new(self)
    }
    #[doc = "Bits 8:15 - DN"]
    #[inline(always)]
    #[must_use]
    pub fn dn(&mut self) -> DN_W<MACPOCR_SPEC, 8> {
        DN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "PTP Offload control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`macpocr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`macpocr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MACPOCR_SPEC;
impl crate::RegisterSpec for MACPOCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`macpocr::R`](R) reader structure"]
impl crate::Readable for MACPOCR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`macpocr::W`](W) writer structure"]
impl crate::Writable for MACPOCR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MACPOCR to value 0"]
impl crate::Resettable for MACPOCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
