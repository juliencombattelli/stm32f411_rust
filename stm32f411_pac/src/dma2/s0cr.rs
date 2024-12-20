#[doc = "Register `S0CR` reader"]
pub type R = crate::R<S0CR_SPEC>;
#[doc = "Register `S0CR` writer"]
pub type W = crate::W<S0CR_SPEC>;
#[doc = "Field `EN` reader - Stream enable / flag stream ready when read low"]
pub type EN_R = crate::BitReader;
#[doc = "Field `EN` writer - Stream enable / flag stream ready when read low"]
pub type EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMEIE` reader - Direct mode error interrupt enable"]
pub type DMEIE_R = crate::BitReader;
#[doc = "Field `DMEIE` writer - Direct mode error interrupt enable"]
pub type DMEIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TEIE` reader - Transfer error interrupt enable"]
pub type TEIE_R = crate::BitReader;
#[doc = "Field `TEIE` writer - Transfer error interrupt enable"]
pub type TEIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HTIE` reader - Half transfer interrupt enable"]
pub type HTIE_R = crate::BitReader;
#[doc = "Field `HTIE` writer - Half transfer interrupt enable"]
pub type HTIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TCIE` reader - Transfer complete interrupt enable"]
pub type TCIE_R = crate::BitReader;
#[doc = "Field `TCIE` writer - Transfer complete interrupt enable"]
pub type TCIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PFCTRL` reader - Peripheral flow controller"]
pub type PFCTRL_R = crate::BitReader;
#[doc = "Field `PFCTRL` writer - Peripheral flow controller"]
pub type PFCTRL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DIR` reader - Data transfer direction"]
pub type DIR_R = crate::FieldReader;
#[doc = "Field `DIR` writer - Data transfer direction"]
pub type DIR_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `CIRC` reader - Circular mode"]
pub type CIRC_R = crate::BitReader;
#[doc = "Field `CIRC` writer - Circular mode"]
pub type CIRC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PINC` reader - Peripheral increment mode"]
pub type PINC_R = crate::BitReader;
#[doc = "Field `PINC` writer - Peripheral increment mode"]
pub type PINC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MINC` reader - Memory increment mode"]
pub type MINC_R = crate::BitReader;
#[doc = "Field `MINC` writer - Memory increment mode"]
pub type MINC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PSIZE` reader - Peripheral data size"]
pub type PSIZE_R = crate::FieldReader;
#[doc = "Field `PSIZE` writer - Peripheral data size"]
pub type PSIZE_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `MSIZE` reader - Memory data size"]
pub type MSIZE_R = crate::FieldReader;
#[doc = "Field `MSIZE` writer - Memory data size"]
pub type MSIZE_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PINCOS` reader - Peripheral increment offset size"]
pub type PINCOS_R = crate::BitReader;
#[doc = "Field `PINCOS` writer - Peripheral increment offset size"]
pub type PINCOS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PL` reader - Priority level"]
pub type PL_R = crate::FieldReader;
#[doc = "Field `PL` writer - Priority level"]
pub type PL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `DBM` reader - Double buffer mode"]
pub type DBM_R = crate::BitReader;
#[doc = "Field `DBM` writer - Double buffer mode"]
pub type DBM_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CT` reader - Current target (only in double buffer mode)"]
pub type CT_R = crate::BitReader;
#[doc = "Field `CT` writer - Current target (only in double buffer mode)"]
pub type CT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PBURST` reader - Peripheral burst transfer configuration"]
pub type PBURST_R = crate::FieldReader;
#[doc = "Field `PBURST` writer - Peripheral burst transfer configuration"]
pub type PBURST_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `MBURST` reader - Memory burst transfer configuration"]
pub type MBURST_R = crate::FieldReader;
#[doc = "Field `MBURST` writer - Memory burst transfer configuration"]
pub type MBURST_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `CHSEL` reader - Channel selection"]
pub type CHSEL_R = crate::FieldReader;
#[doc = "Field `CHSEL` writer - Channel selection"]
pub type CHSEL_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bit 0 - Stream enable / flag stream ready when read low"]
    #[inline(always)]
    pub fn en(&self) -> EN_R {
        EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Direct mode error interrupt enable"]
    #[inline(always)]
    pub fn dmeie(&self) -> DMEIE_R {
        DMEIE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Transfer error interrupt enable"]
    #[inline(always)]
    pub fn teie(&self) -> TEIE_R {
        TEIE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Half transfer interrupt enable"]
    #[inline(always)]
    pub fn htie(&self) -> HTIE_R {
        HTIE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Transfer complete interrupt enable"]
    #[inline(always)]
    pub fn tcie(&self) -> TCIE_R {
        TCIE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Peripheral flow controller"]
    #[inline(always)]
    pub fn pfctrl(&self) -> PFCTRL_R {
        PFCTRL_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 6:7 - Data transfer direction"]
    #[inline(always)]
    pub fn dir(&self) -> DIR_R {
        DIR_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bit 8 - Circular mode"]
    #[inline(always)]
    pub fn circ(&self) -> CIRC_R {
        CIRC_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Peripheral increment mode"]
    #[inline(always)]
    pub fn pinc(&self) -> PINC_R {
        PINC_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Memory increment mode"]
    #[inline(always)]
    pub fn minc(&self) -> MINC_R {
        MINC_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bits 11:12 - Peripheral data size"]
    #[inline(always)]
    pub fn psize(&self) -> PSIZE_R {
        PSIZE_R::new(((self.bits >> 11) & 3) as u8)
    }
    #[doc = "Bits 13:14 - Memory data size"]
    #[inline(always)]
    pub fn msize(&self) -> MSIZE_R {
        MSIZE_R::new(((self.bits >> 13) & 3) as u8)
    }
    #[doc = "Bit 15 - Peripheral increment offset size"]
    #[inline(always)]
    pub fn pincos(&self) -> PINCOS_R {
        PINCOS_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:17 - Priority level"]
    #[inline(always)]
    pub fn pl(&self) -> PL_R {
        PL_R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bit 18 - Double buffer mode"]
    #[inline(always)]
    pub fn dbm(&self) -> DBM_R {
        DBM_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Current target (only in double buffer mode)"]
    #[inline(always)]
    pub fn ct(&self) -> CT_R {
        CT_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bits 21:22 - Peripheral burst transfer configuration"]
    #[inline(always)]
    pub fn pburst(&self) -> PBURST_R {
        PBURST_R::new(((self.bits >> 21) & 3) as u8)
    }
    #[doc = "Bits 23:24 - Memory burst transfer configuration"]
    #[inline(always)]
    pub fn mburst(&self) -> MBURST_R {
        MBURST_R::new(((self.bits >> 23) & 3) as u8)
    }
    #[doc = "Bits 25:27 - Channel selection"]
    #[inline(always)]
    pub fn chsel(&self) -> CHSEL_R {
        CHSEL_R::new(((self.bits >> 25) & 7) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Stream enable / flag stream ready when read low"]
    #[inline(always)]
    #[must_use]
    pub fn en(&mut self) -> EN_W<S0CR_SPEC> {
        EN_W::new(self, 0)
    }
    #[doc = "Bit 1 - Direct mode error interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn dmeie(&mut self) -> DMEIE_W<S0CR_SPEC> {
        DMEIE_W::new(self, 1)
    }
    #[doc = "Bit 2 - Transfer error interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn teie(&mut self) -> TEIE_W<S0CR_SPEC> {
        TEIE_W::new(self, 2)
    }
    #[doc = "Bit 3 - Half transfer interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn htie(&mut self) -> HTIE_W<S0CR_SPEC> {
        HTIE_W::new(self, 3)
    }
    #[doc = "Bit 4 - Transfer complete interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn tcie(&mut self) -> TCIE_W<S0CR_SPEC> {
        TCIE_W::new(self, 4)
    }
    #[doc = "Bit 5 - Peripheral flow controller"]
    #[inline(always)]
    #[must_use]
    pub fn pfctrl(&mut self) -> PFCTRL_W<S0CR_SPEC> {
        PFCTRL_W::new(self, 5)
    }
    #[doc = "Bits 6:7 - Data transfer direction"]
    #[inline(always)]
    #[must_use]
    pub fn dir(&mut self) -> DIR_W<S0CR_SPEC> {
        DIR_W::new(self, 6)
    }
    #[doc = "Bit 8 - Circular mode"]
    #[inline(always)]
    #[must_use]
    pub fn circ(&mut self) -> CIRC_W<S0CR_SPEC> {
        CIRC_W::new(self, 8)
    }
    #[doc = "Bit 9 - Peripheral increment mode"]
    #[inline(always)]
    #[must_use]
    pub fn pinc(&mut self) -> PINC_W<S0CR_SPEC> {
        PINC_W::new(self, 9)
    }
    #[doc = "Bit 10 - Memory increment mode"]
    #[inline(always)]
    #[must_use]
    pub fn minc(&mut self) -> MINC_W<S0CR_SPEC> {
        MINC_W::new(self, 10)
    }
    #[doc = "Bits 11:12 - Peripheral data size"]
    #[inline(always)]
    #[must_use]
    pub fn psize(&mut self) -> PSIZE_W<S0CR_SPEC> {
        PSIZE_W::new(self, 11)
    }
    #[doc = "Bits 13:14 - Memory data size"]
    #[inline(always)]
    #[must_use]
    pub fn msize(&mut self) -> MSIZE_W<S0CR_SPEC> {
        MSIZE_W::new(self, 13)
    }
    #[doc = "Bit 15 - Peripheral increment offset size"]
    #[inline(always)]
    #[must_use]
    pub fn pincos(&mut self) -> PINCOS_W<S0CR_SPEC> {
        PINCOS_W::new(self, 15)
    }
    #[doc = "Bits 16:17 - Priority level"]
    #[inline(always)]
    #[must_use]
    pub fn pl(&mut self) -> PL_W<S0CR_SPEC> {
        PL_W::new(self, 16)
    }
    #[doc = "Bit 18 - Double buffer mode"]
    #[inline(always)]
    #[must_use]
    pub fn dbm(&mut self) -> DBM_W<S0CR_SPEC> {
        DBM_W::new(self, 18)
    }
    #[doc = "Bit 19 - Current target (only in double buffer mode)"]
    #[inline(always)]
    #[must_use]
    pub fn ct(&mut self) -> CT_W<S0CR_SPEC> {
        CT_W::new(self, 19)
    }
    #[doc = "Bits 21:22 - Peripheral burst transfer configuration"]
    #[inline(always)]
    #[must_use]
    pub fn pburst(&mut self) -> PBURST_W<S0CR_SPEC> {
        PBURST_W::new(self, 21)
    }
    #[doc = "Bits 23:24 - Memory burst transfer configuration"]
    #[inline(always)]
    #[must_use]
    pub fn mburst(&mut self) -> MBURST_W<S0CR_SPEC> {
        MBURST_W::new(self, 23)
    }
    #[doc = "Bits 25:27 - Channel selection"]
    #[inline(always)]
    #[must_use]
    pub fn chsel(&mut self) -> CHSEL_W<S0CR_SPEC> {
        CHSEL_W::new(self, 25)
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
#[doc = "stream x configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`s0cr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`s0cr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct S0CR_SPEC;
impl crate::RegisterSpec for S0CR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`s0cr::R`](R) reader structure"]
impl crate::Readable for S0CR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`s0cr::W`](W) writer structure"]
impl crate::Writable for S0CR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets S0CR to value 0"]
impl crate::Resettable for S0CR_SPEC {
    const RESET_VALUE: u32 = 0;
}
