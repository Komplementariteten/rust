#[doc = "Register `MACWTR` reader"]
pub type R = crate::R<MACWTR_SPEC>;
#[doc = "Register `MACWTR` writer"]
pub type W = crate::W<MACWTR_SPEC>;
#[doc = "Field `WTO` reader - WTO"]
pub type WTO_R = crate::FieldReader;
#[doc = "Field `WTO` writer - WTO"]
pub type WTO_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O>;
#[doc = "Field `PWE` reader - PWE"]
pub type PWE_R = crate::BitReader;
#[doc = "Field `PWE` writer - PWE"]
pub type PWE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bits 0:3 - WTO"]
    #[inline(always)]
    pub fn wto(&self) -> WTO_R {
        WTO_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 8 - PWE"]
    #[inline(always)]
    pub fn pwe(&self) -> PWE_R {
        PWE_R::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - WTO"]
    #[inline(always)]
    #[must_use]
    pub fn wto(&mut self) -> WTO_W<MACWTR_SPEC, 0> {
        WTO_W::new(self)
    }
    #[doc = "Bit 8 - PWE"]
    #[inline(always)]
    #[must_use]
    pub fn pwe(&mut self) -> PWE_W<MACWTR_SPEC, 8> {
        PWE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Watchdog timeout register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`macwtr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`macwtr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MACWTR_SPEC;
impl crate::RegisterSpec for MACWTR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`macwtr::R`](R) reader structure"]
impl crate::Readable for MACWTR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`macwtr::W`](W) writer structure"]
impl crate::Writable for MACWTR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MACWTR to value 0"]
impl crate::Resettable for MACWTR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
