#[doc = "Register `FDCAN_RXF0S` reader"]
pub type R = crate::R<FDCAN_RXF0S_SPEC>;
#[doc = "Register `FDCAN_RXF0S` writer"]
pub type W = crate::W<FDCAN_RXF0S_SPEC>;
#[doc = "Field `F0FL` reader - Rx FIFO 0 Fill Level"]
pub type F0FL_R = crate::FieldReader;
#[doc = "Field `F0FL` writer - Rx FIFO 0 Fill Level"]
pub type F0FL_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 7, O>;
#[doc = "Field `F0G` reader - Rx FIFO 0 Get Index"]
pub type F0G_R = crate::FieldReader;
#[doc = "Field `F0G` writer - Rx FIFO 0 Get Index"]
pub type F0G_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 6, O>;
#[doc = "Field `F0P` reader - Rx FIFO 0 Put Index"]
pub type F0P_R = crate::FieldReader;
#[doc = "Field `F0P` writer - Rx FIFO 0 Put Index"]
pub type F0P_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 6, O>;
#[doc = "Field `F0F` reader - Rx FIFO 0 Full"]
pub type F0F_R = crate::BitReader;
#[doc = "Field `F0F` writer - Rx FIFO 0 Full"]
pub type F0F_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `RF0L` reader - Rx FIFO 0 Message Lost"]
pub type RF0L_R = crate::BitReader;
#[doc = "Field `RF0L` writer - Rx FIFO 0 Message Lost"]
pub type RF0L_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bits 0:6 - Rx FIFO 0 Fill Level"]
    #[inline(always)]
    pub fn f0fl(&self) -> F0FL_R {
        F0FL_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 8:13 - Rx FIFO 0 Get Index"]
    #[inline(always)]
    pub fn f0g(&self) -> F0G_R {
        F0G_R::new(((self.bits >> 8) & 0x3f) as u8)
    }
    #[doc = "Bits 16:21 - Rx FIFO 0 Put Index"]
    #[inline(always)]
    pub fn f0p(&self) -> F0P_R {
        F0P_R::new(((self.bits >> 16) & 0x3f) as u8)
    }
    #[doc = "Bit 24 - Rx FIFO 0 Full"]
    #[inline(always)]
    pub fn f0f(&self) -> F0F_R {
        F0F_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Rx FIFO 0 Message Lost"]
    #[inline(always)]
    pub fn rf0l(&self) -> RF0L_R {
        RF0L_R::new(((self.bits >> 25) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:6 - Rx FIFO 0 Fill Level"]
    #[inline(always)]
    #[must_use]
    pub fn f0fl(&mut self) -> F0FL_W<FDCAN_RXF0S_SPEC, 0> {
        F0FL_W::new(self)
    }
    #[doc = "Bits 8:13 - Rx FIFO 0 Get Index"]
    #[inline(always)]
    #[must_use]
    pub fn f0g(&mut self) -> F0G_W<FDCAN_RXF0S_SPEC, 8> {
        F0G_W::new(self)
    }
    #[doc = "Bits 16:21 - Rx FIFO 0 Put Index"]
    #[inline(always)]
    #[must_use]
    pub fn f0p(&mut self) -> F0P_W<FDCAN_RXF0S_SPEC, 16> {
        F0P_W::new(self)
    }
    #[doc = "Bit 24 - Rx FIFO 0 Full"]
    #[inline(always)]
    #[must_use]
    pub fn f0f(&mut self) -> F0F_W<FDCAN_RXF0S_SPEC, 24> {
        F0F_W::new(self)
    }
    #[doc = "Bit 25 - Rx FIFO 0 Message Lost"]
    #[inline(always)]
    #[must_use]
    pub fn rf0l(&mut self) -> RF0L_W<FDCAN_RXF0S_SPEC, 25> {
        RF0L_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "FDCAN Rx FIFO 0 Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fdcan_rxf0s::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fdcan_rxf0s::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FDCAN_RXF0S_SPEC;
impl crate::RegisterSpec for FDCAN_RXF0S_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fdcan_rxf0s::R`](R) reader structure"]
impl crate::Readable for FDCAN_RXF0S_SPEC {}
#[doc = "`write(|w| ..)` method takes [`fdcan_rxf0s::W`](W) writer structure"]
impl crate::Writable for FDCAN_RXF0S_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets FDCAN_RXF0S to value 0"]
impl crate::Resettable for FDCAN_RXF0S_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
