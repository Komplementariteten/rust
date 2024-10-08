#[doc = "Register `EEFDR2` reader"]
pub type R = crate::R<EEFDR2_SPEC>;
#[doc = "Register `EEFDR2` writer"]
pub type W = crate::W<EEFDR2_SPEC>;
#[doc = "Field `EE6LTCH` reader - External Event 6 latch"]
pub type EE6LTCH_R = crate::BitReader;
#[doc = "Field `EE6LTCH` writer - External Event 6 latch"]
pub type EE6LTCH_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `EE6FLTR` reader - External Event 6 filter"]
pub type EE6FLTR_R = crate::FieldReader;
#[doc = "Field `EE6FLTR` writer - External Event 6 filter"]
pub type EE6FLTR_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O>;
#[doc = "Field `EE7LTCH` reader - External Event 7 latch"]
pub type EE7LTCH_R = crate::BitReader;
#[doc = "Field `EE7LTCH` writer - External Event 7 latch"]
pub type EE7LTCH_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `EE7FLTR` reader - External Event 7 filter"]
pub type EE7FLTR_R = crate::FieldReader;
#[doc = "Field `EE7FLTR` writer - External Event 7 filter"]
pub type EE7FLTR_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O>;
#[doc = "Field `EE8LTCH` reader - External Event 8 latch"]
pub type EE8LTCH_R = crate::BitReader;
#[doc = "Field `EE8LTCH` writer - External Event 8 latch"]
pub type EE8LTCH_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `EE8FLTR` reader - External Event 8 filter"]
pub type EE8FLTR_R = crate::FieldReader;
#[doc = "Field `EE8FLTR` writer - External Event 8 filter"]
pub type EE8FLTR_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O>;
#[doc = "Field `EE9LTCH` reader - External Event 9 latch"]
pub type EE9LTCH_R = crate::BitReader;
#[doc = "Field `EE9LTCH` writer - External Event 9 latch"]
pub type EE9LTCH_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `EE9FLTR` reader - External Event 9 filter"]
pub type EE9FLTR_R = crate::FieldReader;
#[doc = "Field `EE9FLTR` writer - External Event 9 filter"]
pub type EE9FLTR_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O>;
#[doc = "Field `EE10LTCH` reader - External Event 10 latch"]
pub type EE10LTCH_R = crate::BitReader;
#[doc = "Field `EE10LTCH` writer - External Event 10 latch"]
pub type EE10LTCH_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `EE10FLTR` reader - External Event 10 filter"]
pub type EE10FLTR_R = crate::FieldReader;
#[doc = "Field `EE10FLTR` writer - External Event 10 filter"]
pub type EE10FLTR_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O>;
impl R {
    #[doc = "Bit 0 - External Event 6 latch"]
    #[inline(always)]
    pub fn ee6ltch(&self) -> EE6LTCH_R {
        EE6LTCH_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:4 - External Event 6 filter"]
    #[inline(always)]
    pub fn ee6fltr(&self) -> EE6FLTR_R {
        EE6FLTR_R::new(((self.bits >> 1) & 0x0f) as u8)
    }
    #[doc = "Bit 6 - External Event 7 latch"]
    #[inline(always)]
    pub fn ee7ltch(&self) -> EE7LTCH_R {
        EE7LTCH_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bits 7:10 - External Event 7 filter"]
    #[inline(always)]
    pub fn ee7fltr(&self) -> EE7FLTR_R {
        EE7FLTR_R::new(((self.bits >> 7) & 0x0f) as u8)
    }
    #[doc = "Bit 12 - External Event 8 latch"]
    #[inline(always)]
    pub fn ee8ltch(&self) -> EE8LTCH_R {
        EE8LTCH_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bits 13:16 - External Event 8 filter"]
    #[inline(always)]
    pub fn ee8fltr(&self) -> EE8FLTR_R {
        EE8FLTR_R::new(((self.bits >> 13) & 0x0f) as u8)
    }
    #[doc = "Bit 18 - External Event 9 latch"]
    #[inline(always)]
    pub fn ee9ltch(&self) -> EE9LTCH_R {
        EE9LTCH_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bits 19:22 - External Event 9 filter"]
    #[inline(always)]
    pub fn ee9fltr(&self) -> EE9FLTR_R {
        EE9FLTR_R::new(((self.bits >> 19) & 0x0f) as u8)
    }
    #[doc = "Bit 24 - External Event 10 latch"]
    #[inline(always)]
    pub fn ee10ltch(&self) -> EE10LTCH_R {
        EE10LTCH_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bits 25:28 - External Event 10 filter"]
    #[inline(always)]
    pub fn ee10fltr(&self) -> EE10FLTR_R {
        EE10FLTR_R::new(((self.bits >> 25) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - External Event 6 latch"]
    #[inline(always)]
    #[must_use]
    pub fn ee6ltch(&mut self) -> EE6LTCH_W<EEFDR2_SPEC, 0> {
        EE6LTCH_W::new(self)
    }
    #[doc = "Bits 1:4 - External Event 6 filter"]
    #[inline(always)]
    #[must_use]
    pub fn ee6fltr(&mut self) -> EE6FLTR_W<EEFDR2_SPEC, 1> {
        EE6FLTR_W::new(self)
    }
    #[doc = "Bit 6 - External Event 7 latch"]
    #[inline(always)]
    #[must_use]
    pub fn ee7ltch(&mut self) -> EE7LTCH_W<EEFDR2_SPEC, 6> {
        EE7LTCH_W::new(self)
    }
    #[doc = "Bits 7:10 - External Event 7 filter"]
    #[inline(always)]
    #[must_use]
    pub fn ee7fltr(&mut self) -> EE7FLTR_W<EEFDR2_SPEC, 7> {
        EE7FLTR_W::new(self)
    }
    #[doc = "Bit 12 - External Event 8 latch"]
    #[inline(always)]
    #[must_use]
    pub fn ee8ltch(&mut self) -> EE8LTCH_W<EEFDR2_SPEC, 12> {
        EE8LTCH_W::new(self)
    }
    #[doc = "Bits 13:16 - External Event 8 filter"]
    #[inline(always)]
    #[must_use]
    pub fn ee8fltr(&mut self) -> EE8FLTR_W<EEFDR2_SPEC, 13> {
        EE8FLTR_W::new(self)
    }
    #[doc = "Bit 18 - External Event 9 latch"]
    #[inline(always)]
    #[must_use]
    pub fn ee9ltch(&mut self) -> EE9LTCH_W<EEFDR2_SPEC, 18> {
        EE9LTCH_W::new(self)
    }
    #[doc = "Bits 19:22 - External Event 9 filter"]
    #[inline(always)]
    #[must_use]
    pub fn ee9fltr(&mut self) -> EE9FLTR_W<EEFDR2_SPEC, 19> {
        EE9FLTR_W::new(self)
    }
    #[doc = "Bit 24 - External Event 10 latch"]
    #[inline(always)]
    #[must_use]
    pub fn ee10ltch(&mut self) -> EE10LTCH_W<EEFDR2_SPEC, 24> {
        EE10LTCH_W::new(self)
    }
    #[doc = "Bits 25:28 - External Event 10 filter"]
    #[inline(always)]
    #[must_use]
    pub fn ee10fltr(&mut self) -> EE10FLTR_W<EEFDR2_SPEC, 25> {
        EE10FLTR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Timerx External Event Filtering Register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`eefdr2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`eefdr2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EEFDR2_SPEC;
impl crate::RegisterSpec for EEFDR2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`eefdr2::R`](R) reader structure"]
impl crate::Readable for EEFDR2_SPEC {}
#[doc = "`write(|w| ..)` method takes [`eefdr2::W`](W) writer structure"]
impl crate::Writable for EEFDR2_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets EEFDR2 to value 0"]
impl crate::Resettable for EEFDR2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
