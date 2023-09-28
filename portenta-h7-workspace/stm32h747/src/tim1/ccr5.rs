#[doc = "Register `CCR5` reader"]
pub type R = crate::R<CCR5_SPEC>;
#[doc = "Register `CCR5` writer"]
pub type W = crate::W<CCR5_SPEC>;
#[doc = "Field `CCR5` reader - Capture/Compare 5 value"]
pub type CCR5_R = crate::FieldReader<u16>;
#[doc = "Field `CCR5` writer - Capture/Compare 5 value"]
pub type CCR5_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 16, O, u16>;
#[doc = "Field `GC5C1` reader - Group Channel 5 and Channel 1"]
pub type GC5C1_R = crate::BitReader;
#[doc = "Field `GC5C1` writer - Group Channel 5 and Channel 1"]
pub type GC5C1_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `GC5C2` reader - Group Channel 5 and Channel 2"]
pub type GC5C2_R = crate::BitReader;
#[doc = "Field `GC5C2` writer - Group Channel 5 and Channel 2"]
pub type GC5C2_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `GC5C3` reader - Group Channel 5 and Channel 3"]
pub type GC5C3_R = crate::BitReader;
#[doc = "Field `GC5C3` writer - Group Channel 5 and Channel 3"]
pub type GC5C3_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bits 0:15 - Capture/Compare 5 value"]
    #[inline(always)]
    pub fn ccr5(&self) -> CCR5_R {
        CCR5_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bit 29 - Group Channel 5 and Channel 1"]
    #[inline(always)]
    pub fn gc5c1(&self) -> GC5C1_R {
        GC5C1_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Group Channel 5 and Channel 2"]
    #[inline(always)]
    pub fn gc5c2(&self) -> GC5C2_R {
        GC5C2_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Group Channel 5 and Channel 3"]
    #[inline(always)]
    pub fn gc5c3(&self) -> GC5C3_R {
        GC5C3_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:15 - Capture/Compare 5 value"]
    #[inline(always)]
    #[must_use]
    pub fn ccr5(&mut self) -> CCR5_W<CCR5_SPEC, 0> {
        CCR5_W::new(self)
    }
    #[doc = "Bit 29 - Group Channel 5 and Channel 1"]
    #[inline(always)]
    #[must_use]
    pub fn gc5c1(&mut self) -> GC5C1_W<CCR5_SPEC, 29> {
        GC5C1_W::new(self)
    }
    #[doc = "Bit 30 - Group Channel 5 and Channel 2"]
    #[inline(always)]
    #[must_use]
    pub fn gc5c2(&mut self) -> GC5C2_W<CCR5_SPEC, 30> {
        GC5C2_W::new(self)
    }
    #[doc = "Bit 31 - Group Channel 5 and Channel 3"]
    #[inline(always)]
    #[must_use]
    pub fn gc5c3(&mut self) -> GC5C3_W<CCR5_SPEC, 31> {
        GC5C3_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "capture/compare register 5\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ccr5::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ccr5::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CCR5_SPEC;
impl crate::RegisterSpec for CCR5_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ccr5::R`](R) reader structure"]
impl crate::Readable for CCR5_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ccr5::W`](W) writer structure"]
impl crate::Writable for CCR5_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CCR5 to value 0"]
impl crate::Resettable for CCR5_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
