#[doc = "Register `D3PMR3` reader"]
pub type R = crate::R<D3PMR3_SPEC>;
#[doc = "Register `D3PMR3` writer"]
pub type W = crate::W<D3PMR3_SPEC>;
#[doc = "Field `MR88` reader - D3 Pending Mask on Event input x+64"]
pub type MR88_R = crate::BitReader;
#[doc = "Field `MR88` writer - D3 Pending Mask on Event input x+64"]
pub type MR88_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 24 - D3 Pending Mask on Event input x+64"]
    #[inline(always)]
    pub fn mr88(&self) -> MR88_R {
        MR88_R::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 24 - D3 Pending Mask on Event input x+64"]
    #[inline(always)]
    #[must_use]
    pub fn mr88(&mut self) -> MR88_W<D3PMR3_SPEC, 24> {
        MR88_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "EXTI D3 pending mask register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`d3pmr3::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`d3pmr3::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct D3PMR3_SPEC;
impl crate::RegisterSpec for D3PMR3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`d3pmr3::R`](R) reader structure"]
impl crate::Readable for D3PMR3_SPEC {}
#[doc = "`write(|w| ..)` method takes [`d3pmr3::W`](W) writer structure"]
impl crate::Writable for D3PMR3_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets D3PMR3 to value 0"]
impl crate::Resettable for D3PMR3_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
