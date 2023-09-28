#[doc = "Register `MAC1USTCR` reader"]
pub type R = crate::R<MAC1USTCR_SPEC>;
#[doc = "Register `MAC1USTCR` writer"]
pub type W = crate::W<MAC1USTCR_SPEC>;
#[doc = "Field `TIC_1US_CNTR` reader - TIC_1US_CNTR"]
pub type TIC_1US_CNTR_R = crate::FieldReader<u16>;
#[doc = "Field `TIC_1US_CNTR` writer - TIC_1US_CNTR"]
pub type TIC_1US_CNTR_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 12, O, u16>;
impl R {
    #[doc = "Bits 0:11 - TIC_1US_CNTR"]
    #[inline(always)]
    pub fn tic_1us_cntr(&self) -> TIC_1US_CNTR_R {
        TIC_1US_CNTR_R::new((self.bits & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - TIC_1US_CNTR"]
    #[inline(always)]
    #[must_use]
    pub fn tic_1us_cntr(&mut self) -> TIC_1US_CNTR_W<MAC1USTCR_SPEC, 0> {
        TIC_1US_CNTR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "1-microsecond-tick counter register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mac1ustcr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mac1ustcr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MAC1USTCR_SPEC;
impl crate::RegisterSpec for MAC1USTCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mac1ustcr::R`](R) reader structure"]
impl crate::Readable for MAC1USTCR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`mac1ustcr::W`](W) writer structure"]
impl crate::Writable for MAC1USTCR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MAC1USTCR to value 0"]
impl crate::Resettable for MAC1USTCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
