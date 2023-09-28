#[doc = "Register `FDCAN_TSCC` reader"]
pub type R = crate::R<FDCAN_TSCC_SPEC>;
#[doc = "Register `FDCAN_TSCC` writer"]
pub type W = crate::W<FDCAN_TSCC_SPEC>;
#[doc = "Field `TSS` reader - Timestamp Select"]
pub type TSS_R = crate::FieldReader;
#[doc = "Field `TSS` writer - Timestamp Select"]
pub type TSS_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
#[doc = "Field `TCP` reader - Timestamp Counter Prescaler"]
pub type TCP_R = crate::FieldReader;
#[doc = "Field `TCP` writer - Timestamp Counter Prescaler"]
pub type TCP_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O>;
impl R {
    #[doc = "Bits 0:1 - Timestamp Select"]
    #[inline(always)]
    pub fn tss(&self) -> TSS_R {
        TSS_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 16:19 - Timestamp Counter Prescaler"]
    #[inline(always)]
    pub fn tcp(&self) -> TCP_R {
        TCP_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Timestamp Select"]
    #[inline(always)]
    #[must_use]
    pub fn tss(&mut self) -> TSS_W<FDCAN_TSCC_SPEC, 0> {
        TSS_W::new(self)
    }
    #[doc = "Bits 16:19 - Timestamp Counter Prescaler"]
    #[inline(always)]
    #[must_use]
    pub fn tcp(&mut self) -> TCP_W<FDCAN_TSCC_SPEC, 16> {
        TCP_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "FDCAN Timestamp Counter Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fdcan_tscc::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fdcan_tscc::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FDCAN_TSCC_SPEC;
impl crate::RegisterSpec for FDCAN_TSCC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fdcan_tscc::R`](R) reader structure"]
impl crate::Readable for FDCAN_TSCC_SPEC {}
#[doc = "`write(|w| ..)` method takes [`fdcan_tscc::W`](W) writer structure"]
impl crate::Writable for FDCAN_TSCC_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets FDCAN_TSCC to value 0"]
impl crate::Resettable for FDCAN_TSCC_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
