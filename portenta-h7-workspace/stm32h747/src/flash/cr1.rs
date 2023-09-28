#[doc = "Register `CR1` reader"]
pub type R = crate::R<CR1_SPEC>;
#[doc = "Register `CR1` writer"]
pub type W = crate::W<CR1_SPEC>;
#[doc = "Field `LOCK1` reader - Bank 1 configuration lock bit"]
pub type LOCK1_R = crate::BitReader;
#[doc = "Field `LOCK1` writer - Bank 1 configuration lock bit"]
pub type LOCK1_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PG1` reader - Bank 1 program enable bit"]
pub type PG1_R = crate::BitReader;
#[doc = "Field `PG1` writer - Bank 1 program enable bit"]
pub type PG1_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SER1` reader - Bank 1 sector erase request"]
pub type SER1_R = crate::BitReader;
#[doc = "Field `SER1` writer - Bank 1 sector erase request"]
pub type SER1_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `BER1` reader - Bank 1 erase request"]
pub type BER1_R = crate::BitReader;
#[doc = "Field `BER1` writer - Bank 1 erase request"]
pub type BER1_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PSIZE1` reader - Bank 1 program size"]
pub type PSIZE1_R = crate::FieldReader;
#[doc = "Field `PSIZE1` writer - Bank 1 program size"]
pub type PSIZE1_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
#[doc = "Field `FW1` reader - Bank 1 write forcing control bit"]
pub type FW1_R = crate::BitReader;
#[doc = "Field `FW1` writer - Bank 1 write forcing control bit"]
pub type FW1_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `START1` reader - Bank 1 bank or sector erase start control bit"]
pub type START1_R = crate::BitReader;
#[doc = "Field `START1` writer - Bank 1 bank or sector erase start control bit"]
pub type START1_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SNB1` reader - Bank 1 sector erase selection number"]
pub type SNB1_R = crate::FieldReader;
#[doc = "Field `SNB1` writer - Bank 1 sector erase selection number"]
pub type SNB1_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 3, O>;
#[doc = "Field `CRC_EN` reader - Bank 1 CRC control bit"]
pub type CRC_EN_R = crate::BitReader;
#[doc = "Field `CRC_EN` writer - Bank 1 CRC control bit"]
pub type CRC_EN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `EOPIE1` reader - Bank 1 end-of-program interrupt control bit"]
pub type EOPIE1_R = crate::BitReader;
#[doc = "Field `EOPIE1` writer - Bank 1 end-of-program interrupt control bit"]
pub type EOPIE1_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `WRPERRIE1` reader - Bank 1 write protection error interrupt enable bit"]
pub type WRPERRIE1_R = crate::BitReader;
#[doc = "Field `WRPERRIE1` writer - Bank 1 write protection error interrupt enable bit"]
pub type WRPERRIE1_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PGSERRIE1` reader - Bank 1 programming sequence error interrupt enable bit"]
pub type PGSERRIE1_R = crate::BitReader;
#[doc = "Field `PGSERRIE1` writer - Bank 1 programming sequence error interrupt enable bit"]
pub type PGSERRIE1_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `STRBERRIE1` reader - Bank 1 strobe error interrupt enable bit"]
pub type STRBERRIE1_R = crate::BitReader;
#[doc = "Field `STRBERRIE1` writer - Bank 1 strobe error interrupt enable bit"]
pub type STRBERRIE1_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `INCERRIE1` reader - Bank 1 inconsistency error interrupt enable bit"]
pub type INCERRIE1_R = crate::BitReader;
#[doc = "Field `INCERRIE1` writer - Bank 1 inconsistency error interrupt enable bit"]
pub type INCERRIE1_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `OPERRIE1` reader - Bank 1 write/erase error interrupt enable bit"]
pub type OPERRIE1_R = crate::BitReader;
#[doc = "Field `OPERRIE1` writer - Bank 1 write/erase error interrupt enable bit"]
pub type OPERRIE1_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `RDPERRIE1` reader - Bank 1 read protection error interrupt enable bit"]
pub type RDPERRIE1_R = crate::BitReader;
#[doc = "Field `RDPERRIE1` writer - Bank 1 read protection error interrupt enable bit"]
pub type RDPERRIE1_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `RDSERRIE1` reader - Bank 1 secure error interrupt enable bit"]
pub type RDSERRIE1_R = crate::BitReader;
#[doc = "Field `RDSERRIE1` writer - Bank 1 secure error interrupt enable bit"]
pub type RDSERRIE1_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SNECCERRIE1` reader - Bank 1 ECC single correction error interrupt enable bit"]
pub type SNECCERRIE1_R = crate::BitReader;
#[doc = "Field `SNECCERRIE1` writer - Bank 1 ECC single correction error interrupt enable bit"]
pub type SNECCERRIE1_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `DBECCERRIE1` reader - Bank 1 ECC double detection error interrupt enable bit"]
pub type DBECCERRIE1_R = crate::BitReader;
#[doc = "Field `DBECCERRIE1` writer - Bank 1 ECC double detection error interrupt enable bit"]
pub type DBECCERRIE1_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CRCENDIE1` reader - Bank 1 end of CRC calculation interrupt enable bit"]
pub type CRCENDIE1_R = crate::BitReader;
#[doc = "Field `CRCENDIE1` writer - Bank 1 end of CRC calculation interrupt enable bit"]
pub type CRCENDIE1_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - Bank 1 configuration lock bit"]
    #[inline(always)]
    pub fn lock1(&self) -> LOCK1_R {
        LOCK1_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Bank 1 program enable bit"]
    #[inline(always)]
    pub fn pg1(&self) -> PG1_R {
        PG1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Bank 1 sector erase request"]
    #[inline(always)]
    pub fn ser1(&self) -> SER1_R {
        SER1_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Bank 1 erase request"]
    #[inline(always)]
    pub fn ber1(&self) -> BER1_R {
        BER1_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:5 - Bank 1 program size"]
    #[inline(always)]
    pub fn psize1(&self) -> PSIZE1_R {
        PSIZE1_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bit 6 - Bank 1 write forcing control bit"]
    #[inline(always)]
    pub fn fw1(&self) -> FW1_R {
        FW1_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Bank 1 bank or sector erase start control bit"]
    #[inline(always)]
    pub fn start1(&self) -> START1_R {
        START1_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:10 - Bank 1 sector erase selection number"]
    #[inline(always)]
    pub fn snb1(&self) -> SNB1_R {
        SNB1_R::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bit 15 - Bank 1 CRC control bit"]
    #[inline(always)]
    pub fn crc_en(&self) -> CRC_EN_R {
        CRC_EN_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Bank 1 end-of-program interrupt control bit"]
    #[inline(always)]
    pub fn eopie1(&self) -> EOPIE1_R {
        EOPIE1_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Bank 1 write protection error interrupt enable bit"]
    #[inline(always)]
    pub fn wrperrie1(&self) -> WRPERRIE1_R {
        WRPERRIE1_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Bank 1 programming sequence error interrupt enable bit"]
    #[inline(always)]
    pub fn pgserrie1(&self) -> PGSERRIE1_R {
        PGSERRIE1_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Bank 1 strobe error interrupt enable bit"]
    #[inline(always)]
    pub fn strberrie1(&self) -> STRBERRIE1_R {
        STRBERRIE1_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 21 - Bank 1 inconsistency error interrupt enable bit"]
    #[inline(always)]
    pub fn incerrie1(&self) -> INCERRIE1_R {
        INCERRIE1_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Bank 1 write/erase error interrupt enable bit"]
    #[inline(always)]
    pub fn operrie1(&self) -> OPERRIE1_R {
        OPERRIE1_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Bank 1 read protection error interrupt enable bit"]
    #[inline(always)]
    pub fn rdperrie1(&self) -> RDPERRIE1_R {
        RDPERRIE1_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Bank 1 secure error interrupt enable bit"]
    #[inline(always)]
    pub fn rdserrie1(&self) -> RDSERRIE1_R {
        RDSERRIE1_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Bank 1 ECC single correction error interrupt enable bit"]
    #[inline(always)]
    pub fn sneccerrie1(&self) -> SNECCERRIE1_R {
        SNECCERRIE1_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Bank 1 ECC double detection error interrupt enable bit"]
    #[inline(always)]
    pub fn dbeccerrie1(&self) -> DBECCERRIE1_R {
        DBECCERRIE1_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Bank 1 end of CRC calculation interrupt enable bit"]
    #[inline(always)]
    pub fn crcendie1(&self) -> CRCENDIE1_R {
        CRCENDIE1_R::new(((self.bits >> 27) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Bank 1 configuration lock bit"]
    #[inline(always)]
    #[must_use]
    pub fn lock1(&mut self) -> LOCK1_W<CR1_SPEC, 0> {
        LOCK1_W::new(self)
    }
    #[doc = "Bit 1 - Bank 1 program enable bit"]
    #[inline(always)]
    #[must_use]
    pub fn pg1(&mut self) -> PG1_W<CR1_SPEC, 1> {
        PG1_W::new(self)
    }
    #[doc = "Bit 2 - Bank 1 sector erase request"]
    #[inline(always)]
    #[must_use]
    pub fn ser1(&mut self) -> SER1_W<CR1_SPEC, 2> {
        SER1_W::new(self)
    }
    #[doc = "Bit 3 - Bank 1 erase request"]
    #[inline(always)]
    #[must_use]
    pub fn ber1(&mut self) -> BER1_W<CR1_SPEC, 3> {
        BER1_W::new(self)
    }
    #[doc = "Bits 4:5 - Bank 1 program size"]
    #[inline(always)]
    #[must_use]
    pub fn psize1(&mut self) -> PSIZE1_W<CR1_SPEC, 4> {
        PSIZE1_W::new(self)
    }
    #[doc = "Bit 6 - Bank 1 write forcing control bit"]
    #[inline(always)]
    #[must_use]
    pub fn fw1(&mut self) -> FW1_W<CR1_SPEC, 6> {
        FW1_W::new(self)
    }
    #[doc = "Bit 7 - Bank 1 bank or sector erase start control bit"]
    #[inline(always)]
    #[must_use]
    pub fn start1(&mut self) -> START1_W<CR1_SPEC, 7> {
        START1_W::new(self)
    }
    #[doc = "Bits 8:10 - Bank 1 sector erase selection number"]
    #[inline(always)]
    #[must_use]
    pub fn snb1(&mut self) -> SNB1_W<CR1_SPEC, 8> {
        SNB1_W::new(self)
    }
    #[doc = "Bit 15 - Bank 1 CRC control bit"]
    #[inline(always)]
    #[must_use]
    pub fn crc_en(&mut self) -> CRC_EN_W<CR1_SPEC, 15> {
        CRC_EN_W::new(self)
    }
    #[doc = "Bit 16 - Bank 1 end-of-program interrupt control bit"]
    #[inline(always)]
    #[must_use]
    pub fn eopie1(&mut self) -> EOPIE1_W<CR1_SPEC, 16> {
        EOPIE1_W::new(self)
    }
    #[doc = "Bit 17 - Bank 1 write protection error interrupt enable bit"]
    #[inline(always)]
    #[must_use]
    pub fn wrperrie1(&mut self) -> WRPERRIE1_W<CR1_SPEC, 17> {
        WRPERRIE1_W::new(self)
    }
    #[doc = "Bit 18 - Bank 1 programming sequence error interrupt enable bit"]
    #[inline(always)]
    #[must_use]
    pub fn pgserrie1(&mut self) -> PGSERRIE1_W<CR1_SPEC, 18> {
        PGSERRIE1_W::new(self)
    }
    #[doc = "Bit 19 - Bank 1 strobe error interrupt enable bit"]
    #[inline(always)]
    #[must_use]
    pub fn strberrie1(&mut self) -> STRBERRIE1_W<CR1_SPEC, 19> {
        STRBERRIE1_W::new(self)
    }
    #[doc = "Bit 21 - Bank 1 inconsistency error interrupt enable bit"]
    #[inline(always)]
    #[must_use]
    pub fn incerrie1(&mut self) -> INCERRIE1_W<CR1_SPEC, 21> {
        INCERRIE1_W::new(self)
    }
    #[doc = "Bit 22 - Bank 1 write/erase error interrupt enable bit"]
    #[inline(always)]
    #[must_use]
    pub fn operrie1(&mut self) -> OPERRIE1_W<CR1_SPEC, 22> {
        OPERRIE1_W::new(self)
    }
    #[doc = "Bit 23 - Bank 1 read protection error interrupt enable bit"]
    #[inline(always)]
    #[must_use]
    pub fn rdperrie1(&mut self) -> RDPERRIE1_W<CR1_SPEC, 23> {
        RDPERRIE1_W::new(self)
    }
    #[doc = "Bit 24 - Bank 1 secure error interrupt enable bit"]
    #[inline(always)]
    #[must_use]
    pub fn rdserrie1(&mut self) -> RDSERRIE1_W<CR1_SPEC, 24> {
        RDSERRIE1_W::new(self)
    }
    #[doc = "Bit 25 - Bank 1 ECC single correction error interrupt enable bit"]
    #[inline(always)]
    #[must_use]
    pub fn sneccerrie1(&mut self) -> SNECCERRIE1_W<CR1_SPEC, 25> {
        SNECCERRIE1_W::new(self)
    }
    #[doc = "Bit 26 - Bank 1 ECC double detection error interrupt enable bit"]
    #[inline(always)]
    #[must_use]
    pub fn dbeccerrie1(&mut self) -> DBECCERRIE1_W<CR1_SPEC, 26> {
        DBECCERRIE1_W::new(self)
    }
    #[doc = "Bit 27 - Bank 1 end of CRC calculation interrupt enable bit"]
    #[inline(always)]
    #[must_use]
    pub fn crcendie1(&mut self) -> CRCENDIE1_W<CR1_SPEC, 27> {
        CRCENDIE1_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "FLASH control register for bank 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cr1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cr1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
