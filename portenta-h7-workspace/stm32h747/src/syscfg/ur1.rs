#[doc = "Register `UR1` reader"]
pub type R = crate::R<UR1_SPEC>;
#[doc = "Register `UR1` writer"]
pub type W = crate::W<UR1_SPEC>;
#[doc = "Field `BCM4` reader - Boot Cortex-M4"]
pub type BCM4_R = crate::BitReader;
#[doc = "Field `BCM4` writer - Boot Cortex-M4"]
pub type BCM4_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `BCM7` reader - Boot Cortex-M7"]
pub type BCM7_R = crate::BitReader;
#[doc = "Field `BCM7` writer - Boot Cortex-M7"]
pub type BCM7_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - Boot Cortex-M4"]
    #[inline(always)]
    pub fn bcm4(&self) -> BCM4_R {
        BCM4_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 16 - Boot Cortex-M7"]
    #[inline(always)]
    pub fn bcm7(&self) -> BCM7_R {
        BCM7_R::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Boot Cortex-M4"]
    #[inline(always)]
    #[must_use]
    pub fn bcm4(&mut self) -> BCM4_W<UR1_SPEC, 0> {
        BCM4_W::new(self)
    }
    #[doc = "Bit 16 - Boot Cortex-M7"]
    #[inline(always)]
    #[must_use]
    pub fn bcm7(&mut self) -> BCM7_W<UR1_SPEC, 16> {
        BCM7_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "SYSCFG user register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ur1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ur1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UR1_SPEC;
impl crate::RegisterSpec for UR1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ur1::R`](R) reader structure"]
impl crate::Readable for UR1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ur1::W`](W) writer structure"]
impl crate::Writable for UR1_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets UR1 to value 0"]
impl crate::Resettable for UR1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
