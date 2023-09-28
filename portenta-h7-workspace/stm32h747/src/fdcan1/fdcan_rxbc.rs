#[doc = "Register `FDCAN_RXBC` reader"]
pub type R = crate::R<FDCAN_RXBC_SPEC>;
#[doc = "Register `FDCAN_RXBC` writer"]
pub type W = crate::W<FDCAN_RXBC_SPEC>;
#[doc = "Field `RBSA` reader - Rx Buffer Start Address"]
pub type RBSA_R = crate::FieldReader<u16>;
#[doc = "Field `RBSA` writer - Rx Buffer Start Address"]
pub type RBSA_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 14, O, u16>;
impl R {
    #[doc = "Bits 2:15 - Rx Buffer Start Address"]
    #[inline(always)]
    pub fn rbsa(&self) -> RBSA_R {
        RBSA_R::new(((self.bits >> 2) & 0x3fff) as u16)
    }
}
impl W {
    #[doc = "Bits 2:15 - Rx Buffer Start Address"]
    #[inline(always)]
    #[must_use]
    pub fn rbsa(&mut self) -> RBSA_W<FDCAN_RXBC_SPEC, 2> {
        RBSA_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "FDCAN Rx Buffer Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fdcan_rxbc::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fdcan_rxbc::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FDCAN_RXBC_SPEC;
impl crate::RegisterSpec for FDCAN_RXBC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fdcan_rxbc::R`](R) reader structure"]
impl crate::Readable for FDCAN_RXBC_SPEC {}
#[doc = "`write(|w| ..)` method takes [`fdcan_rxbc::W`](W) writer structure"]
impl crate::Writable for FDCAN_RXBC_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets FDCAN_RXBC to value 0"]
impl crate::Resettable for FDCAN_RXBC_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
