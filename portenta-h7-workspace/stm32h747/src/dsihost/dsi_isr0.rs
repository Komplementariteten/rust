#[doc = "Register `DSI_ISR0` reader"]
pub type R = crate::R<DSI_ISR0_SPEC>;
#[doc = "Field `AE0` reader - AE0"]
pub type AE0_R = crate::BitReader;
#[doc = "Field `AE1` reader - AE1"]
pub type AE1_R = crate::BitReader;
#[doc = "Field `AE2` reader - AE2"]
pub type AE2_R = crate::BitReader;
#[doc = "Field `AE3` reader - AE3"]
pub type AE3_R = crate::BitReader;
#[doc = "Field `AE4` reader - AE4"]
pub type AE4_R = crate::BitReader;
#[doc = "Field `AE5` reader - AE5"]
pub type AE5_R = crate::BitReader;
#[doc = "Field `AE6` reader - AE6"]
pub type AE6_R = crate::BitReader;
#[doc = "Field `AE7` reader - AE7"]
pub type AE7_R = crate::BitReader;
#[doc = "Field `AE8` reader - AE8"]
pub type AE8_R = crate::BitReader;
#[doc = "Field `AE9` reader - AE9"]
pub type AE9_R = crate::BitReader;
#[doc = "Field `AE10` reader - AE10"]
pub type AE10_R = crate::BitReader;
#[doc = "Field `AE11` reader - AE11"]
pub type AE11_R = crate::BitReader;
#[doc = "Field `AE12` reader - AE12"]
pub type AE12_R = crate::BitReader;
#[doc = "Field `AE13` reader - AE13"]
pub type AE13_R = crate::BitReader;
#[doc = "Field `AE14` reader - AE14"]
pub type AE14_R = crate::BitReader;
#[doc = "Field `AE15` reader - AE15"]
pub type AE15_R = crate::BitReader;
#[doc = "Field `PE0` reader - PE0"]
pub type PE0_R = crate::BitReader;
#[doc = "Field `PE1` reader - PE1"]
pub type PE1_R = crate::BitReader;
#[doc = "Field `PE2` reader - PE2"]
pub type PE2_R = crate::BitReader;
#[doc = "Field `PE3` reader - PE3"]
pub type PE3_R = crate::BitReader;
#[doc = "Field `PE4` reader - PE4"]
pub type PE4_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - AE0"]
    #[inline(always)]
    pub fn ae0(&self) -> AE0_R {
        AE0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - AE1"]
    #[inline(always)]
    pub fn ae1(&self) -> AE1_R {
        AE1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - AE2"]
    #[inline(always)]
    pub fn ae2(&self) -> AE2_R {
        AE2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - AE3"]
    #[inline(always)]
    pub fn ae3(&self) -> AE3_R {
        AE3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - AE4"]
    #[inline(always)]
    pub fn ae4(&self) -> AE4_R {
        AE4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - AE5"]
    #[inline(always)]
    pub fn ae5(&self) -> AE5_R {
        AE5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - AE6"]
    #[inline(always)]
    pub fn ae6(&self) -> AE6_R {
        AE6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - AE7"]
    #[inline(always)]
    pub fn ae7(&self) -> AE7_R {
        AE7_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - AE8"]
    #[inline(always)]
    pub fn ae8(&self) -> AE8_R {
        AE8_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - AE9"]
    #[inline(always)]
    pub fn ae9(&self) -> AE9_R {
        AE9_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - AE10"]
    #[inline(always)]
    pub fn ae10(&self) -> AE10_R {
        AE10_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - AE11"]
    #[inline(always)]
    pub fn ae11(&self) -> AE11_R {
        AE11_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - AE12"]
    #[inline(always)]
    pub fn ae12(&self) -> AE12_R {
        AE12_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - AE13"]
    #[inline(always)]
    pub fn ae13(&self) -> AE13_R {
        AE13_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - AE14"]
    #[inline(always)]
    pub fn ae14(&self) -> AE14_R {
        AE14_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - AE15"]
    #[inline(always)]
    pub fn ae15(&self) -> AE15_R {
        AE15_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - PE0"]
    #[inline(always)]
    pub fn pe0(&self) -> PE0_R {
        PE0_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - PE1"]
    #[inline(always)]
    pub fn pe1(&self) -> PE1_R {
        PE1_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - PE2"]
    #[inline(always)]
    pub fn pe2(&self) -> PE2_R {
        PE2_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - PE3"]
    #[inline(always)]
    pub fn pe3(&self) -> PE3_R {
        PE3_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - PE4"]
    #[inline(always)]
    pub fn pe4(&self) -> PE4_R {
        PE4_R::new(((self.bits >> 20) & 1) != 0)
    }
}
#[doc = "DSI Host interrupt and status register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dsi_isr0::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DSI_ISR0_SPEC;
impl crate::RegisterSpec for DSI_ISR0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dsi_isr0::R`](R) reader structure"]
impl crate::Readable for DSI_ISR0_SPEC {}
#[doc = "`reset()` method sets DSI_ISR0 to value 0"]
impl crate::Resettable for DSI_ISR0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
