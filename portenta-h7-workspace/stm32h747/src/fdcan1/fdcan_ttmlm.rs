#[doc = "Register `FDCAN_TTMLM` reader"]
pub type R = crate::R<FDCAN_TTMLM_SPEC>;
#[doc = "Register `FDCAN_TTMLM` writer"]
pub type W = crate::W<FDCAN_TTMLM_SPEC>;
#[doc = "Field `CCM` reader - Cycle Count Max"]
pub type CCM_R = crate::FieldReader;
#[doc = "Field `CCM` writer - Cycle Count Max"]
pub type CCM_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 6, O>;
#[doc = "Field `CSS` reader - Cycle Start Synchronization"]
pub type CSS_R = crate::FieldReader;
#[doc = "Field `CSS` writer - Cycle Start Synchronization"]
pub type CSS_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
#[doc = "Field `TXEW` reader - Tx Enable Window"]
pub type TXEW_R = crate::FieldReader;
#[doc = "Field `TXEW` writer - Tx Enable Window"]
pub type TXEW_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O>;
#[doc = "Field `ENTT` reader - Expected Number of Tx Triggers"]
pub type ENTT_R = crate::FieldReader<u16>;
#[doc = "Field `ENTT` writer - Expected Number of Tx Triggers"]
pub type ENTT_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 12, O, u16>;
impl R {
    #[doc = "Bits 0:5 - Cycle Count Max"]
    #[inline(always)]
    pub fn ccm(&self) -> CCM_R {
        CCM_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 6:7 - Cycle Start Synchronization"]
    #[inline(always)]
    pub fn css(&self) -> CSS_R {
        CSS_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:11 - Tx Enable Window"]
    #[inline(always)]
    pub fn txew(&self) -> TXEW_R {
        TXEW_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 16:27 - Expected Number of Tx Triggers"]
    #[inline(always)]
    pub fn entt(&self) -> ENTT_R {
        ENTT_R::new(((self.bits >> 16) & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:5 - Cycle Count Max"]
    #[inline(always)]
    #[must_use]
    pub fn ccm(&mut self) -> CCM_W<FDCAN_TTMLM_SPEC, 0> {
        CCM_W::new(self)
    }
    #[doc = "Bits 6:7 - Cycle Start Synchronization"]
    #[inline(always)]
    #[must_use]
    pub fn css(&mut self) -> CSS_W<FDCAN_TTMLM_SPEC, 6> {
        CSS_W::new(self)
    }
    #[doc = "Bits 8:11 - Tx Enable Window"]
    #[inline(always)]
    #[must_use]
    pub fn txew(&mut self) -> TXEW_W<FDCAN_TTMLM_SPEC, 8> {
        TXEW_W::new(self)
    }
    #[doc = "Bits 16:27 - Expected Number of Tx Triggers"]
    #[inline(always)]
    #[must_use]
    pub fn entt(&mut self) -> ENTT_W<FDCAN_TTMLM_SPEC, 16> {
        ENTT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "FDCAN TT Matrix Limits Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fdcan_ttmlm::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fdcan_ttmlm::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FDCAN_TTMLM_SPEC;
impl crate::RegisterSpec for FDCAN_TTMLM_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fdcan_ttmlm::R`](R) reader structure"]
impl crate::Readable for FDCAN_TTMLM_SPEC {}
#[doc = "`write(|w| ..)` method takes [`fdcan_ttmlm::W`](W) writer structure"]
impl crate::Writable for FDCAN_TTMLM_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets FDCAN_TTMLM to value 0"]
impl crate::Resettable for FDCAN_TTMLM_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
