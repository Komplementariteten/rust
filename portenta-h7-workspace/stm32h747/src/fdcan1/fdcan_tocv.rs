#[doc = "Register `FDCAN_TOCV` reader"]
pub type R = crate::R<FDCAN_TOCV_SPEC>;
#[doc = "Register `FDCAN_TOCV` writer"]
pub type W = crate::W<FDCAN_TOCV_SPEC>;
#[doc = "Field `TOC` reader - Timeout Counter"]
pub type TOC_R = crate::FieldReader<u16>;
#[doc = "Field `TOC` writer - Timeout Counter"]
pub type TOC_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 16, O, u16>;
impl R {
    #[doc = "Bits 0:15 - Timeout Counter"]
    #[inline(always)]
    pub fn toc(&self) -> TOC_R {
        TOC_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Timeout Counter"]
    #[inline(always)]
    #[must_use]
    pub fn toc(&mut self) -> TOC_W<FDCAN_TOCV_SPEC, 0> {
        TOC_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "FDCAN Timeout Counter Value Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fdcan_tocv::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fdcan_tocv::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FDCAN_TOCV_SPEC;
impl crate::RegisterSpec for FDCAN_TOCV_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fdcan_tocv::R`](R) reader structure"]
impl crate::Readable for FDCAN_TOCV_SPEC {}
#[doc = "`write(|w| ..)` method takes [`fdcan_tocv::W`](W) writer structure"]
impl crate::Writable for FDCAN_TOCV_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets FDCAN_TOCV to value 0"]
impl crate::Resettable for FDCAN_TOCV_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
