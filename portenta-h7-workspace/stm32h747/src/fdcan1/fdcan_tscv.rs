#[doc = "Register `FDCAN_TSCV` reader"]
pub type R = crate::R<FDCAN_TSCV_SPEC>;
#[doc = "Register `FDCAN_TSCV` writer"]
pub type W = crate::W<FDCAN_TSCV_SPEC>;
#[doc = "Field `TSC` reader - Timestamp Counter"]
pub type TSC_R = crate::FieldReader<u16>;
#[doc = "Field `TSC` writer - Timestamp Counter"]
pub type TSC_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 16, O, u16>;
impl R {
    #[doc = "Bits 0:15 - Timestamp Counter"]
    #[inline(always)]
    pub fn tsc(&self) -> TSC_R {
        TSC_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Timestamp Counter"]
    #[inline(always)]
    #[must_use]
    pub fn tsc(&mut self) -> TSC_W<FDCAN_TSCV_SPEC, 0> {
        TSC_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "FDCAN Timestamp Counter Value Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fdcan_tscv::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fdcan_tscv::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FDCAN_TSCV_SPEC;
impl crate::RegisterSpec for FDCAN_TSCV_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fdcan_tscv::R`](R) reader structure"]
impl crate::Readable for FDCAN_TSCV_SPEC {}
#[doc = "`write(|w| ..)` method takes [`fdcan_tscv::W`](W) writer structure"]
impl crate::Writable for FDCAN_TSCV_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets FDCAN_TSCV to value 0"]
impl crate::Resettable for FDCAN_TSCV_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
