#[doc = "Register `FDCAN_TXESC` reader"]
pub type R = crate::R<FDCAN_TXESC_SPEC>;
#[doc = "Register `FDCAN_TXESC` writer"]
pub type W = crate::W<FDCAN_TXESC_SPEC>;
#[doc = "Field `TBDS` reader - Tx Buffer Data Field Size:"]
pub type TBDS_R = crate::FieldReader;
#[doc = "Field `TBDS` writer - Tx Buffer Data Field Size:"]
pub type TBDS_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 3, O>;
impl R {
    #[doc = "Bits 0:2 - Tx Buffer Data Field Size:"]
    #[inline(always)]
    pub fn tbds(&self) -> TBDS_R {
        TBDS_R::new((self.bits & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Tx Buffer Data Field Size:"]
    #[inline(always)]
    #[must_use]
    pub fn tbds(&mut self) -> TBDS_W<FDCAN_TXESC_SPEC, 0> {
        TBDS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "FDCAN Tx Buffer Element Size Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fdcan_txesc::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fdcan_txesc::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FDCAN_TXESC_SPEC;
impl crate::RegisterSpec for FDCAN_TXESC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fdcan_txesc::R`](R) reader structure"]
impl crate::Readable for FDCAN_TXESC_SPEC {}
#[doc = "`write(|w| ..)` method takes [`fdcan_txesc::W`](W) writer structure"]
impl crate::Writable for FDCAN_TXESC_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets FDCAN_TXESC to value 0"]
impl crate::Resettable for FDCAN_TXESC_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
