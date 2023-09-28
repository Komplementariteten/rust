#[doc = "Register `OTG_HS_DIEPEMPMSK` reader"]
pub type R = crate::R<OTG_HS_DIEPEMPMSK_SPEC>;
#[doc = "Register `OTG_HS_DIEPEMPMSK` writer"]
pub type W = crate::W<OTG_HS_DIEPEMPMSK_SPEC>;
#[doc = "Field `INEPTXFEM` reader - IN EP Tx FIFO empty interrupt mask bits"]
pub type INEPTXFEM_R = crate::FieldReader<u16>;
#[doc = "Field `INEPTXFEM` writer - IN EP Tx FIFO empty interrupt mask bits"]
pub type INEPTXFEM_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 16, O, u16>;
impl R {
    #[doc = "Bits 0:15 - IN EP Tx FIFO empty interrupt mask bits"]
    #[inline(always)]
    pub fn ineptxfem(&self) -> INEPTXFEM_R {
        INEPTXFEM_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - IN EP Tx FIFO empty interrupt mask bits"]
    #[inline(always)]
    #[must_use]
    pub fn ineptxfem(&mut self) -> INEPTXFEM_W<OTG_HS_DIEPEMPMSK_SPEC, 0> {
        INEPTXFEM_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "OTG_HS device IN endpoint FIFO empty interrupt mask register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`otg_hs_diepempmsk::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`otg_hs_diepempmsk::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OTG_HS_DIEPEMPMSK_SPEC;
impl crate::RegisterSpec for OTG_HS_DIEPEMPMSK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`otg_hs_diepempmsk::R`](R) reader structure"]
impl crate::Readable for OTG_HS_DIEPEMPMSK_SPEC {}
#[doc = "`write(|w| ..)` method takes [`otg_hs_diepempmsk::W`](W) writer structure"]
impl crate::Writable for OTG_HS_DIEPEMPMSK_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets OTG_HS_DIEPEMPMSK to value 0"]
impl crate::Resettable for OTG_HS_DIEPEMPMSK_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
