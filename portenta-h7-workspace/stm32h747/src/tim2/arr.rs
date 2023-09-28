#[doc = "Register `ARR` reader"]
pub type R = crate::R<ARR_SPEC>;
#[doc = "Register `ARR` writer"]
pub type W = crate::W<ARR_SPEC>;
#[doc = "Field `ARR_L` reader - Low Auto-reload value"]
pub type ARR_L_R = crate::FieldReader<u16>;
#[doc = "Field `ARR_L` writer - Low Auto-reload value"]
pub type ARR_L_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 16, O, u16>;
#[doc = "Field `ARR_H` reader - High Auto-reload value"]
pub type ARR_H_R = crate::FieldReader<u16>;
#[doc = "Field `ARR_H` writer - High Auto-reload value"]
pub type ARR_H_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 16, O, u16>;
impl R {
    #[doc = "Bits 0:15 - Low Auto-reload value"]
    #[inline(always)]
    pub fn arr_l(&self) -> ARR_L_R {
        ARR_L_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - High Auto-reload value"]
    #[inline(always)]
    pub fn arr_h(&self) -> ARR_H_R {
        ARR_H_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Low Auto-reload value"]
    #[inline(always)]
    #[must_use]
    pub fn arr_l(&mut self) -> ARR_L_W<ARR_SPEC, 0> {
        ARR_L_W::new(self)
    }
    #[doc = "Bits 16:31 - High Auto-reload value"]
    #[inline(always)]
    #[must_use]
    pub fn arr_h(&mut self) -> ARR_H_W<ARR_SPEC, 16> {
        ARR_H_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "auto-reload register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`arr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`arr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ARR_SPEC;
impl crate::RegisterSpec for ARR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`arr::R`](R) reader structure"]
impl crate::Readable for ARR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`arr::W`](W) writer structure"]
impl crate::Writable for ARR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ARR to value 0"]
impl crate::Resettable for ARR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
