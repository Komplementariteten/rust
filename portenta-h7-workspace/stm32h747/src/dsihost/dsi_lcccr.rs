#[doc = "Register `DSI_LCCCR` reader"]
pub type R = crate::R<DSI_LCCCR_SPEC>;
#[doc = "Field `COLC` reader - COLC"]
pub type COLC_R = crate::FieldReader;
#[doc = "Field `LPE` reader - LPE"]
pub type LPE_R = crate::BitReader;
impl R {
    #[doc = "Bits 0:3 - COLC"]
    #[inline(always)]
    pub fn colc(&self) -> COLC_R {
        COLC_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 8 - LPE"]
    #[inline(always)]
    pub fn lpe(&self) -> LPE_R {
        LPE_R::new(((self.bits >> 8) & 1) != 0)
    }
}
#[doc = "DSI Host LTDC current color coding register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dsi_lcccr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DSI_LCCCR_SPEC;
impl crate::RegisterSpec for DSI_LCCCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dsi_lcccr::R`](R) reader structure"]
impl crate::Readable for DSI_LCCCR_SPEC {}
#[doc = "`reset()` method sets DSI_LCCCR to value 0"]
impl crate::Resettable for DSI_LCCCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
