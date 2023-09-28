#[doc = "Register `MACQTxFCR` reader"]
pub type R = crate::R<MACQTX_FCR_SPEC>;
#[doc = "Register `MACQTxFCR` writer"]
pub type W = crate::W<MACQTX_FCR_SPEC>;
#[doc = "Field `FCB_BPA` reader - FCB_BPA"]
pub type FCB_BPA_R = crate::BitReader;
#[doc = "Field `FCB_BPA` writer - FCB_BPA"]
pub type FCB_BPA_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TFE` reader - TFE"]
pub type TFE_R = crate::BitReader;
#[doc = "Field `TFE` writer - TFE"]
pub type TFE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PLT` reader - PLT"]
pub type PLT_R = crate::FieldReader;
#[doc = "Field `PLT` writer - PLT"]
pub type PLT_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 3, O>;
#[doc = "Field `DZPQ` reader - DZPQ"]
pub type DZPQ_R = crate::BitReader;
#[doc = "Field `DZPQ` writer - DZPQ"]
pub type DZPQ_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PT` reader - PT"]
pub type PT_R = crate::FieldReader<u16>;
#[doc = "Field `PT` writer - PT"]
pub type PT_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 16, O, u16>;
impl R {
    #[doc = "Bit 0 - FCB_BPA"]
    #[inline(always)]
    pub fn fcb_bpa(&self) -> FCB_BPA_R {
        FCB_BPA_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - TFE"]
    #[inline(always)]
    pub fn tfe(&self) -> TFE_R {
        TFE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 4:6 - PLT"]
    #[inline(always)]
    pub fn plt(&self) -> PLT_R {
        PLT_R::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bit 7 - DZPQ"]
    #[inline(always)]
    pub fn dzpq(&self) -> DZPQ_R {
        DZPQ_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 16:31 - PT"]
    #[inline(always)]
    pub fn pt(&self) -> PT_R {
        PT_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bit 0 - FCB_BPA"]
    #[inline(always)]
    #[must_use]
    pub fn fcb_bpa(&mut self) -> FCB_BPA_W<MACQTX_FCR_SPEC, 0> {
        FCB_BPA_W::new(self)
    }
    #[doc = "Bit 1 - TFE"]
    #[inline(always)]
    #[must_use]
    pub fn tfe(&mut self) -> TFE_W<MACQTX_FCR_SPEC, 1> {
        TFE_W::new(self)
    }
    #[doc = "Bits 4:6 - PLT"]
    #[inline(always)]
    #[must_use]
    pub fn plt(&mut self) -> PLT_W<MACQTX_FCR_SPEC, 4> {
        PLT_W::new(self)
    }
    #[doc = "Bit 7 - DZPQ"]
    #[inline(always)]
    #[must_use]
    pub fn dzpq(&mut self) -> DZPQ_W<MACQTX_FCR_SPEC, 7> {
        DZPQ_W::new(self)
    }
    #[doc = "Bits 16:31 - PT"]
    #[inline(always)]
    #[must_use]
    pub fn pt(&mut self) -> PT_W<MACQTX_FCR_SPEC, 16> {
        PT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Tx Queue flow control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`macqtx_fcr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`macqtx_fcr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MACQTX_FCR_SPEC;
impl crate::RegisterSpec for MACQTX_FCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`macqtx_fcr::R`](R) reader structure"]
impl crate::Readable for MACQTX_FCR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`macqtx_fcr::W`](W) writer structure"]
impl crate::Writable for MACQTX_FCR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MACQTxFCR to value 0"]
impl crate::Resettable for MACQTX_FCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
