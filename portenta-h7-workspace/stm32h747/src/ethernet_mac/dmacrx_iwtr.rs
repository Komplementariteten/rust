#[doc = "Register `DMACRxIWTR` reader"]
pub type R = crate::R<DMACRX_IWTR_SPEC>;
#[doc = "Register `DMACRxIWTR` writer"]
pub type W = crate::W<DMACRX_IWTR_SPEC>;
#[doc = "Field `RWT` reader - Receive Interrupt Watchdog Timer Count"]
pub type RWT_R = crate::FieldReader;
#[doc = "Field `RWT` writer - Receive Interrupt Watchdog Timer Count"]
pub type RWT_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
impl R {
    #[doc = "Bits 0:7 - Receive Interrupt Watchdog Timer Count"]
    #[inline(always)]
    pub fn rwt(&self) -> RWT_R {
        RWT_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Receive Interrupt Watchdog Timer Count"]
    #[inline(always)]
    #[must_use]
    pub fn rwt(&mut self) -> RWT_W<DMACRX_IWTR_SPEC, 0> {
        RWT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Channel Rx interrupt watchdog timer register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dmacrx_iwtr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dmacrx_iwtr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DMACRX_IWTR_SPEC;
impl crate::RegisterSpec for DMACRX_IWTR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dmacrx_iwtr::R`](R) reader structure"]
impl crate::Readable for DMACRX_IWTR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dmacrx_iwtr::W`](W) writer structure"]
impl crate::Writable for DMACRX_IWTR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DMACRxIWTR to value 0"]
impl crate::Resettable for DMACRX_IWTR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
