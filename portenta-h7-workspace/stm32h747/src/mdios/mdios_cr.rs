#[doc = "Register `MDIOS_CR` reader"]
pub type R = crate::R<MDIOS_CR_SPEC>;
#[doc = "Register `MDIOS_CR` writer"]
pub type W = crate::W<MDIOS_CR_SPEC>;
#[doc = "Field `EN` reader - Peripheral enable"]
pub type EN_R = crate::BitReader;
#[doc = "Field `EN` writer - Peripheral enable"]
pub type EN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `WRIE` reader - Register write interrupt enable"]
pub type WRIE_R = crate::BitReader;
#[doc = "Field `WRIE` writer - Register write interrupt enable"]
pub type WRIE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `RDIE` reader - Register Read Interrupt Enable"]
pub type RDIE_R = crate::BitReader;
#[doc = "Field `RDIE` writer - Register Read Interrupt Enable"]
pub type RDIE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `EIE` reader - Error interrupt enable"]
pub type EIE_R = crate::BitReader;
#[doc = "Field `EIE` writer - Error interrupt enable"]
pub type EIE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `DPC` reader - Disable Preamble Check"]
pub type DPC_R = crate::BitReader;
#[doc = "Field `DPC` writer - Disable Preamble Check"]
pub type DPC_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PORT_ADDRESS` reader - Slaves's address"]
pub type PORT_ADDRESS_R = crate::FieldReader;
#[doc = "Field `PORT_ADDRESS` writer - Slaves's address"]
pub type PORT_ADDRESS_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 5, O>;
impl R {
    #[doc = "Bit 0 - Peripheral enable"]
    #[inline(always)]
    pub fn en(&self) -> EN_R {
        EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Register write interrupt enable"]
    #[inline(always)]
    pub fn wrie(&self) -> WRIE_R {
        WRIE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Register Read Interrupt Enable"]
    #[inline(always)]
    pub fn rdie(&self) -> RDIE_R {
        RDIE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Error interrupt enable"]
    #[inline(always)]
    pub fn eie(&self) -> EIE_R {
        EIE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 7 - Disable Preamble Check"]
    #[inline(always)]
    pub fn dpc(&self) -> DPC_R {
        DPC_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:12 - Slaves's address"]
    #[inline(always)]
    pub fn port_address(&self) -> PORT_ADDRESS_R {
        PORT_ADDRESS_R::new(((self.bits >> 8) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Peripheral enable"]
    #[inline(always)]
    #[must_use]
    pub fn en(&mut self) -> EN_W<MDIOS_CR_SPEC, 0> {
        EN_W::new(self)
    }
    #[doc = "Bit 1 - Register write interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn wrie(&mut self) -> WRIE_W<MDIOS_CR_SPEC, 1> {
        WRIE_W::new(self)
    }
    #[doc = "Bit 2 - Register Read Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn rdie(&mut self) -> RDIE_W<MDIOS_CR_SPEC, 2> {
        RDIE_W::new(self)
    }
    #[doc = "Bit 3 - Error interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn eie(&mut self) -> EIE_W<MDIOS_CR_SPEC, 3> {
        EIE_W::new(self)
    }
    #[doc = "Bit 7 - Disable Preamble Check"]
    #[inline(always)]
    #[must_use]
    pub fn dpc(&mut self) -> DPC_W<MDIOS_CR_SPEC, 7> {
        DPC_W::new(self)
    }
    #[doc = "Bits 8:12 - Slaves's address"]
    #[inline(always)]
    #[must_use]
    pub fn port_address(&mut self) -> PORT_ADDRESS_W<MDIOS_CR_SPEC, 8> {
        PORT_ADDRESS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "MDIOS configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mdios_cr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mdios_cr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MDIOS_CR_SPEC;
impl crate::RegisterSpec for MDIOS_CR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mdios_cr::R`](R) reader structure"]
impl crate::Readable for MDIOS_CR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`mdios_cr::W`](W) writer structure"]
impl crate::Writable for MDIOS_CR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MDIOS_CR to value 0"]
impl crate::Resettable for MDIOS_CR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
