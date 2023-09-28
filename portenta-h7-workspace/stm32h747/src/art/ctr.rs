#[doc = "Register `CTR` reader"]
pub type R = crate::R<CTR_SPEC>;
#[doc = "Register `CTR` writer"]
pub type W = crate::W<CTR_SPEC>;
#[doc = "Field `EN` reader - Cache enable"]
pub type EN_R = crate::BitReader;
#[doc = "Field `EN` writer - Cache enable"]
pub type EN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PCACHEADDR` reader - Cacheable page index"]
pub type PCACHEADDR_R = crate::FieldReader<u16>;
#[doc = "Field `PCACHEADDR` writer - Cacheable page index"]
pub type PCACHEADDR_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 12, O, u16>;
impl R {
    #[doc = "Bit 0 - Cache enable"]
    #[inline(always)]
    pub fn en(&self) -> EN_R {
        EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 8:19 - Cacheable page index"]
    #[inline(always)]
    pub fn pcacheaddr(&self) -> PCACHEADDR_R {
        PCACHEADDR_R::new(((self.bits >> 8) & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bit 0 - Cache enable"]
    #[inline(always)]
    #[must_use]
    pub fn en(&mut self) -> EN_W<CTR_SPEC, 0> {
        EN_W::new(self)
    }
    #[doc = "Bits 8:19 - Cacheable page index"]
    #[inline(always)]
    #[must_use]
    pub fn pcacheaddr(&mut self) -> PCACHEADDR_W<CTR_SPEC, 8> {
        PCACHEADDR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CTR_SPEC;
impl crate::RegisterSpec for CTR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctr::R`](R) reader structure"]
impl crate::Readable for CTR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ctr::W`](W) writer structure"]
impl crate::Writable for CTR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CTR to value 0x04"]
impl crate::Resettable for CTR_SPEC {
    const RESET_VALUE: Self::Ux = 0x04;
}
