#[doc = "Register `MACRxFCR` reader"]
pub type R = crate::R<MACRX_FCR_SPEC>;
#[doc = "Register `MACRxFCR` writer"]
pub type W = crate::W<MACRX_FCR_SPEC>;
#[doc = "Field `RFE` reader - RFE"]
pub type RFE_R = crate::BitReader;
#[doc = "Field `RFE` writer - RFE"]
pub type RFE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `UP` reader - UP"]
pub type UP_R = crate::BitReader;
#[doc = "Field `UP` writer - UP"]
pub type UP_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - RFE"]
    #[inline(always)]
    pub fn rfe(&self) -> RFE_R {
        RFE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - UP"]
    #[inline(always)]
    pub fn up(&self) -> UP_R {
        UP_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - RFE"]
    #[inline(always)]
    #[must_use]
    pub fn rfe(&mut self) -> RFE_W<MACRX_FCR_SPEC, 0> {
        RFE_W::new(self)
    }
    #[doc = "Bit 1 - UP"]
    #[inline(always)]
    #[must_use]
    pub fn up(&mut self) -> UP_W<MACRX_FCR_SPEC, 1> {
        UP_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Rx flow control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`macrx_fcr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`macrx_fcr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MACRX_FCR_SPEC;
impl crate::RegisterSpec for MACRX_FCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`macrx_fcr::R`](R) reader structure"]
impl crate::Readable for MACRX_FCR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`macrx_fcr::W`](W) writer structure"]
impl crate::Writable for MACRX_FCR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MACRxFCR to value 0"]
impl crate::Resettable for MACRX_FCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
