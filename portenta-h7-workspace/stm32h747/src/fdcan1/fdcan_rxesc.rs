#[doc = "Register `FDCAN_RXESC` reader"]
pub type R = crate::R<FDCAN_RXESC_SPEC>;
#[doc = "Register `FDCAN_RXESC` writer"]
pub type W = crate::W<FDCAN_RXESC_SPEC>;
#[doc = "Field `F0DS` reader - Rx FIFO 1 Data Field Size:"]
pub type F0DS_R = crate::FieldReader;
#[doc = "Field `F0DS` writer - Rx FIFO 1 Data Field Size:"]
pub type F0DS_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 3, O>;
#[doc = "Field `F1DS` reader - Rx FIFO 0 Data Field Size:"]
pub type F1DS_R = crate::FieldReader;
#[doc = "Field `F1DS` writer - Rx FIFO 0 Data Field Size:"]
pub type F1DS_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 3, O>;
#[doc = "Field `RBDS` reader - Rx Buffer Data Field Size:"]
pub type RBDS_R = crate::FieldReader;
#[doc = "Field `RBDS` writer - Rx Buffer Data Field Size:"]
pub type RBDS_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 3, O>;
impl R {
    #[doc = "Bits 0:2 - Rx FIFO 1 Data Field Size:"]
    #[inline(always)]
    pub fn f0ds(&self) -> F0DS_R {
        F0DS_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 4:6 - Rx FIFO 0 Data Field Size:"]
    #[inline(always)]
    pub fn f1ds(&self) -> F1DS_R {
        F1DS_R::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bits 8:10 - Rx Buffer Data Field Size:"]
    #[inline(always)]
    pub fn rbds(&self) -> RBDS_R {
        RBDS_R::new(((self.bits >> 8) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Rx FIFO 1 Data Field Size:"]
    #[inline(always)]
    #[must_use]
    pub fn f0ds(&mut self) -> F0DS_W<FDCAN_RXESC_SPEC, 0> {
        F0DS_W::new(self)
    }
    #[doc = "Bits 4:6 - Rx FIFO 0 Data Field Size:"]
    #[inline(always)]
    #[must_use]
    pub fn f1ds(&mut self) -> F1DS_W<FDCAN_RXESC_SPEC, 4> {
        F1DS_W::new(self)
    }
    #[doc = "Bits 8:10 - Rx Buffer Data Field Size:"]
    #[inline(always)]
    #[must_use]
    pub fn rbds(&mut self) -> RBDS_W<FDCAN_RXESC_SPEC, 8> {
        RBDS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "FDCAN Rx Buffer Element Size Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fdcan_rxesc::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fdcan_rxesc::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FDCAN_RXESC_SPEC;
impl crate::RegisterSpec for FDCAN_RXESC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fdcan_rxesc::R`](R) reader structure"]
impl crate::Readable for FDCAN_RXESC_SPEC {}
#[doc = "`write(|w| ..)` method takes [`fdcan_rxesc::W`](W) writer structure"]
impl crate::Writable for FDCAN_RXESC_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets FDCAN_RXESC to value 0"]
impl crate::Resettable for FDCAN_RXESC_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
