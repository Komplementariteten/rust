#[doc = "Register `UR3` reader"]
pub type R = crate::R<UR3_SPEC>;
#[doc = "Register `UR3` writer"]
pub type W = crate::W<UR3_SPEC>;
#[doc = "Field `BCM4_ADD1` reader - Cortex-M4 Boot Address 0"]
pub type BCM4_ADD1_R = crate::FieldReader<u16>;
#[doc = "Field `BCM4_ADD1` writer - Cortex-M4 Boot Address 0"]
pub type BCM4_ADD1_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 16, O, u16>;
#[doc = "Field `BCM7_ADD1` reader - Cortex-M7 Boot Address 1"]
pub type BCM7_ADD1_R = crate::FieldReader<u16>;
#[doc = "Field `BCM7_ADD1` writer - Cortex-M7 Boot Address 1"]
pub type BCM7_ADD1_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 16, O, u16>;
impl R {
    #[doc = "Bits 0:15 - Cortex-M4 Boot Address 0"]
    #[inline(always)]
    pub fn bcm4_add1(&self) -> BCM4_ADD1_R {
        BCM4_ADD1_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - Cortex-M7 Boot Address 1"]
    #[inline(always)]
    pub fn bcm7_add1(&self) -> BCM7_ADD1_R {
        BCM7_ADD1_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Cortex-M4 Boot Address 0"]
    #[inline(always)]
    #[must_use]
    pub fn bcm4_add1(&mut self) -> BCM4_ADD1_W<UR3_SPEC, 0> {
        BCM4_ADD1_W::new(self)
    }
    #[doc = "Bits 16:31 - Cortex-M7 Boot Address 1"]
    #[inline(always)]
    #[must_use]
    pub fn bcm7_add1(&mut self) -> BCM7_ADD1_W<UR3_SPEC, 16> {
        BCM7_ADD1_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "SYSCFG user register 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ur3::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ur3::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UR3_SPEC;
impl crate::RegisterSpec for UR3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ur3::R`](R) reader structure"]
impl crate::Readable for UR3_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ur3::W`](W) writer structure"]
impl crate::Writable for UR3_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets UR3 to value 0"]
impl crate::Resettable for UR3_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
