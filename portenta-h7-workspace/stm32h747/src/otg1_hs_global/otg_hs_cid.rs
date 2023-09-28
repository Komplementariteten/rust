#[doc = "Register `OTG_HS_CID` reader"]
pub type R = crate::R<OTG_HS_CID_SPEC>;
#[doc = "Register `OTG_HS_CID` writer"]
pub type W = crate::W<OTG_HS_CID_SPEC>;
#[doc = "Field `PRODUCT_ID` reader - Product ID field"]
pub type PRODUCT_ID_R = crate::FieldReader<u32>;
#[doc = "Field `PRODUCT_ID` writer - Product ID field"]
pub type PRODUCT_ID_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 32, O, u32>;
impl R {
    #[doc = "Bits 0:31 - Product ID field"]
    #[inline(always)]
    pub fn product_id(&self) -> PRODUCT_ID_R {
        PRODUCT_ID_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Product ID field"]
    #[inline(always)]
    #[must_use]
    pub fn product_id(&mut self) -> PRODUCT_ID_W<OTG_HS_CID_SPEC, 0> {
        PRODUCT_ID_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "OTG_HS core ID register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`otg_hs_cid::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`otg_hs_cid::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OTG_HS_CID_SPEC;
impl crate::RegisterSpec for OTG_HS_CID_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`otg_hs_cid::R`](R) reader structure"]
impl crate::Readable for OTG_HS_CID_SPEC {}
#[doc = "`write(|w| ..)` method takes [`otg_hs_cid::W`](W) writer structure"]
impl crate::Writable for OTG_HS_CID_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets OTG_HS_CID to value 0x1200"]
impl crate::Resettable for OTG_HS_CID_SPEC {
    const RESET_VALUE: Self::Ux = 0x1200;
}
