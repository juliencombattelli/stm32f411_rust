#[doc = "Register `S0M1AR` reader"]
pub type R = crate::R<S0M1AR_SPEC>;
#[doc = "Register `S0M1AR` writer"]
pub type W = crate::W<S0M1AR_SPEC>;
#[doc = "Field `M1A` reader - Memory 1 address (used in case of Double buffer mode)"]
pub type M1A_R = crate::FieldReader<u32>;
#[doc = "Field `M1A` writer - Memory 1 address (used in case of Double buffer mode)"]
pub type M1A_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Memory 1 address (used in case of Double buffer mode)"]
    #[inline(always)]
    pub fn m1a(&self) -> M1A_R {
        M1A_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Memory 1 address (used in case of Double buffer mode)"]
    #[inline(always)]
    #[must_use]
    pub fn m1a(&mut self) -> M1A_W<S0M1AR_SPEC> {
        M1A_W::new(self, 0)
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
#[doc = "stream x memory 1 address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`s0m1ar::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`s0m1ar::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct S0M1AR_SPEC;
impl crate::RegisterSpec for S0M1AR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`s0m1ar::R`](R) reader structure"]
impl crate::Readable for S0M1AR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`s0m1ar::W`](W) writer structure"]
impl crate::Writable for S0M1AR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets S0M1AR to value 0"]
impl crate::Resettable for S0M1AR_SPEC {
    const RESET_VALUE: u32 = 0;
}
