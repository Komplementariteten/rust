#[doc = "Register `FDCAN_TTTS` reader"]
pub type R = crate::R<FDCAN_TTTS_SPEC>;
#[doc = "Register `FDCAN_TTTS` writer"]
pub type W = crate::W<FDCAN_TTTS_SPEC>;
#[doc = "Field `SWTDEL` reader - Stop watch trigger input selection"]
pub type SWTDEL_R = crate::FieldReader;
#[doc = "Field `SWTDEL` writer - Stop watch trigger input selection"]
pub type SWTDEL_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
#[doc = "Field `EVTSEL` reader - Event trigger input selection"]
pub type EVTSEL_R = crate::FieldReader;
#[doc = "Field `EVTSEL` writer - Event trigger input selection"]
pub type EVTSEL_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
impl R {
    #[doc = "Bits 0:1 - Stop watch trigger input selection"]
    #[inline(always)]
    pub fn swtdel(&self) -> SWTDEL_R {
        SWTDEL_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 4:5 - Event trigger input selection"]
    #[inline(always)]
    pub fn evtsel(&self) -> EVTSEL_R {
        EVTSEL_R::new(((self.bits >> 4) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Stop watch trigger input selection"]
    #[inline(always)]
    #[must_use]
    pub fn swtdel(&mut self) -> SWTDEL_W<FDCAN_TTTS_SPEC, 0> {
        SWTDEL_W::new(self)
    }
    #[doc = "Bits 4:5 - Event trigger input selection"]
    #[inline(always)]
    #[must_use]
    pub fn evtsel(&mut self) -> EVTSEL_W<FDCAN_TTTS_SPEC, 4> {
        EVTSEL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "FDCAN TT Trigger Select Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fdcan_ttts::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fdcan_ttts::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FDCAN_TTTS_SPEC;
impl crate::RegisterSpec for FDCAN_TTTS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fdcan_ttts::R`](R) reader structure"]
impl crate::Readable for FDCAN_TTTS_SPEC {}
#[doc = "`write(|w| ..)` method takes [`fdcan_ttts::W`](W) writer structure"]
impl crate::Writable for FDCAN_TTTS_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets FDCAN_TTTS to value 0"]
impl crate::Resettable for FDCAN_TTTS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
