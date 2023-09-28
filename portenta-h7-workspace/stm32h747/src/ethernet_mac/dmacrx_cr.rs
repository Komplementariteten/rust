#[doc = "Register `DMACRxCR` reader"]
pub type R = crate::R<DMACRX_CR_SPEC>;
#[doc = "Register `DMACRxCR` writer"]
pub type W = crate::W<DMACRX_CR_SPEC>;
#[doc = "Field `SR` reader - Start or Stop Receive Command"]
pub type SR_R = crate::BitReader;
#[doc = "Field `SR` writer - Start or Stop Receive Command"]
pub type SR_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `RBSZ` reader - Receive Buffer size"]
pub type RBSZ_R = crate::FieldReader<u16>;
#[doc = "Field `RBSZ` writer - Receive Buffer size"]
pub type RBSZ_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 14, O, u16>;
#[doc = "Field `RXPBL` reader - RXPBL"]
pub type RXPBL_R = crate::FieldReader;
#[doc = "Field `RXPBL` writer - RXPBL"]
pub type RXPBL_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 6, O>;
#[doc = "Field `RPF` reader - DMA Rx Channel Packet Flush"]
pub type RPF_R = crate::BitReader;
#[doc = "Field `RPF` writer - DMA Rx Channel Packet Flush"]
pub type RPF_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - Start or Stop Receive Command"]
    #[inline(always)]
    pub fn sr(&self) -> SR_R {
        SR_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:14 - Receive Buffer size"]
    #[inline(always)]
    pub fn rbsz(&self) -> RBSZ_R {
        RBSZ_R::new(((self.bits >> 1) & 0x3fff) as u16)
    }
    #[doc = "Bits 16:21 - RXPBL"]
    #[inline(always)]
    pub fn rxpbl(&self) -> RXPBL_R {
        RXPBL_R::new(((self.bits >> 16) & 0x3f) as u8)
    }
    #[doc = "Bit 31 - DMA Rx Channel Packet Flush"]
    #[inline(always)]
    pub fn rpf(&self) -> RPF_R {
        RPF_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Start or Stop Receive Command"]
    #[inline(always)]
    #[must_use]
    pub fn sr(&mut self) -> SR_W<DMACRX_CR_SPEC, 0> {
        SR_W::new(self)
    }
    #[doc = "Bits 1:14 - Receive Buffer size"]
    #[inline(always)]
    #[must_use]
    pub fn rbsz(&mut self) -> RBSZ_W<DMACRX_CR_SPEC, 1> {
        RBSZ_W::new(self)
    }
    #[doc = "Bits 16:21 - RXPBL"]
    #[inline(always)]
    #[must_use]
    pub fn rxpbl(&mut self) -> RXPBL_W<DMACRX_CR_SPEC, 16> {
        RXPBL_W::new(self)
    }
    #[doc = "Bit 31 - DMA Rx Channel Packet Flush"]
    #[inline(always)]
    #[must_use]
    pub fn rpf(&mut self) -> RPF_W<DMACRX_CR_SPEC, 31> {
        RPF_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Channel receive control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dmacrx_cr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dmacrx_cr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DMACRX_CR_SPEC;
impl crate::RegisterSpec for DMACRX_CR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dmacrx_cr::R`](R) reader structure"]
impl crate::Readable for DMACRX_CR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dmacrx_cr::W`](W) writer structure"]
impl crate::Writable for DMACRX_CR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DMACRxCR to value 0"]
impl crate::Resettable for DMACRX_CR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
