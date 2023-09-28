#[doc = "Register `SHHR` reader"]
pub type R = crate::R<SHHR_SPEC>;
#[doc = "Register `SHHR` writer"]
pub type W = crate::W<SHHR_SPEC>;
#[doc = "Field `THOLD1` reader - DAC Channel 1 hold Time (only valid in sample &amp;amp; hold mode) Hold time= (THOLD\\[9:0\\]) x T LSI"]
pub type THOLD1_R = crate::FieldReader<u16>;
#[doc = "Field `THOLD1` writer - DAC Channel 1 hold Time (only valid in sample &amp;amp; hold mode) Hold time= (THOLD\\[9:0\\]) x T LSI"]
pub type THOLD1_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 10, O, u16>;
#[doc = "Field `THOLD2` reader - DAC Channel 2 hold time (only valid in sample &amp;amp; hold mode). Hold time= (THOLD\\[9:0\\]) x T LSI"]
pub type THOLD2_R = crate::FieldReader<u16>;
#[doc = "Field `THOLD2` writer - DAC Channel 2 hold time (only valid in sample &amp;amp; hold mode). Hold time= (THOLD\\[9:0\\]) x T LSI"]
pub type THOLD2_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 10, O, u16>;
impl R {
    #[doc = "Bits 0:9 - DAC Channel 1 hold Time (only valid in sample &amp;amp; hold mode) Hold time= (THOLD\\[9:0\\]) x T LSI"]
    #[inline(always)]
    pub fn thold1(&self) -> THOLD1_R {
        THOLD1_R::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bits 16:25 - DAC Channel 2 hold time (only valid in sample &amp;amp; hold mode). Hold time= (THOLD\\[9:0\\]) x T LSI"]
    #[inline(always)]
    pub fn thold2(&self) -> THOLD2_R {
        THOLD2_R::new(((self.bits >> 16) & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:9 - DAC Channel 1 hold Time (only valid in sample &amp;amp; hold mode) Hold time= (THOLD\\[9:0\\]) x T LSI"]
    #[inline(always)]
    #[must_use]
    pub fn thold1(&mut self) -> THOLD1_W<SHHR_SPEC, 0> {
        THOLD1_W::new(self)
    }
    #[doc = "Bits 16:25 - DAC Channel 2 hold time (only valid in sample &amp;amp; hold mode). Hold time= (THOLD\\[9:0\\]) x T LSI"]
    #[inline(always)]
    #[must_use]
    pub fn thold2(&mut self) -> THOLD2_W<SHHR_SPEC, 16> {
        THOLD2_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "DAC Sample and Hold hold time register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`shhr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`shhr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SHHR_SPEC;
impl crate::RegisterSpec for SHHR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`shhr::R`](R) reader structure"]
impl crate::Readable for SHHR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`shhr::W`](W) writer structure"]
impl crate::Writable for SHHR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SHHR to value 0x0001_0001"]
impl crate::Resettable for SHHR_SPEC {
    const RESET_VALUE: Self::Ux = 0x0001_0001;
}
