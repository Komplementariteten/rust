#[doc = "Register `MACPPSWR` reader"]
pub type R = crate::R<MACPPSWR_SPEC>;
#[doc = "Register `MACPPSWR` writer"]
pub type W = crate::W<MACPPSWR_SPEC>;
#[doc = "Field `PPSWIDTH0` reader - PPSWIDTH0"]
pub type PPSWIDTH0_R = crate::FieldReader<u32>;
#[doc = "Field `PPSWIDTH0` writer - PPSWIDTH0"]
pub type PPSWIDTH0_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 32, O, u32>;
impl R {
    #[doc = "Bits 0:31 - PPSWIDTH0"]
    #[inline(always)]
    pub fn ppswidth0(&self) -> PPSWIDTH0_R {
        PPSWIDTH0_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - PPSWIDTH0"]
    #[inline(always)]
    #[must_use]
    pub fn ppswidth0(&mut self) -> PPSWIDTH0_W<MACPPSWR_SPEC, 0> {
        PPSWIDTH0_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "PPS width register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`macppswr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`macppswr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MACPPSWR_SPEC;
impl crate::RegisterSpec for MACPPSWR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`macppswr::R`](R) reader structure"]
impl crate::Readable for MACPPSWR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`macppswr::W`](W) writer structure"]
impl crate::Writable for MACPPSWR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MACPPSWR to value 0"]
impl crate::Resettable for MACPPSWR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
