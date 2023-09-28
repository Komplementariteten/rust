#[doc = "Register `WKUPCR` reader"]
pub type R = crate::R<WKUPCR_SPEC>;
#[doc = "Register `WKUPCR` writer"]
pub type W = crate::W<WKUPCR_SPEC>;
#[doc = "Field `WKUPC` reader - Clear Wakeup pin flag for WKUP. These bits are always read as 0."]
pub type WKUPC_R = crate::FieldReader;
#[doc = "Field `WKUPC` writer - Clear Wakeup pin flag for WKUP. These bits are always read as 0."]
pub type WKUPC_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 6, O>;
impl R {
    #[doc = "Bits 0:5 - Clear Wakeup pin flag for WKUP. These bits are always read as 0."]
    #[inline(always)]
    pub fn wkupc(&self) -> WKUPC_R {
        WKUPC_R::new((self.bits & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - Clear Wakeup pin flag for WKUP. These bits are always read as 0."]
    #[inline(always)]
    #[must_use]
    pub fn wkupc(&mut self) -> WKUPC_W<WKUPCR_SPEC, 0> {
        WKUPC_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "reset only by system reset, not reset by wakeup from Standby mode5 wait states are required when writing this register (when clearing a WKUPF bit in PWR_WKUPFR, the AHB write access will complete after the WKUPF has been cleared).\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wkupcr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wkupcr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WKUPCR_SPEC;
impl crate::RegisterSpec for WKUPCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`wkupcr::R`](R) reader structure"]
impl crate::Readable for WKUPCR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`wkupcr::W`](W) writer structure"]
impl crate::Writable for WKUPCR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets WKUPCR to value 0"]
impl crate::Resettable for WKUPCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
