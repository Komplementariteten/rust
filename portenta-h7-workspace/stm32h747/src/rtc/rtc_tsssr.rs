#[doc = "Register `RTC_TSSSR` reader"]
pub type R = crate::R<RTC_TSSSR_SPEC>;
#[doc = "Field `SS` reader - Sub second value SS\\[15:0\\]
is the value of the synchronous prescaler counter when the timestamp event occurred."]
pub type SS_R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - Sub second value SS\\[15:0\\]
is the value of the synchronous prescaler counter when the timestamp event occurred."]
    #[inline(always)]
    pub fn ss(&self) -> SS_R {
        SS_R::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "The content of this register is valid only when RTC_ISR/TSF is set. It is cleared when the RTC_ISR/TSF bit is reset.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rtc_tsssr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RTC_TSSSR_SPEC;
impl crate::RegisterSpec for RTC_TSSSR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rtc_tsssr::R`](R) reader structure"]
impl crate::Readable for RTC_TSSSR_SPEC {}
#[doc = "`reset()` method sets RTC_TSSSR to value 0"]
impl crate::Resettable for RTC_TSSSR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
