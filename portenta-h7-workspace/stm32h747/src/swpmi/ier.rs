#[doc = "Register `IER` reader"]
pub type R = crate::R<IER_SPEC>;
#[doc = "Register `IER` writer"]
pub type W = crate::W<IER_SPEC>;
#[doc = "Field `RXBFIE` reader - Receive buffer full interrupt enable"]
pub type RXBFIE_R = crate::BitReader;
#[doc = "Field `RXBFIE` writer - Receive buffer full interrupt enable"]
pub type RXBFIE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TXBEIE` reader - Transmit buffer empty interrupt enable"]
pub type TXBEIE_R = crate::BitReader;
#[doc = "Field `TXBEIE` writer - Transmit buffer empty interrupt enable"]
pub type TXBEIE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `RXBERIE` reader - Receive CRC error interrupt enable"]
pub type RXBERIE_R = crate::BitReader;
#[doc = "Field `RXBERIE` writer - Receive CRC error interrupt enable"]
pub type RXBERIE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `RXOVRIE` reader - Receive overrun error interrupt enable"]
pub type RXOVRIE_R = crate::BitReader;
#[doc = "Field `RXOVRIE` writer - Receive overrun error interrupt enable"]
pub type RXOVRIE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TXUNRIE` reader - Transmit underrun error interrupt enable"]
pub type TXUNRIE_R = crate::BitReader;
#[doc = "Field `TXUNRIE` writer - Transmit underrun error interrupt enable"]
pub type TXUNRIE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `RIE` reader - Receive interrupt enable"]
pub type RIE_R = crate::BitReader;
#[doc = "Field `RIE` writer - Receive interrupt enable"]
pub type RIE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TIE` reader - Transmit interrupt enable"]
pub type TIE_R = crate::BitReader;
#[doc = "Field `TIE` writer - Transmit interrupt enable"]
pub type TIE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TCIE` reader - Transmit complete interrupt enable"]
pub type TCIE_R = crate::BitReader;
#[doc = "Field `TCIE` writer - Transmit complete interrupt enable"]
pub type TCIE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SRIE` reader - Slave resume interrupt enable"]
pub type SRIE_R = crate::BitReader;
#[doc = "Field `SRIE` writer - Slave resume interrupt enable"]
pub type SRIE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `RDYIE` reader - Transceiver ready interrupt enable"]
pub type RDYIE_R = crate::BitReader;
#[doc = "Field `RDYIE` writer - Transceiver ready interrupt enable"]
pub type RDYIE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - Receive buffer full interrupt enable"]
    #[inline(always)]
    pub fn rxbfie(&self) -> RXBFIE_R {
        RXBFIE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Transmit buffer empty interrupt enable"]
    #[inline(always)]
    pub fn txbeie(&self) -> TXBEIE_R {
        TXBEIE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Receive CRC error interrupt enable"]
    #[inline(always)]
    pub fn rxberie(&self) -> RXBERIE_R {
        RXBERIE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Receive overrun error interrupt enable"]
    #[inline(always)]
    pub fn rxovrie(&self) -> RXOVRIE_R {
        RXOVRIE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Transmit underrun error interrupt enable"]
    #[inline(always)]
    pub fn txunrie(&self) -> TXUNRIE_R {
        TXUNRIE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Receive interrupt enable"]
    #[inline(always)]
    pub fn rie(&self) -> RIE_R {
        RIE_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Transmit interrupt enable"]
    #[inline(always)]
    pub fn tie(&self) -> TIE_R {
        TIE_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Transmit complete interrupt enable"]
    #[inline(always)]
    pub fn tcie(&self) -> TCIE_R {
        TCIE_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Slave resume interrupt enable"]
    #[inline(always)]
    pub fn srie(&self) -> SRIE_R {
        SRIE_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 11 - Transceiver ready interrupt enable"]
    #[inline(always)]
    pub fn rdyie(&self) -> RDYIE_R {
        RDYIE_R::new(((self.bits >> 11) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Receive buffer full interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn rxbfie(&mut self) -> RXBFIE_W<IER_SPEC, 0> {
        RXBFIE_W::new(self)
    }
    #[doc = "Bit 1 - Transmit buffer empty interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn txbeie(&mut self) -> TXBEIE_W<IER_SPEC, 1> {
        TXBEIE_W::new(self)
    }
    #[doc = "Bit 2 - Receive CRC error interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn rxberie(&mut self) -> RXBERIE_W<IER_SPEC, 2> {
        RXBERIE_W::new(self)
    }
    #[doc = "Bit 3 - Receive overrun error interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn rxovrie(&mut self) -> RXOVRIE_W<IER_SPEC, 3> {
        RXOVRIE_W::new(self)
    }
    #[doc = "Bit 4 - Transmit underrun error interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn txunrie(&mut self) -> TXUNRIE_W<IER_SPEC, 4> {
        TXUNRIE_W::new(self)
    }
    #[doc = "Bit 5 - Receive interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn rie(&mut self) -> RIE_W<IER_SPEC, 5> {
        RIE_W::new(self)
    }
    #[doc = "Bit 6 - Transmit interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn tie(&mut self) -> TIE_W<IER_SPEC, 6> {
        TIE_W::new(self)
    }
    #[doc = "Bit 7 - Transmit complete interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn tcie(&mut self) -> TCIE_W<IER_SPEC, 7> {
        TCIE_W::new(self)
    }
    #[doc = "Bit 8 - Slave resume interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn srie(&mut self) -> SRIE_W<IER_SPEC, 8> {
        SRIE_W::new(self)
    }
    #[doc = "Bit 11 - Transceiver ready interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn rdyie(&mut self) -> RDYIE_W<IER_SPEC, 11> {
        RDYIE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "SWPMI Interrupt Enable register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ier::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ier::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IER_SPEC;
impl crate::RegisterSpec for IER_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ier::R`](R) reader structure"]
impl crate::Readable for IER_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ier::W`](W) writer structure"]
impl crate::Writable for IER_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets IER to value 0"]
impl crate::Resettable for IER_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
