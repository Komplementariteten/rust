#[doc = "Register `DMACTxDTPR` reader"]
pub type R = crate::R<DMACTX_DTPR_SPEC>;
#[doc = "Register `DMACTxDTPR` writer"]
pub type W = crate::W<DMACTX_DTPR_SPEC>;
#[doc = "Field `TDT` reader - Transmit Descriptor Tail Pointer"]
pub type TDT_R = crate::FieldReader<u32>;
#[doc = "Field `TDT` writer - Transmit Descriptor Tail Pointer"]
pub type TDT_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 30, O, u32>;
impl R {
    #[doc = "Bits 2:31 - Transmit Descriptor Tail Pointer"]
    #[inline(always)]
    pub fn tdt(&self) -> TDT_R {
        TDT_R::new((self.bits >> 2) & 0x3fff_ffff)
    }
}
impl W {
    #[doc = "Bits 2:31 - Transmit Descriptor Tail Pointer"]
    #[inline(always)]
    #[must_use]
    pub fn tdt(&mut self) -> TDT_W<DMACTX_DTPR_SPEC, 2> {
        TDT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Channel Tx descriptor tail pointer register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dmactx_dtpr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dmactx_dtpr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DMACTX_DTPR_SPEC;
impl crate::RegisterSpec for DMACTX_DTPR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dmactx_dtpr::R`](R) reader structure"]
impl crate::Readable for DMACTX_DTPR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dmactx_dtpr::W`](W) writer structure"]
impl crate::Writable for DMACTX_DTPR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DMACTxDTPR to value 0"]
impl crate::Resettable for DMACTX_DTPR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
