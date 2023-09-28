#[doc = "Register `OPAMP2_HSOTR` reader"]
pub type R = crate::R<OPAMP2_HSOTR_SPEC>;
#[doc = "Register `OPAMP2_HSOTR` writer"]
pub type W = crate::W<OPAMP2_HSOTR_SPEC>;
#[doc = "Field `TRIMLPOFFSETN` reader - Trim for NMOS differential pairs"]
pub type TRIMLPOFFSETN_R = crate::FieldReader;
#[doc = "Field `TRIMLPOFFSETN` writer - Trim for NMOS differential pairs"]
pub type TRIMLPOFFSETN_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 5, O>;
#[doc = "Field `TRIMLPOFFSETP` reader - Trim for PMOS differential pairs"]
pub type TRIMLPOFFSETP_R = crate::FieldReader;
#[doc = "Field `TRIMLPOFFSETP` writer - Trim for PMOS differential pairs"]
pub type TRIMLPOFFSETP_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 5, O>;
impl R {
    #[doc = "Bits 0:4 - Trim for NMOS differential pairs"]
    #[inline(always)]
    pub fn trimlpoffsetn(&self) -> TRIMLPOFFSETN_R {
        TRIMLPOFFSETN_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 8:12 - Trim for PMOS differential pairs"]
    #[inline(always)]
    pub fn trimlpoffsetp(&self) -> TRIMLPOFFSETP_R {
        TRIMLPOFFSETP_R::new(((self.bits >> 8) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - Trim for NMOS differential pairs"]
    #[inline(always)]
    #[must_use]
    pub fn trimlpoffsetn(&mut self) -> TRIMLPOFFSETN_W<OPAMP2_HSOTR_SPEC, 0> {
        TRIMLPOFFSETN_W::new(self)
    }
    #[doc = "Bits 8:12 - Trim for PMOS differential pairs"]
    #[inline(always)]
    #[must_use]
    pub fn trimlpoffsetp(&mut self) -> TRIMLPOFFSETP_W<OPAMP2_HSOTR_SPEC, 8> {
        TRIMLPOFFSETP_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "OPAMP2 offset trimming register in low-power mode\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`opamp2_hsotr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`opamp2_hsotr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OPAMP2_HSOTR_SPEC;
impl crate::RegisterSpec for OPAMP2_HSOTR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`opamp2_hsotr::R`](R) reader structure"]
impl crate::Readable for OPAMP2_HSOTR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`opamp2_hsotr::W`](W) writer structure"]
impl crate::Writable for OPAMP2_HSOTR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets OPAMP2_HSOTR to value 0"]
impl crate::Resettable for OPAMP2_HSOTR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
