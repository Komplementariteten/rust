#[doc = "Register `PIR` reader"]
pub type R = crate::R<PIR_SPEC>;
#[doc = "Register `PIR` writer"]
pub type W = crate::W<PIR_SPEC>;
#[doc = "Field `INTERVAL` reader - Polling interval Number of CLK cycles between to read during automatic polling phases. This field can be written only when BUSY = 0."]
pub type INTERVAL_R = crate::FieldReader<u16>;
#[doc = "Field `INTERVAL` writer - Polling interval Number of CLK cycles between to read during automatic polling phases. This field can be written only when BUSY = 0."]
pub type INTERVAL_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 16, O, u16>;
impl R {
    #[doc = "Bits 0:15 - Polling interval Number of CLK cycles between to read during automatic polling phases. This field can be written only when BUSY = 0."]
    #[inline(always)]
    pub fn interval(&self) -> INTERVAL_R {
        INTERVAL_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Polling interval Number of CLK cycles between to read during automatic polling phases. This field can be written only when BUSY = 0."]
    #[inline(always)]
    #[must_use]
    pub fn interval(&mut self) -> INTERVAL_W<PIR_SPEC, 0> {
        INTERVAL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "QUADSPI polling interval register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pir::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pir::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PIR_SPEC;
impl crate::RegisterSpec for PIR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pir::R`](R) reader structure"]
impl crate::Readable for PIR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`pir::W`](W) writer structure"]
impl crate::Writable for PIR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PIR to value 0"]
impl crate::Resettable for PIR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
