#[doc = "Register `AHBPCR` reader"]
pub type R = crate::R<AHBPCR_SPEC>;
#[doc = "Register `AHBPCR` writer"]
pub type W = crate::W<AHBPCR_SPEC>;
#[doc = "Field `EN` reader - EN"]
pub type EN_R = crate::BitReader;
#[doc = "Field `EN` writer - EN"]
pub type EN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SZ` reader - SZ"]
pub type SZ_R = crate::FieldReader;
#[doc = "Field `SZ` writer - SZ"]
pub type SZ_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 3, O>;
impl R {
    #[doc = "Bit 0 - EN"]
    #[inline(always)]
    pub fn en(&self) -> EN_R {
        EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:3 - SZ"]
    #[inline(always)]
    pub fn sz(&self) -> SZ_R {
        SZ_R::new(((self.bits >> 1) & 7) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - EN"]
    #[inline(always)]
    #[must_use]
    pub fn en(&mut self) -> EN_W<AHBPCR_SPEC, 0> {
        EN_W::new(self)
    }
    #[doc = "Bits 1:3 - SZ"]
    #[inline(always)]
    #[must_use]
    pub fn sz(&mut self) -> SZ_W<AHBPCR_SPEC, 1> {
        SZ_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "AHBP Control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ahbpcr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ahbpcr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AHBPCR_SPEC;
impl crate::RegisterSpec for AHBPCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ahbpcr::R`](R) reader structure"]
impl crate::Readable for AHBPCR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ahbpcr::W`](W) writer structure"]
impl crate::Writable for AHBPCR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets AHBPCR to value 0"]
impl crate::Resettable for AHBPCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
