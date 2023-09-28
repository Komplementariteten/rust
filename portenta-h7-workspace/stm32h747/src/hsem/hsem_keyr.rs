#[doc = "Register `HSEM_KEYR` reader"]
pub type R = crate::R<HSEM_KEYR_SPEC>;
#[doc = "Register `HSEM_KEYR` writer"]
pub type W = crate::W<HSEM_KEYR_SPEC>;
#[doc = "Field `KEY` reader - Semaphore Clear Key"]
pub type KEY_R = crate::FieldReader<u16>;
#[doc = "Field `KEY` writer - Semaphore Clear Key"]
pub type KEY_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 16, O, u16>;
impl R {
    #[doc = "Bits 16:31 - Semaphore Clear Key"]
    #[inline(always)]
    pub fn key(&self) -> KEY_R {
        KEY_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 16:31 - Semaphore Clear Key"]
    #[inline(always)]
    #[must_use]
    pub fn key(&mut self) -> KEY_W<HSEM_KEYR_SPEC, 16> {
        KEY_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "HSEM Interrupt clear register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hsem_keyr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hsem_keyr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HSEM_KEYR_SPEC;
impl crate::RegisterSpec for HSEM_KEYR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hsem_keyr::R`](R) reader structure"]
impl crate::Readable for HSEM_KEYR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`hsem_keyr::W`](W) writer structure"]
impl crate::Writable for HSEM_KEYR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets HSEM_KEYR to value 0"]
impl crate::Resettable for HSEM_KEYR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
