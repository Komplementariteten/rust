#[doc = "Register `RTOR` reader"]
pub type R = crate::R<RTOR_SPEC>;
#[doc = "Register `RTOR` writer"]
pub type W = crate::W<RTOR_SPEC>;
#[doc = "Field `RTO` reader - Receiver timeout value"]
pub type RTO_R = crate::FieldReader<u32>;
#[doc = "Field `RTO` writer - Receiver timeout value"]
pub type RTO_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 24, O, u32>;
#[doc = "Field `BLEN` reader - Block Length"]
pub type BLEN_R = crate::FieldReader;
#[doc = "Field `BLEN` writer - Block Length"]
pub type BLEN_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
impl R {
    #[doc = "Bits 0:23 - Receiver timeout value"]
    #[inline(always)]
    pub fn rto(&self) -> RTO_R {
        RTO_R::new(self.bits & 0x00ff_ffff)
    }
    #[doc = "Bits 24:31 - Block Length"]
    #[inline(always)]
    pub fn blen(&self) -> BLEN_R {
        BLEN_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:23 - Receiver timeout value"]
    #[inline(always)]
    #[must_use]
    pub fn rto(&mut self) -> RTO_W<RTOR_SPEC, 0> {
        RTO_W::new(self)
    }
    #[doc = "Bits 24:31 - Block Length"]
    #[inline(always)]
    #[must_use]
    pub fn blen(&mut self) -> BLEN_W<RTOR_SPEC, 24> {
        BLEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Receiver timeout register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rtor::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rtor::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RTOR_SPEC;
impl crate::RegisterSpec for RTOR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rtor::R`](R) reader structure"]
impl crate::Readable for RTOR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`rtor::W`](W) writer structure"]
impl crate::Writable for RTOR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RTOR to value 0"]
impl crate::Resettable for RTOR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
