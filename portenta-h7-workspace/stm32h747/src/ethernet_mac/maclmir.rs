#[doc = "Register `MACLMIR` reader"]
pub type R = crate::R<MACLMIR_SPEC>;
#[doc = "Register `MACLMIR` writer"]
pub type W = crate::W<MACLMIR_SPEC>;
#[doc = "Field `LSI` reader - LSI"]
pub type LSI_R = crate::FieldReader;
#[doc = "Field `LSI` writer - LSI"]
pub type LSI_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
#[doc = "Field `DRSYNCR` reader - DRSYNCR"]
pub type DRSYNCR_R = crate::FieldReader;
#[doc = "Field `DRSYNCR` writer - DRSYNCR"]
pub type DRSYNCR_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 3, O>;
#[doc = "Field `LMPDRI` reader - LMPDRI"]
pub type LMPDRI_R = crate::FieldReader;
#[doc = "Field `LMPDRI` writer - LMPDRI"]
pub type LMPDRI_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
impl R {
    #[doc = "Bits 0:7 - LSI"]
    #[inline(always)]
    pub fn lsi(&self) -> LSI_R {
        LSI_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:10 - DRSYNCR"]
    #[inline(always)]
    pub fn drsyncr(&self) -> DRSYNCR_R {
        DRSYNCR_R::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bits 24:31 - LMPDRI"]
    #[inline(always)]
    pub fn lmpdri(&self) -> LMPDRI_R {
        LMPDRI_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - LSI"]
    #[inline(always)]
    #[must_use]
    pub fn lsi(&mut self) -> LSI_W<MACLMIR_SPEC, 0> {
        LSI_W::new(self)
    }
    #[doc = "Bits 8:10 - DRSYNCR"]
    #[inline(always)]
    #[must_use]
    pub fn drsyncr(&mut self) -> DRSYNCR_W<MACLMIR_SPEC, 8> {
        DRSYNCR_W::new(self)
    }
    #[doc = "Bits 24:31 - LMPDRI"]
    #[inline(always)]
    #[must_use]
    pub fn lmpdri(&mut self) -> LMPDRI_W<MACLMIR_SPEC, 24> {
        LMPDRI_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Log message interval register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`maclmir::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`maclmir::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MACLMIR_SPEC;
impl crate::RegisterSpec for MACLMIR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`maclmir::R`](R) reader structure"]
impl crate::Readable for MACLMIR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`maclmir::W`](W) writer structure"]
impl crate::Writable for MACLMIR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MACLMIR to value 0"]
impl crate::Resettable for MACLMIR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
