#[doc = "Register `CR1` reader"]
pub type R = crate::R<CR1_SPEC>;
#[doc = "Register `CR1` writer"]
pub type W = crate::W<CR1_SPEC>;
#[doc = "Field `SPE` reader - Serial Peripheral Enable"]
pub type SPE_R = crate::BitReader;
#[doc = "Field `SPE` writer - Serial Peripheral Enable"]
pub type SPE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `MASRX` reader - Master automatic SUSP in Receive mode"]
pub type MASRX_R = crate::BitReader;
#[doc = "Field `MASRX` writer - Master automatic SUSP in Receive mode"]
pub type MASRX_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CSTART` reader - Master transfer start"]
pub type CSTART_R = crate::BitReader;
#[doc = "Field `CSUSP` writer - Master SUSPend request"]
pub type CSUSP_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `HDDIR` reader - Rx/Tx direction at Half-duplex mode"]
pub type HDDIR_R = crate::BitReader;
#[doc = "Field `HDDIR` writer - Rx/Tx direction at Half-duplex mode"]
pub type HDDIR_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SSI` reader - Internal SS signal input level"]
pub type SSI_R = crate::BitReader;
#[doc = "Field `SSI` writer - Internal SS signal input level"]
pub type SSI_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CRC33_17` reader - 32-bit CRC polynomial configuration"]
pub type CRC33_17_R = crate::BitReader;
#[doc = "Field `CRC33_17` writer - 32-bit CRC polynomial configuration"]
pub type CRC33_17_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `RCRCI` reader - CRC calculation initialization pattern control for receiver"]
pub type RCRCI_R = crate::BitReader;
#[doc = "Field `RCRCI` writer - CRC calculation initialization pattern control for receiver"]
pub type RCRCI_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TCRCI` reader - CRC calculation initialization pattern control for transmitter"]
pub type TCRCI_R = crate::BitReader;
#[doc = "Field `TCRCI` writer - CRC calculation initialization pattern control for transmitter"]
pub type TCRCI_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `IOLOCK` reader - Locking the AF configuration of associated IOs"]
pub type IOLOCK_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Serial Peripheral Enable"]
    #[inline(always)]
    pub fn spe(&self) -> SPE_R {
        SPE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 8 - Master automatic SUSP in Receive mode"]
    #[inline(always)]
    pub fn masrx(&self) -> MASRX_R {
        MASRX_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Master transfer start"]
    #[inline(always)]
    pub fn cstart(&self) -> CSTART_R {
        CSTART_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 11 - Rx/Tx direction at Half-duplex mode"]
    #[inline(always)]
    pub fn hddir(&self) -> HDDIR_R {
        HDDIR_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Internal SS signal input level"]
    #[inline(always)]
    pub fn ssi(&self) -> SSI_R {
        SSI_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - 32-bit CRC polynomial configuration"]
    #[inline(always)]
    pub fn crc33_17(&self) -> CRC33_17_R {
        CRC33_17_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - CRC calculation initialization pattern control for receiver"]
    #[inline(always)]
    pub fn rcrci(&self) -> RCRCI_R {
        RCRCI_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - CRC calculation initialization pattern control for transmitter"]
    #[inline(always)]
    pub fn tcrci(&self) -> TCRCI_R {
        TCRCI_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Locking the AF configuration of associated IOs"]
    #[inline(always)]
    pub fn iolock(&self) -> IOLOCK_R {
        IOLOCK_R::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Serial Peripheral Enable"]
    #[inline(always)]
    #[must_use]
    pub fn spe(&mut self) -> SPE_W<CR1_SPEC, 0> {
        SPE_W::new(self)
    }
    #[doc = "Bit 8 - Master automatic SUSP in Receive mode"]
    #[inline(always)]
    #[must_use]
    pub fn masrx(&mut self) -> MASRX_W<CR1_SPEC, 8> {
        MASRX_W::new(self)
    }
    #[doc = "Bit 10 - Master SUSPend request"]
    #[inline(always)]
    #[must_use]
    pub fn csusp(&mut self) -> CSUSP_W<CR1_SPEC, 10> {
        CSUSP_W::new(self)
    }
    #[doc = "Bit 11 - Rx/Tx direction at Half-duplex mode"]
    #[inline(always)]
    #[must_use]
    pub fn hddir(&mut self) -> HDDIR_W<CR1_SPEC, 11> {
        HDDIR_W::new(self)
    }
    #[doc = "Bit 12 - Internal SS signal input level"]
    #[inline(always)]
    #[must_use]
    pub fn ssi(&mut self) -> SSI_W<CR1_SPEC, 12> {
        SSI_W::new(self)
    }
    #[doc = "Bit 13 - 32-bit CRC polynomial configuration"]
    #[inline(always)]
    #[must_use]
    pub fn crc33_17(&mut self) -> CRC33_17_W<CR1_SPEC, 13> {
        CRC33_17_W::new(self)
    }
    #[doc = "Bit 14 - CRC calculation initialization pattern control for receiver"]
    #[inline(always)]
    #[must_use]
    pub fn rcrci(&mut self) -> RCRCI_W<CR1_SPEC, 14> {
        RCRCI_W::new(self)
    }
    #[doc = "Bit 15 - CRC calculation initialization pattern control for transmitter"]
    #[inline(always)]
    #[must_use]
    pub fn tcrci(&mut self) -> TCRCI_W<CR1_SPEC, 15> {
        TCRCI_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "control register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cr1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cr1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CR1_SPEC;
impl crate::RegisterSpec for CR1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cr1::R`](R) reader structure"]
impl crate::Readable for CR1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`cr1::W`](W) writer structure"]
impl crate::Writable for CR1_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CR1 to value 0"]
impl crate::Resettable for CR1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
