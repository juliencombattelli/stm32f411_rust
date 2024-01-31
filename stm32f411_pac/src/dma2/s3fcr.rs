#[doc = "Register `S3FCR` reader"]
pub type R = crate::R<S3FCR_SPEC>;
#[doc = "Register `S3FCR` writer"]
pub type W = crate::W<S3FCR_SPEC>;
#[doc = "Field `FTH` reader - FIFO threshold selection"]
pub type FTH_R = crate::FieldReader;
#[doc = "Field `FTH` writer - FIFO threshold selection"]
pub type FTH_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `DMDIS` reader - Direct mode disable"]
pub type DMDIS_R = crate::BitReader;
#[doc = "Field `DMDIS` writer - Direct mode disable"]
pub type DMDIS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FS` reader - FIFO status"]
pub type FS_R = crate::FieldReader;
#[doc = "Field `FEIE` reader - FIFO error interrupt enable"]
pub type FEIE_R = crate::BitReader;
#[doc = "Field `FEIE` writer - FIFO error interrupt enable"]
pub type FEIE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:1 - FIFO threshold selection"]
    #[inline(always)]
    pub fn fth(&self) -> FTH_R {
        FTH_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 2 - Direct mode disable"]
    #[inline(always)]
    pub fn dmdis(&self) -> DMDIS_R {
        DMDIS_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 3:5 - FIFO status"]
    #[inline(always)]
    pub fn fs(&self) -> FS_R {
        FS_R::new(((self.bits >> 3) & 7) as u8)
    }
    #[doc = "Bit 7 - FIFO error interrupt enable"]
    #[inline(always)]
    pub fn feie(&self) -> FEIE_R {
        FEIE_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - FIFO threshold selection"]
    #[inline(always)]
    #[must_use]
    pub fn fth(&mut self) -> FTH_W<S3FCR_SPEC> {
        FTH_W::new(self, 0)
    }
    #[doc = "Bit 2 - Direct mode disable"]
    #[inline(always)]
    #[must_use]
    pub fn dmdis(&mut self) -> DMDIS_W<S3FCR_SPEC> {
        DMDIS_W::new(self, 2)
    }
    #[doc = "Bit 7 - FIFO error interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn feie(&mut self) -> FEIE_W<S3FCR_SPEC> {
        FEIE_W::new(self, 7)
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
#[doc = "stream x FIFO control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`s3fcr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`s3fcr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct S3FCR_SPEC;
impl crate::RegisterSpec for S3FCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`s3fcr::R`](R) reader structure"]
impl crate::Readable for S3FCR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`s3fcr::W`](W) writer structure"]
impl crate::Writable for S3FCR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets S3FCR to value 0x21"]
impl crate::Resettable for S3FCR_SPEC {
    const RESET_VALUE: u32 = 0x21;
}
