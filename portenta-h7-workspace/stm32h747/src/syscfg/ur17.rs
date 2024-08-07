#[doc = "Register `UR17` reader"]
pub type R = crate::R<UR17_SPEC>;
#[doc = "Field `IO_HSLV` reader - I/O high speed / low voltage"]
pub type IO_HSLV_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - I/O high speed / low voltage"]
    #[inline(always)]
    pub fn io_hslv(&self) -> IO_HSLV_R {
        IO_HSLV_R::new((self.bits & 1) != 0)
    }
}
#[doc = "SYSCFG user register 17\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ur17::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UR17_SPEC;
impl crate::RegisterSpec for UR17_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ur17::R`](R) reader structure"]
impl crate::Readable for UR17_SPEC {}
#[doc = "`reset()` method sets UR17 to value 0"]
impl crate::Resettable for UR17_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
