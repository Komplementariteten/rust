#[doc = "Register `DFSDM_FLT0JDATAR` reader"]
pub type R = crate::R<DFSDM_FLT0JDATAR_SPEC>;
#[doc = "Field `JDATACH` reader - Injected channel most recently converted"]
pub type JDATACH_R = crate::FieldReader;
#[doc = "Field `JDATA` reader - Injected group conversion data"]
pub type JDATA_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:2 - Injected channel most recently converted"]
    #[inline(always)]
    pub fn jdatach(&self) -> JDATACH_R {
        JDATACH_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 8:31 - Injected group conversion data"]
    #[inline(always)]
    pub fn jdata(&self) -> JDATA_R {
        JDATA_R::new((self.bits >> 8) & 0x00ff_ffff)
    }
}
#[doc = "data register for injected group\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dfsdm_flt0jdatar::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DFSDM_FLT0JDATAR_SPEC;
impl crate::RegisterSpec for DFSDM_FLT0JDATAR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dfsdm_flt0jdatar::R`](R) reader structure"]
impl crate::Readable for DFSDM_FLT0JDATAR_SPEC {}
#[doc = "`reset()` method sets DFSDM_FLT0JDATAR to value 0"]
impl crate::Resettable for DFSDM_FLT0JDATAR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
