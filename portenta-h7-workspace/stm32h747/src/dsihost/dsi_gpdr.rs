#[doc = "Register `DSI_GPDR` reader"]
pub type R = crate::R<DSI_GPDR_SPEC>;
#[doc = "Register `DSI_GPDR` writer"]
pub type W = crate::W<DSI_GPDR_SPEC>;
#[doc = "Field `DATA1` reader - DATA1"]
pub type DATA1_R = crate::FieldReader;
#[doc = "Field `DATA1` writer - DATA1"]
pub type DATA1_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
#[doc = "Field `DATA2` reader - DATA2"]
pub type DATA2_R = crate::FieldReader;
#[doc = "Field `DATA2` writer - DATA2"]
pub type DATA2_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
#[doc = "Field `DATA3` reader - DATA3"]
pub type DATA3_R = crate::FieldReader;
#[doc = "Field `DATA3` writer - DATA3"]
pub type DATA3_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
#[doc = "Field `DATA4` reader - DATA4"]
pub type DATA4_R = crate::FieldReader;
#[doc = "Field `DATA4` writer - DATA4"]
pub type DATA4_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
impl R {
    #[doc = "Bits 0:7 - DATA1"]
    #[inline(always)]
    pub fn data1(&self) -> DATA1_R {
        DATA1_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - DATA2"]
    #[inline(always)]
    pub fn data2(&self) -> DATA2_R {
        DATA2_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - DATA3"]
    #[inline(always)]
    pub fn data3(&self) -> DATA3_R {
        DATA3_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - DATA4"]
    #[inline(always)]
    pub fn data4(&self) -> DATA4_R {
        DATA4_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - DATA1"]
    #[inline(always)]
    #[must_use]
    pub fn data1(&mut self) -> DATA1_W<DSI_GPDR_SPEC, 0> {
        DATA1_W::new(self)
    }
    #[doc = "Bits 8:15 - DATA2"]
    #[inline(always)]
    #[must_use]
    pub fn data2(&mut self) -> DATA2_W<DSI_GPDR_SPEC, 8> {
        DATA2_W::new(self)
    }
    #[doc = "Bits 16:23 - DATA3"]
    #[inline(always)]
    #[must_use]
    pub fn data3(&mut self) -> DATA3_W<DSI_GPDR_SPEC, 16> {
        DATA3_W::new(self)
    }
    #[doc = "Bits 24:31 - DATA4"]
    #[inline(always)]
    #[must_use]
    pub fn data4(&mut self) -> DATA4_W<DSI_GPDR_SPEC, 24> {
        DATA4_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "DSI Host generic payload data register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dsi_gpdr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dsi_gpdr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DSI_GPDR_SPEC;
impl crate::RegisterSpec for DSI_GPDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dsi_gpdr::R`](R) reader structure"]
impl crate::Readable for DSI_GPDR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dsi_gpdr::W`](W) writer structure"]
impl crate::Writable for DSI_GPDR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DSI_GPDR to value 0"]
impl crate::Resettable for DSI_GPDR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
