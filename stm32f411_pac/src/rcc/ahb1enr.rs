#[doc = "Register `AHB1ENR` reader"]
pub type R = crate::R<AHB1ENR_SPEC>;
#[doc = "Register `AHB1ENR` writer"]
pub type W = crate::W<AHB1ENR_SPEC>;
#[doc = "Field `GPIOAEN` reader - IO port A clock enable"]
pub type GPIOAEN_R = crate::BitReader;
#[doc = "Field `GPIOAEN` writer - IO port A clock enable"]
pub type GPIOAEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIOBEN` reader - IO port B clock enable"]
pub type GPIOBEN_R = crate::BitReader;
#[doc = "Field `GPIOBEN` writer - IO port B clock enable"]
pub type GPIOBEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIOCEN` reader - IO port C clock enable"]
pub type GPIOCEN_R = crate::BitReader;
#[doc = "Field `GPIOCEN` writer - IO port C clock enable"]
pub type GPIOCEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIODEN` reader - IO port D clock enable"]
pub type GPIODEN_R = crate::BitReader;
#[doc = "Field `GPIODEN` writer - IO port D clock enable"]
pub type GPIODEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIOEEN` reader - IO port E clock enable"]
pub type GPIOEEN_R = crate::BitReader;
#[doc = "Field `GPIOEEN` writer - IO port E clock enable"]
pub type GPIOEEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIOHEN` reader - IO port H clock enable"]
pub type GPIOHEN_R = crate::BitReader;
#[doc = "Field `GPIOHEN` writer - IO port H clock enable"]
pub type GPIOHEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CRCEN` reader - CRC clock enable"]
pub type CRCEN_R = crate::BitReader;
#[doc = "Field `CRCEN` writer - CRC clock enable"]
pub type CRCEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMA1EN` reader - DMA1 clock enable"]
pub type DMA1EN_R = crate::BitReader;
#[doc = "Field `DMA1EN` writer - DMA1 clock enable"]
pub type DMA1EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMA2EN` reader - DMA2 clock enable"]
pub type DMA2EN_R = crate::BitReader;
#[doc = "Field `DMA2EN` writer - DMA2 clock enable"]
pub type DMA2EN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - IO port A clock enable"]
    #[inline(always)]
    pub fn gpioaen(&self) -> GPIOAEN_R {
        GPIOAEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - IO port B clock enable"]
    #[inline(always)]
    pub fn gpioben(&self) -> GPIOBEN_R {
        GPIOBEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - IO port C clock enable"]
    #[inline(always)]
    pub fn gpiocen(&self) -> GPIOCEN_R {
        GPIOCEN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - IO port D clock enable"]
    #[inline(always)]
    pub fn gpioden(&self) -> GPIODEN_R {
        GPIODEN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - IO port E clock enable"]
    #[inline(always)]
    pub fn gpioeen(&self) -> GPIOEEN_R {
        GPIOEEN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 7 - IO port H clock enable"]
    #[inline(always)]
    pub fn gpiohen(&self) -> GPIOHEN_R {
        GPIOHEN_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 12 - CRC clock enable"]
    #[inline(always)]
    pub fn crcen(&self) -> CRCEN_R {
        CRCEN_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 21 - DMA1 clock enable"]
    #[inline(always)]
    pub fn dma1en(&self) -> DMA1EN_R {
        DMA1EN_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - DMA2 clock enable"]
    #[inline(always)]
    pub fn dma2en(&self) -> DMA2EN_R {
        DMA2EN_R::new(((self.bits >> 22) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - IO port A clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn gpioaen(&mut self) -> GPIOAEN_W<AHB1ENR_SPEC> {
        GPIOAEN_W::new(self, 0)
    }
    #[doc = "Bit 1 - IO port B clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn gpioben(&mut self) -> GPIOBEN_W<AHB1ENR_SPEC> {
        GPIOBEN_W::new(self, 1)
    }
    #[doc = "Bit 2 - IO port C clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn gpiocen(&mut self) -> GPIOCEN_W<AHB1ENR_SPEC> {
        GPIOCEN_W::new(self, 2)
    }
    #[doc = "Bit 3 - IO port D clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn gpioden(&mut self) -> GPIODEN_W<AHB1ENR_SPEC> {
        GPIODEN_W::new(self, 3)
    }
    #[doc = "Bit 4 - IO port E clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn gpioeen(&mut self) -> GPIOEEN_W<AHB1ENR_SPEC> {
        GPIOEEN_W::new(self, 4)
    }
    #[doc = "Bit 7 - IO port H clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn gpiohen(&mut self) -> GPIOHEN_W<AHB1ENR_SPEC> {
        GPIOHEN_W::new(self, 7)
    }
    #[doc = "Bit 12 - CRC clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn crcen(&mut self) -> CRCEN_W<AHB1ENR_SPEC> {
        CRCEN_W::new(self, 12)
    }
    #[doc = "Bit 21 - DMA1 clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn dma1en(&mut self) -> DMA1EN_W<AHB1ENR_SPEC> {
        DMA1EN_W::new(self, 21)
    }
    #[doc = "Bit 22 - DMA2 clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn dma2en(&mut self) -> DMA2EN_W<AHB1ENR_SPEC> {
        DMA2EN_W::new(self, 22)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "AHB1 peripheral clock register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ahb1enr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ahb1enr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AHB1ENR_SPEC;
impl crate::RegisterSpec for AHB1ENR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ahb1enr::R`](R) reader structure"]
impl crate::Readable for AHB1ENR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ahb1enr::W`](W) writer structure"]
impl crate::Writable for AHB1ENR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets AHB1ENR to value 0x0010_0000"]
impl crate::Resettable for AHB1ENR_SPEC {
    const RESET_VALUE: u32 = 0x0010_0000;
}