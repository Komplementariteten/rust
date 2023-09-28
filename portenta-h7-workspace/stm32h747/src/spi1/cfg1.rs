#[doc = "Register `CFG1` reader"]
pub type R = crate::R<CFG1_SPEC>;
#[doc = "Register `CFG1` writer"]
pub type W = crate::W<CFG1_SPEC>;
#[doc = "Field `DSIZE` reader - Number of bits in at single SPI data frame"]
pub type DSIZE_R = crate::FieldReader;
#[doc = "Field `DSIZE` writer - Number of bits in at single SPI data frame"]
pub type DSIZE_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 5, O>;
#[doc = "Field `FTHVL` reader - threshold level"]
pub type FTHVL_R = crate::FieldReader;
#[doc = "Field `FTHVL` writer - threshold level"]
pub type FTHVL_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O>;
#[doc = "Field `UDRCFG` reader - Behavior of slave transmitter at underrun condition"]
pub type UDRCFG_R = crate::FieldReader;
#[doc = "Field `UDRCFG` writer - Behavior of slave transmitter at underrun condition"]
pub type UDRCFG_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
#[doc = "Field `UDRDET` reader - Detection of underrun condition at slave transmitter"]
pub type UDRDET_R = crate::FieldReader;
#[doc = "Field `UDRDET` writer - Detection of underrun condition at slave transmitter"]
pub type UDRDET_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
#[doc = "Field `RXDMAEN` reader - Rx DMA stream enable"]
pub type RXDMAEN_R = crate::BitReader;
#[doc = "Field `RXDMAEN` writer - Rx DMA stream enable"]
pub type RXDMAEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TXDMAEN` reader - Tx DMA stream enable"]
pub type TXDMAEN_R = crate::BitReader;
#[doc = "Field `TXDMAEN` writer - Tx DMA stream enable"]
pub type TXDMAEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CRCSIZE` reader - Length of CRC frame to be transacted and compared"]
pub type CRCSIZE_R = crate::FieldReader;
#[doc = "Field `CRCSIZE` writer - Length of CRC frame to be transacted and compared"]
pub type CRCSIZE_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 5, O>;
#[doc = "Field `CRCEN` reader - Hardware CRC computation enable"]
pub type CRCEN_R = crate::BitReader;
#[doc = "Field `CRCEN` writer - Hardware CRC computation enable"]
pub type CRCEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `MBR` reader - Master baud rate"]
pub type MBR_R = crate::FieldReader;
#[doc = "Field `MBR` writer - Master baud rate"]
pub type MBR_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 3, O>;
impl R {
    #[doc = "Bits 0:4 - Number of bits in at single SPI data frame"]
    #[inline(always)]
    pub fn dsize(&self) -> DSIZE_R {
        DSIZE_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 5:8 - threshold level"]
    #[inline(always)]
    pub fn fthvl(&self) -> FTHVL_R {
        FTHVL_R::new(((self.bits >> 5) & 0x0f) as u8)
    }
    #[doc = "Bits 9:10 - Behavior of slave transmitter at underrun condition"]
    #[inline(always)]
    pub fn udrcfg(&self) -> UDRCFG_R {
        UDRCFG_R::new(((self.bits >> 9) & 3) as u8)
    }
    #[doc = "Bits 11:12 - Detection of underrun condition at slave transmitter"]
    #[inline(always)]
    pub fn udrdet(&self) -> UDRDET_R {
        UDRDET_R::new(((self.bits >> 11) & 3) as u8)
    }
    #[doc = "Bit 14 - Rx DMA stream enable"]
    #[inline(always)]
    pub fn rxdmaen(&self) -> RXDMAEN_R {
        RXDMAEN_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Tx DMA stream enable"]
    #[inline(always)]
    pub fn txdmaen(&self) -> TXDMAEN_R {
        TXDMAEN_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:20 - Length of CRC frame to be transacted and compared"]
    #[inline(always)]
    pub fn crcsize(&self) -> CRCSIZE_R {
        CRCSIZE_R::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bit 22 - Hardware CRC computation enable"]
    #[inline(always)]
    pub fn crcen(&self) -> CRCEN_R {
        CRCEN_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bits 28:30 - Master baud rate"]
    #[inline(always)]
    pub fn mbr(&self) -> MBR_R {
        MBR_R::new(((self.bits >> 28) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - Number of bits in at single SPI data frame"]
    #[inline(always)]
    #[must_use]
    pub fn dsize(&mut self) -> DSIZE_W<CFG1_SPEC, 0> {
        DSIZE_W::new(self)
    }
    #[doc = "Bits 5:8 - threshold level"]
    #[inline(always)]
    #[must_use]
    pub fn fthvl(&mut self) -> FTHVL_W<CFG1_SPEC, 5> {
        FTHVL_W::new(self)
    }
    #[doc = "Bits 9:10 - Behavior of slave transmitter at underrun condition"]
    #[inline(always)]
    #[must_use]
    pub fn udrcfg(&mut self) -> UDRCFG_W<CFG1_SPEC, 9> {
        UDRCFG_W::new(self)
    }
    #[doc = "Bits 11:12 - Detection of underrun condition at slave transmitter"]
    #[inline(always)]
    #[must_use]
    pub fn udrdet(&mut self) -> UDRDET_W<CFG1_SPEC, 11> {
        UDRDET_W::new(self)
    }
    #[doc = "Bit 14 - Rx DMA stream enable"]
    #[inline(always)]
    #[must_use]
    pub fn rxdmaen(&mut self) -> RXDMAEN_W<CFG1_SPEC, 14> {
        RXDMAEN_W::new(self)
    }
    #[doc = "Bit 15 - Tx DMA stream enable"]
    #[inline(always)]
    #[must_use]
    pub fn txdmaen(&mut self) -> TXDMAEN_W<CFG1_SPEC, 15> {
        TXDMAEN_W::new(self)
    }
    #[doc = "Bits 16:20 - Length of CRC frame to be transacted and compared"]
    #[inline(always)]
    #[must_use]
    pub fn crcsize(&mut self) -> CRCSIZE_W<CFG1_SPEC, 16> {
        CRCSIZE_W::new(self)
    }
    #[doc = "Bit 22 - Hardware CRC computation enable"]
    #[inline(always)]
    #[must_use]
    pub fn crcen(&mut self) -> CRCEN_W<CFG1_SPEC, 22> {
        CRCEN_W::new(self)
    }
    #[doc = "Bits 28:30 - Master baud rate"]
    #[inline(always)]
    #[must_use]
    pub fn mbr(&mut self) -> MBR_W<CFG1_SPEC, 28> {
        MBR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "configuration register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CFG1_SPEC;
impl crate::RegisterSpec for CFG1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfg1::R`](R) reader structure"]
impl crate::Readable for CFG1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`cfg1::W`](W) writer structure"]
impl crate::Writable for CFG1_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CFG1 to value 0x0007_0007"]
impl crate::Resettable for CFG1_SPEC {
    const RESET_VALUE: Self::Ux = 0x0007_0007;
}
