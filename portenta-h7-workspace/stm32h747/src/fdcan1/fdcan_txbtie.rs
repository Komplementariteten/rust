#[doc = "Register `FDCAN_TXBTIE` reader"]
pub type R = crate::R<FDCAN_TXBTIE_SPEC>;
#[doc = "Register `FDCAN_TXBTIE` writer"]
pub type W = crate::W<FDCAN_TXBTIE_SPEC>;
#[doc = "Field `TIE` reader - Transmission Interrupt Enable"]
pub type TIE_R = crate::FieldReader<u32>;
#[doc = "Field `TIE` writer - Transmission Interrupt Enable"]
pub type TIE_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 32, O, u32>;
impl R {
    #[doc = "Bits 0:31 - Transmission Interrupt Enable"]
    #[inline(always)]
    pub fn tie(&self) -> TIE_R {
        TIE_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Transmission Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn tie(&mut self) -> TIE_W<FDCAN_TXBTIE_SPEC, 0> {
        TIE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "FDCAN Tx Buffer Transmission Interrupt Enable Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fdcan_txbtie::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fdcan_txbtie::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FDCAN_TXBTIE_SPEC;
impl crate::RegisterSpec for FDCAN_TXBTIE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fdcan_txbtie::R`](R) reader structure"]
impl crate::Readable for FDCAN_TXBTIE_SPEC {}
#[doc = "`write(|w| ..)` method takes [`fdcan_txbtie::W`](W) writer structure"]
impl crate::Writable for FDCAN_TXBTIE_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets FDCAN_TXBTIE to value 0"]
impl crate::Resettable for FDCAN_TXBTIE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
