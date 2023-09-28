#[doc = "Register `CR2` reader"]
pub type R = crate::R<CR2_SPEC>;
#[doc = "Register `CR2` writer"]
pub type W = crate::W<CR2_SPEC>;
#[doc = "Field `LOCK2` reader - Bank 2 configuration lock bit"]
pub type LOCK2_R = crate::BitReader;
#[doc = "Field `LOCK2` writer - Bank 2 configuration lock bit"]
pub type LOCK2_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PG2` reader - Bank 2 program enable bit"]
pub type PG2_R = crate::BitReader;
#[doc = "Field `PG2` writer - Bank 2 program enable bit"]
pub type PG2_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SER2` reader - Bank 2 sector erase request"]
pub type SER2_R = crate::BitReader;
#[doc = "Field `SER2` writer - Bank 2 sector erase request"]
pub type SER2_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `BER2` reader - Bank 2 erase request"]
pub type BER2_R = crate::BitReader;
#[doc = "Field `BER2` writer - Bank 2 erase request"]
pub type BER2_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PSIZE2` reader - Bank 2 program size"]
pub type PSIZE2_R = crate::FieldReader;
#[doc = "Field `PSIZE2` writer - Bank 2 program size"]
pub type PSIZE2_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
#[doc = "Field `FW2` reader - Bank 2 write forcing control bit"]
pub type FW2_R = crate::BitReader;
#[doc = "Field `FW2` writer - Bank 2 write forcing control bit"]
pub type FW2_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `START2` reader - Bank 2 bank or sector erase start control bit"]
pub type START2_R = crate::BitReader;
#[doc = "Field `START2` writer - Bank 2 bank or sector erase start control bit"]
pub type START2_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SNB2` reader - Bank 2 sector erase selection number"]
pub type SNB2_R = crate::FieldReader;
#[doc = "Field `SNB2` writer - Bank 2 sector erase selection number"]
pub type SNB2_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 3, O>;
#[doc = "Field `CRC_EN` reader - Bank 2 CRC control bit"]
pub type CRC_EN_R = crate::BitReader;
#[doc = "Field `CRC_EN` writer - Bank 2 CRC control bit"]
pub type CRC_EN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `EOPIE2` reader - Bank 2 end-of-program interrupt control bit"]
pub type EOPIE2_R = crate::BitReader;
#[doc = "Field `EOPIE2` writer - Bank 2 end-of-program interrupt control bit"]
pub type EOPIE2_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `WRPERRIE2` reader - Bank 2 write protection error interrupt enable bit"]
pub type WRPERRIE2_R = crate::BitReader;
#[doc = "Field `WRPERRIE2` writer - Bank 2 write protection error interrupt enable bit"]
pub type WRPERRIE2_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PGSERRIE2` reader - Bank 2 programming sequence error interrupt enable bit"]
pub type PGSERRIE2_R = crate::BitReader;
#[doc = "Field `PGSERRIE2` writer - Bank 2 programming sequence error interrupt enable bit"]
pub type PGSERRIE2_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `STRBERRIE2` reader - Bank 2 strobe error interrupt enable bit"]
pub type STRBERRIE2_R = crate::BitReader;
#[doc = "Field `STRBERRIE2` writer - Bank 2 strobe error interrupt enable bit"]
pub type STRBERRIE2_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `INCERRIE2` reader - Bank 2 inconsistency error interrupt enable bit"]
pub type INCERRIE2_R = crate::BitReader;
#[doc = "Field `INCERRIE2` writer - Bank 2 inconsistency error interrupt enable bit"]
pub type INCERRIE2_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `OPERRIE2` reader - Bank 2 write/erase error interrupt enable bit"]
pub type OPERRIE2_R = crate::BitReader;
#[doc = "Field `OPERRIE2` writer - Bank 2 write/erase error interrupt enable bit"]
pub type OPERRIE2_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `RDPERRIE2` reader - Bank 2 read protection error interrupt enable bit"]
pub type RDPERRIE2_R = crate::BitReader;
#[doc = "Field `RDPERRIE2` writer - Bank 2 read protection error interrupt enable bit"]
pub type RDPERRIE2_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `RDSERRIE2` reader - Bank 2 secure error interrupt enable bit"]
pub type RDSERRIE2_R = crate::BitReader;
#[doc = "Field `RDSERRIE2` writer - Bank 2 secure error interrupt enable bit"]
pub type RDSERRIE2_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SNECCERRIE2` reader - Bank 2 ECC single correction error interrupt enable bit"]
pub type SNECCERRIE2_R = crate::BitReader;
#[doc = "Field `SNECCERRIE2` writer - Bank 2 ECC single correction error interrupt enable bit"]
pub type SNECCERRIE2_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `DBECCERRIE2` reader - Bank 2 ECC double detection error interrupt enable bit"]
pub type DBECCERRIE2_R = crate::BitReader;
#[doc = "Field `DBECCERRIE2` writer - Bank 2 ECC double detection error interrupt enable bit"]
pub type DBECCERRIE2_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CRCENDIE2` reader - Bank 2 end of CRC calculation interrupt enable bit"]
pub type CRCENDIE2_R = crate::BitReader;
#[doc = "Field `CRCENDIE2` writer - Bank 2 end of CRC calculation interrupt enable bit"]
pub type CRCENDIE2_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - Bank 2 configuration lock bit"]
    #[inline(always)]
    pub fn lock2(&self) -> LOCK2_R {
        LOCK2_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Bank 2 program enable bit"]
    #[inline(always)]
    pub fn pg2(&self) -> PG2_R {
        PG2_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Bank 2 sector erase request"]
    #[inline(always)]
    pub fn ser2(&self) -> SER2_R {
        SER2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Bank 2 erase request"]
    #[inline(always)]
    pub fn ber2(&self) -> BER2_R {
        BER2_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:5 - Bank 2 program size"]
    #[inline(always)]
    pub fn psize2(&self) -> PSIZE2_R {
        PSIZE2_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bit 6 - Bank 2 write forcing control bit"]
    #[inline(always)]
    pub fn fw2(&self) -> FW2_R {
        FW2_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Bank 2 bank or sector erase start control bit"]
    #[inline(always)]
    pub fn start2(&self) -> START2_R {
        START2_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:10 - Bank 2 sector erase selection number"]
    #[inline(always)]
    pub fn snb2(&self) -> SNB2_R {
        SNB2_R::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bit 15 - Bank 2 CRC control bit"]
    #[inline(always)]
    pub fn crc_en(&self) -> CRC_EN_R {
        CRC_EN_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Bank 2 end-of-program interrupt control bit"]
    #[inline(always)]
    pub fn eopie2(&self) -> EOPIE2_R {
        EOPIE2_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Bank 2 write protection error interrupt enable bit"]
    #[inline(always)]
    pub fn wrperrie2(&self) -> WRPERRIE2_R {
        WRPERRIE2_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Bank 2 programming sequence error interrupt enable bit"]
    #[inline(always)]
    pub fn pgserrie2(&self) -> PGSERRIE2_R {
        PGSERRIE2_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Bank 2 strobe error interrupt enable bit"]
    #[inline(always)]
    pub fn strberrie2(&self) -> STRBERRIE2_R {
        STRBERRIE2_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 21 - Bank 2 inconsistency error interrupt enable bit"]
    #[inline(always)]
    pub fn incerrie2(&self) -> INCERRIE2_R {
        INCERRIE2_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Bank 2 write/erase error interrupt enable bit"]
    #[inline(always)]
    pub fn operrie2(&self) -> OPERRIE2_R {
        OPERRIE2_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Bank 2 read protection error interrupt enable bit"]
    #[inline(always)]
    pub fn rdperrie2(&self) -> RDPERRIE2_R {
        RDPERRIE2_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Bank 2 secure error interrupt enable bit"]
    #[inline(always)]
    pub fn rdserrie2(&self) -> RDSERRIE2_R {
        RDSERRIE2_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Bank 2 ECC single correction error interrupt enable bit"]
    #[inline(always)]
    pub fn sneccerrie2(&self) -> SNECCERRIE2_R {
        SNECCERRIE2_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Bank 2 ECC double detection error interrupt enable bit"]
    #[inline(always)]
    pub fn dbeccerrie2(&self) -> DBECCERRIE2_R {
        DBECCERRIE2_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Bank 2 end of CRC calculation interrupt enable bit"]
    #[inline(always)]
    pub fn crcendie2(&self) -> CRCENDIE2_R {
        CRCENDIE2_R::new(((self.bits >> 27) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Bank 2 configuration lock bit"]
    #[inline(always)]
    #[must_use]
    pub fn lock2(&mut self) -> LOCK2_W<CR2_SPEC, 0> {
        LOCK2_W::new(self)
    }
    #[doc = "Bit 1 - Bank 2 program enable bit"]
    #[inline(always)]
    #[must_use]
    pub fn pg2(&mut self) -> PG2_W<CR2_SPEC, 1> {
        PG2_W::new(self)
    }
    #[doc = "Bit 2 - Bank 2 sector erase request"]
    #[inline(always)]
    #[must_use]
    pub fn ser2(&mut self) -> SER2_W<CR2_SPEC, 2> {
        SER2_W::new(self)
    }
    #[doc = "Bit 3 - Bank 2 erase request"]
    #[inline(always)]
    #[must_use]
    pub fn ber2(&mut self) -> BER2_W<CR2_SPEC, 3> {
        BER2_W::new(self)
    }
    #[doc = "Bits 4:5 - Bank 2 program size"]
    #[inline(always)]
    #[must_use]
    pub fn psize2(&mut self) -> PSIZE2_W<CR2_SPEC, 4> {
        PSIZE2_W::new(self)
    }
    #[doc = "Bit 6 - Bank 2 write forcing control bit"]
    #[inline(always)]
    #[must_use]
    pub fn fw2(&mut self) -> FW2_W<CR2_SPEC, 6> {
        FW2_W::new(self)
    }
    #[doc = "Bit 7 - Bank 2 bank or sector erase start control bit"]
    #[inline(always)]
    #[must_use]
    pub fn start2(&mut self) -> START2_W<CR2_SPEC, 7> {
        START2_W::new(self)
    }
    #[doc = "Bits 8:10 - Bank 2 sector erase selection number"]
    #[inline(always)]
    #[must_use]
    pub fn snb2(&mut self) -> SNB2_W<CR2_SPEC, 8> {
        SNB2_W::new(self)
    }
    #[doc = "Bit 15 - Bank 2 CRC control bit"]
    #[inline(always)]
    #[must_use]
    pub fn crc_en(&mut self) -> CRC_EN_W<CR2_SPEC, 15> {
        CRC_EN_W::new(self)
    }
    #[doc = "Bit 16 - Bank 2 end-of-program interrupt control bit"]
    #[inline(always)]
    #[must_use]
    pub fn eopie2(&mut self) -> EOPIE2_W<CR2_SPEC, 16> {
        EOPIE2_W::new(self)
    }
    #[doc = "Bit 17 - Bank 2 write protection error interrupt enable bit"]
    #[inline(always)]
    #[must_use]
    pub fn wrperrie2(&mut self) -> WRPERRIE2_W<CR2_SPEC, 17> {
        WRPERRIE2_W::new(self)
    }
    #[doc = "Bit 18 - Bank 2 programming sequence error interrupt enable bit"]
    #[inline(always)]
    #[must_use]
    pub fn pgserrie2(&mut self) -> PGSERRIE2_W<CR2_SPEC, 18> {
        PGSERRIE2_W::new(self)
    }
    #[doc = "Bit 19 - Bank 2 strobe error interrupt enable bit"]
    #[inline(always)]
    #[must_use]
    pub fn strberrie2(&mut self) -> STRBERRIE2_W<CR2_SPEC, 19> {
        STRBERRIE2_W::new(self)
    }
    #[doc = "Bit 21 - Bank 2 inconsistency error interrupt enable bit"]
    #[inline(always)]
    #[must_use]
    pub fn incerrie2(&mut self) -> INCERRIE2_W<CR2_SPEC, 21> {
        INCERRIE2_W::new(self)
    }
    #[doc = "Bit 22 - Bank 2 write/erase error interrupt enable bit"]
    #[inline(always)]
    #[must_use]
    pub fn operrie2(&mut self) -> OPERRIE2_W<CR2_SPEC, 22> {
        OPERRIE2_W::new(self)
    }
    #[doc = "Bit 23 - Bank 2 read protection error interrupt enable bit"]
    #[inline(always)]
    #[must_use]
    pub fn rdperrie2(&mut self) -> RDPERRIE2_W<CR2_SPEC, 23> {
        RDPERRIE2_W::new(self)
    }
    #[doc = "Bit 24 - Bank 2 secure error interrupt enable bit"]
    #[inline(always)]
    #[must_use]
    pub fn rdserrie2(&mut self) -> RDSERRIE2_W<CR2_SPEC, 24> {
        RDSERRIE2_W::new(self)
    }
    #[doc = "Bit 25 - Bank 2 ECC single correction error interrupt enable bit"]
    #[inline(always)]
    #[must_use]
    pub fn sneccerrie2(&mut self) -> SNECCERRIE2_W<CR2_SPEC, 25> {
        SNECCERRIE2_W::new(self)
    }
    #[doc = "Bit 26 - Bank 2 ECC double detection error interrupt enable bit"]
    #[inline(always)]
    #[must_use]
    pub fn dbeccerrie2(&mut self) -> DBECCERRIE2_W<CR2_SPEC, 26> {
        DBECCERRIE2_W::new(self)
    }
    #[doc = "Bit 27 - Bank 2 end of CRC calculation interrupt enable bit"]
    #[inline(always)]
    #[must_use]
    pub fn crcendie2(&mut self) -> CRCENDIE2_W<CR2_SPEC, 27> {
        CRCENDIE2_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "FLASH control register for bank 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cr2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cr2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CR2_SPEC;
impl crate::RegisterSpec for CR2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cr2::R`](R) reader structure"]
impl crate::Readable for CR2_SPEC {}
#[doc = "`write(|w| ..)` method takes [`cr2::W`](W) writer structure"]
impl crate::Writable for CR2_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CR2 to value 0"]
impl crate::Resettable for CR2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
