#[doc = "Register `CCCR` reader"]
pub type R = crate::R<CCCR_SPEC>;
#[doc = "Register `CCCR` writer"]
pub type W = crate::W<CCCR_SPEC>;
#[doc = "Field `NCC` reader - NMOS compensation code"]
pub type NCC_R = crate::FieldReader;
#[doc = "Field `NCC` writer - NMOS compensation code"]
pub type NCC_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O>;
#[doc = "Field `PCC` reader - PMOS compensation code"]
pub type PCC_R = crate::FieldReader;
#[doc = "Field `PCC` writer - PMOS compensation code"]
pub type PCC_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O>;
impl R {
    #[doc = "Bits 0:3 - NMOS compensation code"]
    #[inline(always)]
    pub fn ncc(&self) -> NCC_R {
        NCC_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - PMOS compensation code"]
    #[inline(always)]
    pub fn pcc(&self) -> PCC_R {
        PCC_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - NMOS compensation code"]
    #[inline(always)]
    #[must_use]
    pub fn ncc(&mut self) -> NCC_W<CCCR_SPEC, 0> {
        NCC_W::new(self)
    }
    #[doc = "Bits 4:7 - PMOS compensation code"]
    #[inline(always)]
    #[must_use]
    pub fn pcc(&mut self) -> PCC_W<CCCR_SPEC, 4> {
        PCC_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "SYSCFG compensation cell code register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cccr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cccr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CCCR_SPEC;
impl crate::RegisterSpec for CCCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cccr::R`](R) reader structure"]
impl crate::Readable for CCCR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`cccr::W`](W) writer structure"]
impl crate::Writable for CCCR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CCCR to value 0"]
impl crate::Resettable for CCCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
