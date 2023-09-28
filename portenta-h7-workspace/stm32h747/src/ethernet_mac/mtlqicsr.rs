#[doc = "Register `MTLQICSR` reader"]
pub type R = crate::R<MTLQICSR_SPEC>;
#[doc = "Register `MTLQICSR` writer"]
pub type W = crate::W<MTLQICSR_SPEC>;
#[doc = "Field `TXUNFIS` reader - TXUNFIS"]
pub type TXUNFIS_R = crate::BitReader;
#[doc = "Field `TXUNFIS` writer - TXUNFIS"]
pub type TXUNFIS_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TXUIE` reader - TXUIE"]
pub type TXUIE_R = crate::BitReader;
#[doc = "Field `TXUIE` writer - TXUIE"]
pub type TXUIE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `RXOVFIS` reader - RXOVFIS"]
pub type RXOVFIS_R = crate::BitReader;
#[doc = "Field `RXOVFIS` writer - RXOVFIS"]
pub type RXOVFIS_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `RXOIE` reader - RXOIE"]
pub type RXOIE_R = crate::BitReader;
#[doc = "Field `RXOIE` writer - RXOIE"]
pub type RXOIE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - TXUNFIS"]
    #[inline(always)]
    pub fn txunfis(&self) -> TXUNFIS_R {
        TXUNFIS_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 8 - TXUIE"]
    #[inline(always)]
    pub fn txuie(&self) -> TXUIE_R {
        TXUIE_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 16 - RXOVFIS"]
    #[inline(always)]
    pub fn rxovfis(&self) -> RXOVFIS_R {
        RXOVFIS_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 24 - RXOIE"]
    #[inline(always)]
    pub fn rxoie(&self) -> RXOIE_R {
        RXOIE_R::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - TXUNFIS"]
    #[inline(always)]
    #[must_use]
    pub fn txunfis(&mut self) -> TXUNFIS_W<MTLQICSR_SPEC, 0> {
        TXUNFIS_W::new(self)
    }
    #[doc = "Bit 8 - TXUIE"]
    #[inline(always)]
    #[must_use]
    pub fn txuie(&mut self) -> TXUIE_W<MTLQICSR_SPEC, 8> {
        TXUIE_W::new(self)
    }
    #[doc = "Bit 16 - RXOVFIS"]
    #[inline(always)]
    #[must_use]
    pub fn rxovfis(&mut self) -> RXOVFIS_W<MTLQICSR_SPEC, 16> {
        RXOVFIS_W::new(self)
    }
    #[doc = "Bit 24 - RXOIE"]
    #[inline(always)]
    #[must_use]
    pub fn rxoie(&mut self) -> RXOIE_W<MTLQICSR_SPEC, 24> {
        RXOIE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Queue interrupt control status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mtlqicsr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mtlqicsr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MTLQICSR_SPEC;
impl crate::RegisterSpec for MTLQICSR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mtlqicsr::R`](R) reader structure"]
impl crate::Readable for MTLQICSR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`mtlqicsr::W`](W) writer structure"]
impl crate::Writable for MTLQICSR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MTLQICSR to value 0"]
impl crate::Resettable for MTLQICSR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
