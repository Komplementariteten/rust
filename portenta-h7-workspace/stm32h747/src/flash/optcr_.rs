#[doc = "Register `OPTCR_` reader"]
pub type R = crate::R<OPTCR__SPEC>;
#[doc = "Register `OPTCR_` writer"]
pub type W = crate::W<OPTCR__SPEC>;
#[doc = "Field `OPTLOCK` reader - FLASH_OPTCR lock option configuration bit"]
pub type OPTLOCK_R = crate::BitReader;
#[doc = "Field `OPTLOCK` writer - FLASH_OPTCR lock option configuration bit"]
pub type OPTLOCK_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `OPTSTART` reader - Option byte start change option configuration bit"]
pub type OPTSTART_R = crate::BitReader;
#[doc = "Field `OPTSTART` writer - Option byte start change option configuration bit"]
pub type OPTSTART_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `MER` reader - Flash mass erase enable bit"]
pub type MER_R = crate::BitReader;
#[doc = "Field `MER` writer - Flash mass erase enable bit"]
pub type MER_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `OPTCHANGEERRIE` reader - Option byte change error interrupt enable bit"]
pub type OPTCHANGEERRIE_R = crate::BitReader;
#[doc = "Field `OPTCHANGEERRIE` writer - Option byte change error interrupt enable bit"]
pub type OPTCHANGEERRIE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SWAP_BANK` reader - Bank swapping configuration bit"]
pub type SWAP_BANK_R = crate::BitReader;
#[doc = "Field `SWAP_BANK` writer - Bank swapping configuration bit"]
pub type SWAP_BANK_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - FLASH_OPTCR lock option configuration bit"]
    #[inline(always)]
    pub fn optlock(&self) -> OPTLOCK_R {
        OPTLOCK_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Option byte start change option configuration bit"]
    #[inline(always)]
    pub fn optstart(&self) -> OPTSTART_R {
        OPTSTART_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 4 - Flash mass erase enable bit"]
    #[inline(always)]
    pub fn mer(&self) -> MER_R {
        MER_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 30 - Option byte change error interrupt enable bit"]
    #[inline(always)]
    pub fn optchangeerrie(&self) -> OPTCHANGEERRIE_R {
        OPTCHANGEERRIE_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Bank swapping configuration bit"]
    #[inline(always)]
    pub fn swap_bank(&self) -> SWAP_BANK_R {
        SWAP_BANK_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - FLASH_OPTCR lock option configuration bit"]
    #[inline(always)]
    #[must_use]
    pub fn optlock(&mut self) -> OPTLOCK_W<OPTCR__SPEC, 0> {
        OPTLOCK_W::new(self)
    }
    #[doc = "Bit 1 - Option byte start change option configuration bit"]
    #[inline(always)]
    #[must_use]
    pub fn optstart(&mut self) -> OPTSTART_W<OPTCR__SPEC, 1> {
        OPTSTART_W::new(self)
    }
    #[doc = "Bit 4 - Flash mass erase enable bit"]
    #[inline(always)]
    #[must_use]
    pub fn mer(&mut self) -> MER_W<OPTCR__SPEC, 4> {
        MER_W::new(self)
    }
    #[doc = "Bit 30 - Option byte change error interrupt enable bit"]
    #[inline(always)]
    #[must_use]
    pub fn optchangeerrie(&mut self) -> OPTCHANGEERRIE_W<OPTCR__SPEC, 30> {
        OPTCHANGEERRIE_W::new(self)
    }
    #[doc = "Bit 31 - Bank swapping configuration bit"]
    #[inline(always)]
    #[must_use]
    pub fn swap_bank(&mut self) -> SWAP_BANK_W<OPTCR__SPEC, 31> {
        SWAP_BANK_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "FLASH option control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`optcr_::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`optcr_::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OPTCR__SPEC;
impl crate::RegisterSpec for OPTCR__SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`optcr_::R`](R) reader structure"]
impl crate::Readable for OPTCR__SPEC {}
#[doc = "`write(|w| ..)` method takes [`optcr_::W`](W) writer structure"]
impl crate::Writable for OPTCR__SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets OPTCR_ to value 0"]
impl crate::Resettable for OPTCR__SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
