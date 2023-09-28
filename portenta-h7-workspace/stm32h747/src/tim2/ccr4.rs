#[doc = "Register `CCR4` reader"]
pub type R = crate::R<CCR4_SPEC>;
#[doc = "Register `CCR4` writer"]
pub type W = crate::W<CCR4_SPEC>;
#[doc = "Field `CCR4_L` reader - Low Capture/Compare value"]
pub type CCR4_L_R = crate::FieldReader<u16>;
#[doc = "Field `CCR4_L` writer - Low Capture/Compare value"]
pub type CCR4_L_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 16, O, u16>;
#[doc = "Field `CCR4_H` reader - High Capture/Compare value"]
pub type CCR4_H_R = crate::FieldReader<u16>;
#[doc = "Field `CCR4_H` writer - High Capture/Compare value"]
pub type CCR4_H_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 16, O, u16>;
impl R {
    #[doc = "Bits 0:15 - Low Capture/Compare value"]
    #[inline(always)]
    pub fn ccr4_l(&self) -> CCR4_L_R {
        CCR4_L_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - High Capture/Compare value"]
    #[inline(always)]
    pub fn ccr4_h(&self) -> CCR4_H_R {
        CCR4_H_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Low Capture/Compare value"]
    #[inline(always)]
    #[must_use]
    pub fn ccr4_l(&mut self) -> CCR4_L_W<CCR4_SPEC, 0> {
        CCR4_L_W::new(self)
    }
    #[doc = "Bits 16:31 - High Capture/Compare value"]
    #[inline(always)]
    #[must_use]
    pub fn ccr4_h(&mut self) -> CCR4_H_W<CCR4_SPEC, 16> {
        CCR4_H_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "capture/compare register 4\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ccr4::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ccr4::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CCR4_SPEC;
impl crate::RegisterSpec for CCR4_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ccr4::R`](R) reader structure"]
impl crate::Readable for CCR4_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ccr4::W`](W) writer structure"]
impl crate::Writable for CCR4_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CCR4 to value 0"]
impl crate::Resettable for CCR4_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
