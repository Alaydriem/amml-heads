
/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

void FUN_145745690(longlong param_1,float *param_2,undefined8 param_3,longlong param_4)

{
  int *piVar1;
  float fVar2;
  float fVar3;
  float fVar4;
  longlong lVar5;
  longlong lVar6;
  code *pcVar7;
  undefined1 auVar8 [12];
  undefined1 auVar9 [16];
  undefined1 auVar10 [16];
  undefined1 auVar11 [16];
  longlong lVar12;
  undefined4 uVar13;
  undefined4 uVar14;
  undefined4 uVar15;
  char cVar16;
  byte bVar17;
  byte bVar18;
  uint uVar19;
  float *pfVar20;
  undefined8 *puVar21;
  longlong *plVar22;
  longlong lVar23;
  float *pfVar24;
  longlong lVar25;
  float *pfVar26;
  float *pfVar27;
  ulonglong uVar28;
  ulonglong uVar29;
  float *pfVar30;
  ulonglong *puVar31;
  longlong lVar32;
  float *pfVar33;
  longlong lVar34;
  longlong lVar35;
  float *pfVar36;
  int iVar37;
  ulonglong uVar38;
  float *pfVar39;
  longlong unaff_GS_OFFSET;
  float fVar40;
  float fVar41;
  undefined1 auVar42 [16];
  undefined1 auVar43 [16];
  undefined1 auVar44 [16];
  undefined1 auVar45 [16];
  undefined1 auVar46 [16];
  undefined8 extraout_XMM0_Qb_00;
  undefined1 auVar47 [16];
  undefined1 auVar48 [16];
  undefined8 extraout_XMM0_Qb_01;
  undefined1 auVar49 [16];
  undefined1 auVar50 [16];
  float fVar51;
  float fVar57;
  undefined1 auVar52 [16];
  undefined1 auVar53 [16];
  undefined1 auVar54 [16];
  undefined1 auVar55 [16];
  undefined1 auVar56 [16];
  float fVar58;
  undefined1 auVar59 [16];
  undefined1 auVar60 [16];
  undefined1 auVar61 [16];
  undefined1 auVar62 [16];
  undefined1 auVar63 [16];
  float fVar64;
  float fVar65;
  float fVar66;
  float fVar67;
  float fVar68;
  float fVar69;
  float fVar70;
  float fVar71;
  undefined1 auVar72 [16];
  float fVar73;
  undefined4 uVar74;
  ulonglong in_stack_fffffffffffff910;
  undefined8 uStack_690;
  float fStack_688;
  float fStack_684;
  float afStack_680 [20];
  undefined8 uStack_630;
  ulonglong uStack_290;
  undefined1 auStack_288 [16];
  undefined1 auStack_278 [16];
  undefined8 *puStack_268;
  ulonglong uStack_260;
  longlong lStack_258;
  longlong lStack_250;
  undefined1 auStack_248 [16];
  float fStack_238;
  uint uStack_234;
  uint uStack_230;
  uint uStack_22c;
  undefined *puStack_220;
  float *pfStack_218;
  float *pfStack_210;
  ulonglong uStack_200;
  longlong *plStack_1e8;
  float *pfStack_1e0;
  float *pfStack_1d8;
  longlong lStack_1d0;
  undefined1 auStack_1c8 [16];
  undefined1 auStack_1b8 [8];
  float fStack_1b0;
  float fStack_1ac;
  undefined1 auStack_1a8 [8];
  float fStack_1a0;
  float fStack_19c;
  undefined1 auStack_198 [16];
  float *pfStack_188;
  float *pfStack_180;
  float *pfStack_178;
  float *pfStack_170;
  ulonglong uStack_168;
  ulonglong uStack_160;
  float fStack_154;
  undefined4 uStack_150;
  undefined4 uStack_14c;
  float *pfStack_148;
  float fStack_140;
  float fStack_13c;
  float fStack_138;
  float fStack_134;
  longlong lStack_130;
  float *pfStack_128;
  float *pfStack_120;
  undefined8 uStack_118;
  undefined4 uStack_110;
  undefined4 uStack_10c;
  undefined1 auStack_108 [8];
  undefined4 uStack_100;
  undefined4 uStack_fc;
  float *pfStack_f8;
  undefined8 uStack_f0;
  undefined8 extraout_XMM0_Qb;
  
  uStack_f0 = 0xfffffffffffffffe;
  pfVar20 = (float *)(*(code *)PTR__guard_dispatch_icall_14f99ef70)(param_3);
  if (pfVar20 == (float *)0x0) {
    return;
  }
  cVar16 = FUN_145741750(param_3,pfVar20,1);
  if (cVar16 != '\0') {
    return;
  }
  cVar16 = FUN_1457354b0(param_3,pfVar20,1);
  if (cVar16 != '\0') {
    return;
  }
  if (*(longlong *)(param_1 + 0x10) == 0) {
    return;
  }
  fVar2 = *(float *)(param_4 + 0x4c);
  fVar3 = *(float *)(param_4 + 0x48);
  bVar17 = *(byte *)(param_4 + 0x18);
  if ((bVar17 & 1) != 0) {
    FUN_141c68360(param_4);
    bVar17 = *(byte *)(param_4 + 0x18);
  }
  fVar41 = *(float *)(param_4 + 0x14);
  if ((bVar17 & 1) != 0) {
    FUN_141c68360(param_4);
  }
  fVar4 = *(float *)(param_4 + 0x10);
  lVar5 = *(longlong *)(param_2 + 4);
  lVar23 = param_1 + 0xe0;
  fVar66 = fVar2 + _DAT_14f25fb80;
  fVar67 = fVar2 + _UNK_14f25fb84;
  cVar16 = FUN_144deeeb0(lVar23);
  bVar17 = 1;
  if (cVar16 == '\0') {
    uStack_690._0_4_ = fVar66;
    uStack_690._4_4_ = fVar67;
    FUN_140e9ad70(lVar23,param_1 + 0x20,*(undefined8 *)(lVar5 + 0xb0),&uStack_690);
    bVar17 = FUN_144deeeb0(lVar23);
  }
  fVar69 = _DAT_14efd24a4 + fVar2;
  lVar23 = param_1 + 0x340;
  cVar16 = FUN_144deeeb0(lVar23);
  if (cVar16 == '\0') {
    uStack_690._0_4_ = fVar66;
    uStack_690._4_4_ = fVar67;
    FUN_140e9ad70(lVar23,param_1 + 0x40,*(undefined8 *)(lVar5 + 0xb0),&uStack_690);
    bVar18 = FUN_144deeeb0(lVar23);
    bVar17 = bVar17 & bVar18;
  }
  fVar68 = 0.0;
  if (0.0 <= fVar69) {
    fVar68 = fVar69;
  }
  lVar23 = param_1 + 0x5a0;
  cVar16 = FUN_144deeeb0(lVar23);
  if (cVar16 == '\0') {
    uStack_690._0_4_ = _DAT_14ef97d80 + fVar68;
    uStack_690._4_4_ = (float)uStack_690;
    FUN_140e9ad70(lVar23,param_1 + 0x60,*(undefined8 *)(lVar5 + 0xb0),&uStack_690);
    bVar18 = FUN_144deeeb0(lVar23);
    bVar17 = bVar17 & bVar18;
  }
  lVar23 = param_1 + 0x800;
  cVar16 = FUN_144deeeb0(lVar23);
  if (cVar16 == '\0') {
    uStack_690._0_4_ = _DAT_14ef97d80 + fVar68;
    uStack_690._4_4_ = (float)uStack_690;
    FUN_140e9ad70(lVar23,param_1 + 0x80,*(undefined8 *)(lVar5 + 0xb0),&uStack_690);
    bVar18 = FUN_144deeeb0(lVar23);
    bVar17 = bVar17 != 0 & bVar18;
  }
  lVar23 = param_1 + 0xa60;
  cVar16 = FUN_144deeeb0(lVar23);
  if (cVar16 == '\0') {
    uStack_690._0_4_ = _DAT_14ef3e020 + fVar68;
    uStack_690._4_4_ = (float)uStack_690;
    FUN_140e9ad70(lVar23,param_1 + 0xa0,*(undefined8 *)(lVar5 + 0xb0),&uStack_690);
    bVar18 = FUN_144deeeb0(lVar23);
    bVar17 = bVar17 != 0 & bVar18;
  }
  lVar23 = param_1 + 0xcc0;
  cVar16 = FUN_144deeeb0(lVar23);
  if (cVar16 == '\0') {
    uStack_690._0_4_ = _DAT_14eec7060 + fVar68;
    uStack_690._4_4_ = (float)uStack_690;
    FUN_140e9ad70(lVar23,param_1 + 0xc0,*(undefined8 *)(lVar5 + 0xb0),&uStack_690);
    bVar18 = FUN_144deeeb0(lVar23);
    bVar17 = bVar17 != 0 & bVar18;
  }
  if (bVar17 == 0) {
    return;
  }
  puVar21 = *(undefined8 **)(param_1 + 0x20);
  if (puVar21 == (undefined8 *)0x0) {
    puVar21 = (undefined8 *)0x1529bf210;
  }
  pfVar39 = (float *)*puVar21;
  if (pfVar39 == (float *)0x0) {
    return;
  }
  if (*(int *)(*(longlong *)(*(longlong *)(unaff_GS_OFFSET + 0x58) + (ulonglong)__tls_index * 8) + 4
              ) < _DAT_1528d6fc8) goto LAB_14574725e;
  while( true ) {
    LOCK();
    *(short *)(pfVar39 + 4) = *(short *)(pfVar39 + 4) + 1;
    UNLOCK();
    if ((*(byte *)((longlong)pfVar39 + 0x12) & 1) == 0) {
      return;
    }
    puVar21 = *(undefined8 **)(param_1 + 0x40);
    if (puVar21 == (undefined8 *)0x0) {
      puVar21 = (undefined8 *)0x1529bf210;
    }
    pfVar39 = (float *)*puVar21;
    if (pfVar39 == (float *)0x0) {
      return;
    }
    if (*(int *)(*(longlong *)(*(longlong *)(unaff_GS_OFFSET + 0x58) + (ulonglong)__tls_index * 8) +
                4) < _DAT_1528d6fc8) goto LAB_1457472f6;
    while( true ) {
      LOCK();
      *(short *)(pfVar39 + 4) = *(short *)(pfVar39 + 4) + 1;
      UNLOCK();
      if ((*(byte *)((longlong)pfVar39 + 0x12) & 1) == 0) {
        return;
      }
      plVar22 = *(longlong **)(param_1 + 0x60);
      if (plVar22 == (longlong *)0x0) {
        plVar22 = (longlong *)0x1529bf210;
      }
      lVar23 = *plVar22;
      if (lVar23 == 0) {
        return;
      }
      if (*(int *)(*(longlong *)(*(longlong *)(unaff_GS_OFFSET + 0x58) + (ulonglong)__tls_index * 8)
                  + 4) < _DAT_1528d6fc8) {
        FUN_14e9380b0(&DAT_1528d6fc8);
        if (_DAT_1528d6fc8 == -1) {
          FUN_14e934b40(FUN_140a771c0);
          FUN_14e938170(&DAT_1528d6fc8);
        }
      }
      LOCK();
      *(short *)(lVar23 + 0x10) = *(short *)(lVar23 + 0x10) + 1;
      UNLOCK();
      if ((*(byte *)(lVar23 + 0x12) & 1) == 0) {
        return;
      }
      plVar22 = *(longlong **)(param_1 + 0x80);
      if (plVar22 == (longlong *)0x0) {
        plVar22 = (longlong *)0x1529bf210;
      }
      lVar23 = *plVar22;
      if (lVar23 == 0) {
        return;
      }
      if (*(int *)(*(longlong *)(*(longlong *)(unaff_GS_OFFSET + 0x58) + (ulonglong)__tls_index * 8)
                  + 4) < _DAT_1528d6fc8) {
        FUN_14e9380b0(&DAT_1528d6fc8);
        if (_DAT_1528d6fc8 == -1) {
          FUN_14e934b40(FUN_140a771c0);
          FUN_14e938170(&DAT_1528d6fc8);
        }
      }
      LOCK();
      *(short *)(lVar23 + 0x10) = *(short *)(lVar23 + 0x10) + 1;
      UNLOCK();
      if ((*(byte *)(lVar23 + 0x12) & 1) == 0) {
        return;
      }
      plVar22 = *(longlong **)(param_1 + 0xa0);
      if (plVar22 == (longlong *)0x0) {
        plVar22 = (longlong *)0x1529bf210;
      }
      lVar23 = *plVar22;
      if (lVar23 == 0) {
        return;
      }
      if (*(int *)(*(longlong *)(*(longlong *)(unaff_GS_OFFSET + 0x58) + (ulonglong)__tls_index * 8)
                  + 4) < _DAT_1528d6fc8) {
        FUN_14e9380b0(&DAT_1528d6fc8);
        if (_DAT_1528d6fc8 == -1) {
          FUN_14e934b40(FUN_140a771c0);
          FUN_14e938170(&DAT_1528d6fc8);
        }
      }
      LOCK();
      *(short *)(lVar23 + 0x10) = *(short *)(lVar23 + 0x10) + 1;
      UNLOCK();
      if ((*(byte *)(lVar23 + 0x12) & 1) == 0) {
        return;
      }
      plVar22 = *(longlong **)(param_1 + 0xc0);
      if (plVar22 == (longlong *)0x0) {
        plVar22 = (longlong *)0x1529bf210;
      }
      lVar23 = *plVar22;
      if (lVar23 == 0) {
        return;
      }
      if (*(int *)(*(longlong *)(*(longlong *)(unaff_GS_OFFSET + 0x58) + (ulonglong)__tls_index * 8)
                  + 4) < _DAT_1528d6fc8) {
        FUN_14e9380b0(&DAT_1528d6fc8);
        if (_DAT_1528d6fc8 == -1) {
          FUN_14e934b40(FUN_140a771c0);
          FUN_14e938170(&DAT_1528d6fc8);
        }
      }
      LOCK();
      *(short *)(lVar23 + 0x10) = *(short *)(lVar23 + 0x10) + 1;
      UNLOCK();
      if ((*(byte *)(lVar23 + 0x12) & 1) == 0) {
        return;
      }
      puVar21 = *(undefined8 **)(lVar5 + 0x30);
      uVar74 = *(undefined4 *)(param_1 + 8);
      auVar55._0_8_ = 0;
      *puVar21 = _DAT_14efd2400;
      *(undefined4 *)(puVar21 + 1) = 0x3f800000;
      *(undefined4 *)((longlong)puVar21 + 0xc) = uVar74;
      *(undefined1 *)(puVar21 + 2) = 1;
      FUN_141913870(*(undefined8 *)(param_1 + 0x10),*(undefined8 *)(param_2 + 4),
                    *(undefined8 *)(lVar5 + 0xb0));
      cVar16 = (*(code *)PTR__guard_dispatch_icall_14f99ef70)();
      if (cVar16 == '\0') {
        lVar23 = (*(code *)PTR__guard_dispatch_icall_14f99ef70)(param_3);
        fVar69 = *(float *)(lVar23 + 0x118);
        uVar38 = *(ulonglong *)(lVar23 + 0x11c);
        fVar58 = (float)uVar38;
        fVar51 = (float)(uVar38 >> 0x20);
        auVar53._0_4_ = SQRT(fVar51 * fVar51 + fVar69 * fVar69 + fVar58 * fVar58);
        if (_DAT_14f07eaf0 <= auVar53._0_4_) {
          auStack_198._0_4_ = fVar58 / auVar53._0_4_;
          auStack_198._4_8_ = uVar38 >> 0x20;
          auStack_198._12_4_ = 0;
          auVar61._4_8_ = uVar38 >> 0x20;
          auVar61._0_4_ = fVar69;
          auVar61._12_4_ = 0;
          auVar53._4_4_ = auVar53._0_4_;
          auVar53._8_4_ = auVar53._0_4_;
          auVar53._12_4_ = auVar53._0_4_;
          _auStack_1a8 = divps(auVar61,auVar53);
        }
        else {
          _auStack_1a8 = ZEXT816(0);
          auStack_198 = ZEXT816(0);
        }
      }
      else {
        FUN_140ba31b0(pfVar20,&uStack_690);
        auVar8._4_8_ = auVar55._0_8_;
        auVar8._0_4_ = fStack_688;
        auVar52._4_8_ = auVar8._0_8_ & 0xffffffff;
        auVar52._0_4_ = (float)uStack_690;
        auVar52._12_4_ = 0;
        fVar69 = SQRT(fStack_688 * fStack_688 +
                      (float)uStack_690 * (float)uStack_690 + uStack_690._4_4_ * uStack_690._4_4_);
        auVar60._4_4_ = fVar69;
        auVar60._0_4_ = fVar69;
        auVar60._8_4_ = fVar69;
        auVar60._12_4_ = fVar69;
        _auStack_1a8 = divps(auVar52,auVar60);
        auStack_198 = ZEXT816(0);
        if (_DAT_14f07eaf0 <= fVar69) {
          auStack_198._4_4_ = fStack_688;
          auStack_198._0_4_ = uStack_690._4_4_ / fVar69;
          auStack_198._8_8_ = 0;
        }
        uVar19 = (uint)(fVar69 < _DAT_14f07eaf0);
        auVar42._0_4_ = (int)(uVar19 << 0x1f) >> 0x1f;
        auVar42._4_4_ = (int)(uVar19 << 0x1f) >> 0x1f;
        auVar42._8_4_ = (int)(uVar19 << 0x1f) >> 0x1f;
        auVar42._12_4_ = (int)(uVar19 << 0x1f) >> 0x1f;
        _auStack_1a8 = ~auVar42 & _auStack_1a8;
        cVar16 = FUN_140aa6f60(param_3);
        if (cVar16 != '\0') {
          auStack_198._4_4_ = auStack_198._4_4_ ^ _UNK_14ef3e054;
          auStack_198._0_4_ = auStack_198._0_4_ ^ _DAT_14ef3e050;
          auStack_198._8_4_ = auStack_198._8_4_ ^ _UNK_14ef3e058;
          auStack_198._12_4_ = auStack_198._12_4_ ^ _UNK_14ef3e05c;
          auStack_1a8._4_4_ = auStack_1a8._4_4_ ^ _UNK_14ef3e054;
          auStack_1a8._0_4_ = auStack_1a8._0_4_ ^ _DAT_14ef3e050;
          fStack_1a0 = (float)((uint)fStack_1a0 ^ _UNK_14ef3e058);
          fStack_19c = (float)((uint)fStack_19c ^ _UNK_14ef3e05c);
        }
      }
      FUN_14072c030(&uStack_690);
      fVar69 = (float)fmodf((float)(longlong)uStack_690 / _DAT_14efc5db0);
      auVar10._8_8_ = 0;
      auVar10._0_8_ = auStack_1c8._8_8_;
      auStack_1c8 = auVar10 << 0x40;
      fStack_134 = (float)CONCAT31(fStack_134._1_3_,_DAT_14f214e90 < fVar69);
      FUN_140bcda70(pfVar20);
      pfStack_188 = (float *)0x0;
      pfStack_180 = (float *)0x0;
      pfStack_178 = (float *)0x0;
      pfVar20 = *(float **)(pfVar20 + 0x3c0);
      param_2 = *(float **)pfVar20;
      if (param_2 == pfVar20) {
        pfStack_f8 = (float *)0x0;
      }
      else {
        do {
          pfVar39 = *(float **)(param_2 + 8);
          if (pfVar39 != (float *)0x0) {
            uStack_690 = pfVar39;
            if (pfStack_180 == pfStack_178) {
              FUN_1402165f0(&pfStack_188,pfStack_180,&uStack_690);
            }
            else {
              *(float **)pfStack_180 = pfVar39;
              pfStack_180 = pfStack_180 + 2;
            }
          }
          param_2 = *(float **)param_2;
        } while (param_2 != pfVar20);
        pfStack_f8 = pfStack_180;
      }
      pfVar39 = pfStack_188;
      uStack_118 = (float *)((longlong)pfStack_f8 - (longlong)pfStack_188 >> 3);
      pfStack_180 = pfStack_f8;
      if ((longlong)uStack_118 < 2) goto LAB_1457465bc;
      uStack_690 = afStack_680;
      fStack_688 = 0.0;
      fStack_684 = 2.24208e-44;
      if ((uint)uStack_118 != 0xffffffff) break;
      puStack_220 = &UNK_14efc5c70;
      pfStack_218 = (float *)0x0;
      pfStack_210 = (float *)0x0;
      plStack_1e8 = (longlong *)&UNK_14f5ba53c;
      pfStack_1e0 = (float *)CONCAT71(pfStack_1e0._1_7_,1);
      FUN_14eec6220(&plStack_1e8,&pfStack_218);
      puStack_220 = &UNK_14efc5c70;
      FUN_14eec6180(&puStack_220,&UNK_14faa7580);
LAB_1457472f6:
      FUN_14e9380b0(&DAT_1528d6fc8);
      if (_DAT_1528d6fc8 == -1) {
        FUN_14e934b40(FUN_140a771c0);
        FUN_14e938170(&DAT_1528d6fc8);
      }
    }
    if (0x10 < (uint)uStack_118) {
      uVar38 = (ulonglong)((longlong)pfStack_f8 - (longlong)pfStack_188) >> 1 & 0x3fffffffc;
      lVar23 = (*(code *)PTR__guard_dispatch_icall_14f99ef70)(_DAT_15233c1a8,uVar38);
      auStack_108 = (undefined1  [8])lVar23;
      pfVar20 = uStack_690;
      if ((uVar38 != 0) && (lVar23 == 0)) {
        FUN_1409bc820(uVar38);
        pfVar20 = uStack_690;
      }
      fVar69 = fStack_688;
      uStack_690 = pfVar20;
      FUN_14eec61e0(auStack_108,pfVar20,(ulonglong)(uint)fStack_688 * 4);
      if ((afStack_680 != pfVar20) && (pfVar20 != (float *)0x0)) {
        (*(code *)PTR__guard_dispatch_icall_14f99ef70)(_DAT_15233c1a8,pfVar20);
      }
      uStack_690 = auStack_108;
      fStack_684 = SUB84(uStack_118,0);
      fStack_688 = fVar69;
    }
    pfVar24 = pfVar39;
    pfVar20 = uStack_690;
    if (pfVar39 != pfStack_f8) {
      do {
        lVar23 = *(longlong *)pfVar24;
        fVar51 = *(float *)(lVar23 + 8) - fStack_140;
        fVar58 = *(float *)(lVar23 + 0xc) - fStack_13c;
        fVar69 = *(float *)(lVar23 + 0x10) - fStack_138;
        fVar69 = fVar69 * fVar69 + fVar58 * fVar58 + fVar51 * fVar51;
        puStack_220 = (undefined *)CONCAT44(puStack_220._4_4_,fVar69);
        if (fStack_688 == fStack_684) {
          uStack_690 = pfVar20;
          FUN_14215a290(&uStack_690,pfVar20 + (uint)fStack_688,1,&puStack_220);
          pfVar20 = uStack_690;
        }
        else {
          pfVar20[(uint)fStack_688] = fVar69;
          fStack_688 = (float)((int)fStack_688 + 1);
        }
        pfVar24 = pfVar24 + 2;
      } while (pfVar24 != pfStack_f8);
    }
    auStack_1b8 = (undefined1  [8])CONCAT44(0,fStack_688);
    uStack_690 = pfVar20;
    if (pfVar39 == pfStack_f8 || auStack_1b8 == (undefined1  [8])0x0) break;
    pfStack_170 = pfVar20 + (longlong)auStack_1b8;
    param_2 = uStack_118;
    if ((longlong)auStack_1b8 <= (longlong)uStack_118) {
      param_2 = (float *)auStack_1b8;
    }
    lVar23 = 0;
    for (pfVar24 = param_2; pfVar24 != (float *)0x1; pfVar24 = (float *)((ulonglong)pfVar24 >> 1)) {
      lVar23 = lVar23 + 2;
    }
    pfStack_218 = pfStack_f8;
    pfStack_210 = pfVar20 + (longlong)auStack_1b8;
    pfStack_1e0 = pfVar39;
    pfStack_1d8 = pfVar20;
    FUN_145767d20(&plStack_1e8,&puStack_220,lVar23);
    if ((longlong)param_2 < 0x11) {
      uStack_118 = pfVar39 + 2;
      if (uStack_118 == pfStack_f8 || auStack_1b8._0_4_ == 1) break;
      pfVar24 = pfVar20 + 1;
      uStack_168 = 1;
      pfVar26 = (float *)0x0;
      pfVar36 = pfVar39;
      pfVar33 = pfVar20;
      while (pfStack_128 = pfVar36, lStack_130 = *(longlong *)uStack_118, lStack_130 != 0) {
        pfStack_120 = (float *)((longlong)pfVar26 + 1);
        fVar69 = *pfVar24;
        auStack_108 = (undefined1  [8])pfVar24;
        if (fVar69 < *pfVar20 || fVar69 == *pfVar20) {
          fVar51 = *pfVar33;
          pfVar26 = pfVar24;
          pfVar36 = uStack_118;
          while (fVar51 < fVar69) {
            *pfVar26 = fVar51;
            *(undefined8 *)pfVar36 = *(undefined8 *)(pfVar36 + -2);
            pfVar36 = pfVar36 + -2;
            pfVar27 = pfVar26 + -2;
            pfVar26 = pfVar26 + -1;
            fVar51 = *pfVar27;
          }
          *pfVar26 = fVar69;
          *(longlong *)pfVar36 = lStack_130;
        }
        else {
          param_2 = pfStack_128 + 4;
          pfVar36 = pfVar33 + 2;
          pfVar27 = uStack_118;
          pfVar30 = pfVar24;
          if ((pfStack_120 < (float *)0x2) ||
             ((pfVar20 < pfVar39 + (longlong)pfVar26 * 2 + 4 &&
              (pfVar39 < pfVar20 + (longlong)pfVar26 + 2)))) {
LAB_14574641e:
            lVar23 = -4;
            do {
              pfVar27 = pfVar27 + -2;
              *(undefined4 *)((longlong)pfVar36 + lVar23) =
                   *(undefined4 *)((longlong)pfVar30 + lVar23);
              *(undefined8 *)((longlong)param_2 + lVar23 * 2) = *(undefined8 *)pfVar27;
              if ((float *)((longlong)pfVar30 + lVar23) == pfVar20) break;
              lVar23 = lVar23 + -4;
            } while (pfVar27 != pfVar39);
          }
          else {
            pfStack_148 = (float *)((ulonglong)pfStack_120 & 0xfffffffffffffffe);
            param_2 = param_2 + (longlong)pfStack_148 * -2;
            pfVar36 = pfVar36 + -(longlong)pfStack_148;
            lVar23 = 0;
            do {
              *(undefined8 *)(pfVar33 + lVar23) = *(undefined8 *)(pfVar33 + lVar23 + -1);
              auVar55._0_8_ = *(undefined8 *)(pfStack_128 + lVar23 * 2 + -2 + 2);
              *(undefined8 *)(pfStack_128 + lVar23 * 2) =
                   *(undefined8 *)(pfStack_128 + lVar23 * 2 + -2);
              *(undefined8 *)(pfStack_128 + lVar23 * 2 + 2) = auVar55._0_8_;
              lVar23 = lVar23 + -2;
            } while (-lVar23 != (uStack_168 & 0xfffffffffffffffe));
            pfVar27 = uStack_118 + (longlong)pfStack_148 * -2;
            pfVar30 = pfVar24 + -(longlong)pfStack_148;
            if (pfStack_120 != pfStack_148) goto LAB_14574641e;
          }
          *pfVar20 = fVar69;
          *(longlong *)pfVar39 = lStack_130;
        }
        uStack_118 = uStack_118 + 2;
        if (uStack_118 == pfStack_f8) goto LAB_145746595;
        pfVar24 = pfVar24 + 1;
        uStack_168 = uStack_168 + 1;
        pfVar33 = pfVar33 + 1;
        pfVar26 = pfStack_120;
        pfVar36 = pfStack_128 + 2;
        if (pfVar24 == pfStack_170) goto LAB_145746595;
      }
    }
    else {
      lVar23 = 4;
      lVar34 = 8;
      uVar19 = 1;
      uStack_168 = 0;
      pfVar26 = (float *)0x0;
      pfVar36 = pfVar39;
      param_2 = pfVar20;
      pfVar24 = pfVar20;
      do {
        pfVar24 = pfVar24 + 1;
        lVar6 = *(longlong *)((longlong)pfVar39 + lVar34);
        auStack_108 = (undefined1  [8])lVar6;
        if (lVar6 == 0) goto LAB_145747259;
        pfStack_120 = (float *)((longlong)pfVar39 + lVar34);
        lStack_130 = lVar23 + (longlong)pfVar20;
        fVar69 = *(float *)((longlong)pfVar20 + lVar23);
        pfStack_128 = (float *)CONCAT71(pfStack_128._1_7_,(char)uVar19);
        if (fVar69 < *pfVar20 || fVar69 == *pfVar20) {
          plVar22 = (longlong *)((longlong)pfVar39 + lVar34);
          pfVar36 = (float *)((longlong)pfVar20 + lVar23);
          fVar51 = *param_2;
          pfVar33 = pfVar24;
          while (fVar51 < fVar69) {
            *pfVar33 = fVar51;
            *plVar22 = plVar22[-1];
            plVar22 = plVar22 + -1;
            pfVar36 = pfVar33 + -1;
            fVar51 = pfVar33[-2];
            pfVar33 = pfVar36;
          }
          *pfVar36 = fVar69;
          *plVar22 = lVar6;
        }
        else {
          fStack_238 = (float)lVar34;
          uStack_234 = (uint)((ulonglong)lVar34 >> 0x20);
          auStack_248._0_8_ = lVar23;
          pfVar36 = pfVar36 + 4;
          lVar32 = lStack_130;
          pfVar33 = pfStack_120;
          if ((~(uint)pfVar26 & 3) != 0) {
            uVar28 = (ulonglong)(uVar19 & 3);
            uVar38 = uStack_168;
            do {
              pfVar33 = pfVar36;
              uVar29 = uVar38;
              pfVar36 = pfVar33 + -2;
              *(undefined4 *)((longlong)pfVar20 + uVar29 + 4) =
                   *(undefined4 *)((longlong)pfVar20 + uVar29);
              *(undefined8 *)((longlong)pfVar39 + uVar29 * 2 + 8) =
                   *(undefined8 *)((longlong)pfVar39 + uVar29 * 2);
              uVar28 = uVar28 - 1;
              uVar38 = uVar29 - 4;
            } while (uVar28 != 0);
            param_2 = (float *)((uVar29 - 4) + (longlong)pfVar20);
            lVar32 = (longlong)pfVar20 + uVar29;
            pfVar33 = pfVar33 + -4;
          }
          if ((float *)0x2 < pfVar26) {
            lVar25 = 0;
            lVar35 = 0;
            do {
              *(undefined4 *)((longlong)param_2 + lVar35 + 4) =
                   *(undefined4 *)(lVar32 + -4 + lVar35);
              *(undefined8 *)((longlong)pfVar36 + lVar35 * 2 + -8) =
                   *(undefined8 *)((longlong)pfVar33 + lVar35 * 2 + -8);
              *(undefined4 *)((longlong)param_2 + lVar35) = *(undefined4 *)(lVar32 + -8 + lVar35);
              *(undefined8 *)((longlong)pfVar36 + lVar35 * 2 + -0x10) =
                   *(undefined8 *)((longlong)pfVar33 + lVar35 * 2 + -0x10);
              *(undefined4 *)((longlong)param_2 + lVar35 + -4) =
                   *(undefined4 *)(lVar32 + -0xc + lVar35);
              *(undefined8 *)((longlong)pfVar36 + lVar35 * 2 + -0x18) =
                   *(undefined8 *)((longlong)pfVar33 + lVar35 * 2 + -0x18);
              *(undefined4 *)((longlong)param_2 + lVar35 + -8) =
                   *(undefined4 *)(lVar32 + -0x10 + lVar35);
              *(undefined8 *)((longlong)pfVar36 + lVar35 * 2 + -0x20) =
                   *(undefined8 *)((longlong)pfVar33 + lVar35 * 2 + -0x20);
              if ((float *)(lVar32 + -0x10 + lVar35) == pfVar20) break;
              lVar12 = lVar25 + -0x20;
              lVar35 = lVar35 + -0x10;
              lVar25 = lVar25 + -0x20;
            } while ((float *)((longlong)pfVar33 + lVar12) != pfVar39);
          }
          *pfVar20 = fVar69;
          *(longlong *)pfVar39 = lVar6;
          pfStack_148 = pfVar24;
        }
        param_2 = (float *)(lVar23 + (longlong)pfVar20);
        pfVar36 = (float *)((longlong)pfVar39 + lVar34);
        lVar34 = lVar34 + 8;
        uStack_118 = pfVar26;
        if (lVar34 == 0x80) break;
        lVar23 = lVar23 + 4;
        uStack_168 = uStack_168 + 4;
        uVar19 = (uint)(byte)((char)uVar19 + 1);
        pfVar26 = (float *)((longlong)pfVar26 + 1);
      } while (lVar23 != 0x40);
      pfVar39 = pfVar39 + 0x20;
      if (pfVar39 == pfStack_f8 || auStack_1b8._0_4_ == 0x10) break;
      pfVar20 = pfVar20 + 0x10;
      while (lVar23 = *(longlong *)pfVar39, lVar23 != 0) {
        fVar69 = *pfVar20;
        fVar51 = pfVar20[-1];
        pfVar24 = pfVar20;
        pfVar26 = pfVar39;
        while (fVar51 < fVar69) {
          *pfVar24 = fVar51;
          *(undefined8 *)pfVar26 = *(undefined8 *)(pfVar26 + -2);
          pfVar26 = pfVar26 + -2;
          pfVar36 = pfVar24 + -2;
          pfVar24 = pfVar24 + -1;
          fVar51 = *pfVar36;
        }
        *pfVar24 = fVar69;
        *(longlong *)pfVar26 = lVar23;
        pfVar20 = pfVar20 + 1;
        if ((pfVar20 == pfStack_170) || (pfVar39 = pfVar39 + 2, pfVar39 == pfStack_f8))
        goto LAB_145746595;
      }
    }
LAB_145747259:
    FUN_140078ed0();
LAB_14574725e:
    FUN_14e9380b0(&DAT_1528d6fc8);
    if (_DAT_1528d6fc8 == -1) {
      FUN_14e934b40(FUN_140a771c0);
      FUN_14e938170(&DAT_1528d6fc8);
    }
  }
LAB_145746595:
  if ((afStack_680 != uStack_690) && (uStack_690 != (float *)0x0)) {
    (*(code *)PTR__guard_dispatch_icall_14f99ef70)();
  }
LAB_1457465bc:
  lVar23 = (*(code *)PTR__guard_dispatch_icall_14f99ef70)(param_3);
  cVar16 = FUN_140aa6de0(param_3);
  pfVar20 = pfStack_f8;
  fVar69 = _DAT_14ef1c470;
  if (cVar16 == '\0') {
    fVar69 = _DAT_14eec7060;
  }
  lVar34 = CONCAT71(auStack_1c8._1_7_,fStack_134._0_1_);
  auStack_1c8._0_8_ = lVar34;
  fVar70 = *(float *)(lVar23 + 0x178) * fVar69;
  fVar51 = *(float *)(lVar23 + 0x160);
  fVar58 = *(float *)(lVar23 + 0x168);
  fVar69 = fVar69 * *(float *)(lVar23 + 0x164);
  fVar57 = (float)*(undefined8 *)(lVar23 + 0x170);
  fVar73 = (float)((ulonglong)*(undefined8 *)(lVar23 + 0x170) >> 0x20);
  fVar40 = SQRT(fVar70 * fVar70 + fVar73 * fVar73 + fVar57 * fVar57);
  fVar64 = 0.0;
  pfStack_f8 = (float *)((ulonglong)pfStack_f8 & 0xffffffff00000000);
  fVar65 = 0.0;
  if (_DAT_14f07eaf0 <= fVar40) {
    fVar64 = fVar70 / fVar40;
    fVar65 = fVar73 / fVar40;
  }
  fVar71 = SQRT(fVar58 * fVar58 + fVar69 * fVar69 + fVar51 * fVar51);
  fVar70 = 0.0;
  if (_DAT_14f07eaf0 <= fVar71) {
    fVar70 = fVar69 / fVar71;
    pfStack_f8._4_4_ = SUB84(pfVar20,4);
    pfStack_f8 = (float *)CONCAT44(pfStack_f8._4_4_,fVar58 / fVar71);
  }
  fVar69 = *(float *)(&UNK_14f25fb90 + lVar34 * 4);
  auVar43._0_8_ =
       acosf(CONCAT44(fVar73,(float)(~-(uint)(fVar40 < _DAT_14f07eaf0) & (uint)(fVar57 / fVar40)) *
                             (float)(~-(uint)(fVar71 < _DAT_14f07eaf0) & (uint)(fVar51 / fVar71)) +
                             fVar65 * fVar70 + fVar64 * pfStack_f8._0_4_));
  auVar43._8_8_ = extraout_XMM0_Qb;
  auStack_1c8._4_12_ = auVar43._4_12_;
  auStack_1c8._0_4_ = (float)auVar43._0_8_ * _DAT_14eee6c00 * _DAT_14f0884b0;
  uVar28 = func_0x00014eec4d20();
  uVar38 = func_0x00014eec4d30();
  if (uVar28 == 24000000) {
    lVar23 = (((longlong)uVar38 % 24000000) * 1000000000) / 24000000 +
             ((longlong)uVar38 / 24000000) * 1000000000;
  }
  else if (uVar28 == 10000000) {
    lVar23 = uVar38 * 100;
  }
  else {
    if ((uVar38 | uVar28) >> 0x20 == 0) {
      uVar29 = (uVar38 & 0xffffffff) / (uVar28 & 0xffffffff);
      uVar38 = (uVar38 & 0xffffffff) % (uVar28 & 0xffffffff);
    }
    else {
      uVar29 = (longlong)uVar38 / (longlong)uVar28;
      uVar38 = (longlong)uVar38 % (longlong)uVar28;
    }
    uVar38 = uVar38 * 1000000000;
    if ((uVar38 | uVar28) >> 0x20 == 0) {
      uVar38 = (uVar38 & 0xffffffff) / (uVar28 & 0xffffffff);
    }
    else {
      uVar38 = (longlong)uVar38 / (longlong)uVar28;
    }
    lVar23 = uVar38 + uVar29 * 1000000000;
  }
  pfStack_f8 = (float *)CONCAT44(pfStack_f8._4_4_,
                                 (float)(lVar23 - *(longlong *)(param_1 + 0xf28)) / _DAT_14efc5db0);
  *(longlong *)(param_1 + 0xf28) = lVar23;
  pfStack_148 = (float *)CONCAT44(pfStack_148._4_4_,fVar3 * _DAT_14eee6c00);
  fVar41 = fVar2 * _DAT_14eee6c00 + fVar41;
  auStack_248 = ZEXT416((uint)fVar41);
  fStack_238 = (float)(_DAT_14ef3e050 ^ auStack_1c8._0_4_);
  uStack_234 = _UNK_14ef3e054 ^ auStack_1c8._4_4_;
  uStack_230 = _UNK_14ef3e058 ^ auStack_1c8._8_4_;
  uStack_22c = _UNK_14ef3e05c ^ auStack_1c8._12_4_;
  fStack_134 = (float)auStack_198._0_4_ * (float)auStack_198._0_4_;
  fVar66 = fVar66 * _DAT_14eee6c00;
  auStack_288 = ZEXT416((uint)((fVar41 + fVar69 + fVar2) - fVar67 * _DAT_14eee6c00));
  auStack_278 = ZEXT416((uint)((fVar41 - (fVar69 + fVar2)) - fVar67 * _DAT_14eee6c00));
  uStack_118 = (float *)CONCAT44(auStack_1a8._4_4_,auStack_1a8._4_4_);
  uStack_110 = auStack_1a8._4_4_;
  uStack_10c = auStack_1a8._4_4_;
  auStack_1b8._4_4_ = auStack_1a8._0_4_;
  auStack_1b8._0_4_ = auStack_1a8._4_4_;
  fStack_1b0 = fStack_1a0;
  fStack_1ac = fStack_19c;
  pfStack_120 = pfStack_180;
  pfVar20 = pfStack_188;
  pfVar39 = pfStack_180;
  if (pfStack_188 != pfStack_180) {
    do {
      while (lVar23 = *(longlong *)pfVar20, *(char *)(lVar23 + 0x60) == '\0') {
        *(undefined1 *)(lVar23 + 0x74) = 0;
        pfVar20 = pfVar20 + 2;
        if (pfVar20 == pfVar39) goto LAB_1457471b7;
      }
      fVar41 = *(float *)(lVar23 + 0xc);
      fVar67 = *(float *)(lVar23 + 0x10);
      fVar69 = *(float *)(lVar23 + 8);
      fVar51 = fVar41;
      fVar40 = fVar69;
      fVar58 = fVar67;
      if (*(char *)(lVar23 + 0x74) == '\x01') {
        fVar51 = *(float *)(lVar23 + 0x6c);
        fVar58 = *(float *)(lVar23 + 0x70);
        fVar40 = *(float *)(lVar23 + 0x68);
      }
      fVar57 = (fVar67 - fStack_138) * (fVar67 - fStack_138);
      fVar70 = (fVar57 + (fVar41 - fStack_13c) * (fVar41 - fStack_13c) +
                         (fVar69 - fStack_140) * (fVar69 - fStack_140) + _DAT_14f25fb98) /
               _DAT_14f25fb9c;
      fVar64 = _DAT_14efd2348;
      fVar65 = 0.0;
      if ((0.0 <= fVar70) && (fVar64 = _DAT_14ef3e01c, fVar65 = 0.0, fVar70 <= _DAT_14eec7060)) {
        fVar64 = fVar70 * _DAT_14f25fba0 + _DAT_14efd2348;
        fVar65 = fVar57;
      }
      auVar44._0_4_ = (float)((uint)fVar64 ^ _DAT_14ef3e050);
      auVar44._4_4_ = (uint)fVar65 ^ _UNK_14ef3e054;
      auVar44._8_4_ = (uint)fVar65 ^ _UNK_14ef3e058;
      auVar44._12_4_ = (uint)fVar65 ^ _UNK_14ef3e05c;
      auVar45._4_12_ = auVar44._4_12_;
      auVar45._0_4_ = auVar44._0_4_ * pfStack_f8._0_4_;
      fVar64 = (float)func_0x00014eec6400(auVar45._0_8_);
      fVar64 = _DAT_14eec7060 - fVar64;
      fVar51 = (fVar41 - fVar51) * fVar64 + fVar51;
      fVar40 = fVar64 * (fVar69 - fVar40) + fVar40;
      fVar58 = fVar64 * (fVar67 - fVar58) + fVar58;
      lVar23 = *(longlong *)pfVar20;
      *(float *)(lVar23 + 0x68) = fVar40;
      *(float *)(lVar23 + 0x6c) = fVar51;
      *(float *)(lVar23 + 0x70) = fVar58;
      if (*(char *)(lVar23 + 0x74) == '\0') {
        *(undefined1 *)(lVar23 + 0x74) = 1;
      }
      fVar41 = _DAT_14f0a8580;
      fVar51 = fVar51 - fStack_13c;
      auVar54._0_4_ = fVar40 - fStack_140;
      auVar54._4_4_ = fVar58 - fStack_138;
      auVar54._8_4_ = (fVar64 * 0.0 + 0.0) - 0.0;
      auVar54._12_4_ = (fVar64 * 0.0 + 0.0) - 0.0;
      auVar46._0_4_ =
           SQRT(auVar54._4_4_ * auVar54._4_4_ + fVar51 * fVar51 + auVar54._0_4_ * auVar54._0_4_);
      fVar67 = 0.0;
      if (_DAT_14f07eaf0 <= auVar46._0_4_) {
        fVar67 = fVar51 / auVar46._0_4_;
      }
      auVar46._4_4_ = auVar46._0_4_;
      auVar46._8_4_ = auVar46._0_4_;
      auVar46._12_4_ = auVar46._0_4_;
      auVar55 = divps(auVar54,auVar46);
      uVar19 = (uint)(auVar46._0_4_ < _DAT_14f07eaf0);
      atan2f(auVar46._0_4_ < _DAT_14f07eaf0,
             ~CONCAT44((int)(uVar19 << 0x1f) >> 0x1f,(int)(uVar19 << 0x1f) >> 0x1f) & auVar55._0_8_)
      ;
      atan2f();
      auVar47._0_8_ = fmodf();
      auVar47._8_8_ = extraout_XMM0_Qb_00;
      fVar69 = (float)auVar47._0_8_;
      if (fVar69 <= _DAT_14efd2470) {
        auVar47._0_4_ = fVar69 + fVar41;
      }
      else if (_DAT_14ef3e024 < fVar69) {
        auVar47._0_4_ = fVar69 + _DAT_14f2040b0;
      }
      if (((float)(auVar47._0_4_ & _DAT_14eee6c10) < _DAT_14f25fba4) &&
         (lVar23 = *(longlong *)pfVar20, *(char *)(lVar23 + 0x50) == '\x01')) {
        _auStack_108 = auVar47;
        (*(code *)PTR__guard_dispatch_icall_14f99ef70)(param_3,&lStack_258);
        lVar34 = lStack_258;
        if (lStack_258 == 0) {
          iVar37 = 1;
        }
        else {
          lStack_130 = CONCAT44(lStack_130._4_4_,fVar67);
          pfStack_170 = *(float **)(lVar23 + 0x48);
          uStack_168 = *(ulonglong *)(lVar23 + 0x30);
          uVar38 = *(ulonglong *)(lVar23 + 0x40);
          pfStack_128 = pfVar20;
          FUN_14eec62d0(&uStack_690,0,0x408);
          if (uVar38 < 0x400) {
            uVar28 = lVar23 + 0x30;
            if ((float *)0xf < pfStack_170) {
              uVar28 = uStack_168;
            }
            FUN_14eec62c0(&uStack_690,uVar28,uVar38);
            uStack_290 = uVar38;
            *(undefined1 *)((longlong)&uStack_690 + uVar38) = 0;
            uStack_260 = uStack_290;
          }
          else {
            uStack_260 = 0;
          }
          puStack_268 = &uStack_690;
          FUN_1403946b0(&puStack_220,&puStack_268);
          in_stack_fffffffffffff910 = in_stack_fffffffffffff910 & 0xffffffffffffff00;
          uVar74 = 0;
          FUN_14541f580(lVar34 + 0x18,&plStack_1e8,&puStack_220,0,0,in_stack_fffffffffffff910);
          if (0xf < uStack_200) {
            uVar38 = uStack_200 + 1;
            pfVar20 = pfStack_218;
            if (0xfff < uVar38) {
              pfVar20 = *(float **)(pfStack_218 + -2);
              if (0x1f < (ulonglong)((longlong)pfStack_218 + (-8 - (longlong)pfVar20)))
              goto LAB_145747412;
              uVar38 = uStack_200 + 0x28;
            }
            FUN_14e934b00(pfVar20,uVar38);
          }
          plVar22 = plStack_1e8;
          if (plStack_1e8 == (longlong *)0x0) {
            plVar22 = (longlong *)0x1529bf210;
          }
          if (*plVar22 == 0) {
            iVar37 = 3;
          }
          else {
            lVar23 = *(longlong *)pfStack_128;
            puVar31 = (ulonglong *)(lVar23 + 0x18);
            if (*(char *)(lVar23 + 0x28) == '\0') {
              puVar31 = (ulonglong *)&UNK_14f0feaa0;
            }
            uStack_168 = *puVar31;
            uStack_160 = puVar31[1];
            fVar67 = *(float *)(lVar23 + 0x5c) * _DAT_14ef97d80;
            fVar69 = *(float *)(lVar23 + 0x58) * _DAT_14ef97d80 + fVar68;
            fVar41 = fVar2 + fVar69 + _DAT_14ef3e01c;
            uVar13 = auStack_108._4_4_;
            uVar14 = uStack_100;
            uVar15 = uStack_fc;
            fVar51 = (float)auStack_108._0_4_ / _DAT_14f25fba4;
            (*(code *)PTR__guard_dispatch_icall_14f99ef70)(param_3);
            FUN_1457e1da0(&uStack_690,lVar5,param_3);
            fVar67 = fVar67 + fVar68;
            fVar51 = fVar51 * _DAT_14eee6c00 * (fVar3 - fVar41) + fVar4 + pfStack_148._0_4_;
            auVar62._4_4_ = uVar13;
            auVar62._0_4_ = fVar51;
            auVar62._8_4_ = uVar14;
            auVar62._12_4_ = uVar15;
            auVar63._4_12_ = auVar62._4_12_;
            auVar63._0_4_ = fVar51 - fVar69 * _DAT_14eee6c00;
            auVar9._8_8_ = uStack_160;
            auVar9._0_8_ = uStack_168;
            auVar55 = maxps(ZEXT416(0),auVar9);
            auVar55 = minps(_DAT_14ef3fde0,auVar55);
            auVar56._0_4_ = (int)(auVar55._0_4_ * _DAT_14f0827e0);
            auVar56._4_4_ = (int)(auVar55._4_4_ * _UNK_14f0827e4);
            auVar56._8_4_ = (int)(auVar55._8_4_ * _UNK_14f0827e8);
            auVar56._12_4_ = (int)(auVar55._12_4_ * _UNK_14f0827ec);
            auVar59._4_4_ = auVar56._4_4_;
            auVar59._0_4_ = auVar56._4_4_;
            auVar59._8_4_ = auVar56._12_4_;
            auVar59._12_4_ = auVar56._12_4_;
            auVar55._0_8_ = CONCAT44(uVar74,auStack_248._0_4_ - fVar67 * _DAT_14eee6c00);
            in_stack_fffffffffffff910 = in_stack_fffffffffffff910 & 0xffffffff00000000;
            FUN_14598a9d0(uStack_630,&uStack_690,&plStack_1e8,auVar63._0_8_,auVar55._0_8_,
                          in_stack_fffffffffffff910,0x3f800000,0,0x3f800000,fVar69,fVar67,0x3f800000
                          ,*(undefined4 *)(param_1 + 8),
                          (int)((auVar59._8_8_ & 0xffffffff) * (ulonglong)_UNK_14f082808) +
                          auVar56._4_4_ * _DAT_14f082800 +
                          (int)((auVar56._8_8_ & 0xffffffff) * (ulonglong)_UNK_14f0827f8) +
                          auVar56._0_4_ * _DAT_14f0827f0,0xffffffff,0x3f800000,0x3f800000);
            uVar74 = (undefined4)((ulonglong)auVar55._0_8_ >> 0x20);
            fVar41 = (float)cosf();
            fVar67 = (float)sinf();
            fVar69 = fVar41 * (float)auStack_1a8._0_4_ + fVar67 * (float)auStack_1b8._0_4_;
            auVar72._4_4_ = fVar41 * (float)auStack_1a8._4_4_ - fVar67 * (float)auStack_1b8._4_4_;
            auVar72._0_4_ = fVar69;
            auVar72._8_4_ = fVar41 * fStack_1a0 + fVar67 * fStack_1b0;
            auVar72._12_4_ = fVar41 * fStack_19c + fVar67 * fStack_1ac;
            auVar48._0_4_ = SQRT(auVar72._4_4_ * auVar72._4_4_ + fStack_134 + fVar69 * fVar69);
            auVar8 = stack0xfffffffffffffefc;
            auVar11._12_4_ = 0;
            auVar11._0_12_ = stack0xfffffffffffffefc;
            _auStack_108 = auVar11 << 0x20;
            if (_DAT_14f07eaf0 <= auVar48._0_4_) {
              auVar48._4_4_ = auVar48._0_4_;
              auVar48._8_4_ = auVar48._0_4_;
              auVar48._12_4_ = auVar48._0_4_;
              auVar55 = divps(auVar72,auVar48);
              auStack_108._0_4_ =
                   SQRT(auVar55._4_4_ * auVar55._4_4_ + auVar55._0_4_ * auVar55._0_4_);
              _auStack_108 = CONCAT124(auVar8,auStack_108._0_4_);
            }
            atan2f();
            atan2f();
            auVar55._0_8_ = fmodf();
            auVar55._8_8_ = extraout_XMM0_Qb_01;
            fVar41 = (float)auVar55._0_8_;
            auVar49._4_12_ = auVar55._4_12_;
            if (fVar41 <= _DAT_14efd2470) {
              auVar50._0_4_ = fVar41 + _DAT_14f0a8580;
              auVar50._4_12_ = auVar49._4_12_;
              auVar55._0_8_ = auVar50._0_8_;
            }
            else if (_DAT_14ef3e024 < fVar41) {
              auVar49._0_4_ = fVar41 + _DAT_14f2040b0;
              auVar55._0_8_ = auVar49._0_8_;
            }
            if ((float)auVar55._0_8_ <= fStack_238) {
              fStack_154 = (fVar51 + 0.0) - fVar66;
              uStack_150 = auStack_288._0_4_;
              uStack_14c = 0;
              puStack_220 = (undefined *)0x1;
              pfStack_218 = &fStack_154;
              in_stack_fffffffffffff910 = 0;
              FUN_140e9b010(lVar5,param_1 + 0x340,param_1 + 0x40,&puStack_220,
                            CONCAT44(uVar74,*(undefined4 *)(param_1 + 8)),0);
            }
            else if ((float)auStack_1c8._0_4_ <= (float)auVar55._0_8_) {
              fStack_154 = fVar51 - fVar66;
              uStack_150 = auStack_278._0_4_;
              uStack_14c = 0;
              puStack_220 = (undefined *)0x1;
              pfStack_218 = &fStack_154;
              in_stack_fffffffffffff910 = 0;
              FUN_140e9b010(lVar5,param_1 + 0xe0,param_1 + 0x20,&puStack_220,
                            CONCAT44(uVar74,*(undefined4 *)(param_1 + 8)),0);
            }
            FUN_1457e2220(&uStack_690);
            iVar37 = 0;
          }
          lVar23 = lStack_1d0;
          if (lStack_1d0 != 0) {
            LOCK();
            piVar1 = (int *)(lStack_1d0 + 8);
            *piVar1 = *piVar1 + -1;
            UNLOCK();
            if (*piVar1 == 0) {
              (*(code *)PTR__guard_dispatch_icall_14f99ef70)(lStack_1d0);
              LOCK();
              piVar1 = (int *)(lVar23 + 0xc);
              *piVar1 = *piVar1 + -1;
              UNLOCK();
              if (*piVar1 == 0) {
                (*(code *)PTR__guard_dispatch_icall_14f99ef70)(lVar23);
              }
            }
          }
          pfVar39 = pfStack_1e0;
          pfVar20 = pfStack_128;
          if (pfStack_1e0 != (float *)0x0) {
            LOCK();
            pfVar24 = pfStack_1e0 + 2;
            *pfVar24 = (float)((int)*pfVar24 + -1);
            UNLOCK();
            if (*pfVar24 == 0.0) {
              (*(code *)PTR__guard_dispatch_icall_14f99ef70)(pfStack_1e0);
              LOCK();
              pfVar24 = pfVar39 + 3;
              *pfVar24 = (float)((int)*pfVar24 + -1);
              UNLOCK();
              pfVar20 = pfStack_128;
              if (*pfVar24 == 0.0) {
                (*(code *)PTR__guard_dispatch_icall_14f99ef70)(pfVar39);
                pfVar20 = pfStack_128;
              }
            }
          }
        }
        lVar23 = lStack_250;
        if (lStack_250 != 0) {
          LOCK();
          piVar1 = (int *)(lStack_250 + 8);
          *piVar1 = *piVar1 + -1;
          UNLOCK();
          if (*piVar1 == 0) {
            (*(code *)PTR__guard_dispatch_icall_14f99ef70)(lStack_250);
            LOCK();
            piVar1 = (int *)(lVar23 + 0xc);
            *piVar1 = *piVar1 + -1;
            UNLOCK();
            if (*piVar1 == 0) {
              (*(code *)PTR__guard_dispatch_icall_14f99ef70)(lVar23);
            }
          }
        }
        pfVar39 = pfStack_120;
        if ((iVar37 != 3) && (iVar37 != 0)) break;
      }
      pfVar20 = pfVar20 + 2;
    } while (pfVar20 != pfVar39);
  }
LAB_1457471b7:
  if (pfStack_188 != (float *)0x0) {
    uVar38 = (longlong)pfStack_178 - (longlong)pfStack_188;
    pfVar20 = pfStack_188;
    if (0xfff < uVar38) {
      pfVar20 = *(float **)(pfStack_188 + -2);
      if (0x1f < (ulonglong)((longlong)pfStack_188 + (-8 - (longlong)pfVar20))) {
LAB_145747412:
        _invoke_watson(0,0,0,0,0);
        pcVar7 = (code *)swi(3);
        (*pcVar7)();
        return;
      }
      uVar38 = uVar38 + 0x27;
    }
    FUN_14e934b00(pfVar20,uVar38);
  }
  return;
}

