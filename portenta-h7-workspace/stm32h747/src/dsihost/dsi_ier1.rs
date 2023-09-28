#[doc = "Register `DSI_IER1` reader"]
pub type R = crate::R<DSI_IER1_SPEC>;
#[doc = "Register `DSI_IER1` writer"]
pub type W = crate::W<DSI_IER1_SPEC>;
#[doc = "Field `TOHSTXIE` reader - TOHSTXIE"]
pub type TOHSTXIE_R = crate::BitReader;
#[doc = "Field `TOHSTXIE` writer - TOHSTXIE"]
pub type TOHSTXIE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TOLPRXIE` reader - TOLPRXIE"]
pub type TOLPRXIE_R = crate::BitReader;
#[doc = "Field `TOLPRXIE` writer - TOLPRXIE"]
pub type TOLPRXIE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `ECCSEIE` reader - ECCSEIE"]
pub type ECCSEIE_R = crate::BitReader;
#[doc = "Field `ECCSEIE` writer - ECCSEIE"]
pub type ECCSEIE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `ECCMEIE` reader - ECCMEIE"]
pub type ECCMEIE_R = crate::BitReader;
#[doc = "Field `ECCMEIE` writer - ECCMEIE"]
pub type ECCMEIE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CRCEIE` reader - CRCEIE"]
pub type CRCEIE_R = crate::BitReader;
#[doc = "Field `CRCEIE` writer - CRCEIE"]
pub type CRCEIE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PSEIE` reader - PSEIE"]
pub type PSEIE_R = crate::BitReader;
#[doc = "Field `PSEIE` writer - PSEIE"]
pub type PSEIE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `EOTPEIE` reader - EOTPEIE"]
pub type EOTPEIE_R = crate::BitReader;
#[doc = "Field `EOTPEIE` writer - EOTPEIE"]
pub type EOTPEIE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `LPWREIE` reader - LPWREIE"]
pub type LPWREIE_R = crate::BitReader;
#[doc = "Field `LPWREIE` writer - LPWREIE"]
pub type LPWREIE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `GCWREIE` reader - GCWREIE"]
pub type GCWREIE_R = crate::BitReader;
#[doc = "Field `GCWREIE` writer - GCWREIE"]
pub type GCWREIE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `GPWREIE` reader - GPWREIE"]
pub type GPWREIE_R = crate::BitReader;
#[doc = "Field `GPWREIE` writer - GPWREIE"]
pub type GPWREIE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `GPTXEIE` reader - GPTXEIE"]
pub type GPTXEIE_R = crate::BitReader;
#[doc = "Field `GPTXEIE` writer - GPTXEIE"]
pub type GPTXEIE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `GPRDEIE` reader - GPRDEIE"]
pub type GPRDEIE_R = crate::BitReader;
#[doc = "Field `GPRDEIE` writer - GPRDEIE"]
pub type GPRDEIE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `GPRXEIE` reader - GPRXEIE"]
pub type GPRXEIE_R = crate::BitReader;
#[doc = "Field `GPRXEIE` writer - GPRXEIE"]
pub type GPRXEIE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - TOHSTXIE"]
    #[inline(always)]
    pub fn tohstxie(&self) -> TOHSTXIE_R {
        TOHSTXIE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - TOLPRXIE"]
    #[inline(always)]
    pub fn tolprxie(&self) -> TOLPRXIE_R {
        TOLPRXIE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - ECCSEIE"]
    #[inline(always)]
    pub fn eccseie(&self) -> ECCSEIE_R {
        ECCSEIE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - ECCMEIE"]
    #[inline(always)]
    pub fn eccmeie(&self) -> ECCMEIE_R {
        ECCMEIE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - CRCEIE"]
    #[inline(always)]
    pub fn crceie(&self) -> CRCEIE_R {
        CRCEIE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - PSEIE"]
    #[inline(always)]
    pub fn pseie(&self) -> PSEIE_R {
        PSEIE_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - EOTPEIE"]
    #[inline(always)]
    pub fn eotpeie(&self) -> EOTPEIE_R {
        EOTPEIE_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - LPWREIE"]
    #[inline(always)]
    pub fn lpwreie(&self) -> LPWREIE_R {
        LPWREIE_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - GCWREIE"]
    #[inline(always)]
    pub fn gcwreie(&self) -> GCWREIE_R {
        GCWREIE_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - GPWREIE"]
    #[inline(always)]
    pub fn gpwreie(&self) -> GPWREIE_R {
        GPWREIE_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - GPTXEIE"]
    #[inline(always)]
    pub fn gptxeie(&self) -> GPTXEIE_R {
        GPTXEIE_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - GPRDEIE"]
    #[inline(always)]
    pub fn gprdeie(&self) -> GPRDEIE_R {
        GPRDEIE_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - GPRXEIE"]
    #[inline(always)]
    pub fn gprxeie(&self) -> GPRXEIE_R {
        GPRXEIE_R::new(((self.bits >> 12) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - TOHSTXIE"]
    #[inline(always)]
    #[must_use]
    pub fn tohstxie(&mut self) -> TOHSTXIE_W<DSI_IER1_SPEC, 0> {
        TOHSTXIE_W::new(self)
    }
    #[doc = "Bit 1 - TOLPRXIE"]
    #[inline(always)]
    #[must_use]
    pub fn tolprxie(&mut self) -> TOLPRXIE_W<DSI_IER1_SPEC, 1> {
        TOLPRXIE_W::new(self)
    }
    #[doc = "Bit 2 - ECCSEIE"]
    #[inline(always)]
    #[must_use]
    pub fn eccseie(&mut self) -> ECCSEIE_W<DSI_IER1_SPEC, 2> {
        ECCSEIE_W::new(self)
    }
    #[doc = "Bit 3 - ECCMEIE"]
    #[inline(always)]
    #[must_use]
    pub fn eccmeie(&mut self) -> ECCMEIE_W<DSI_IER1_SPEC, 3> {
        ECCMEIE_W::new(self)
    }
    #[doc = "Bit 4 - CRCEIE"]
    #[inline(always)]
    #[must_use]
    pub fn crceie(&mut self) -> CRCEIE_W<DSI_IER1_SPEC, 4> {
        CRCEIE_W::new(self)
    }
    #[doc = "Bit 5 - PSEIE"]
    #[inline(always)]
    #[must_use]
    pub fn pseie(&mut self) -> PSEIE_W<DSI_IER1_SPEC, 5> {
        PSEIE_W::new(self)
    }
    #[doc = "Bit 6 - EOTPEIE"]
    #[inline(always)]
    #[must_use]
    pub fn eotpeie(&mut self) -> EOTPEIE_W<DSI_IER1_SPEC, 6> {
        EOTPEIE_W::new(self)
    }
    #[doc = "Bit 7 - LPWREIE"]
    #[inline(always)]
    #[must_use]
    pub fn lpwreie(&mut self) -> LPWREIE_W<DSI_IER1_SPEC, 7> {
        LPWREIE_W::new(self)
    }
    #[doc = "Bit 8 - GCWREIE"]
    #[inline(always)]
    #[must_use]
    pub fn gcwreie(&mut self) -> GCWREIE_W<DSI_IER1_SPEC, 8> {
        GCWREIE_W::new(self)
    }
    #[doc = "Bit 9 - GPWREIE"]
    #[inline(always)]
    #[must_use]
    pub fn gpwreie(&mut self) -> GPWREIE_W<DSI_IER1_SPEC, 9> {
        GPWREIE_W::new(self)
    }
    #[doc = "Bit 10 - GPTXEIE"]
    #[inline(always)]
    #[must_use]
    pub fn gptxeie(&mut self) -> GPTXEIE_W<DSI_IER1_SPEC, 10> {
        GPTXEIE_W::new(self)
    }
    #[doc = "Bit 11 - GPRDEIE"]
    #[inline(always)]
    #[must_use]
    pub fn gprdeie(&mut self) -> GPRDEIE_W<DSI_IER1_SPEC, 11> {
        GPRDEIE_W::new(self)
    }
    #[doc = "Bit 12 - GPRXEIE"]
    #[inline(always)]
    #[must_use]
    pub fn gprxeie(&mut self) -> GPRXEIE_W<DSI_IER1_SPEC, 12> {
        GPRXEIE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "DSI Host interrupt enable register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dsi_ier1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dsi_ier1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DSI_IER1_SPEC;
impl crate::RegisterSpec for DSI_IER1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dsi_ier1::R`](R) reader structure"]
impl crate::Readable for DSI_IER1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dsi_ier1::W`](W) writer structure"]
impl crate::Writable for DSI_IER1_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DSI_IER1 to value 0"]
impl crate::Resettable for DSI_IER1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
