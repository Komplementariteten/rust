#[doc = "Register `MACSSIR` reader"]
pub type R = crate::R<MACSSIR_SPEC>;
#[doc = "Register `MACSSIR` writer"]
pub type W = crate::W<MACSSIR_SPEC>;
#[doc = "Field `SNSINC` reader - SNSINC"]
pub type SNSINC_R = crate::FieldReader;
#[doc = "Field `SNSINC` writer - SNSINC"]
pub type SNSINC_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
#[doc = "Field `SSINC` reader - SSINC"]
pub type SSINC_R = crate::FieldReader;
#[doc = "Field `SSINC` writer - SSINC"]
pub type SSINC_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
impl R {
    #[doc = "Bits 8:15 - SNSINC"]
    #[inline(always)]
    pub fn snsinc(&self) -> SNSINC_R {
        SNSINC_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - SSINC"]
    #[inline(always)]
    pub fn ssinc(&self) -> SSINC_R {
        SSINC_R::new(((self.bits >> 16) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 8:15 - SNSINC"]
    #[inline(always)]
    #[must_use]
    pub fn snsinc(&mut self) -> SNSINC_W<MACSSIR_SPEC, 8> {
        SNSINC_W::new(self)
    }
    #[doc = "Bits 16:23 - SSINC"]
    #[inline(always)]
    #[must_use]
    pub fn ssinc(&mut self) -> SSINC_W<MACSSIR_SPEC, 16> {
        SSINC_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Sub-second increment register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`macssir::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`macssir::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MACSSIR_SPEC;
impl crate::RegisterSpec for MACSSIR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`macssir::R`](R) reader structure"]
impl crate::Readable for MACSSIR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`macssir::W`](W) writer structure"]
impl crate::Writable for MACSSIR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MACSSIR to value 0"]
impl crate::Resettable for MACSSIR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
