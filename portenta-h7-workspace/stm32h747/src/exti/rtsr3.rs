#[doc = "Register `RTSR3` reader"]
pub type R = crate::R<RTSR3_SPEC>;
#[doc = "Register `RTSR3` writer"]
pub type W = crate::W<RTSR3_SPEC>;
#[doc = "Field `TR82` reader - Rising trigger event configuration bit of Configurable Event input x+64"]
pub type TR82_R = crate::BitReader;
#[doc = "Field `TR82` writer - Rising trigger event configuration bit of Configurable Event input x+64"]
pub type TR82_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TR84` reader - Rising trigger event configuration bit of Configurable Event input x+64"]
pub type TR84_R = crate::BitReader;
#[doc = "Field `TR84` writer - Rising trigger event configuration bit of Configurable Event input x+64"]
pub type TR84_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TR85` reader - Rising trigger event configuration bit of Configurable Event input x+64"]
pub type TR85_R = crate::BitReader;
#[doc = "Field `TR85` writer - Rising trigger event configuration bit of Configurable Event input x+64"]
pub type TR85_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TR86` reader - Rising trigger event configuration bit of Configurable Event input x+64"]
pub type TR86_R = crate::BitReader;
#[doc = "Field `TR86` writer - Rising trigger event configuration bit of Configurable Event input x+64"]
pub type TR86_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 18 - Rising trigger event configuration bit of Configurable Event input x+64"]
    #[inline(always)]
    pub fn tr82(&self) -> TR82_R {
        TR82_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 20 - Rising trigger event configuration bit of Configurable Event input x+64"]
    #[inline(always)]
    pub fn tr84(&self) -> TR84_R {
        TR84_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Rising trigger event configuration bit of Configurable Event input x+64"]
    #[inline(always)]
    pub fn tr85(&self) -> TR85_R {
        TR85_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Rising trigger event configuration bit of Configurable Event input x+64"]
    #[inline(always)]
    pub fn tr86(&self) -> TR86_R {
        TR86_R::new(((self.bits >> 22) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 18 - Rising trigger event configuration bit of Configurable Event input x+64"]
    #[inline(always)]
    #[must_use]
    pub fn tr82(&mut self) -> TR82_W<RTSR3_SPEC, 18> {
        TR82_W::new(self)
    }
    #[doc = "Bit 20 - Rising trigger event configuration bit of Configurable Event input x+64"]
    #[inline(always)]
    #[must_use]
    pub fn tr84(&mut self) -> TR84_W<RTSR3_SPEC, 20> {
        TR84_W::new(self)
    }
    #[doc = "Bit 21 - Rising trigger event configuration bit of Configurable Event input x+64"]
    #[inline(always)]
    #[must_use]
    pub fn tr85(&mut self) -> TR85_W<RTSR3_SPEC, 21> {
        TR85_W::new(self)
    }
    #[doc = "Bit 22 - Rising trigger event configuration bit of Configurable Event input x+64"]
    #[inline(always)]
    #[must_use]
    pub fn tr86(&mut self) -> TR86_W<RTSR3_SPEC, 22> {
        TR86_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "EXTI rising trigger selection register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rtsr3::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rtsr3::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RTSR3_SPEC;
impl crate::RegisterSpec for RTSR3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rtsr3::R`](R) reader structure"]
impl crate::Readable for RTSR3_SPEC {}
#[doc = "`write(|w| ..)` method takes [`rtsr3::W`](W) writer structure"]
impl crate::Writable for RTSR3_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RTSR3 to value 0"]
impl crate::Resettable for RTSR3_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
