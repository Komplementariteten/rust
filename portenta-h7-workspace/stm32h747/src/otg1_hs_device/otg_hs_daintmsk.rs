#[doc = "Register `OTG_HS_DAINTMSK` reader"]
pub type R = crate::R<OTG_HS_DAINTMSK_SPEC>;
#[doc = "Register `OTG_HS_DAINTMSK` writer"]
pub type W = crate::W<OTG_HS_DAINTMSK_SPEC>;
#[doc = "Field `IEPM` reader - IN EP interrupt mask bits"]
pub type IEPM_R = crate::FieldReader<u16>;
#[doc = "Field `IEPM` writer - IN EP interrupt mask bits"]
pub type IEPM_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 16, O, u16>;
#[doc = "Field `OEPM` reader - OUT EP interrupt mask bits"]
pub type OEPM_R = crate::FieldReader<u16>;
#[doc = "Field `OEPM` writer - OUT EP interrupt mask bits"]
pub type OEPM_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 16, O, u16>;
impl R {
    #[doc = "Bits 0:15 - IN EP interrupt mask bits"]
    #[inline(always)]
    pub fn iepm(&self) -> IEPM_R {
        IEPM_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - OUT EP interrupt mask bits"]
    #[inline(always)]
    pub fn oepm(&self) -> OEPM_R {
        OEPM_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - IN EP interrupt mask bits"]
    #[inline(always)]
    #[must_use]
    pub fn iepm(&mut self) -> IEPM_W<OTG_HS_DAINTMSK_SPEC, 0> {
        IEPM_W::new(self)
    }
    #[doc = "Bits 16:31 - OUT EP interrupt mask bits"]
    #[inline(always)]
    #[must_use]
    pub fn oepm(&mut self) -> OEPM_W<OTG_HS_DAINTMSK_SPEC, 16> {
        OEPM_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "OTG_HS all endpoints interrupt mask register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`otg_hs_daintmsk::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`otg_hs_daintmsk::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OTG_HS_DAINTMSK_SPEC;
impl crate::RegisterSpec for OTG_HS_DAINTMSK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`otg_hs_daintmsk::R`](R) reader structure"]
impl crate::Readable for OTG_HS_DAINTMSK_SPEC {}
#[doc = "`write(|w| ..)` method takes [`otg_hs_daintmsk::W`](W) writer structure"]
impl crate::Writable for OTG_HS_DAINTMSK_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets OTG_HS_DAINTMSK to value 0"]
impl crate::Resettable for OTG_HS_DAINTMSK_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
