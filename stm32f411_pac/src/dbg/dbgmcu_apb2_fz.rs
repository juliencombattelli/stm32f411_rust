#[doc = "Register `DBGMCU_APB2_FZ` reader"]
pub type R = crate::R<DBGMCU_APB2_FZ_SPEC>;
#[doc = "Register `DBGMCU_APB2_FZ` writer"]
pub type W = crate::W<DBGMCU_APB2_FZ_SPEC>;
#[doc = "Field `DBG_TIM1_STOP` reader - TIM1 counter stopped when core is halted"]
pub type DBG_TIM1_STOP_R = crate::BitReader;
#[doc = "Field `DBG_TIM1_STOP` writer - TIM1 counter stopped when core is halted"]
pub type DBG_TIM1_STOP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DBG_TIM9_STOP` reader - TIM9 counter stopped when core is halted"]
pub type DBG_TIM9_STOP_R = crate::BitReader;
#[doc = "Field `DBG_TIM9_STOP` writer - TIM9 counter stopped when core is halted"]
pub type DBG_TIM9_STOP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DBG_TIM10_STOP` reader - TIM10 counter stopped when core is halted"]
pub type DBG_TIM10_STOP_R = crate::BitReader;
#[doc = "Field `DBG_TIM10_STOP` writer - TIM10 counter stopped when core is halted"]
pub type DBG_TIM10_STOP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DBG_TIM11_STOP` reader - TIM11 counter stopped when core is halted"]
pub type DBG_TIM11_STOP_R = crate::BitReader;
#[doc = "Field `DBG_TIM11_STOP` writer - TIM11 counter stopped when core is halted"]
pub type DBG_TIM11_STOP_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - TIM1 counter stopped when core is halted"]
    #[inline(always)]
    pub fn dbg_tim1_stop(&self) -> DBG_TIM1_STOP_R {
        DBG_TIM1_STOP_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 16 - TIM9 counter stopped when core is halted"]
    #[inline(always)]
    pub fn dbg_tim9_stop(&self) -> DBG_TIM9_STOP_R {
        DBG_TIM9_STOP_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - TIM10 counter stopped when core is halted"]
    #[inline(always)]
    pub fn dbg_tim10_stop(&self) -> DBG_TIM10_STOP_R {
        DBG_TIM10_STOP_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - TIM11 counter stopped when core is halted"]
    #[inline(always)]
    pub fn dbg_tim11_stop(&self) -> DBG_TIM11_STOP_R {
        DBG_TIM11_STOP_R::new(((self.bits >> 18) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - TIM1 counter stopped when core is halted"]
    #[inline(always)]
    #[must_use]
    pub fn dbg_tim1_stop(&mut self) -> DBG_TIM1_STOP_W<DBGMCU_APB2_FZ_SPEC> {
        DBG_TIM1_STOP_W::new(self, 0)
    }
    #[doc = "Bit 16 - TIM9 counter stopped when core is halted"]
    #[inline(always)]
    #[must_use]
    pub fn dbg_tim9_stop(&mut self) -> DBG_TIM9_STOP_W<DBGMCU_APB2_FZ_SPEC> {
        DBG_TIM9_STOP_W::new(self, 16)
    }
    #[doc = "Bit 17 - TIM10 counter stopped when core is halted"]
    #[inline(always)]
    #[must_use]
    pub fn dbg_tim10_stop(&mut self) -> DBG_TIM10_STOP_W<DBGMCU_APB2_FZ_SPEC> {
        DBG_TIM10_STOP_W::new(self, 17)
    }
    #[doc = "Bit 18 - TIM11 counter stopped when core is halted"]
    #[inline(always)]
    #[must_use]
    pub fn dbg_tim11_stop(&mut self) -> DBG_TIM11_STOP_W<DBGMCU_APB2_FZ_SPEC> {
        DBG_TIM11_STOP_W::new(self, 18)
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
#[doc = "Debug MCU APB2 Freeze registe\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dbgmcu_apb2_fz::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dbgmcu_apb2_fz::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DBGMCU_APB2_FZ_SPEC;
impl crate::RegisterSpec for DBGMCU_APB2_FZ_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dbgmcu_apb2_fz::R`](R) reader structure"]
impl crate::Readable for DBGMCU_APB2_FZ_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dbgmcu_apb2_fz::W`](W) writer structure"]
impl crate::Writable for DBGMCU_APB2_FZ_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DBGMCU_APB2_FZ to value 0"]
impl crate::Resettable for DBGMCU_APB2_FZ_SPEC {
    const RESET_VALUE: u32 = 0;
}
