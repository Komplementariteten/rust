#[doc = "Register `CPUIMR3` reader"]
pub type R = crate::R<CPUIMR3_SPEC>;
#[doc = "Field `MR64` reader - CPU Interrupt Mask on Direct Event input x+64"]
pub type MR64_R = crate::BitReader;
#[doc = "Field `MR65` reader - CPU Interrupt Mask on Direct Event input x+64"]
pub type MR65_R = crate::BitReader;
#[doc = "Field `MR66` reader - CPU Interrupt Mask on Direct Event input x+64"]
pub type MR66_R = crate::BitReader;
#[doc = "Field `MR67` reader - CPU Interrupt Mask on Direct Event input x+64"]
pub type MR67_R = crate::BitReader;
#[doc = "Field `MR68` reader - CPU Interrupt Mask on Direct Event input x+64"]
pub type MR68_R = crate::BitReader;
#[doc = "Field `MR69` reader - CPU Interrupt Mask on Direct Event input x+64"]
pub type MR69_R = crate::BitReader;
#[doc = "Field `MR70` reader - CPU Interrupt Mask on Direct Event input x+64"]
pub type MR70_R = crate::BitReader;
#[doc = "Field `MR71` reader - CPU Interrupt Mask on Direct Event input x+64"]
pub type MR71_R = crate::BitReader;
#[doc = "Field `MR72` reader - CPU Interrupt Mask on Direct Event input x+64"]
pub type MR72_R = crate::BitReader;
#[doc = "Field `MR73` reader - CPU Interrupt Mask on Direct Event input x+64"]
pub type MR73_R = crate::BitReader;
#[doc = "Field `MR74` reader - CPU Interrupt Mask on Direct Event input x+64"]
pub type MR74_R = crate::BitReader;
#[doc = "Field `MR75` reader - CPU Interrupt Mask on Direct Event input x+64"]
pub type MR75_R = crate::BitReader;
#[doc = "Field `MR76` reader - CPU Interrupt Mask on Direct Event input x+64"]
pub type MR76_R = crate::BitReader;
#[doc = "Field `MR77` reader - CPU Interrupt Mask on Direct Event input x+64"]
pub type MR77_R = crate::BitReader;
#[doc = "Field `MR78` reader - CPU Interrupt Mask on Direct Event input x+64"]
pub type MR78_R = crate::BitReader;
#[doc = "Field `MR79` reader - CPU Interrupt Mask on Direct Event input x+64"]
pub type MR79_R = crate::BitReader;
#[doc = "Field `MR80` reader - CPU Interrupt Mask on Direct Event input x+64"]
pub type MR80_R = crate::BitReader;
#[doc = "Field `MR82` reader - CPU Interrupt Mask on Direct Event input x+64"]
pub type MR82_R = crate::BitReader;
#[doc = "Field `MR84` reader - CPU Interrupt Mask on Direct Event input x+64"]
pub type MR84_R = crate::BitReader;
#[doc = "Field `MR85` reader - CPU Interrupt Mask on Direct Event input x+64"]
pub type MR85_R = crate::BitReader;
#[doc = "Field `MR86` reader - CPU Interrupt Mask on Direct Event input x+64"]
pub type MR86_R = crate::BitReader;
#[doc = "Field `MR87` reader - CPU Interrupt Mask on Direct Event input x+64"]
pub type MR87_R = crate::BitReader;
#[doc = "Field `MR88` reader - CPU Interrupt Mask on Direct Event input x+64"]
pub type MR88_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - CPU Interrupt Mask on Direct Event input x+64"]
    #[inline(always)]
    pub fn mr64(&self) -> MR64_R {
        MR64_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - CPU Interrupt Mask on Direct Event input x+64"]
    #[inline(always)]
    pub fn mr65(&self) -> MR65_R {
        MR65_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - CPU Interrupt Mask on Direct Event input x+64"]
    #[inline(always)]
    pub fn mr66(&self) -> MR66_R {
        MR66_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - CPU Interrupt Mask on Direct Event input x+64"]
    #[inline(always)]
    pub fn mr67(&self) -> MR67_R {
        MR67_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - CPU Interrupt Mask on Direct Event input x+64"]
    #[inline(always)]
    pub fn mr68(&self) -> MR68_R {
        MR68_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - CPU Interrupt Mask on Direct Event input x+64"]
    #[inline(always)]
    pub fn mr69(&self) -> MR69_R {
        MR69_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - CPU Interrupt Mask on Direct Event input x+64"]
    #[inline(always)]
    pub fn mr70(&self) -> MR70_R {
        MR70_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - CPU Interrupt Mask on Direct Event input x+64"]
    #[inline(always)]
    pub fn mr71(&self) -> MR71_R {
        MR71_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - CPU Interrupt Mask on Direct Event input x+64"]
    #[inline(always)]
    pub fn mr72(&self) -> MR72_R {
        MR72_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - CPU Interrupt Mask on Direct Event input x+64"]
    #[inline(always)]
    pub fn mr73(&self) -> MR73_R {
        MR73_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - CPU Interrupt Mask on Direct Event input x+64"]
    #[inline(always)]
    pub fn mr74(&self) -> MR74_R {
        MR74_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - CPU Interrupt Mask on Direct Event input x+64"]
    #[inline(always)]
    pub fn mr75(&self) -> MR75_R {
        MR75_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - CPU Interrupt Mask on Direct Event input x+64"]
    #[inline(always)]
    pub fn mr76(&self) -> MR76_R {
        MR76_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - CPU Interrupt Mask on Direct Event input x+64"]
    #[inline(always)]
    pub fn mr77(&self) -> MR77_R {
        MR77_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - CPU Interrupt Mask on Direct Event input x+64"]
    #[inline(always)]
    pub fn mr78(&self) -> MR78_R {
        MR78_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - CPU Interrupt Mask on Direct Event input x+64"]
    #[inline(always)]
    pub fn mr79(&self) -> MR79_R {
        MR79_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - CPU Interrupt Mask on Direct Event input x+64"]
    #[inline(always)]
    pub fn mr80(&self) -> MR80_R {
        MR80_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 18 - CPU Interrupt Mask on Direct Event input x+64"]
    #[inline(always)]
    pub fn mr82(&self) -> MR82_R {
        MR82_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 20 - CPU Interrupt Mask on Direct Event input x+64"]
    #[inline(always)]
    pub fn mr84(&self) -> MR84_R {
        MR84_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - CPU Interrupt Mask on Direct Event input x+64"]
    #[inline(always)]
    pub fn mr85(&self) -> MR85_R {
        MR85_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - CPU Interrupt Mask on Direct Event input x+64"]
    #[inline(always)]
    pub fn mr86(&self) -> MR86_R {
        MR86_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - CPU Interrupt Mask on Direct Event input x+64"]
    #[inline(always)]
    pub fn mr87(&self) -> MR87_R {
        MR87_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - CPU Interrupt Mask on Direct Event input x+64"]
    #[inline(always)]
    pub fn mr88(&self) -> MR88_R {
        MR88_R::new(((self.bits >> 24) & 1) != 0)
    }
}
#[doc = "EXTI interrupt mask register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cpuimr3::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CPUIMR3_SPEC;
impl crate::RegisterSpec for CPUIMR3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cpuimr3::R`](R) reader structure"]
impl crate::Readable for CPUIMR3_SPEC {}
#[doc = "`reset()` method sets CPUIMR3 to value 0"]
impl crate::Resettable for CPUIMR3_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
