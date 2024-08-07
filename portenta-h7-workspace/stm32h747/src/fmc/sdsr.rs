#[doc = "Register `SDSR` reader"]
pub type R = crate::R<SDSR_SPEC>;
#[doc = "Field `RE` reader - Refresh error flag An interrupt is generated if REIE = 1 and RE = 1"]
pub type RE_R = crate::BitReader;
#[doc = "Field `MODES1` reader - Status Mode for Bank 1 These bits define the Status Mode of SDRAM Bank 1."]
pub type MODES1_R = crate::FieldReader;
#[doc = "Field `MODES2` reader - Status Mode for Bank 2 These bits define the Status Mode of SDRAM Bank 2."]
pub type MODES2_R = crate::FieldReader;
impl R {
    #[doc = "Bit 0 - Refresh error flag An interrupt is generated if REIE = 1 and RE = 1"]
    #[inline(always)]
    pub fn re(&self) -> RE_R {
        RE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:2 - Status Mode for Bank 1 These bits define the Status Mode of SDRAM Bank 1."]
    #[inline(always)]
    pub fn modes1(&self) -> MODES1_R {
        MODES1_R::new(((self.bits >> 1) & 3) as u8)
    }
    #[doc = "Bits 3:4 - Status Mode for Bank 2 These bits define the Status Mode of SDRAM Bank 2."]
    #[inline(always)]
    pub fn modes2(&self) -> MODES2_R {
        MODES2_R::new(((self.bits >> 3) & 3) as u8)
    }
}
#[doc = "SDRAM Status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sdsr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SDSR_SPEC;
impl crate::RegisterSpec for SDSR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sdsr::R`](R) reader structure"]
impl crate::Readable for SDSR_SPEC {}
#[doc = "`reset()` method sets SDSR to value 0"]
impl crate::Resettable for SDSR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
