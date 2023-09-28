#[doc = "Register `RTC_ALRMAR` reader"]
pub type R = crate::R<RTC_ALRMAR_SPEC>;
#[doc = "Register `RTC_ALRMAR` writer"]
pub type W = crate::W<RTC_ALRMAR_SPEC>;
#[doc = "Field `SU` reader - Second units in BCD format."]
pub type SU_R = crate::FieldReader;
#[doc = "Field `SU` writer - Second units in BCD format."]
pub type SU_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O>;
#[doc = "Field `ST` reader - Second tens in BCD format."]
pub type ST_R = crate::FieldReader;
#[doc = "Field `ST` writer - Second tens in BCD format."]
pub type ST_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 3, O>;
#[doc = "Field `MSK1` reader - Alarm A seconds mask"]
pub type MSK1_R = crate::BitReader;
#[doc = "Field `MSK1` writer - Alarm A seconds mask"]
pub type MSK1_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `MNU` reader - Minute units in BCD format."]
pub type MNU_R = crate::FieldReader;
#[doc = "Field `MNU` writer - Minute units in BCD format."]
pub type MNU_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O>;
#[doc = "Field `MNT` reader - Minute tens in BCD format."]
pub type MNT_R = crate::FieldReader;
#[doc = "Field `MNT` writer - Minute tens in BCD format."]
pub type MNT_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 3, O>;
#[doc = "Field `MSK2` reader - Alarm A minutes mask"]
pub type MSK2_R = crate::BitReader;
#[doc = "Field `MSK2` writer - Alarm A minutes mask"]
pub type MSK2_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `HU` reader - Hour units in BCD format."]
pub type HU_R = crate::FieldReader;
#[doc = "Field `HU` writer - Hour units in BCD format."]
pub type HU_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O>;
#[doc = "Field `HT` reader - Hour tens in BCD format."]
pub type HT_R = crate::FieldReader;
#[doc = "Field `HT` writer - Hour tens in BCD format."]
pub type HT_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
#[doc = "Field `PM` reader - AM/PM notation"]
pub type PM_R = crate::BitReader;
#[doc = "Field `PM` writer - AM/PM notation"]
pub type PM_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `MSK3` reader - Alarm A hours mask"]
pub type MSK3_R = crate::BitReader;
#[doc = "Field `MSK3` writer - Alarm A hours mask"]
pub type MSK3_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `DU` reader - Date units or day in BCD format."]
pub type DU_R = crate::FieldReader;
#[doc = "Field `DU` writer - Date units or day in BCD format."]
pub type DU_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O>;
#[doc = "Field `DT` reader - Date tens in BCD format."]
pub type DT_R = crate::FieldReader;
#[doc = "Field `DT` writer - Date tens in BCD format."]
pub type DT_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
#[doc = "Field `WDSEL` reader - Week day selection"]
pub type WDSEL_R = crate::BitReader;
#[doc = "Field `WDSEL` writer - Week day selection"]
pub type WDSEL_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `MSK4` reader - Alarm A date mask"]
pub type MSK4_R = crate::BitReader;
#[doc = "Field `MSK4` writer - Alarm A date mask"]
pub type MSK4_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bits 0:3 - Second units in BCD format."]
    #[inline(always)]
    pub fn su(&self) -> SU_R {
        SU_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:6 - Second tens in BCD format."]
    #[inline(always)]
    pub fn st(&self) -> ST_R {
        ST_R::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bit 7 - Alarm A seconds mask"]
    #[inline(always)]
    pub fn msk1(&self) -> MSK1_R {
        MSK1_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:11 - Minute units in BCD format."]
    #[inline(always)]
    pub fn mnu(&self) -> MNU_R {
        MNU_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:14 - Minute tens in BCD format."]
    #[inline(always)]
    pub fn mnt(&self) -> MNT_R {
        MNT_R::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bit 15 - Alarm A minutes mask"]
    #[inline(always)]
    pub fn msk2(&self) -> MSK2_R {
        MSK2_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:19 - Hour units in BCD format."]
    #[inline(always)]
    pub fn hu(&self) -> HU_R {
        HU_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:21 - Hour tens in BCD format."]
    #[inline(always)]
    pub fn ht(&self) -> HT_R {
        HT_R::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bit 22 - AM/PM notation"]
    #[inline(always)]
    pub fn pm(&self) -> PM_R {
        PM_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Alarm A hours mask"]
    #[inline(always)]
    pub fn msk3(&self) -> MSK3_R {
        MSK3_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bits 24:27 - Date units or day in BCD format."]
    #[inline(always)]
    pub fn du(&self) -> DU_R {
        DU_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:29 - Date tens in BCD format."]
    #[inline(always)]
    pub fn dt(&self) -> DT_R {
        DT_R::new(((self.bits >> 28) & 3) as u8)
    }
    #[doc = "Bit 30 - Week day selection"]
    #[inline(always)]
    pub fn wdsel(&self) -> WDSEL_R {
        WDSEL_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Alarm A date mask"]
    #[inline(always)]
    pub fn msk4(&self) -> MSK4_R {
        MSK4_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - Second units in BCD format."]
    #[inline(always)]
    #[must_use]
    pub fn su(&mut self) -> SU_W<RTC_ALRMAR_SPEC, 0> {
        SU_W::new(self)
    }
    #[doc = "Bits 4:6 - Second tens in BCD format."]
    #[inline(always)]
    #[must_use]
    pub fn st(&mut self) -> ST_W<RTC_ALRMAR_SPEC, 4> {
        ST_W::new(self)
    }
    #[doc = "Bit 7 - Alarm A seconds mask"]
    #[inline(always)]
    #[must_use]
    pub fn msk1(&mut self) -> MSK1_W<RTC_ALRMAR_SPEC, 7> {
        MSK1_W::new(self)
    }
    #[doc = "Bits 8:11 - Minute units in BCD format."]
    #[inline(always)]
    #[must_use]
    pub fn mnu(&mut self) -> MNU_W<RTC_ALRMAR_SPEC, 8> {
        MNU_W::new(self)
    }
    #[doc = "Bits 12:14 - Minute tens in BCD format."]
    #[inline(always)]
    #[must_use]
    pub fn mnt(&mut self) -> MNT_W<RTC_ALRMAR_SPEC, 12> {
        MNT_W::new(self)
    }
    #[doc = "Bit 15 - Alarm A minutes mask"]
    #[inline(always)]
    #[must_use]
    pub fn msk2(&mut self) -> MSK2_W<RTC_ALRMAR_SPEC, 15> {
        MSK2_W::new(self)
    }
    #[doc = "Bits 16:19 - Hour units in BCD format."]
    #[inline(always)]
    #[must_use]
    pub fn hu(&mut self) -> HU_W<RTC_ALRMAR_SPEC, 16> {
        HU_W::new(self)
    }
    #[doc = "Bits 20:21 - Hour tens in BCD format."]
    #[inline(always)]
    #[must_use]
    pub fn ht(&mut self) -> HT_W<RTC_ALRMAR_SPEC, 20> {
        HT_W::new(self)
    }
    #[doc = "Bit 22 - AM/PM notation"]
    #[inline(always)]
    #[must_use]
    pub fn pm(&mut self) -> PM_W<RTC_ALRMAR_SPEC, 22> {
        PM_W::new(self)
    }
    #[doc = "Bit 23 - Alarm A hours mask"]
    #[inline(always)]
    #[must_use]
    pub fn msk3(&mut self) -> MSK3_W<RTC_ALRMAR_SPEC, 23> {
        MSK3_W::new(self)
    }
    #[doc = "Bits 24:27 - Date units or day in BCD format."]
    #[inline(always)]
    #[must_use]
    pub fn du(&mut self) -> DU_W<RTC_ALRMAR_SPEC, 24> {
        DU_W::new(self)
    }
    #[doc = "Bits 28:29 - Date tens in BCD format."]
    #[inline(always)]
    #[must_use]
    pub fn dt(&mut self) -> DT_W<RTC_ALRMAR_SPEC, 28> {
        DT_W::new(self)
    }
    #[doc = "Bit 30 - Week day selection"]
    #[inline(always)]
    #[must_use]
    pub fn wdsel(&mut self) -> WDSEL_W<RTC_ALRMAR_SPEC, 30> {
        WDSEL_W::new(self)
    }
    #[doc = "Bit 31 - Alarm A date mask"]
    #[inline(always)]
    #[must_use]
    pub fn msk4(&mut self) -> MSK4_W<RTC_ALRMAR_SPEC, 31> {
        MSK4_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "This register can be written only when ALRAWF is set to 1 in RTC_ISR, or in initialization mode.This register is write protected. The write access procedure is described in RTC register write protection on page9.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rtc_alrmar::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rtc_alrmar::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RTC_ALRMAR_SPEC;
impl crate::RegisterSpec for RTC_ALRMAR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rtc_alrmar::R`](R) reader structure"]
impl crate::Readable for RTC_ALRMAR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`rtc_alrmar::W`](W) writer structure"]
impl crate::Writable for RTC_ALRMAR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RTC_ALRMAR to value 0"]
impl crate::Resettable for RTC_ALRMAR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
