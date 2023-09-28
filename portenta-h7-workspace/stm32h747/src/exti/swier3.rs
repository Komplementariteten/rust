#[doc = "Register `SWIER3` reader"]
pub type R = crate::R<SWIER3_SPEC>;
#[doc = "Register `SWIER3` writer"]
pub type W = crate::W<SWIER3_SPEC>;
#[doc = "Field `SWIER82` reader - Software interrupt on line x+64"]
pub type SWIER82_R = crate::BitReader;
#[doc = "Field `SWIER82` writer - Software interrupt on line x+64"]
pub type SWIER82_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SWIER84` reader - Software interrupt on line x+64"]
pub type SWIER84_R = crate::BitReader;
#[doc = "Field `SWIER84` writer - Software interrupt on line x+64"]
pub type SWIER84_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SWIER85` reader - Software interrupt on line x+64"]
pub type SWIER85_R = crate::BitReader;
#[doc = "Field `SWIER85` writer - Software interrupt on line x+64"]
pub type SWIER85_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SWIER86` reader - Software interrupt on line x+64"]
pub type SWIER86_R = crate::BitReader;
#[doc = "Field `SWIER86` writer - Software interrupt on line x+64"]
pub type SWIER86_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 18 - Software interrupt on line x+64"]
    #[inline(always)]
    pub fn swier82(&self) -> SWIER82_R {
        SWIER82_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 20 - Software interrupt on line x+64"]
    #[inline(always)]
    pub fn swier84(&self) -> SWIER84_R {
        SWIER84_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Software interrupt on line x+64"]
    #[inline(always)]
    pub fn swier85(&self) -> SWIER85_R {
        SWIER85_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Software interrupt on line x+64"]
    #[inline(always)]
    pub fn swier86(&self) -> SWIER86_R {
        SWIER86_R::new(((self.bits >> 22) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 18 - Software interrupt on line x+64"]
    #[inline(always)]
    #[must_use]
    pub fn swier82(&mut self) -> SWIER82_W<SWIER3_SPEC, 18> {
        SWIER82_W::new(self)
    }
    #[doc = "Bit 20 - Software interrupt on line x+64"]
    #[inline(always)]
    #[must_use]
    pub fn swier84(&mut self) -> SWIER84_W<SWIER3_SPEC, 20> {
        SWIER84_W::new(self)
    }
    #[doc = "Bit 21 - Software interrupt on line x+64"]
    #[inline(always)]
    #[must_use]
    pub fn swier85(&mut self) -> SWIER85_W<SWIER3_SPEC, 21> {
        SWIER85_W::new(self)
    }
    #[doc = "Bit 22 - Software interrupt on line x+64"]
    #[inline(always)]
    #[must_use]
    pub fn swier86(&mut self) -> SWIER86_W<SWIER3_SPEC, 22> {
        SWIER86_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "EXTI software interrupt event register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swier3::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swier3::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SWIER3_SPEC;
impl crate::RegisterSpec for SWIER3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`swier3::R`](R) reader structure"]
impl crate::Readable for SWIER3_SPEC {}
#[doc = "`write(|w| ..)` method takes [`swier3::W`](W) writer structure"]
impl crate::Writable for SWIER3_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SWIER3 to value 0"]
impl crate::Resettable for SWIER3_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
