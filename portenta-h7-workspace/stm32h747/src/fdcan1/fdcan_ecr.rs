#[doc = "Register `FDCAN_ECR` reader"]
pub type R = crate::R<FDCAN_ECR_SPEC>;
#[doc = "Register `FDCAN_ECR` writer"]
pub type W = crate::W<FDCAN_ECR_SPEC>;
#[doc = "Field `TEC` reader - Transmit Error Counter"]
pub type TEC_R = crate::FieldReader;
#[doc = "Field `TEC` writer - Transmit Error Counter"]
pub type TEC_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
#[doc = "Field `TREC` reader - Receive Error Counter"]
pub type TREC_R = crate::FieldReader;
#[doc = "Field `TREC` writer - Receive Error Counter"]
pub type TREC_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 7, O>;
#[doc = "Field `RP` reader - Receive Error Passive"]
pub type RP_R = crate::BitReader;
#[doc = "Field `RP` writer - Receive Error Passive"]
pub type RP_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CEL` reader - AN Error Logging"]
pub type CEL_R = crate::FieldReader;
#[doc = "Field `CEL` writer - AN Error Logging"]
pub type CEL_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
impl R {
    #[doc = "Bits 0:7 - Transmit Error Counter"]
    #[inline(always)]
    pub fn tec(&self) -> TEC_R {
        TEC_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:14 - Receive Error Counter"]
    #[inline(always)]
    pub fn trec(&self) -> TREC_R {
        TREC_R::new(((self.bits >> 8) & 0x7f) as u8)
    }
    #[doc = "Bit 15 - Receive Error Passive"]
    #[inline(always)]
    pub fn rp(&self) -> RP_R {
        RP_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:23 - AN Error Logging"]
    #[inline(always)]
    pub fn cel(&self) -> CEL_R {
        CEL_R::new(((self.bits >> 16) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Transmit Error Counter"]
    #[inline(always)]
    #[must_use]
    pub fn tec(&mut self) -> TEC_W<FDCAN_ECR_SPEC, 0> {
        TEC_W::new(self)
    }
    #[doc = "Bits 8:14 - Receive Error Counter"]
    #[inline(always)]
    #[must_use]
    pub fn trec(&mut self) -> TREC_W<FDCAN_ECR_SPEC, 8> {
        TREC_W::new(self)
    }
    #[doc = "Bit 15 - Receive Error Passive"]
    #[inline(always)]
    #[must_use]
    pub fn rp(&mut self) -> RP_W<FDCAN_ECR_SPEC, 15> {
        RP_W::new(self)
    }
    #[doc = "Bits 16:23 - AN Error Logging"]
    #[inline(always)]
    #[must_use]
    pub fn cel(&mut self) -> CEL_W<FDCAN_ECR_SPEC, 16> {
        CEL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "FDCAN Error Counter Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fdcan_ecr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fdcan_ecr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FDCAN_ECR_SPEC;
impl crate::RegisterSpec for FDCAN_ECR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fdcan_ecr::R`](R) reader structure"]
impl crate::Readable for FDCAN_ECR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`fdcan_ecr::W`](W) writer structure"]
impl crate::Writable for FDCAN_ECR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets FDCAN_ECR to value 0"]
impl crate::Resettable for FDCAN_ECR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
