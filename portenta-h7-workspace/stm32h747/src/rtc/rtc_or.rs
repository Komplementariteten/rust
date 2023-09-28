#[doc = "Register `RTC_OR` reader"]
pub type R = crate::R<RTC_OR_SPEC>;
#[doc = "Register `RTC_OR` writer"]
pub type W = crate::W<RTC_OR_SPEC>;
#[doc = "Field `RTC_ALARM_TYPE` reader - RTC_ALARM output type on PC13"]
pub type RTC_ALARM_TYPE_R = crate::BitReader;
#[doc = "Field `RTC_ALARM_TYPE` writer - RTC_ALARM output type on PC13"]
pub type RTC_ALARM_TYPE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `RTC_OUT_RMP` reader - RTC_OUT remap"]
pub type RTC_OUT_RMP_R = crate::BitReader;
#[doc = "Field `RTC_OUT_RMP` writer - RTC_OUT remap"]
pub type RTC_OUT_RMP_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - RTC_ALARM output type on PC13"]
    #[inline(always)]
    pub fn rtc_alarm_type(&self) -> RTC_ALARM_TYPE_R {
        RTC_ALARM_TYPE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - RTC_OUT remap"]
    #[inline(always)]
    pub fn rtc_out_rmp(&self) -> RTC_OUT_RMP_R {
        RTC_OUT_RMP_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - RTC_ALARM output type on PC13"]
    #[inline(always)]
    #[must_use]
    pub fn rtc_alarm_type(&mut self) -> RTC_ALARM_TYPE_W<RTC_OR_SPEC, 0> {
        RTC_ALARM_TYPE_W::new(self)
    }
    #[doc = "Bit 1 - RTC_OUT remap"]
    #[inline(always)]
    #[must_use]
    pub fn rtc_out_rmp(&mut self) -> RTC_OUT_RMP_W<RTC_OR_SPEC, 1> {
        RTC_OUT_RMP_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "RTC option register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rtc_or::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rtc_or::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RTC_OR_SPEC;
impl crate::RegisterSpec for RTC_OR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rtc_or::R`](R) reader structure"]
impl crate::Readable for RTC_OR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`rtc_or::W`](W) writer structure"]
impl crate::Writable for RTC_OR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RTC_OR to value 0"]
impl crate::Resettable for RTC_OR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
