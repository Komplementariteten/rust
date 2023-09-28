#[doc = "Register `SR2` reader"]
pub type R = crate::R<SR2_SPEC>;
#[doc = "Register `SR2` writer"]
pub type W = crate::W<SR2_SPEC>;
#[doc = "Field `BSY2` reader - Bank 2 ongoing program flag"]
pub type BSY2_R = crate::BitReader;
#[doc = "Field `BSY2` writer - Bank 2 ongoing program flag"]
pub type BSY2_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `WBNE2` reader - Bank 2 write buffer not empty flag"]
pub type WBNE2_R = crate::BitReader;
#[doc = "Field `WBNE2` writer - Bank 2 write buffer not empty flag"]
pub type WBNE2_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `QW2` reader - Bank 2 wait queue flag"]
pub type QW2_R = crate::BitReader;
#[doc = "Field `QW2` writer - Bank 2 wait queue flag"]
pub type QW2_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CRC_BUSY2` reader - Bank 2 CRC busy flag"]
pub type CRC_BUSY2_R = crate::BitReader;
#[doc = "Field `CRC_BUSY2` writer - Bank 2 CRC busy flag"]
pub type CRC_BUSY2_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `EOP2` reader - Bank 2 end-of-program flag"]
pub type EOP2_R = crate::BitReader;
#[doc = "Field `EOP2` writer - Bank 2 end-of-program flag"]
pub type EOP2_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `WRPERR2` reader - Bank 2 write protection error flag"]
pub type WRPERR2_R = crate::BitReader;
#[doc = "Field `WRPERR2` writer - Bank 2 write protection error flag"]
pub type WRPERR2_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PGSERR2` reader - Bank 2 programming sequence error flag"]
pub type PGSERR2_R = crate::BitReader;
#[doc = "Field `PGSERR2` writer - Bank 2 programming sequence error flag"]
pub type PGSERR2_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `STRBERR2` reader - Bank 2 strobe error flag"]
pub type STRBERR2_R = crate::BitReader;
#[doc = "Field `STRBERR2` writer - Bank 2 strobe error flag"]
pub type STRBERR2_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `INCERR2` reader - Bank 2 inconsistency error flag"]
pub type INCERR2_R = crate::BitReader;
#[doc = "Field `INCERR2` writer - Bank 2 inconsistency error flag"]
pub type INCERR2_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `OPERR2` reader - Bank 2 write/erase error flag"]
pub type OPERR2_R = crate::BitReader;
#[doc = "Field `OPERR2` writer - Bank 2 write/erase error flag"]
pub type OPERR2_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `RDPERR2` reader - Bank 2 read protection error flag"]
pub type RDPERR2_R = crate::BitReader;
#[doc = "Field `RDPERR2` writer - Bank 2 read protection error flag"]
pub type RDPERR2_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `RDSERR2` reader - Bank 2 secure error flag"]
pub type RDSERR2_R = crate::BitReader;
#[doc = "Field `RDSERR2` writer - Bank 2 secure error flag"]
pub type RDSERR2_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SNECCERR2` reader - Bank 2 single correction error flag"]
pub type SNECCERR2_R = crate::BitReader;
#[doc = "Field `SNECCERR2` writer - Bank 2 single correction error flag"]
pub type SNECCERR2_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `DBECCERR2` reader - Bank 2 ECC double detection error flag"]
pub type DBECCERR2_R = crate::BitReader;
#[doc = "Field `DBECCERR2` writer - Bank 2 ECC double detection error flag"]
pub type DBECCERR2_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CRCEND2` reader - Bank 2 CRC-complete flag"]
pub type CRCEND2_R = crate::BitReader;
#[doc = "Field `CRCEND2` writer - Bank 2 CRC-complete flag"]
pub type CRCEND2_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - Bank 2 ongoing program flag"]
    #[inline(always)]
    pub fn bsy2(&self) -> BSY2_R {
        BSY2_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Bank 2 write buffer not empty flag"]
    #[inline(always)]
    pub fn wbne2(&self) -> WBNE2_R {
        WBNE2_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Bank 2 wait queue flag"]
    #[inline(always)]
    pub fn qw2(&self) -> QW2_R {
        QW2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Bank 2 CRC busy flag"]
    #[inline(always)]
    pub fn crc_busy2(&self) -> CRC_BUSY2_R {
        CRC_BUSY2_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 16 - Bank 2 end-of-program flag"]
    #[inline(always)]
    pub fn eop2(&self) -> EOP2_R {
        EOP2_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Bank 2 write protection error flag"]
    #[inline(always)]
    pub fn wrperr2(&self) -> WRPERR2_R {
        WRPERR2_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Bank 2 programming sequence error flag"]
    #[inline(always)]
    pub fn pgserr2(&self) -> PGSERR2_R {
        PGSERR2_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Bank 2 strobe error flag"]
    #[inline(always)]
    pub fn strberr2(&self) -> STRBERR2_R {
        STRBERR2_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 21 - Bank 2 inconsistency error flag"]
    #[inline(always)]
    pub fn incerr2(&self) -> INCERR2_R {
        INCERR2_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Bank 2 write/erase error flag"]
    #[inline(always)]
    pub fn operr2(&self) -> OPERR2_R {
        OPERR2_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Bank 2 read protection error flag"]
    #[inline(always)]
    pub fn rdperr2(&self) -> RDPERR2_R {
        RDPERR2_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Bank 2 secure error flag"]
    #[inline(always)]
    pub fn rdserr2(&self) -> RDSERR2_R {
        RDSERR2_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Bank 2 single correction error flag"]
    #[inline(always)]
    pub fn sneccerr2(&self) -> SNECCERR2_R {
        SNECCERR2_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Bank 2 ECC double detection error flag"]
    #[inline(always)]
    pub fn dbeccerr2(&self) -> DBECCERR2_R {
        DBECCERR2_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Bank 2 CRC-complete flag"]
    #[inline(always)]
    pub fn crcend2(&self) -> CRCEND2_R {
        CRCEND2_R::new(((self.bits >> 27) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Bank 2 ongoing program flag"]
    #[inline(always)]
    #[must_use]
    pub fn bsy2(&mut self) -> BSY2_W<SR2_SPEC, 0> {
        BSY2_W::new(self)
    }
    #[doc = "Bit 1 - Bank 2 write buffer not empty flag"]
    #[inline(always)]
    #[must_use]
    pub fn wbne2(&mut self) -> WBNE2_W<SR2_SPEC, 1> {
        WBNE2_W::new(self)
    }
    #[doc = "Bit 2 - Bank 2 wait queue flag"]
    #[inline(always)]
    #[must_use]
    pub fn qw2(&mut self) -> QW2_W<SR2_SPEC, 2> {
        QW2_W::new(self)
    }
    #[doc = "Bit 3 - Bank 2 CRC busy flag"]
    #[inline(always)]
    #[must_use]
    pub fn crc_busy2(&mut self) -> CRC_BUSY2_W<SR2_SPEC, 3> {
        CRC_BUSY2_W::new(self)
    }
    #[doc = "Bit 16 - Bank 2 end-of-program flag"]
    #[inline(always)]
    #[must_use]
    pub fn eop2(&mut self) -> EOP2_W<SR2_SPEC, 16> {
        EOP2_W::new(self)
    }
    #[doc = "Bit 17 - Bank 2 write protection error flag"]
    #[inline(always)]
    #[must_use]
    pub fn wrperr2(&mut self) -> WRPERR2_W<SR2_SPEC, 17> {
        WRPERR2_W::new(self)
    }
    #[doc = "Bit 18 - Bank 2 programming sequence error flag"]
    #[inline(always)]
    #[must_use]
    pub fn pgserr2(&mut self) -> PGSERR2_W<SR2_SPEC, 18> {
        PGSERR2_W::new(self)
    }
    #[doc = "Bit 19 - Bank 2 strobe error flag"]
    #[inline(always)]
    #[must_use]
    pub fn strberr2(&mut self) -> STRBERR2_W<SR2_SPEC, 19> {
        STRBERR2_W::new(self)
    }
    #[doc = "Bit 21 - Bank 2 inconsistency error flag"]
    #[inline(always)]
    #[must_use]
    pub fn incerr2(&mut self) -> INCERR2_W<SR2_SPEC, 21> {
        INCERR2_W::new(self)
    }
    #[doc = "Bit 22 - Bank 2 write/erase error flag"]
    #[inline(always)]
    #[must_use]
    pub fn operr2(&mut self) -> OPERR2_W<SR2_SPEC, 22> {
        OPERR2_W::new(self)
    }
    #[doc = "Bit 23 - Bank 2 read protection error flag"]
    #[inline(always)]
    #[must_use]
    pub fn rdperr2(&mut self) -> RDPERR2_W<SR2_SPEC, 23> {
        RDPERR2_W::new(self)
    }
    #[doc = "Bit 24 - Bank 2 secure error flag"]
    #[inline(always)]
    #[must_use]
    pub fn rdserr2(&mut self) -> RDSERR2_W<SR2_SPEC, 24> {
        RDSERR2_W::new(self)
    }
    #[doc = "Bit 25 - Bank 2 single correction error flag"]
    #[inline(always)]
    #[must_use]
    pub fn sneccerr2(&mut self) -> SNECCERR2_W<SR2_SPEC, 25> {
        SNECCERR2_W::new(self)
    }
    #[doc = "Bit 26 - Bank 2 ECC double detection error flag"]
    #[inline(always)]
    #[must_use]
    pub fn dbeccerr2(&mut self) -> DBECCERR2_W<SR2_SPEC, 26> {
        DBECCERR2_W::new(self)
    }
    #[doc = "Bit 27 - Bank 2 CRC-complete flag"]
    #[inline(always)]
    #[must_use]
    pub fn crcend2(&mut self) -> CRCEND2_W<SR2_SPEC, 27> {
        CRCEND2_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "FLASH status register for bank 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sr2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sr2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SR2_SPEC;
impl crate::RegisterSpec for SR2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sr2::R`](R) reader structure"]
impl crate::Readable for SR2_SPEC {}
#[doc = "`write(|w| ..)` method takes [`sr2::W`](W) writer structure"]
impl crate::Writable for SR2_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SR2 to value 0"]
impl crate::Resettable for SR2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
