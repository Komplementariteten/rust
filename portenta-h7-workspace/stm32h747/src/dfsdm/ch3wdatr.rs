#[doc = "Register `CH3WDATR` reader"]
pub type R = crate::R<CH3WDATR_SPEC>;
#[doc = "Register `CH3WDATR` writer"]
pub type W = crate::W<CH3WDATR_SPEC>;
#[doc = "Field `WDATA` reader - WDATA"]
pub type WDATA_R = crate::FieldReader<u16>;
#[doc = "Field `WDATA` writer - WDATA"]
pub type WDATA_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 16, O, u16>;
impl R {
    #[doc = "Bits 0:15 - WDATA"]
    #[inline(always)]
    pub fn wdata(&self) -> WDATA_R {
        WDATA_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - WDATA"]
    #[inline(always)]
    #[must_use]
    pub fn wdata(&mut self) -> WDATA_W<CH3WDATR_SPEC, 0> {
        WDATA_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "CH3WDATR\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch3wdatr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch3wdatr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CH3WDATR_SPEC;
impl crate::RegisterSpec for CH3WDATR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ch3wdatr::R`](R) reader structure"]
impl crate::Readable for CH3WDATR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ch3wdatr::W`](W) writer structure"]
impl crate::Writable for CH3WDATR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CH3WDATR to value 0"]
impl crate::Resettable for CH3WDATR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
