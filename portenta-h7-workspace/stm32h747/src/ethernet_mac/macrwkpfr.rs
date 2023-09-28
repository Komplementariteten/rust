#[doc = "Register `MACRWKPFR` reader"]
pub type R = crate::R<MACRWKPFR_SPEC>;
#[doc = "Register `MACRWKPFR` writer"]
pub type W = crate::W<MACRWKPFR_SPEC>;
#[doc = "Field `WKUPFRMFTR` reader - WKUPFRMFTR"]
pub type WKUPFRMFTR_R = crate::FieldReader<u32>;
#[doc = "Field `WKUPFRMFTR` writer - WKUPFRMFTR"]
pub type WKUPFRMFTR_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 32, O, u32>;
impl R {
    #[doc = "Bits 0:31 - WKUPFRMFTR"]
    #[inline(always)]
    pub fn wkupfrmftr(&self) -> WKUPFRMFTR_R {
        WKUPFRMFTR_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - WKUPFRMFTR"]
    #[inline(always)]
    #[must_use]
    pub fn wkupfrmftr(&mut self) -> WKUPFRMFTR_W<MACRWKPFR_SPEC, 0> {
        WKUPFRMFTR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Remove wakeup packet filter register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`macrwkpfr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`macrwkpfr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MACRWKPFR_SPEC;
impl crate::RegisterSpec for MACRWKPFR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`macrwkpfr::R`](R) reader structure"]
impl crate::Readable for MACRWKPFR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`macrwkpfr::W`](W) writer structure"]
impl crate::Writable for MACRWKPFR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MACRWKPFR to value 0"]
impl crate::Resettable for MACRWKPFR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
