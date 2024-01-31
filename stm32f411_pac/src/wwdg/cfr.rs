#[doc = "Register `CFR` reader"]
pub type R = crate::R<CFR_SPEC>;
#[doc = "Register `CFR` writer"]
pub type W = crate::W<CFR_SPEC>;
#[doc = "Field `W` reader - 7-bit window value"]
pub type W_R = crate::FieldReader;
#[doc = "Field `W` writer - 7-bit window value"]
pub type W_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `WDGTB0` reader - Timer base"]
pub type WDGTB0_R = crate::BitReader;
#[doc = "Field `WDGTB0` writer - Timer base"]
pub type WDGTB0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WDGTB1` reader - Timer base"]
pub type WDGTB1_R = crate::BitReader;
#[doc = "Field `WDGTB1` writer - Timer base"]
pub type WDGTB1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EWI` reader - Early wakeup interrupt"]
pub type EWI_R = crate::BitReader;
#[doc = "Field `EWI` writer - Early wakeup interrupt"]
pub type EWI_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:6 - 7-bit window value"]
    #[inline(always)]
    pub fn w(&self) -> W_R {
        W_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bit 7 - Timer base"]
    #[inline(always)]
    pub fn wdgtb0(&self) -> WDGTB0_R {
        WDGTB0_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Timer base"]
    #[inline(always)]
    pub fn wdgtb1(&self) -> WDGTB1_R {
        WDGTB1_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Early wakeup interrupt"]
    #[inline(always)]
    pub fn ewi(&self) -> EWI_R {
        EWI_R::new(((self.bits >> 9) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:6 - 7-bit window value"]
    #[inline(always)]
    #[must_use]
    pub fn w(&mut self) -> W_W<CFR_SPEC> {
        W_W::new(self, 0)
    }
    #[doc = "Bit 7 - Timer base"]
    #[inline(always)]
    #[must_use]
    pub fn wdgtb0(&mut self) -> WDGTB0_W<CFR_SPEC> {
        WDGTB0_W::new(self, 7)
    }
    #[doc = "Bit 8 - Timer base"]
    #[inline(always)]
    #[must_use]
    pub fn wdgtb1(&mut self) -> WDGTB1_W<CFR_SPEC> {
        WDGTB1_W::new(self, 8)
    }
    #[doc = "Bit 9 - Early wakeup interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn ewi(&mut self) -> EWI_W<CFR_SPEC> {
        EWI_W::new(self, 9)
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
#[doc = "Configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CFR_SPEC;
impl crate::RegisterSpec for CFR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfr::R`](R) reader structure"]
impl crate::Readable for CFR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`cfr::W`](W) writer structure"]
impl crate::Writable for CFR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CFR to value 0x7f"]
impl crate::Resettable for CFR_SPEC {
    const RESET_VALUE: u32 = 0x7f;
}
