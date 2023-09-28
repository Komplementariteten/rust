#[doc = "Register `DMACTxDLAR` reader"]
pub type R = crate::R<DMACTX_DLAR_SPEC>;
#[doc = "Register `DMACTxDLAR` writer"]
pub type W = crate::W<DMACTX_DLAR_SPEC>;
#[doc = "Field `TDESLA` reader - Start of Transmit List"]
pub type TDESLA_R = crate::FieldReader<u32>;
#[doc = "Field `TDESLA` writer - Start of Transmit List"]
pub type TDESLA_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 30, O, u32>;
impl R {
    #[doc = "Bits 2:31 - Start of Transmit List"]
    #[inline(always)]
    pub fn tdesla(&self) -> TDESLA_R {
        TDESLA_R::new((self.bits >> 2) & 0x3fff_ffff)
    }
}
impl W {
    #[doc = "Bits 2:31 - Start of Transmit List"]
    #[inline(always)]
    #[must_use]
    pub fn tdesla(&mut self) -> TDESLA_W<DMACTX_DLAR_SPEC, 2> {
        TDESLA_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Channel Tx descriptor list address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dmactx_dlar::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dmactx_dlar::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DMACTX_DLAR_SPEC;
impl crate::RegisterSpec for DMACTX_DLAR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dmactx_dlar::R`](R) reader structure"]
impl crate::Readable for DMACTX_DLAR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dmactx_dlar::W`](W) writer structure"]
impl crate::Writable for DMACTX_DLAR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DMACTxDLAR to value 0"]
impl crate::Resettable for DMACTX_DLAR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
