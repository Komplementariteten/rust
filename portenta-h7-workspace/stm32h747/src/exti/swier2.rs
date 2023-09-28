#[doc = "Register `SWIER2` reader"]
pub type R = crate::R<SWIER2_SPEC>;
#[doc = "Register `SWIER2` writer"]
pub type W = crate::W<SWIER2_SPEC>;
#[doc = "Field `SWIER49` reader - Software interrupt on line x+32"]
pub type SWIER49_R = crate::BitReader;
#[doc = "Field `SWIER49` writer - Software interrupt on line x+32"]
pub type SWIER49_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SWIER51` reader - Software interrupt on line x+32"]
pub type SWIER51_R = crate::BitReader;
#[doc = "Field `SWIER51` writer - Software interrupt on line x+32"]
pub type SWIER51_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 17 - Software interrupt on line x+32"]
    #[inline(always)]
    pub fn swier49(&self) -> SWIER49_R {
        SWIER49_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 19 - Software interrupt on line x+32"]
    #[inline(always)]
    pub fn swier51(&self) -> SWIER51_R {
        SWIER51_R::new(((self.bits >> 19) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 17 - Software interrupt on line x+32"]
    #[inline(always)]
    #[must_use]
    pub fn swier49(&mut self) -> SWIER49_W<SWIER2_SPEC, 17> {
        SWIER49_W::new(self)
    }
    #[doc = "Bit 19 - Software interrupt on line x+32"]
    #[inline(always)]
    #[must_use]
    pub fn swier51(&mut self) -> SWIER51_W<SWIER2_SPEC, 19> {
        SWIER51_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "EXTI software interrupt event register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swier2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swier2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SWIER2_SPEC;
impl crate::RegisterSpec for SWIER2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`swier2::R`](R) reader structure"]
impl crate::Readable for SWIER2_SPEC {}
#[doc = "`write(|w| ..)` method takes [`swier2::W`](W) writer structure"]
impl crate::Writable for SWIER2_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SWIER2 to value 0"]
impl crate::Resettable for SWIER2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
