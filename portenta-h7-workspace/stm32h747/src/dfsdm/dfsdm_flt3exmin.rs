#[doc = "Register `DFSDM_FLT3EXMIN` reader"]
pub type R = crate::R<DFSDM_FLT3EXMIN_SPEC>;
#[doc = "Field `EXMINCH` reader - Extremes detector minimum data channel"]
pub type EXMINCH_R = crate::FieldReader;
#[doc = "Field `EXMIN` reader - EXMIN"]
pub type EXMIN_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:2 - Extremes detector minimum data channel"]
    #[inline(always)]
    pub fn exminch(&self) -> EXMINCH_R {
        EXMINCH_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 8:31 - EXMIN"]
    #[inline(always)]
    pub fn exmin(&self) -> EXMIN_R {
        EXMIN_R::new((self.bits >> 8) & 0x00ff_ffff)
    }
}
#[doc = "Extremes detector minimum register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dfsdm_flt3exmin::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DFSDM_FLT3EXMIN_SPEC;
impl crate::RegisterSpec for DFSDM_FLT3EXMIN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dfsdm_flt3exmin::R`](R) reader structure"]
impl crate::Readable for DFSDM_FLT3EXMIN_SPEC {}
#[doc = "`reset()` method sets DFSDM_FLT3EXMIN to value 0x7fff_ff00"]
impl crate::Resettable for DFSDM_FLT3EXMIN_SPEC {
    const RESET_VALUE: Self::Ux = 0x7fff_ff00;
}
