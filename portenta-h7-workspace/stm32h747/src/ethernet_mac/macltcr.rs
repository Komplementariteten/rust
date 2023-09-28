#[doc = "Register `MACLTCR` reader"]
pub type R = crate::R<MACLTCR_SPEC>;
#[doc = "Register `MACLTCR` writer"]
pub type W = crate::W<MACLTCR_SPEC>;
#[doc = "Field `TWT` reader - TWT"]
pub type TWT_R = crate::FieldReader<u16>;
#[doc = "Field `TWT` writer - TWT"]
pub type TWT_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 16, O, u16>;
#[doc = "Field `LST` reader - LST"]
pub type LST_R = crate::FieldReader<u16>;
#[doc = "Field `LST` writer - LST"]
pub type LST_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 10, O, u16>;
impl R {
    #[doc = "Bits 0:15 - TWT"]
    #[inline(always)]
    pub fn twt(&self) -> TWT_R {
        TWT_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:25 - LST"]
    #[inline(always)]
    pub fn lst(&self) -> LST_R {
        LST_R::new(((self.bits >> 16) & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - TWT"]
    #[inline(always)]
    #[must_use]
    pub fn twt(&mut self) -> TWT_W<MACLTCR_SPEC, 0> {
        TWT_W::new(self)
    }
    #[doc = "Bits 16:25 - LST"]
    #[inline(always)]
    #[must_use]
    pub fn lst(&mut self) -> LST_W<MACLTCR_SPEC, 16> {
        LST_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "LPI timers control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`macltcr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`macltcr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MACLTCR_SPEC;
impl crate::RegisterSpec for MACLTCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`macltcr::R`](R) reader structure"]
impl crate::Readable for MACLTCR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`macltcr::W`](W) writer structure"]
impl crate::Writable for MACLTCR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MACLTCR to value 0x03e8_0000"]
impl crate::Resettable for MACLTCR_SPEC {
    const RESET_VALUE: Self::Ux = 0x03e8_0000;
}
