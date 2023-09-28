#[doc = "Register `SR1` reader"]
pub type R = crate::R<SR1_SPEC>;
#[doc = "Register `SR1` writer"]
pub type W = crate::W<SR1_SPEC>;
#[doc = "Field `BSY1` reader - Bank 1 ongoing program flag"]
pub type BSY1_R = crate::BitReader;
#[doc = "Field `BSY1` writer - Bank 1 ongoing program flag"]
pub type BSY1_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `WBNE1` reader - Bank 1 write buffer not empty flag"]
pub type WBNE1_R = crate::BitReader;
#[doc = "Field `WBNE1` writer - Bank 1 write buffer not empty flag"]
pub type WBNE1_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `QW1` reader - Bank 1 wait queue flag"]
pub type QW1_R = crate::BitReader;
#[doc = "Field `QW1` writer - Bank 1 wait queue flag"]
pub type QW1_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CRC_BUSY1` reader - Bank 1 CRC busy flag"]
pub type CRC_BUSY1_R = crate::BitReader;
#[doc = "Field `CRC_BUSY1` writer - Bank 1 CRC busy flag"]
pub type CRC_BUSY1_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `EOP1` reader - Bank 1 end-of-program flag"]
pub type EOP1_R = crate::BitReader;
#[doc = "Field `EOP1` writer - Bank 1 end-of-program flag"]
pub type EOP1_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `WRPERR1` reader - Bank 1 write protection error flag"]
pub type WRPERR1_R = crate::BitReader;
#[doc = "Field `WRPERR1` writer - Bank 1 write protection error flag"]
pub type WRPERR1_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PGSERR1` reader - Bank 1 programming sequence error flag"]
pub type PGSERR1_R = crate::BitReader;
#[doc = "Field `PGSERR1` writer - Bank 1 programming sequence error flag"]
pub type PGSERR1_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `STRBERR1` reader - Bank 1 strobe error flag"]
pub type STRBERR1_R = crate::BitReader;
#[doc = "Field `STRBERR1` writer - Bank 1 strobe error flag"]
pub type STRBERR1_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `INCERR1` reader - Bank 1 inconsistency error flag"]
pub type INCERR1_R = crate::BitReader;
#[doc = "Field `INCERR1` writer - Bank 1 inconsistency error flag"]
pub type INCERR1_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `OPERR1` reader - Bank 1 write/erase error flag"]
pub type OPERR1_R = crate::BitReader;
#[doc = "Field `OPERR1` writer - Bank 1 write/erase error flag"]
pub type OPERR1_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `RDPERR1` reader - Bank 1 read protection error flag"]
pub type RDPERR1_R = crate::BitReader;
#[doc = "Field `RDPERR1` writer - Bank 1 read protection error flag"]
pub type RDPERR1_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `RDSERR1` reader - Bank 1 secure error flag"]
pub type RDSERR1_R = crate::BitReader;
#[doc = "Field `RDSERR1` writer - Bank 1 secure error flag"]
pub type RDSERR1_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SNECCERR11` reader - Bank 1 single correction error flag"]
pub type SNECCERR11_R = crate::BitReader;
#[doc = "Field `SNECCERR11` writer - Bank 1 single correction error flag"]
pub type SNECCERR11_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `DBECCERR1` reader - Bank 1 ECC double detection error flag"]
pub type DBECCERR1_R = crate::BitReader;
#[doc = "Field `DBECCERR1` writer - Bank 1 ECC double detection error flag"]
pub type DBECCERR1_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CRCEND1` reader - Bank 1 CRC-complete flag"]
pub type CRCEND1_R = crate::BitReader;
#[doc = "Field `CRCEND1` writer - Bank 1 CRC-complete flag"]
pub type CRCEND1_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - Bank 1 ongoing program flag"]
    #[inline(always)]
    pub fn bsy1(&self) -> BSY1_R {
        BSY1_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Bank 1 write buffer not empty flag"]
    #[inline(always)]
    pub fn wbne1(&self) -> WBNE1_R {
        WBNE1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Bank 1 wait queue flag"]
    #[inline(always)]
    pub fn qw1(&self) -> QW1_R {
        QW1_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Bank 1 CRC busy flag"]
    #[inline(always)]
    pub fn crc_busy1(&self) -> CRC_BUSY1_R {
        CRC_BUSY1_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 16 - Bank 1 end-of-program flag"]
    #[inline(always)]
    pub fn eop1(&self) -> EOP1_R {
        EOP1_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Bank 1 write protection error flag"]
    #[inline(always)]
    pub fn wrperr1(&self) -> WRPERR1_R {
        WRPERR1_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Bank 1 programming sequence error flag"]
    #[inline(always)]
    pub fn pgserr1(&self) -> PGSERR1_R {
        PGSERR1_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Bank 1 strobe error flag"]
    #[inline(always)]
    pub fn strberr1(&self) -> STRBERR1_R {
        STRBERR1_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 21 - Bank 1 inconsistency error flag"]
    #[inline(always)]
    pub fn incerr1(&self) -> INCERR1_R {
        INCERR1_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Bank 1 write/erase error flag"]
    #[inline(always)]
    pub fn operr1(&self) -> OPERR1_R {
        OPERR1_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Bank 1 read protection error flag"]
    #[inline(always)]
    pub fn rdperr1(&self) -> RDPERR1_R {
        RDPERR1_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Bank 1 secure error flag"]
    #[inline(always)]
    pub fn rdserr1(&self) -> RDSERR1_R {
        RDSERR1_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Bank 1 single correction error flag"]
    #[inline(always)]
    pub fn sneccerr11(&self) -> SNECCERR11_R {
        SNECCERR11_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Bank 1 ECC double detection error flag"]
    #[inline(always)]
    pub fn dbeccerr1(&self) -> DBECCERR1_R {
        DBECCERR1_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Bank 1 CRC-complete flag"]
    #[inline(always)]
    pub fn crcend1(&self) -> CRCEND1_R {
        CRCEND1_R::new(((self.bits >> 27) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Bank 1 ongoing program flag"]
    #[inline(always)]
    #[must_use]
    pub fn bsy1(&mut self) -> BSY1_W<SR1_SPEC, 0> {
        BSY1_W::new(self)
    }
    #[doc = "Bit 1 - Bank 1 write buffer not empty flag"]
    #[inline(always)]
    #[must_use]
    pub fn wbne1(&mut self) -> WBNE1_W<SR1_SPEC, 1> {
        WBNE1_W::new(self)
    }
    #[doc = "Bit 2 - Bank 1 wait queue flag"]
    #[inline(always)]
    #[must_use]
    pub fn qw1(&mut self) -> QW1_W<SR1_SPEC, 2> {
        QW1_W::new(self)
    }
    #[doc = "Bit 3 - Bank 1 CRC busy flag"]
    #[inline(always)]
    #[must_use]
    pub fn crc_busy1(&mut self) -> CRC_BUSY1_W<SR1_SPEC, 3> {
        CRC_BUSY1_W::new(self)
    }
    #[doc = "Bit 16 - Bank 1 end-of-program flag"]
    #[inline(always)]
    #[must_use]
    pub fn eop1(&mut self) -> EOP1_W<SR1_SPEC, 16> {
        EOP1_W::new(self)
    }
    #[doc = "Bit 17 - Bank 1 write protection error flag"]
    #[inline(always)]
    #[must_use]
    pub fn wrperr1(&mut self) -> WRPERR1_W<SR1_SPEC, 17> {
        WRPERR1_W::new(self)
    }
    #[doc = "Bit 18 - Bank 1 programming sequence error flag"]
    #[inline(always)]
    #[must_use]
    pub fn pgserr1(&mut self) -> PGSERR1_W<SR1_SPEC, 18> {
        PGSERR1_W::new(self)
    }
    #[doc = "Bit 19 - Bank 1 strobe error flag"]
    #[inline(always)]
    #[must_use]
    pub fn strberr1(&mut self) -> STRBERR1_W<SR1_SPEC, 19> {
        STRBERR1_W::new(self)
    }
    #[doc = "Bit 21 - Bank 1 inconsistency error flag"]
    #[inline(always)]
    #[must_use]
    pub fn incerr1(&mut self) -> INCERR1_W<SR1_SPEC, 21> {
        INCERR1_W::new(self)
    }
    #[doc = "Bit 22 - Bank 1 write/erase error flag"]
    #[inline(always)]
    #[must_use]
    pub fn operr1(&mut self) -> OPERR1_W<SR1_SPEC, 22> {
        OPERR1_W::new(self)
    }
    #[doc = "Bit 23 - Bank 1 read protection error flag"]
    #[inline(always)]
    #[must_use]
    pub fn rdperr1(&mut self) -> RDPERR1_W<SR1_SPEC, 23> {
        RDPERR1_W::new(self)
    }
    #[doc = "Bit 24 - Bank 1 secure error flag"]
    #[inline(always)]
    #[must_use]
    pub fn rdserr1(&mut self) -> RDSERR1_W<SR1_SPEC, 24> {
        RDSERR1_W::new(self)
    }
    #[doc = "Bit 25 - Bank 1 single correction error flag"]
    #[inline(always)]
    #[must_use]
    pub fn sneccerr11(&mut self) -> SNECCERR11_W<SR1_SPEC, 25> {
        SNECCERR11_W::new(self)
    }
    #[doc = "Bit 26 - Bank 1 ECC double detection error flag"]
    #[inline(always)]
    #[must_use]
    pub fn dbeccerr1(&mut self) -> DBECCERR1_W<SR1_SPEC, 26> {
        DBECCERR1_W::new(self)
    }
    #[doc = "Bit 27 - Bank 1 CRC-complete flag"]
    #[inline(always)]
    #[must_use]
    pub fn crcend1(&mut self) -> CRCEND1_W<SR1_SPEC, 27> {
        CRCEND1_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "FLASH status register for bank 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sr1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sr1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SR1_SPEC;
impl crate::RegisterSpec for SR1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sr1::R`](R) reader structure"]
impl crate::Readable for SR1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`sr1::W`](W) writer structure"]
impl crate::Writable for SR1_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SR1 to value 0"]
impl crate::Resettable for SR1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
