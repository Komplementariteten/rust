#[doc = "Register `ESUR` reader"]
pub type R = crate::R<ESUR_SPEC>;
#[doc = "Register `ESUR` writer"]
pub type W = crate::W<ESUR_SPEC>;
#[doc = "Field `FSU` reader - Frame start delimiter unmask"]
pub type FSU_R = crate::FieldReader;
#[doc = "Field `FSU` writer - Frame start delimiter unmask"]
pub type FSU_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
#[doc = "Field `LSU` reader - Line start delimiter unmask"]
pub type LSU_R = crate::FieldReader;
#[doc = "Field `LSU` writer - Line start delimiter unmask"]
pub type LSU_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
#[doc = "Field `LEU` reader - Line end delimiter unmask"]
pub type LEU_R = crate::FieldReader;
#[doc = "Field `LEU` writer - Line end delimiter unmask"]
pub type LEU_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
#[doc = "Field `FEU` reader - Frame end delimiter unmask"]
pub type FEU_R = crate::FieldReader;
#[doc = "Field `FEU` writer - Frame end delimiter unmask"]
pub type FEU_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
impl R {
    #[doc = "Bits 0:7 - Frame start delimiter unmask"]
    #[inline(always)]
    pub fn fsu(&self) -> FSU_R {
        FSU_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Line start delimiter unmask"]
    #[inline(always)]
    pub fn lsu(&self) -> LSU_R {
        LSU_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Line end delimiter unmask"]
    #[inline(always)]
    pub fn leu(&self) -> LEU_R {
        LEU_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Frame end delimiter unmask"]
    #[inline(always)]
    pub fn feu(&self) -> FEU_R {
        FEU_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Frame start delimiter unmask"]
    #[inline(always)]
    #[must_use]
    pub fn fsu(&mut self) -> FSU_W<ESUR_SPEC, 0> {
        FSU_W::new(self)
    }
    #[doc = "Bits 8:15 - Line start delimiter unmask"]
    #[inline(always)]
    #[must_use]
    pub fn lsu(&mut self) -> LSU_W<ESUR_SPEC, 8> {
        LSU_W::new(self)
    }
    #[doc = "Bits 16:23 - Line end delimiter unmask"]
    #[inline(always)]
    #[must_use]
    pub fn leu(&mut self) -> LEU_W<ESUR_SPEC, 16> {
        LEU_W::new(self)
    }
    #[doc = "Bits 24:31 - Frame end delimiter unmask"]
    #[inline(always)]
    #[must_use]
    pub fn feu(&mut self) -> FEU_W<ESUR_SPEC, 24> {
        FEU_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "embedded synchronization unmask register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`esur::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`esur::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ESUR_SPEC;
impl crate::RegisterSpec for ESUR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`esur::R`](R) reader structure"]
impl crate::Readable for ESUR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`esur::W`](W) writer structure"]
impl crate::Writable for ESUR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ESUR to value 0"]
impl crate::Resettable for ESUR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
